use types::rb_value;
use unsafe_binding::string::{rb_str_new_cstr, rb_string_value_cstr};
use util::{cstr_as_string, str_as_ptr};

pub fn new(string: &str) -> rb_value {
    unsafe {
        rb_str_new_cstr(str_as_ptr(string))
    }
}

pub fn from_value(value: rb_value) -> String {
    unsafe {
        let str = rb_string_value_cstr(&value);

        cstr_as_string(str)
    }
}
