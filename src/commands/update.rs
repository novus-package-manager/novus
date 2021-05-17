use colored::Colorize;
use crate::utils::{cache, handle_error, get_package};
use crate::classes::package::Package;
use handle_error::handle_error_and_exit;
use std::{fs::File, u64};
use std::io::{BufWriter, Write, BufReader};
use indicatif::{ProgressBar, ProgressStyle};
use cache::check_cache;
use get_package::get_package;

pub fn updater(packages: Vec<String>) {
  let mut handles = vec![];
  let mut sizes = vec![];
  let mut multi = false; 
  for pkg in packages.iter() { 
      let package: Package = get_package(pkg.as_str());
      sizes.push(package.versions[&package.latest_version].size);
  }            
  let mut max_size = sizes[0];
  for i in 0..sizes.len() {
      if sizes[i] > max_size
      {
          max_size = sizes[i];
      }
  }
  if sizes.len() > 1 {
      multi = true;
      println!("{}", "Installing Packages".bright_green());
  }
  for pkg in packages.iter() {  
      let mut max = true;       
      let pkg_clone = pkg.clone();   
      let package: Package = get_package(pkg_clone.as_str());
      let latest_version = package.latest_version;
      let display_name = package.display_name;
      let url = package.versions[&latest_version].url.clone();
      let threads = package.versions[&latest_version].threads.clone();
      let iswitch = package.versions[&latest_version].iswitches.clone();
      let temp = std::env::var("TEMP").unwrap_or_else(|e| handle_error_and_exit(e.to_string()));
      let package_name = package.package_name;
      let loc = format!(r"{}\novus\{}@{}.exe", temp, package_name, latest_version);
      if package.versions[&latest_version].size != max_size {
          max = false;
      }
      let exists = check_cache(package_name.clone(), latest_version);   
      handles.push(std::thread::spawn(move|| {    
          if !exists {
            threadeddownload(url, loc.clone(), threads, display_name, package_name, max, multi);
          }
          install(&iswitch, loc.clone());
      }));
  }
  for handle in handles {
      handle.join().unwrap_or_else(|e| handle_error_and_exit("Failed to join handles".to_string()));
  }
  println!("{}", "Successfully installed packages".bright_magenta());
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
pub async fn threadeddownload(url: String, output: String, threads: u64, display_name: String, package_name: String, max: bool, multi: bool) {
  // let start = Instant::now();
  let mut handles = vec![];
  let res = reqwest::get(url.to_string()).await.unwrap();
  let total_length = res.content_length().unwrap();
  let temp = std::env::var("TEMP").unwrap();

  if !multi {
    println!("{}", format!("Installing {}", display_name))
  }

  if max {
    let progress_bar = ProgressBar::new(total_length);
    progress_bar.set_style(ProgressStyle::default_bar()
              .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
              .progress_chars("#>-"));
  
    for index in 0..threads {
      let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index+1);
      let (start, end) = get_splits(index+1, total_length, threads);
      let pb = progress_bar.clone();
      let mut file = BufWriter::new(File::create(loc).unwrap());
      let url = url.clone();
      handles.push(tokio::spawn(async move {    
        let client = reqwest::Client::new();
        let mut response = client.get(url)
        .header("range", format!("bytes={}-{}", start, end))
        .send().await.unwrap();    
        while let Some(chunk) = response.chunk().await.unwrap() {
          pb.inc(chunk.len() as u64);
          let _ = file.write(&*chunk);
        }    
       }));
    }
  
    futures::future::join_all(handles).await;
  
    progress_bar.finish();
  }
  else {
    for index in 0..threads {
      let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index+1);
      let (start, end) = get_splits(index+1, total_length, threads);
      let mut file = BufWriter::new(File::create(loc).unwrap());
      let url = url.clone();
      handles.push(tokio::spawn(async move {    
        let client = reqwest::Client::new();
        let mut response = client.get(url)
        .header("range", format!("bytes={}-{}", start, end))
        .send().await.unwrap();    
        while let Some(chunk) = response.chunk().await.unwrap() {
          let _ = file.write(&*chunk);
        }    
       }));
    }
  
    futures::future::join_all(handles).await;
  }

  let mut file = File::create(output.clone()).unwrap();

  for index in 0..threads {
    let loc = format!(r"{}\novus\setup_{}{}.tmp", temp, package_name, index+1);
    let mut buf: Vec<u8> = vec![];
    let downloaded_file = File::open(loc).unwrap();
    let mut reader = BufReader::new(downloaded_file);
    let _ = std::io::copy(&mut reader, &mut buf);
    let _ = file.write_all(&buf);
  }  

  // println!("download time: {:?}", start.elapsed());
}

#[allow(unused)]
pub fn install(iswitch: &Vec<String>, output_file: String) { 
  let _cmd = std::process::Command::new(output_file).arg(iswitch.join(" ")).spawn().unwrap().wait_with_output().unwrap();
}
