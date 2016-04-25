use binding::global::rb_cObject;
use types::{Argc, Id, Value};
use ruby_sys::util::{rb_const_get, rb_funcallv, rb_intern};
use std::ffi::CString;

pub fn get_constant(name: &str, _parent_object: Value) -> Value {
    let constant_id = internal_id(name);

    unsafe {
        rb_const_get(rb_cObject, constant_id)
    }
}

pub fn internal_id(string: &str) -> Id {
    let string = CString::new(string).unwrap();
    let id;
    {
        id = unsafe {
            rb_intern(string.as_ptr())
        };
    }

    id
}

pub fn call_method(receiver: Value, method: &str, argc: Argc, argv: *const Value) -> Value {
    let method_id = internal_id(method);

    unsafe {
        rb_funcallv(receiver, method_id, argc, argv)
    }
}
