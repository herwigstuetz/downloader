use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    File(io::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// Downloads the `url` to the directory `tmp`.
///
/// Return the `PathBuf` to the downloaded file.
///
/// # Errors
///
/// If `url` cannot be retrieved, or the file cannot be saved to `tmp`, returns `Error`.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// let file = downloader::download("https://sh.rustup.rs", &PathBuf::from("/tmp"));
/// ```
pub fn download(url: &str, tmp: &Path) -> Result<PathBuf> {
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
    let filename = content_disposition
        .split("; ")
        .filter(|s| s.contains("filename"))
        .collect::<String>()
        .split('=')
        .collect::<Vec<&str>>()
        .get(1)
        .map(|file| file.to_string());

    let filename = match filename {
        Some(filename) => filename,
        None => "out.download".to_string(),
    };

    // Append filename to tmp directory
    let fname = tmp.join(filename);

    // Open destination file
    let mut dest = match File::create(&fname) {
        Ok(file) => file,
        Err(err) => return Err(Error::File(err)),
    };

    // Write data from response (attachment) into destination
    match io::copy(&mut response, &mut dest) {
        Ok(_) => Ok(fname),
        Err(err) => Err(Error::File(err)),
    }
}
