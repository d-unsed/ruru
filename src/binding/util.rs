use binding::global::rb_cObject;
use types::{Argc, Id, Value};
use unsafe_binding::util::{rb_const_get, rb_funcallv, rb_intern};
use util::str_as_ptr;

pub fn get_constant(name: &str, parent_object: Value) -> Value {
    let constant_id = internal_id(name);

    unsafe {
        rb_const_get(rb_cObject, constant_id)
    }
}

pub fn internal_id(string: &str) -> Id {
    unsafe {
        rb_intern(str_as_ptr(string))
    }
}

pub fn call_method(receiver: Value, method: &str, argc: Argc, argv: *const Value) -> Value {
    let method_id = internal_id(method);

    unsafe {
        rb_funcallv(receiver, method_id, argc, argv)
    }
}
