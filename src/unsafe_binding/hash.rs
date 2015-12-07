use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_hash_new() -> types::rb_value;
    pub fn rb_hash_aref(hash: types::rb_value, key: types::rb_value) -> types::rb_value;
    pub fn rb_hash_aset(hash: types::rb_value,
                        key: types::rb_value,
                        value: types::rb_value) -> types::rb_value;
}
