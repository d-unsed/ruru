use std::ffi::{CStr, CString};

use types;

pub fn str_as_ptr(str: &str) -> *const types::c_char {
    CString::new(str).unwrap().as_ptr()
}

pub fn cstr_as_string(str: *const types::c_char) -> String {
    unsafe {
        CStr::from_ptr(str).to_string_lossy().into_owned()
    }
}
