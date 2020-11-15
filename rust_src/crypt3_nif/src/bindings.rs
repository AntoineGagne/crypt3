use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn encrypt(key: String, salt: String) -> Option<String> {
    unsafe {
        let key_ = CString::new(key)
            .expect("Could not encode key to a C string")
            .as_ptr();
        let salt_ = CString::new(salt)
            .expect("Could not encode salt to a C string")
            .as_ptr();
        Some(CStr::from_ptr(crypt(key_, salt_)).to_str().ok()?.to_owned())
    }
}

#[link(name = "crypt")]
extern "C" {
    fn crypt(key: *const c_char, salt: *const c_char) -> *const c_char;
}
