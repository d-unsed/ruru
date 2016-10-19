use std::slice;

use ruby_sys::string;

use types::{c_char, c_long, Value};
use util;

pub fn new(string: &str) -> Value {
    let str = string.as_ptr() as *const c_char;
    let len = string.len() as c_long;

    unsafe { string::rb_str_new(str, len) }
}

pub fn from_value(value: Value) -> String {
    let str = unsafe { string::rb_string_value_cstr(&value) };

    util::cstr_as_string(str)
}

pub fn from_value_unchecked(value: Value) -> String {
    unsafe {
        let str = string::rb_string_value_ptr(&value) as *const u8;
        let len = string::rb_str_len(value) as usize;

        let vec = slice::from_raw_parts(str, len).to_vec();

        String::from_utf8_unchecked(vec)
    }
}

pub fn bytesize(value: Value) -> i64 {
    unsafe { string::rb_str_len(value) as i64 }
}
