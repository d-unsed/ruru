use std::ffi::CString;

use types;

pub fn str_as_ptr(str: &str) -> *const types::c_char {
    CString::new(str).unwrap().as_ptr()
}
