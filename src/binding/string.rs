use types;
use unsafe_binding::string;
use util;

pub fn new(string: &str) -> types::rb_value {
    unsafe {
        string::rb_str_new_cstr(util::str_as_ptr(string))
    }
}

pub fn from_value(value: types::rb_value) -> String {
    unsafe {
        let str = string::rb_string_value_cstr(&value);

        util::cstr_as_string(str)
    }
}
