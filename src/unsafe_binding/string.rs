use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_str_new_cstr(str: *const types::c_char) -> types::rb_value;
    pub fn rb_string_value_cstr(str: *const types::rb_value) -> *const types::c_char;
}
