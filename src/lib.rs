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
fn download(url: String, tmp: String) -> Result<String> {
    let mut response = match reqwest::blocking::get(&url) {
        Ok(response) => response,
        Err(err) => return Err(Error::Request(err)),
    };

    let fname = &response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let fname = Path::new(&tmp).join(fname);

    let mut dest = match File::create(&fname) {
        Ok(file) => file,
        Err(err) => return Err(Error::File(err)),
    };

    match io::copy(&mut response, &mut dest) {
        Ok(_) => Ok(fname.into_os_string().into_string().unwrap()),
        Err(err) => Err(Error::File(err)),
    }
}
