use types;
use unsafe_bindings::string;
use util;

pub fn new_string(string: &str) -> types::rb_value {
    unsafe {
        string::rb_str_new_cstr(util::str_as_ptr(string))
    }
}

pub fn string_from_falue(value: types::rb_value) -> String {
    unsafe {
        let str = string::rb_string_value_cstr(&value);

        util::cstr_as_str(str)
    }
}
