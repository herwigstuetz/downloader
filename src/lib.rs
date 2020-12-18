use std::path::Path;
use std::fs::File;
use std::io;

use reqwest;


#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    File(io::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[no_mangle]
pub fn download(url: &str, tmp: &str) -> Result<String> {

    // Make GET request for url
    let mut response = match reqwest::blocking::get(url) {
        Ok(response) => response,
        Err(err) => return Err(Error::Request(err)),
    };

    // Get Content-Disposition header:
    // > Content-Disposition: attachment; filename="filename.jpg"
    let content_disposition = response
        .headers()
        .get("Content-Disposition")
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");

    // Get filename from Content-Disposition
    let filename : String = content_disposition
        .split("; ")
        .filter(|s| s.contains("filename"))
        .collect::<String>()
        .split("=")
        .collect::<Vec<&str>>()[1].to_string();
    println!("headers: {:#?}", &filename);

    // Append filename to tmp directory
    let fname = Path::new(tmp).join(filename);

    // Open destination file
    let mut dest = match File::create(&fname) {
        Ok(file) => file,
        Err(err) => return Err(Error::File(err)),
    };

    // Write data from response (attachment) into destination
    match io::copy(&mut response, &mut dest) {
        Ok(_) => Ok(fname.into_os_string().into_string().unwrap()),
        Err(err) => Err(Error::File(err)),
    }
}
