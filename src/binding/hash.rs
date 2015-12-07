use types;
use unsafe_binding::hash;

pub fn new() -> types::rb_value {
    unsafe {
        hash::rb_hash_new()
    }
}

pub fn aref(hash: types::rb_value, key: types::rb_value) -> types::rb_value {
    unsafe {
        hash::rb_hash_aref(hash, key)
    }
}

pub fn aset(hash: types::rb_value, key: types::rb_value, value: types::rb_value) -> types::rb_value {
    unsafe {
        hash::rb_hash_aset(hash, key, value)
    }
}
