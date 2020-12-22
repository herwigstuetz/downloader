use downloader;

use std::env;
use std::path::Path;
use std::string::String;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = &args[1];

    println!("Attempting to download url {}", url);
    match downloader::download(url, Path::new(".")) {
        Ok(file) => println!("Downloaded file to {}", file.to_str().unwrap_or("")),
        Err(_err) => println!("Could not download file"),
    };
}
