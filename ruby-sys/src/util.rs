use types::{Argc, c_char, Id, Value};

extern "C" {
    pub fn rb_const_get(klass: Value, id: Id) -> Value;
    pub fn rb_funcallv(receiver: Value, method: Id, argc: Argc, argv: *const Value) -> Value;
    pub fn rb_intern(name: *const c_char) -> Id;
}
