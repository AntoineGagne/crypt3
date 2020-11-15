use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub enum EncodedField {
    Key,
    Salt,
}

pub enum EncryptionError {
    Encoding(EncodedField),
    Decoding(String),
}

pub fn encrypt(key: String, salt: String) -> Result<String, EncryptionError> {
    unsafe {
        let key_ = CString::new(key)
            .map_err(|_| EncryptionError::Encoding(EncodedField::Key))?
            .as_ptr();
        let salt_ = CString::new(salt)
            .map_err(|_| EncryptionError::Encoding(EncodedField::Salt))?
            .as_ptr();
        Ok(CStr::from_ptr(crypt(key_, salt_))
            .to_str()
            .map_err(|x| EncryptionError::Decoding(x.to_string()))?
            .to_owned())
    }
}

#[link(name = "crypt")]
extern "C" {
    fn crypt(key: *const c_char, salt: *const c_char) -> *const c_char;
}
