use colored::Colorize;
use difflib::get_close_matches;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use utils::classes::package::Package;
use utils::handle_error::handle_error_and_exit;
use zip::ZipArchive;
use mslnk::ShellLink;

pub async fn portable_installer(
    package: Package,
    update: bool,
    no_color: bool,
    no_progress: bool,
    max_size: u64,
    multi: bool,
) {
    let package_name = package.package_name;
    let display_name = package.display_name;
    let threads = package.threads;
    let latest_version = package.latest_version;
    let exec_name = package.exec_name;

    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate appdata directory".to_string())
    });

    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| {
        handle_error_and_exit("Failed to locate user profile directory".to_string())
    });

    let tools_dir = Path::new(&user_profile).join("novus").join("tools");

    if !tools_dir.exists() {
        fs::create_dir(tools_dir.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    }

    let shims_dir = Path::new(&user_profile).join("novus").join("shims");

    if !shims_dir.exists() {
        fs::create_dir(shims_dir.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    }

    let mut desired_version = "0".to_string();
    let mut max = true;

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
                    display_name.clone()
                ))
            });
    } else {
        desired_version = latest_version.to_string();
    }
    if package.versions[&desired_version.to_string()].size != max_size || no_progress {
        max = false;
    }

    let output = format!(
        r"{}\novus\{}@{}.zip",
        appdata, package_name, desired_version
    );

    let url = package.versions[&desired_version.to_string()].url.clone();

    if !Path::new(&output).exists() {
        threadeddownload(
            url,
            output.clone(),
            threads,
            package_name.clone(),
            max,
            no_color,
        )
        .await;
    }

    let zip_file = File::open(output.clone()).unwrap();

    let package_version = format!("{}@{}", package_name, latest_version);

    let extract_dir = tools_dir.join(package_version);

    extract_file(zip_file, no_color, multi, display_name.clone(), extract_dir.clone()).await;

    let (shim, copy_dir): (PathBuf, PathBuf) = check_shims(shims_dir, extract_dir, exec_name);

    if !copy_dir.exists() {
        File::create(copy_dir.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let content = format!("@echo off \n \"{}\" %*", shim.display().to_string());
        fs::write(copy_dir, content).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    }

    let star_menu_loc = format!(r"{}\Microsoft\Windows\Start Menu\Programs\Novus", appdata);
    let start_menu_dir = Path::new(&star_menu_loc);
    if !start_menu_dir.exists() {
        fs::create_dir(start_menu_dir.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    }

    let shortcut_loc = start_menu_dir.join(format!("{}.lnk", display_name.clone())); 

    let sl = ShellLink::new(shim).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    sl.create_lnk(shortcut_loc).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
}

fn check_shims(
    shims_dir: PathBuf,
    extract_dir: PathBuf,
    mut exec_name: String,
) -> (PathBuf, PathBuf) {
    let mut files: Vec<String> = vec![];
    let mut path_initial: &PathBuf = &extract_dir.clone();

    let mut paths: Vec<PathBuf> = vec![];
    let mut filepaths: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(extract_dir).unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
    {
        let entry = entry.unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let path_temp = entry.path();
        paths.push(path_temp.clone());
        if path_temp.is_file() {
            filepaths.push(path_temp);
        }
    }

    if paths.len() == 1 && filepaths.len() == 0 {
        path_initial = &paths[0];
    }

    let mut path: PathBuf = path_initial.to_owned();

    if exec_name.contains("\\") {
        let path_split: Vec<&str> = exec_name.split("\\").collect();
        for i in 0..path_split.len() - 1 {
            path = path_initial.join(path_split[i]);
        }
        exec_name = path_split[path_split.len() - 1].to_string();
    }    

    for entry in fs::read_dir(path.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
    {
        let entry = entry.unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let display = entry.path().display().to_string();
        let file: Vec<&str> = display.split(r"\").collect();
        let name: &str = file[file.len() - 1];
        files.push(name.to_string());
    }

    let mut files_str: Vec<&str> = vec![];

    for i in 0..files.len() {
        files_str.push(&files[i]);
    }

    let compare_file = exec_name.clone() + ".exe";

    let file: &str = get_close_matches(&compare_file, files_str.clone(), 1, 0.0)[0];

    // println!("compare: {}\nfiles: {:?}\nmatch: {}", compare_file, files_str.clone(), file);

    let shim: PathBuf = path.join(file);

    let copy_file = format!("{}.bat", exec_name);

    let copy_dir: PathBuf = shims_dir.join(copy_file);

    // println!("shim: {}\ncopy_dir: {}", shim.display().to_string(), copy_dir.display().to_string());

    (shim, copy_dir)
}

#[allow(unused_assignments)]
async fn extract_file(
    zip_file: File,
    no_color: bool,
    multi: bool,
    display_name: String,
    extract_dir: PathBuf,
) {
    let progress_bar = ProgressBar::new(0);
    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    let mut text = String::new();
    if multi {
        if no_color {
            text = format!("Extracting Packages");
        } else {
            text = format!("{}", "Extracting Packages".bright_cyan());
        }
    } else {
        if no_color {
            text = format!("Extracting {}", display_name)
        } else {
            text = format!(
                "{}{}",
                "Extracting ".bright_cyan(),
                display_name.bright_cyan()
            )
        }
    }

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
        let mut archive =
            ZipArchive::new(zip_file).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

        archive
            .extract(&extract_dir)
            .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
    });

    let _ = cmd.await;
    completed.store(true, Ordering::Relaxed);
    let _ = handle.await;
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

async fn threadeddownload(
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
                    .template(("Downloading".to_string() + " [{wide_bar:.white}] {bytes}/{total_bytes}").as_str())
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
