use crate::classes::package::Package;
use crate::utils::{cache, checksum, get_package, handle_error};
use cache::check_cache;
use checksum::verify_checksum;
use colored::Colorize;
use get_package::get_package;
use handle_error::handle_error_and_exit;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufReader, BufWriter, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs::File, u64};
// use std::time::Instant;

pub async fn installer(packages: Vec<String>, flags: Vec<String>) {
    let mut no_progress = false;
    let mut no_color = false;
    if flags.contains(&"--no-color".to_string()) || flags.contains(&"-nc".to_string()) {
        no_color = true;
    }
    if flags.contains(&"--no-progress".to_string()) || flags.contains(&"-np".to_string()) {
        no_progress = true;
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
        if no_color {
            println!("Installing Packages");
        } else {
            println!("{}", "Installing Packages".bright_green());
        }
    }
    let start = std::time::Instant::now();
    for pkg in packages.iter() {
        let mut max = true;
        let pkg_split: Vec<&str> = pkg.split("@").collect();
        let mut pkg_name = pkg.as_str();
        let mut desired_version = "0";
        if pkg_split.len() == 2 {
            pkg_name = pkg_split[0];
            desired_version = pkg_split[1];
        }
        let pkg_clone = pkg_name.clone();
        let package: Package = get_package(pkg_clone).await;
        let latest_version = package.latest_version;
        if desired_version == "0" {
            desired_version = latest_version.as_str();
        }
        let display_name = package.display_name;
        let threads = package.threads;
        if multi == false {
            if no_color {
                println!("Installing {}", display_name);
            } else {
                println!(
                    "{} {}",
                    "Installing".bright_green(),
                    display_name.bright_green()
                );
            }
        }
        package
            .versions
            .get(&desired_version.to_string())
            .unwrap_or_else(|| {
                handle_error_and_exit("That version does not exist yet".to_string())
            });
        let url = package.versions[&desired_version.to_string()].url.clone();
        let checksum = package.versions[&desired_version.to_string()]
            .checksum
            .clone();
        let file_type = package.versions[&desired_version.to_string()]
            .file_type
            .clone();
        let iswitch = package.iswitches.clone();
        let temp = std::env::var("TEMP")
            .unwrap_or_else(|e| handle_error_and_exit(format!("{} install.rs:50", e.to_string())));
        let package_name = package.package_name;
        let loc = format!(
            r"{}\novus\{}@{}{}",
            temp, package_name, desired_version, file_type
        );
        if package.versions[&desired_version.to_string()].size != max_size {
            max = false;
        }
        let exists = check_cache(
            package_name.clone(),
            desired_version.to_string().clone(),
            file_type.clone(),
        );
        if no_progress {
            max = false
        }
        handles.push(tokio::spawn(async move {
            if !exists {
                threadeddownload(
                    url,
                    loc.clone(),
                    threads,
                    package_name,
                    checksum,
                    true,
                    max,
                    no_color,
                )
                .await;
            }
            install(
                &iswitch,
                loc.clone(),
                display_name,
                multi,
                no_color,
                file_type,
            )
            .await;
        }));
    }

    futures::future::join_all(handles).await;
    if no_color {
        println!("Successfully installed packages");
    } else {
        println!("{}", "Successfully installed packages".bright_magenta());
    }
    println!("Completed in {:?}", start.elapsed());
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
    checksum: String,
    get_checksum: bool,
    max: bool,
    no_color: bool,
) {
    // let start = Instant::now();
    let mut handles = vec![];
    let res = reqwest::get(url.to_string())
        .await
        .unwrap_or_else(|_| handle_error_and_exit("Failed to get download url!".to_string()));
    let total_length = res
        .content_length()
        .unwrap_or_else(|| handle_error_and_exit("An Unexpected Error Occured!".to_string()));
    let temp = std::env::var("TEMP")
        .unwrap_or_else(|e| handle_error_and_exit(format!("{} install.rs:106", e.to_string())));

    if max {
        let progress_bar = ProgressBar::new(total_length);
        if no_color {
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{wide_bar:.white}] {bytes}/{total_bytes} ({eta})",
                    )
                    .progress_chars("=>-"),
            );
        } else {
            progress_bar.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue/magenta}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("=>-"));
        }

        for index in 0..threads {
            let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index + 1);
            let (start, end) = get_splits(index + 1, total_length, threads);
            let pb = progress_bar.clone();
            let mut file = BufWriter::new(File::create(loc).unwrap_or_else(|e| {
                handle_error_and_exit(format!("{} install.rs:119", e.to_string()))
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
                        handle_error_and_exit(format!("{} install.rs:129", e.to_string()))
                    });

                while let Some(chunk) = response.chunk().await.unwrap_or_else(|e| {
                    handle_error_and_exit(format!("{} install.rs:134", e.to_string()))
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
            let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index + 1);
            let (start, end) = get_splits(index + 1, total_length, threads);
            let mut file = BufWriter::new(File::create(loc).unwrap_or_else(|e| {
                handle_error_and_exit(format!("{} install.rs:150", e.to_string()))
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
                        handle_error_and_exit(format!("{} install.rs:160", e.to_string()))
                    });
                while let Some(chunk) = response.chunk().await.unwrap_or_else(|e| {
                    handle_error_and_exit(format!("{} install.rs:164", e.to_string()))
                }) {
                    let _ = file.write(&*chunk);
                }
            }));
        }

        futures::future::join_all(handles).await;
    }

    let mut file = File::create(output.clone())
        .unwrap_or_else(|e| handle_error_and_exit(format!("{} install.rs:175", e.to_string())));

    let temp = std::env::var("TEMP").unwrap();

    for index in 0..threads {
        let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index + 1);
        let mut buf: Vec<u8> = vec![];
        let downloaded_file = File::open(loc.clone())
            .unwrap_or_else(|e| handle_error_and_exit(format!("{} install.rs:183", e.to_string())));
        let mut reader = BufReader::new(downloaded_file);
        let _ = std::io::copy(&mut reader, &mut buf);
        let _ = file.write_all(&buf);
        let _ = std::fs::remove_file(loc);
    }

    if get_checksum {
        verify_checksum(output, checksum, no_color);
    }

    // println!("download time: {:?}", start.elapsed());
}

