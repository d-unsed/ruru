use types;
use unsafe_binding::array;

pub fn new() -> types::rb_value {
    unsafe {
        array::rb_ary_new()
    }
}

pub fn entry(array: types::rb_value, offset: types::c_long) -> types::rb_value {
    unsafe {
        array::rb_ary_entry(array, offset)
    }
}

pub fn push(array: types::rb_value, item: types::rb_value) -> types::rb_value {
    unsafe {
        array::rb_ary_push(array, item)
    }
}
