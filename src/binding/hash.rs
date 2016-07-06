use ruby_sys::hash;

use types::Value;

pub fn new() -> Value {
    unsafe { hash::rb_hash_new() }
}

pub fn aref(hash: Value, key: Value) -> Value {
    unsafe { hash::rb_hash_aref(hash, key) }
}

pub fn aset(hash: Value, key: Value, value: Value) -> Value {
    unsafe { hash::rb_hash_aset(hash, key, value) }
}
