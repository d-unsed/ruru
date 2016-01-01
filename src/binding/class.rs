use types;
use unsafe_binding::class;
use util;

use class::traits::RawObject;

pub fn define_class(name: &str, superclass: types::rb_value) -> types::rb_value {
    unsafe {
        class::rb_define_class(util::str_as_ptr(name), superclass)
    }
}

pub fn define_module(name: &str) -> types::rb_value {
    unsafe {
        class::rb_define_module(util::str_as_ptr(name))
    }
}

pub fn object_class(object: types::rb_value) -> types::rb_value {
    unsafe {
        class::rb_obj_class(object)
    }
}

pub fn new_instance(klass: types::rb_value, argc: types::argc, argv: *const types::rb_value) -> types::rb_value {
    unsafe {
        class::rb_class_new_instance(argc, argv, klass)
    }
}

pub fn define_method<T: RawObject>(klass: types::rb_value, name: &str, callback: types::callback<T>) {
    unsafe {
        class::rb_define_method(klass, util::str_as_ptr(name), callback as types::callback_ptr, -1);
    }
}

pub fn define_singleton_method<T: RawObject>(klass: types::rb_value, name: &str, callback: types::callback<T>) {
    unsafe {
        class::rb_define_singleton_method(klass, util::str_as_ptr(name), callback as types::callback_ptr, -1);
    }
}
