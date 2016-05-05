use binding::global::rb_cObject;
use types::{Argc, Id, Value};
use ruby_sys::util::{rb_const_get, rb_funcallv, rb_intern};
pub use ruby_sys::util::rb_type as get_type;
use util::str_to_cstring;

pub fn get_constant(name: &str, _parent_object: Value) -> Value {
    let constant_id = internal_id(name);

    unsafe { rb_const_get(rb_cObject, constant_id) }
}

pub fn internal_id(string: &str) -> Id {
    unsafe { rb_intern(str_to_cstring(string).as_ptr()) }
}

pub fn call_method(receiver: Value, method: &str, argc: Argc, argv: *const Value) -> Value {
    let method_id = internal_id(method);

    unsafe { rb_funcallv(receiver, method_id, argc, argv) }
}