#[allow(unused)]
pub async fn install(
    iswitch: &Vec<String>,
    output_file: String,
    display_name: String,
    multi: bool,
    no_color: bool,
    file_type: String,
) {
    let progress_bar = ProgressBar::new(0);
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
        while !completed_clone.load(Ordering::Relaxed) {
            progress_bar.inc(5);
            tokio::time::sleep(std::time::Duration::from_millis(100));
        }
        progress_bar.finish_and_clear();
    });

    let cmd = tokio::spawn(async move {
        if file_type == ".exe" {
            let _cmd = std::process::Command::new(output_file.clone())
                .args(switch)
                .spawn()
                .unwrap_or_else(|e| {
                    handle_error_and_exit(format!("{} install.rs:227", e.to_string()))
                })
                .wait_with_output()
                .unwrap_or_else(|e| {
                    handle_error_and_exit(format!("{} install.rs:229", e.to_string()))
                });
        }
        if file_type == ".msi" {
            let _cmd = std::process::Command::new("MsiExec")
                .args(&["/i", output_file.clone().as_str(), "/passive"])
                .spawn()
                .unwrap_or_else(|e| {
                    handle_error_and_exit(format!("{} install.rs:227", e.to_string()))
                })
                .wait_with_output()
                .unwrap_or_else(|e| {
                    handle_error_and_exit(format!("{} install.rs:229", e.to_string()))
                });
        }
    });

    let _ = cmd.await;
    completed.store(true, Ordering::Relaxed);
    let _ = handle.await;
}
