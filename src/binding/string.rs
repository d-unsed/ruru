use types::Value;
use unsafe_binding::string::{rb_str_new_cstr, rb_string_value_cstr};
use util::cstr_as_string;
use std::ffi::CString;

pub fn new(string: &str) -> Value {
    let string = CString::new(string).unwrap();

    unsafe {
        rb_str_new_cstr(string.as_ptr())
    }
}

pub fn from_value(value: Value) -> String {
    unsafe {
        let str = rb_string_value_cstr(&value);

        cstr_as_string(str)
    }
}
