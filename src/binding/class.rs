use types::{Argc, Callback, CallbackPtr, Value};
use unsafe_binding::class::{rb_class_new_instance, rb_define_class, rb_define_method,
                            rb_define_module, rb_define_singleton_method, rb_obj_class};
use util::str_as_ptr;

use class::traits::Object;

pub fn define_class(name: &str, superclass: Value) -> Value {
    unsafe {
        rb_define_class(str_as_ptr(name), superclass)
    }
}

pub fn define_module(name: &str) -> Value {
    unsafe {
        rb_define_module(str_as_ptr(name))
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

pub fn define_method<I: Object, O: Object>(klass: Value, name: &str, callback: Callback<I, O>) {
    unsafe {
        rb_define_method(klass, str_as_ptr(name), callback as CallbackPtr, -1);
    }
}

pub fn define_singleton_method<I: Object, O: Object>(klass: Value, name: &str, callback: Callback<I, O>) {
    unsafe {
        rb_define_singleton_method(klass, str_as_ptr(name), callback as CallbackPtr, -1);
    }
}
