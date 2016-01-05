use types::{Argc, c_char, Id, Value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_const_get(klass: Value, id: Value) -> Value;
    pub fn rb_funcallv(receiver: Value, method: Value, argc: Argc, argv: *const Value) -> Value;
    pub fn rb_intern(name: *const c_char) -> Id;
}
