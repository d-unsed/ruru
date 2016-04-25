use types::{c_char, Value};

extern "C" {
    pub fn rb_str_new_cstr(str: *const c_char) -> Value;
    pub fn rb_string_value_cstr(str: *const Value) -> *const c_char;
}
