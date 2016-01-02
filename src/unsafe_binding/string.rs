use types::{c_char, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_str_new_cstr(str: *const c_char) -> rb_value;
    pub fn rb_string_value_cstr(str: *const rb_value) -> *const c_char;
}
