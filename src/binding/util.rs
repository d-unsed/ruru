use ruby_sys::util as ruby_sys_util;

use binding::util as binding_util;
use binding::global::rb_cObject;
use types::{Argc, Id, Value};
use util;

pub fn get_constant(name: &str, _parent_object: Value) -> Value {
    let constant_id = binding_util::internal_id(name);

    unsafe { ruby_sys_util::rb_const_get(rb_cObject, constant_id) }
}

pub fn internal_id(string: &str) -> Id {
    let str = util::str_to_cstring(string);

    unsafe { ruby_sys_util::rb_intern(str.as_ptr()) }
}

pub fn call_method(receiver: Value, method: &str, argc: Argc, argv: *const Value) -> Value {
    let method_id = binding_util::internal_id(method);

    unsafe { ruby_sys_util::rb_funcallv(receiver, method_id, argc, argv) }
}
