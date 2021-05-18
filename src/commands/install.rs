use crate::utils::{checksum, handle_error, cache, get_package};
use get_package::{get_package};
use crate::classes::package::Package;
use checksum::verify_checksum;
use cache::check_cache;
use colored::Colorize;
use handle_error::handle_error_and_exit;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufReader, BufWriter, Write};
use std::{fs::File, u64};

// use std::time::Instant;

pub fn installer(packages: Vec<String>) {
    let mut handles = vec![];
    let mut sizes = vec![];
    let mut multi = false;
    for pkg in packages.iter() {
        let package: Package = get_package(pkg.as_str());
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
        println!("{}", "Installing Packages".bright_green());
    }
    let start = std::time::Instant::now();
    for pkg in packages.iter() {
        let mut max = true;
        let pkg_clone = pkg.clone();
        let package: Package = get_package(pkg_clone.as_str());
        let latest_version = package.latest_version;
        let display_name = package.display_name;
        let threads = package.threads;
        if multi == false {
            println!(
                "{} {}",
                "Installing".bright_green(),
                display_name.bright_green()
            );
        }
        let url = package.versions[&latest_version].url.clone();
        let checksum = package.versions[&latest_version].checksum.clone();        
        let iswitch = package.iswitches.clone();
        let temp = std::env::var("TEMP").unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let package_name = package.package_name;
        let loc = format!(r"{}\novus\{}@{}.exe", temp, package_name, latest_version);
        if package.versions[&latest_version].size != max_size {
            max = false;
        }
        let exists = check_cache(package_name.clone(), latest_version.clone());
        handles.push(std::thread::spawn(move || {
            if !exists {
                threadeddownload(url, loc.clone(), threads, package_name, checksum, true, max);
            }
            install(&iswitch, loc.clone(), display_name, multi);
        }));
    }
    for handle in handles {
        handle
            .join()
            .unwrap_or_else(|_| handle_error_and_exit("An error occured!".to_string()));
    }
    println!("{}", "Successfully installed packages".bright_magenta());
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

#[tokio::main]
pub async fn threadeddownload(
    url: String,
    output: String,
    threads: u64,
    package_name: String,
    checksum: String,
    get_checksum: bool,
    max: bool,
) {
    // let start = Instant::now();
    let mut handles = vec![];
    let res = reqwest::get(url.to_string())
        .await
        .unwrap_or_else(|_| handle_error_and_exit("Failed to get download url!".to_string()));
    let total_length = res
        .content_length()
        .unwrap_or_else(|| handle_error_and_exit("An Unexpected Error Occured!".to_string()));
    let temp = std::env::var("TEMP").unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    if max {
        let progress_bar = ProgressBar::new(total_length);
        progress_bar.set_style(ProgressStyle::default_bar()
              .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue/magenta}] {bytes}/{total_bytes} ({eta})")
              .progress_chars("=>-"));

        for index in 0..threads {
            let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index + 1);
            let (start, end) = get_splits(index + 1, total_length, threads);
            let pb = progress_bar.clone();
            let mut file = BufWriter::new(
                File::create(loc).unwrap_or_else(|e| handle_error_and_exit(e.to_string())),
            );
            let url = url.clone();
            handles.push(tokio::spawn(async move {
                let client = reqwest::Client::new();
                let mut response = client
                    .get(url)
                    .header("range", format!("bytes={}-{}", start, end))
                    .send()
                    .await
                    .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

                while let Some(chunk) = response
                    .chunk()
                    .await
                    .unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
                {
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
            let mut file = BufWriter::new(
                File::create(loc).unwrap_or_else(|e| handle_error_and_exit(e.to_string())),
            );
            let url = url.clone();
            handles.push(tokio::spawn(async move {
                let client = reqwest::Client::new();
                let mut response = client
                    .get(url)
                    .header("range", format!("bytes={}-{}", start, end))
                    .send()
                    .await
                    .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
                while let Some(chunk) = response
                    .chunk()
                    .await
                    .unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
                {
                    let _ = file.write(&*chunk);
                }
            }));
        }

        futures::future::join_all(handles).await;
    }

    let mut file =
        File::create(output.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    let temp = std::env::var("TEMP").unwrap();

    for index in 0..threads {
        let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index + 1);
        let mut buf: Vec<u8> = vec![];
        let downloaded_file =
            File::open(loc.clone()).unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
        let mut reader = BufReader::new(downloaded_file);
        let _ = std::io::copy(&mut reader, &mut buf);
        let _ = file.write_all(&buf);        
        let _ = std::fs::remove_file(loc);
    }    

    tokio::spawn(async move {
        if get_checksum {
            verify_checksum(output, checksum);
        }
    });

    // delete_temp_cache(package_name, threads);

    // println!("download time: {:?}", start.elapsed());
}

#[allow(unused)]
pub fn install(iswitch: &Vec<String>, output_file: String, display_name: String, multi: bool) {
    let progress_bar = ProgressBar::new(9999999);
    let pb = progress_bar.clone();
    std::thread::spawn(move || {
        let mut text = String::new();
        if multi {
            text = format!("{}", "Installing Packages".bright_cyan());
        } else {
            text = format!(
                "{}{}",
                "Installing ".bright_cyan(),
                display_name.bright_cyan()
            )
        }

        progress_bar.clone().enable_steady_tick(150);

        progress_bar.clone().set_style(
            ProgressStyle::default_spinner()
                .template(("{spinner:.green}".to_string() + format!(" {}", text).as_str()).as_str())
                .tick_chars("┤┘┴└├┌┬┐ "));        
    });
    let _cmd = std::process::Command::new(output_file)
        .arg(iswitch.join(" "))
        .spawn()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()))
        .wait_with_output()
        .unwrap_or_else(|e| handle_error_and_exit(e.to_string()));

    pb.finish_and_clear();
    // process::exit(0);
}
