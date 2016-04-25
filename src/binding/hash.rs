use types::Value;
use ruby_sys::hash::{rb_hash_aref, rb_hash_aset, rb_hash_new};

pub fn new() -> Value {
    unsafe {
        rb_hash_new()
    }
}

pub fn aref(hash: Value, key: Value) -> Value {
    unsafe {
        rb_hash_aref(hash, key)
    }
}

pub fn aset(hash: Value, key: Value, value: Value) -> Value {
    unsafe {
        rb_hash_aset(hash, key, value)
    }
}
