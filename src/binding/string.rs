use std::slice::from_raw_parts;

use ruby_sys::string::{rb_str_len, rb_str_new_cstr, rb_string_value_cstr, rb_string_value_ptr};
use types::Value;
use util::{cstr_as_string, str_to_cstring};

pub fn new(string: &str) -> Value {
    unsafe { rb_str_new_cstr(str_to_cstring(string).as_ptr()) }
}

pub fn from_value(value: Value) -> String {
    unsafe {
        let str = rb_string_value_cstr(&value);

        cstr_as_string(str)
    }
}

pub fn from_value_unchecked(value: Value) -> String {
    unsafe {
        let str = rb_string_value_ptr(&value) as *const u8;
        let len = rb_str_len(value) as usize;

        let vec = from_raw_parts(str, len).to_vec();

        String::from_utf8_unchecked(vec)
    }
}
