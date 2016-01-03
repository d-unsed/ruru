use types::rb_value;
use unsafe_binding::hash::{rb_hash_aref, rb_hash_aset, rb_hash_new};

pub fn new() -> rb_value {
    unsafe {
        rb_hash_new()
    }
}

pub fn aref(hash: rb_value, key: rb_value) -> rb_value {
    unsafe {
        rb_hash_aref(hash, key)
    }
}

pub fn aset(hash: rb_value, key: rb_value, value: rb_value) -> rb_value {
    unsafe {
        rb_hash_aset(hash, key, value)
    }
}
