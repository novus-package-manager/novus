use crate::commands::install_portable::portable_installer;
use cache::check_cache;
use checksum::verify_checksum;
use colored::Colorize;
use get_package::get_package;
use handle_error::handle_error_and_exit;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{copy, read_dir, remove_dir_all, remove_file, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::u64;
use utils::autoelevate::autoelevateinstall;
use utils::classes::package::Package;
use utils::registry::check_installed;
use utils::{cache, checksum, get_package, handle_error};
use zip::ZipArchive;
use utils::classes::config::Config;

pub async fn installer(inital_packages: Vec<String>, package_list: Vec<&str>, flags: Vec<String>, update: bool, config: Config) -> i32 {
    let mut no_progress = config.no_progress;
    let mut no_color = config.no_color;
    let mut confirm = config.confirm;
    let mut portable_flag = config.portable;
    let mut multithreaded = config.multithreaded;
    let mut installpath: String = config.installpath;
    if flags.contains(&"--no-color".to_string()) || flags.contains(&"-nc".to_string()) {
        no_color = true;
    }
    if flags.contains(&"--no-progress".to_string()) || flags.contains(&"-np".to_string()) {
        no_progress = true;
    }
    if flags.contains(&"--yes".to_string()) || flags.contains(&"-y".to_string()) {
        confirm = true;
    }
    if flags.contains(&"--portable".to_string()) || flags.contains(&"-p".to_string()) {
        portable_flag = true;
    }
    if flags.contains(&"--multithreaded".to_string()) || flags.contains(&"-m".to_string()) {
        multithreaded = true;
    }

    let mut packages: Vec<String> = inital_packages.clone();

    if portable_flag {
        for pkg in inital_packages {
            let mut name: &str = &pkg;
            let mut version: &str = "0";

            if pkg.contains("@") {
                let version_split: Vec<&str> = pkg.split("@").collect();
                name = version_split[0];
                version = version_split[1];
            }

            let pkg_portable = name.clone().to_string() + "-portable";
            let pkg_portable_version = name.clone().to_string() + "-portable@" + version;
            let index = packages.iter().position(|x| *x == pkg.clone()).unwrap();
            packages.remove(index);

            if package_list.contains(&pkg_portable.as_str()) {
                packages.push(pkg_portable_version.to_string())
            }
            else {
                println!("{} {}", "Couldn't find a portable package for".bright_red(), pkg_portable.bright_red());
                if packages.len() == 0 {
                    process::exit(1);
                }
            }
        }
    }

    let mut handles = vec![];
    let mut sizes = vec![];
    let mut multi = false;
    for pkg in packages.iter() {
        let pkg_split: Vec<&str> = pkg.split("@").collect();
        let mut pkg_name = pkg.as_str();
        if pkg_split.len() == 2 {
            pkg_name = pkg_split[0];
        }
        let package: Package = get_package(pkg_name).await;
        sizes.push(package.versions[&package.latest_version].size);
    }
    let mut max_size = sizes[0];
    for i in 0..sizes.len() {
        if sizes[i] > max_size {
            max_size = sizes[i];
        }
    }
    if sizes.len() > 1 {
        multi = true;
    }

    println!("{}", "Installing Packages".bright_green());

    for pkg in packages.iter() {
        let mut max = true;
        let pkg_split: Vec<&str> = pkg.split("@").collect();
        let mut pkg_name = pkg.as_str();
        let mut desired_version = "0".to_string();
        if pkg_split.len() == 2 {
            pkg_name = pkg_split[0];
            desired_version = pkg_split[1].to_string();
        }
        let pkg_clone = pkg_name.clone();
        let package: Package = get_package(pkg_clone).await;

        let portable = package.portable;

        if no_progress {
            max = false
        }

        if portable == Some(true) {
            portable_installer(package, update, no_color, no_progress, max_size, multi, desired_version).await;
        } else {
            if !confirm && !update {
                if check_installed(package.display_name.clone()) {
                    print!(
                        "{} is already installed on your system. Do you want to reinstall it? (Y/N): ",
                        package.display_name
                    );
                    std::io::stdout()
                        .flush()
                        .ok()
                        .expect("Could not flush stdout");
                    let mut string: String = String::new();
                    let _ = std::io::stdin().read_line(&mut string);
                    if string.trim().to_lowercase() != "y" {
                        continue;
                    }
                }
            }
            let latest_version = package.latest_version;
            let display_name = package.display_name;
            let mut threads = 1;
            if multithreaded {
                threads = package.threads;
            }
            if !update {
                if desired_version == "0" {
                    desired_version = latest_version.to_string();
                }
                package
                    .versions
                    .get(&desired_version.to_string())
                    .unwrap_or_else(|| {
                        handle_error_and_exit(format!(
                            "That version of {} does not exist yet",
                            pkg_clone
                        ))
                    });
            } else {
                desired_version = latest_version.to_string();
            }
            let url = package.versions[&desired_version.to_string()].url.clone();
            let checksum = package.versions[&desired_version.to_string()]
                .checksum
                .clone();
            let file_type = package.versions[&desired_version.to_string()]
                .file_type
                .clone();
            let iswitch = package.iswitches.clone();
            let appdata = std::env::var("APPDATA").unwrap_or_else(|e| {
                handle_error_and_exit(e.to_string())
            });
            let package_name = package.package_name;
            let mut loc = format!(
                r"{}\novus\{}@{}{}",
                appdata, package_name, desired_version, file_type
            );
            if package.versions[&desired_version.to_string()].size != max_size {
                max = false;
            }
            let mut executable_type = file_type.clone();
            if file_type == ".zip" {
                executable_type = ".exe".to_string();
            }
            let exists = check_cache(
                package_name.clone(),
                desired_version.to_string().clone(),
                executable_type.clone(),
            );
            handles.push(tokio::spawn(async move {
                if !exists {
                    let mut new_loc = loc.clone();
                    if url.contains(".zip") {
                        new_loc = loc.replace(".exe", ".zip").replace(".msi", ".zip");
                    }
                    threadeddownload(
                        url.clone(),
                        new_loc.clone(),
                        threads,
                        package_name.clone(),
                        max,
                        no_color,
                    )
                    .await;
                }
                if !url.contains(".zip") {
                    if checksum.clone() != "any" {
                        if !verify_checksum(loc.clone(), checksum.clone()) {
                            println!("{}", "Clearing cache and retrying".bright_blue());
                            utils::cache::clear_cache_for_package(&package_name);
                            threadeddownload(
                                url.clone(),
                                loc.clone(),
                                threads,
                                package_name.clone(),
                                max,
                                no_color,
                            )
                            .await;
                            if !verify_checksum(loc.clone(), checksum.clone()) {
                                println!(
                                    "{} {}",
                                    "Failed to Install".bright_red(),
                                    display_name.bright_red()
                                );
                                process::exit(1);
                            }
                        }
                    }
                    else {
                        if !confirm {
                            print!(
                                "Cannot verify checksum for {}. Do you want to continue with the installation? (Y/N): ",
                                display_name
                            );
                            std::io::stdout()
                                .flush()
                                .ok()
                                .expect("Could not flush stdout");
                            let mut string: String = String::new();
                            let _ = std::io::stdin().read_line(&mut string);
                            if string.trim().to_lowercase() != "y" {
                                process::exit(0);
                            }
                        }                        
                    }
                }

                if url.contains(".zip") {
                    if !Path::new(&loc).exists() {
                        let new_loc = loc.replace(".exe", ".zip").replace(".msi", ".zip");
                        loc = extract_file(
                            new_loc.clone(),
                            appdata,
                            package_name.clone(),
                            desired_version.clone(),
                        );
                    }
                    if !verify_checksum(loc.clone(), checksum.clone()) {
                        println!(
                            "{} {}",
                            "Failed to Install".bright_red(),
                            display_name.bright_red()
                        );
                        process::exit(1);
                    }
                }

                let code: i32 = install(
                    &iswitch,
                    loc.clone(),
                    display_name,
                    package_name,
                    desired_version,
                    multi,
                    no_color,
                    file_type,
                )
                .await;
                code
            }));
        }
    }

    let code_arr = futures::future::join_all(handles).await;
    let mut codes: Vec<i32> = vec![];
    for code_element in code_arr {
        codes.push(
            code_element.unwrap_or_else(|_| {
                handle_error_and_exit("Failed to retrieve exit code".to_string())
            }),
        )
    }

    if codes.contains(&1) {
        return 1;
    }

    0
}

fn extract_file(
    loc: String,
    appdata: String,
    package_name: String,
    desired_version: String,
) -> String {
    // Extract exe from package

    let zip_file = File::open(loc.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let mut archive =
        ZipArchive::new(zip_file).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let extract_dir = format!(r"{}\novus\{}@{}", appdata, package_name, desired_version);

    archive
        .extract(&extract_dir)
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let mut path: String = String::new();

    for entry in
        read_dir(Path::new(&extract_dir)).unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
    {
        let entry = entry.unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        path = entry.path().display().to_string();
    }

    let mut filetype = ".exe";

    if path.contains(".msi") {
        filetype = ".msi";
    }

    let copy_dir = format!(
        r"{}\novus\{}@{}{}",
        appdata, package_name, desired_version, filetype
    );

    copy(path, copy_dir.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    remove_dir_all(extract_dir).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    remove_file(loc.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    copy_dir
}

#[allow(unused)]
fn get_splits(i: u64, total_length: u64, threads: u64) -> (u64, u64) {
    let mut start = ((total_length / threads) * (i - 1)) + 1;
    let mut end = (total_length / threads) * i;

    if i == 1 {
        start = 0;
    }

    if i == threads {
        end = total_length;
    }

    (start, end)
}

pub async fn threadeddownload(
    url: String,
    output: String,
    threads: u64,
    package_name: String,
    max: bool,
    no_color: bool,
) {
    let mut handles = vec![];
    let res = reqwest::get(url.to_string())
        .await
        .unwrap_or_else(|_| handle_error_and_exit("Failed to get download url!".to_string()));
    let total_length = res
        .content_length()
        .unwrap_or_else(|| handle_error_and_exit("An Unexpected Error Occured!".to_string()));
    let appdata = std::env::var("APPDATA")
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    if max {
        let progress_bar = ProgressBar::new(total_length);
        if no_color {
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template(("Downloading".bright_cyan().to_string() + " [{wide_bar:.white}] {bytes}/{total_bytes}").as_str())
                    .progress_chars("=> "),
            );
        } else {
            progress_bar.set_style(ProgressStyle::default_bar()
            .template(("Downloading".bright_cyan().to_string() + " [{wide_bar:.cyan}] {bytes}/{total_bytes}").as_str())
            .progress_chars("=> "));
        }

        for index in 0..threads {
            let loc = format!(r"{}\novus\setup_{}{}.tmp", appdata, package_name, index + 1);
            let (start, end) = get_splits(index + 1, total_length, threads);
            let pb = progress_bar.clone();
            let mut file = BufWriter::new(File::create(loc).unwrap_or_else(|e| {
                handle_error_and_exit(e.to_string())
            }));
            let url = url.clone();
            handles.push(tokio::spawn(async move {
                let client = reqwest::Client::new();
                let mut response = client
                    .get(url)
                    .header("range", format!("bytes={}-{}", start, end))
                    .send()
                    .await
                    .unwrap_or_else(|e| {
                        handle_error_and_exit(e.to_string())
                    });

                while let Some(chunk) = response.chunk().await.unwrap_or_else(|e| {
                    handle_error_and_exit(e.to_string())
                }) {
                    pb.inc(chunk.len() as u64);
                    let _ = file.write(&*chunk);
                }
            }));
        }

        futures::future::join_all(handles).await;

        progress_bar.finish();
    } else {
        for index in 0..threads {
            let loc = format!(r"{}\novus\setup_{}{}.tmp", appdata, package_name, index + 1);
            let (start, end) = get_splits(index + 1, total_length, threads);
            let mut file = BufWriter::new(File::create(loc).unwrap_or_else(|e| {
                handle_error_and_exit(e.to_string())
            }));
            let url = url.clone();
            handles.push(tokio::spawn(async move {
                let client = reqwest::Client::new();
                let mut response = client
                    .get(url)
                    .header("range", format!("bytes={}-{}", start, end))
                    .send()
                    .await
                    .unwrap_or_else(|e| {
                        handle_error_and_exit(e.to_string())
                    });
                while let Some(chunk) = response.chunk().await.unwrap_or_else(|e| {
                    handle_error_and_exit(e.to_string())
                }) {
                    let _ = file.write(&*chunk);
                }
            }));
        }

        futures::future::join_all(handles).await;
    }

    let mut file = File::create(output.clone())
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let appdata = std::env::var("APPDATA").unwrap();

    for index in 0..threads {
        let loc = format!(r"{}\novus\setup_{}{}.tmp", appdata, package_name, index + 1);
        let mut buf: Vec<u8> = vec![];
        let downloaded_file = File::open(loc.clone())
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let mut reader = BufReader::new(downloaded_file);
        let _ = std::io::copy(&mut reader, &mut buf);
        let _ = file.write_all(&buf);
        let _ = std::fs::remove_file(loc);
    }
}

#[allow(unused)]
pub async fn install(
    iswitch: &Vec<String>,
    output_file: String,
    display_name: String,
    package_name: String,
    version: String,
    multi: bool,
    no_color: bool,
    file_type: String,
) -> i32 {
    let progress_bar = ProgressBar::new(0);
    let pb = progress_bar.clone();
    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    let mut text = String::new();
    if multi {
        if no_color {
            text = format!("Installing Packages");
        } else {
            text = format!("{}", "Installing Packages".bright_cyan());
        }
    } else {
        if no_color {
            text = format!("Installing {}", display_name)
        } else {
            text = format!(
                "{}{}",
                "Installing ".bright_cyan(),
                display_name.bright_cyan()
            )
        }
    }

    let switch = iswitch.clone();

    if no_color {
        progress_bar.clone().set_style(
            ProgressStyle::default_spinner()
                .template(("{spinner:.white}".to_string() + format!(" {}", text).as_str()).as_str())
                .tick_chars("┤┘┴└├┌┬┐ "),
        );
    } else {
        progress_bar.clone().set_style(
            ProgressStyle::default_spinner()
                .template(("{spinner:.green}".to_string() + format!(" {}", text).as_str()).as_str())
                .tick_chars("┤┘┴└├┌┬┐ "),
        );
    }

    let handle = tokio::spawn(async move {
        if !multi {
            while !completed_clone.load(Ordering::Relaxed) {
                progress_bar.inc(5);
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
        progress_bar.finish_and_clear();

    });

    let cmd = tokio::spawn(async move { 
        let output;

        if file_type == ".exe" {
            output = process::Command::new(output_file.clone())
                .args(switch.clone())
                .output();
        } else if file_type == ".msi" {
            // let target_dir: &str = r##"TARGETDIR="E:\Program Files""##;
            let target_path = r"E:\Program Files";
            let target_dir = format!(r##"TARGETDIR="{}""##, target_path);
            let args = format!("msiexec /i {} {} /passive", output_file, target_dir);
            println!("args: {}", args);

            let bat_contents = format!(
"@ECHO off
msiexec /i {} {} /passive", output_file, target_dir);

            let temp = std::env::var("TEMP").unwrap_or_else(|_| handle_error_and_exit("Failed to find temp directory".to_string()));
            let loc = format!(r"{}\run_msi.bat", temp);
            let path = std::path::Path::new(loc.as_str());
            let _ = std::fs::File::create(path).unwrap_or_else(|_| handle_error_and_exit("Failed to create bat file".to_string()));
            std::fs::write(path, bat_contents).unwrap_or_else(|_| handle_error_and_exit("Failed to write bat file".to_string()));

            output = std::process::Command::new(path)
                .output();      
                
            std::fs::remove_file(path).unwrap_or_else(|_| handle_error_and_exit("Failed to remove bat file".to_string()));
        } else {
            output = process::Command::new("powershell")
                .arg(output_file.clone())
                .output();
        }

        let mut code = 0;

        let output = output.unwrap_or_else(|e| {
            if e.to_string().contains("requires elevation") {
                let package_ver = package_name + "@" + &version;
                code = autoelevateinstall(package_ver);
                pb.println(format!("{}", "Auto Elevating".bright_cyan()));
                pb.finish_and_clear();
                process::exit(0)
            } else {
                handle_error_and_exit(e.to_string());
            }
        });

        code = output
            .status
            .code()
            .unwrap_or_else(|| handle_error_and_exit("Failed to retrieve exit code".to_string()));
        if code == 1 {
            let error_message = String::from_utf8(output.stderr)
                .unwrap_or("Failed to install packages".to_string());
            install_fail(&error_message, pb);
        } else {
            install_success(pb);
        }

        code
    });

    let code = cmd.await;
    completed.store(true, Ordering::Relaxed);
    let _ = handle.await;

    code.unwrap_or_else(|_| handle_error_and_exit("Failed to retrieve exit code".to_string()))
}

fn install_fail(msg: &str, pb: ProgressBar) {
    pb.println(format!("{}", msg.bright_red()));
    pb.finish_and_clear();
    process::exit(0);
}

fn install_success(pb: ProgressBar) {
    pb.println(format!(
        "{}",
        "Successfully installed packages".bright_magenta()
    ));
    pb.finish_and_clear();
}
