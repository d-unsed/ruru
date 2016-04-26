use types::Value;

extern "C" {
    pub fn rb_hash_aref(hash: Value, key: Value) -> Value;
    pub fn rb_hash_aset(hash: Value, key: Value, value: Value) -> Value;
    pub fn rb_hash_new() -> Value;
}
