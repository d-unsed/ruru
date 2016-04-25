use types::{c_long, Value};

extern "C" {
    pub fn rb_ary_entry(array: Value, offset: c_long) -> Value;
    pub fn rb_ary_join(array: Value, separator: Value) -> Value;
    pub fn rb_ary_new() -> Value;
    pub fn rb_ary_push(array: Value, item: Value) -> Value;
    pub fn rb_ary_store(array: Value, index: c_long, item: Value) -> Value;
}
