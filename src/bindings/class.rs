use std::ffi::CString;

use types;
use unsafe_bindings;

pub fn define_class(name: &str, superclass: types::rb_value) -> types::rb_value {
    let name = CString::new(name).unwrap().as_ptr();

    unsafe {
        unsafe_bindings::class::rb_define_class(name, superclass)
    }
}

pub fn define_module(name: &str) -> types::rb_value {
    let name = CString::new(name).unwrap().as_ptr();

    unsafe {
        unsafe_bindings::class::rb_define_module(name)
    }
}

pub fn define_method(klass: types::rb_value,
                     name: &str,
                     callback: extern fn(types::rb_value) -> types::rb_value,
                     argc: i32) {
    let name = CString::new(name).unwrap().as_ptr();

    unsafe {
        unsafe_bindings::class::rb_define_method(klass, name, callback, argc);
    }
}

pub fn define_singleton_method(klass: types::rb_value,
                               name: &str,
                               callback: extern fn(types::rb_value) -> types::rb_value,
                               argc: i32) {
    let name = CString::new(name).unwrap().as_ptr();

    unsafe {
        unsafe_bindings::class::rb_define_singleton_method(klass, name, callback, argc);
    }
}
