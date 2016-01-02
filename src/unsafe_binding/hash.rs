use types::rb_value;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_hash_aref(hash: rb_value, key: rb_value) -> rb_value;
    pub fn rb_hash_aset(hash: rb_value, key: rb_value, value: rb_value) -> rb_value;
    pub fn rb_hash_new() -> rb_value;
}
