use super::downloader;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

/// Downloads `url` to the directory `tmp` and returns the path to the
/// downloaded file.
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

    let file = downloader::download(url, Path::new(tmp));

    file.ok()
        .as_ref()
        .and_then(|f| f.to_str())
        .and_then(|s| CString::new(s).ok())
        .and_then(|s| Some(s.into_raw()))
        .unwrap_or(std::ptr::null_mut())
}

/// Frees `char` pointers returned by `dl_download`.
#[no_mangle]
pub extern "C" fn dl_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
