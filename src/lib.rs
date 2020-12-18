use std::fs::File;
use std::io;
use std::path::Path;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use reqwest;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    File(io::Error),
}

type Result<T> = std::result::Result<T, Error>;

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
    let filename: String = content_disposition
        .split("; ")
        .filter(|s| s.contains("filename"))
        .collect::<String>()
        .split("=")
        .collect::<Vec<&str>>()[1]
        .to_string();
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

#[no_mangle]
pub extern "C" fn dl_download(url: *const c_char, tmp: *const c_char) -> *mut c_char {
    if url.is_null() {
        return std::ptr::null_mut();
    }

    if tmp.is_null() {
        return std::ptr::null_mut();
    }

    // safe because is_null() check
    let url = match unsafe { CStr::from_ptr(url).to_str() } {
        Ok(url) => url,
        Err(_) => return std::ptr::null_mut(),
    };

    let tmp = match unsafe { CStr::from_ptr(tmp).to_str() } {
        Ok(tmp) => tmp,
        Err(_) => return std::ptr::null_mut(),
    };

    let file = download(url, tmp);

    match file {
        Ok(file) => CString::new(file).unwrap().into_raw(),
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn dl_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
