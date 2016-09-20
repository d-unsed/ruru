use ruby_sys::util as ruby_sys_util;

use types::{Argc, Id, Value};
use util;

pub fn get_constant(name: &str, parent_object: Value) -> Value {
    let constant_id = internal_id(name);

    unsafe { ruby_sys_util::rb_const_get(parent_object, constant_id) }
}

pub fn internal_id(string: &str) -> Id {
    let str = util::str_to_cstring(string);

    unsafe { ruby_sys_util::rb_intern(str.as_ptr()) }
}

pub fn call_method(receiver: Value, method: &str, argc: Argc, argv: *const Value) -> Value {
    let method_id = internal_id(method);

    unsafe { ruby_sys_util::rb_funcallv(receiver, method_id, argc, argv) }
}
