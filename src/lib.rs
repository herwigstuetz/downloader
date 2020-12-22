use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    File(io::Error),
}

type Result<T> = std::result::Result<T, Error>;

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
        .split("=")
        .collect::<Vec<&str>>()
        .get(1)
        .and_then(|file| Some(file.to_string()));

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
        Ok(_) => Ok(PathBuf::from(fname)),
        Err(err) => Err(Error::File(err)),
    }
}

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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

    let file = download(url, Path::new(tmp));

    file.ok()
        .as_ref()
        .and_then(|f| f.to_str())
        .and_then(|s| CString::new(s).ok())
        .and_then(|s| Some(s.into_raw()))
        .unwrap_or(std::ptr::null_mut())
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

#[cfg(test)]
mod tests {
    use actix_rt;
    use actix_web::{http::header, web, App, HttpResponse, HttpServer};

    use tempfile;

    use tokio;

    use std::sync::Arc;

    async fn serve() -> std::io::Result<()> {
        let server = HttpServer::new(|| {
            App::new()
                .route(
                    "/file",
                    web::get().to(|| HttpResponse::Ok().body("file content")),
                )
                .route(
                    "/foo",
                    web::get().to(|| {
                        HttpResponse::Ok()
                            .header(header::CONTENT_DISPOSITION, "attachment; filename=foo.bar")
                            .body("file content")
                    }),
                )
        })
        .bind("127.0.0.1:8080")?
        .run();

        let _ = tokio::spawn(server);

        Ok(())
    }

    #[actix_rt::test]
    async fn writes_default_filename() {
        let _srv = serve().await;

        let tmpdir = Arc::new(tempfile::tempdir().expect("Could not create temp dir for test"));

        let tmp = tmpdir.clone();
        let file = actix_web::web::block(move || {
            super::download("http://127.0.0.1:8080/file", tmp.path())
        })
        .await;

        assert!(file.unwrap() == tmpdir.path().join("out.download"));
    }

    #[actix_rt::test]
    async fn uses_content_disposition() {
        let _srv = serve().await;

        let tmpdir = Arc::new(tempfile::tempdir().expect("Could not create temp dir for test"));

        let tmp = tmpdir.clone();
        let file =
            actix_web::web::block(move || super::download("http://127.0.0.1:8080/foo", tmp.path()))
                .await;

        assert!(file.unwrap() == tmpdir.path().join("foo.bar"));
    }
}
