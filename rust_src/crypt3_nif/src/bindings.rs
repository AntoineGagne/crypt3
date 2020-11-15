use std::ffi::{CString, CStr};
use std::os::raw::c_char;

pub fn encrypt(key: String, salt: String) -> Option<String> {
    unsafe {
        let key_ = CString::new(key).ok()?.as_ptr();
        let salt_ = CString::new(salt).ok()?.as_ptr();
        Some(CStr::from_ptr(crypt(key_, salt_)).to_str().ok()?.to_owned())
    }
}

#[link(name = "crypt")]
extern "C" {
    fn crypt(key: *const c_char, salt: *const c_char) -> *const c_char;
}

