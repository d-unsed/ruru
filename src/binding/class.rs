use types::{argc, callback, callback_ptr, rb_value};
use unsafe_binding::class::{rb_class_new_instance, rb_define_class, rb_define_method,
                            rb_define_module, rb_define_singleton_method, rb_obj_class};
use util::str_as_ptr;

use class::traits::RawObject;

pub fn define_class(name: &str, superclass: rb_value) -> rb_value {
    unsafe {
        rb_define_class(str_as_ptr(name), superclass)
    }
}

pub fn define_module(name: &str) -> rb_value {
    unsafe {
        rb_define_module(str_as_ptr(name))
    }
}

pub fn object_class(object: rb_value) -> rb_value {
    unsafe {
        rb_obj_class(object)
    }
}

pub fn new_instance(klass: rb_value, argc: argc, argv: *const rb_value) -> rb_value {
    unsafe {
        rb_class_new_instance(argc, argv, klass)
    }
}

pub fn define_method<T: RawObject>(klass: rb_value, name: &str, callback: callback<T>) {
    unsafe {
        rb_define_method(klass, str_as_ptr(name), callback as callback_ptr, -1);
    }
}

pub fn define_singleton_method<T: RawObject>(klass: rb_value, name: &str, callback: callback<T>) {
    unsafe {
        rb_define_singleton_method(klass, str_as_ptr(name), callback as callback_ptr, -1);
    }
}
