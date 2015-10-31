use types;
use unsafe_bindings::array;

pub fn array_new() -> types::rb_value {
    unsafe {
        array::rb_ary_new()
    }
}

pub fn array_entry(array: types::rb_value, offset: types::c_long) -> types::rb_value {
    unsafe {
        array::rb_ary_entry(array, offset)
    }
}

pub fn array_push(array: types::rb_value, item: types::rb_value) -> types::rb_value {
    unsafe {
        array::rb_ary_push(array, item)
    }
}
