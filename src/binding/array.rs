use types::{c_long, rb_value};
use unsafe_binding::array::{rb_ary_new, rb_ary_entry, rb_ary_join, rb_ary_push, rb_ary_store};

pub fn new() -> rb_value {
    unsafe {
        rb_ary_new()
    }
}

pub fn entry(array: rb_value, offset: i64) -> rb_value {
    unsafe {
        rb_ary_entry(array, offset as c_long)
    }
}

pub fn join(array: rb_value, separator: rb_value) -> rb_value {
    unsafe {
        rb_ary_join(array, separator)
    }
}

pub fn push(array: rb_value, item: rb_value) -> rb_value {
    unsafe {
        rb_ary_push(array, item)
    }
}

pub fn store(array: rb_value, offset: i64, item: rb_value) -> rb_value {
    unsafe {
        rb_ary_store(array, offset as c_long, item)
    }
}
