use types::{c_long, Value};
use ruby_sys::array::{rb_ary_new, rb_ary_entry, rb_ary_join, rb_ary_push, rb_ary_store};

pub fn new() -> Value {
    unsafe { rb_ary_new() }
}

pub fn entry(array: Value, offset: i64) -> Value {
    unsafe { rb_ary_entry(array, offset as c_long) }
}

pub fn join(array: Value, separator: Value) -> Value {
    unsafe { rb_ary_join(array, separator) }
}

pub fn push(array: Value, item: Value) -> Value {
    unsafe { rb_ary_push(array, item) }
}

pub fn store(array: Value, offset: i64, item: Value) -> Value {
    unsafe { rb_ary_store(array, offset as c_long, item) }
}
