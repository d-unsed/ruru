use types::{c_long, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_ary_entry(array: rb_value, offset: c_long) -> rb_value;
    pub fn rb_ary_join(array: rb_value, separator: rb_value) -> rb_value;
    pub fn rb_ary_new() -> rb_value;
    pub fn rb_ary_push(array: rb_value, item: rb_value) -> rb_value;
    pub fn rb_ary_store(array: rb_value, index: c_long, item: rb_value) -> rb_value;
}
