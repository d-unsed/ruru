use ruby_sys::array;

use types::{c_long, Value};

pub fn new() -> Value {
    unsafe { array::rb_ary_new() }
}

pub fn entry(array: Value, offset: i64) -> Value {
    unsafe { array::rb_ary_entry(array, offset as c_long) }
}

pub fn join(array: Value, separator: Value) -> Value {
    unsafe { array::rb_ary_join(array, separator) }
}

pub fn push(array: Value, item: Value) -> Value {
    unsafe { array::rb_ary_push(array, item) }
}

pub fn store(array: Value, offset: i64, item: Value) -> Value {
    unsafe { array::rb_ary_store(array, offset as c_long, item) }
}
