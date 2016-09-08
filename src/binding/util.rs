use ruby_sys::util as ruby_sys_util;

use binding::util as binding_util;
use types::{Argc, c_void, Id, Value};
use util;

pub fn get_constant(name: &str, parent_object: Value) -> Value {
    let constant_id = binding_util::internal_id(name);

    unsafe { ruby_sys_util::rb_const_get(parent_object, constant_id) }
}

pub fn internal_id(string: &str) -> Id {
    let str = util::str_to_cstring(string);

    unsafe { ruby_sys_util::rb_intern(str.as_ptr()) }
}

pub fn call_method(receiver: Value, method: &str, argc: Argc, argv: *const Value) -> Value {
    let method_id = binding_util::internal_id(method);

    unsafe { ruby_sys_util::rb_funcallv(receiver, method_id, argc, argv) }
}

pub fn wrap_closure_to_ptr<F, R>(mut func: F) -> *const c_void
    where F: FnMut() -> R
{
    let wrap_return = || {
        let r = func();
        Box::into_raw(Box::new(r)) as *const c_void
    };
    let fnbox = Box::new(wrap_return) as Box<FnMut() -> *const c_void>;
    Box::into_raw(Box::new(fnbox)) as *const c_void
}

pub unsafe fn unwrap_data_from_ptr<R>(ptr: *mut c_void) -> R {
    *Box::from_raw(ptr as *mut R)
}
