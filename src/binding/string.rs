use ruby_sys::string;

use types::{c_char, c_long, Value};
use util;

pub fn new(string: &str) -> Value {
    let str = string.as_ptr() as *const c_char;
    let len = string.len() as c_long;

    unsafe { string::rb_str_new(str, len) }
}

pub fn new_utf8(string: &str) -> Value {
    let str = string.as_ptr() as *const c_char;
    let len = string.len() as c_long;

    unsafe { string::rb_utf8_str_new(str, len) }
}

pub fn value_to_string(value: Value) -> String {
    unsafe {
        let str = string::rb_string_value_cstr(&value);

        util::cstr_to_string(str)
    }
}

pub fn value_to_string_unchecked(value: Value) -> String {
    unsafe {
        let str = string::rb_string_value_ptr(&value) as *const u8;
        let len = string::rb_str_len(value) as usize;

        let vec = ::std::slice::from_raw_parts(str, len).to_vec();

        String::from_utf8_unchecked(vec)
    }
}

pub fn value_to_str<'a>(value: Value) -> &'a str {
    unsafe {
        let str = string::rb_string_value_cstr(&value);

        util::cstr_to_str(str)
    }
}

pub fn value_to_str_unchecked<'a>(value: Value) -> &'a str {
    unsafe {
        let str = string::rb_string_value_ptr(&value) as *const u8;
        let len = string::rb_str_len(value) as usize;

        let slice = ::std::slice::from_raw_parts(str, len);

        ::std::str::from_utf8_unchecked(slice)
    }
}

pub fn bytesize(value: Value) -> i64 {
    unsafe { string::rb_str_len(value) as i64 }
}
