use std::ffi::{CStr, CString};

use binding::global;
use types;

pub fn str_as_ptr(str: &str) -> *const types::c_char {
    CString::new(str).unwrap().as_ptr()
}

pub fn cstr_as_string(str: *const types::c_char) -> String {
    unsafe {
        CStr::from_ptr(str).to_string_lossy().into_owned()
    }
}

pub fn bool_to_value(state: bool) -> types::rb_value {
    let value = match state {
        false => global::RubySpecialConsts::False,
        true => global::RubySpecialConsts::True
    };

    value as types::rb_value
}

pub fn value_to_bool(value: types::rb_value) -> bool {
    if value == (global::RubySpecialConsts::False as types::rb_value) {
        false
    } else {
        true
    }
}
