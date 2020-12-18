use std::env;
use std::vec::Vec;
use std::string::String;

use downloader::*;

fn main() {
    let args : Vec<String> = env::args().collect();

    let url = &args[1];

    println!("Attempting to download url {}", url);
    match download(url, ".") {
        Ok(file) => println!("Downloaded file to {}", file),
        Err(_err) => println!("Could not download file"),
    };
}
