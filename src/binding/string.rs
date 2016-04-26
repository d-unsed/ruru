use types::Value;
use ruby_sys::string::{rb_str_new_cstr, rb_string_value_cstr};
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
