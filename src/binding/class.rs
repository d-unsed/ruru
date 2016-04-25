use binding::util::internal_id;
use types::{Argc, Callback, CallbackPtr, Value};
use ruby_sys::class::{rb_class_new_instance, rb_define_class, rb_define_method,
                            rb_ivar_get, rb_ivar_set, rb_define_singleton_method,
                            rb_obj_class};
use class::traits::Object;
use std::ffi::CString;

pub fn define_class(name: &str, superclass: Value) -> Value {
    let string = CString::new(name).unwrap();
    unsafe {
        rb_define_class(string.as_ptr(), superclass)
    }
}

pub fn object_class(object: Value) -> Value {
    unsafe {
        rb_obj_class(object)
    }
}

pub fn new_instance(klass: Value, argc: Argc, argv: *const Value) -> Value {
    unsafe {
        rb_class_new_instance(argc, argv, klass)
    }
}

pub fn instance_variable_get(object: Value, name: &str) -> Value {
    unsafe {
        rb_ivar_get(object, internal_id(name))
    }
}

pub fn instance_variable_set(object: Value, name: &str, value: Value) -> Value {
    unsafe {
        rb_ivar_set(object, internal_id(name), value)
    }
}

pub fn define_method<I: Object, O: Object>(klass: Value, name: &str, callback: Callback<I, O>) {
    let string = CString::new(name).unwrap();
    unsafe {
        rb_define_method(klass, string.as_ptr(), callback as CallbackPtr, -1);
    }
}

pub fn define_singleton_method<I: Object, O: Object>(klass: Value, name: &str, callback: Callback<I, O>) {
    let string = CString::new(name).unwrap();
    unsafe {
        rb_define_singleton_method(klass, string.as_ptr(), callback as CallbackPtr, -1);
    }
}
