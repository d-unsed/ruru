use types::{argc, c_char, c_int, callback_ptr, rb_value};

#[link(name = "ruby")]
extern "C" {
    pub fn rb_class_new_instance(argc: argc, argv: *const rb_value, klass: rb_value) -> rb_value;
    pub fn rb_define_class(name: *const c_char, superclass: rb_value) -> rb_value;
    pub fn rb_define_method(klass: rb_value, name: *const c_char, callback: callback_ptr, argc: c_int);
    pub fn rb_define_module(name: *const c_char) -> rb_value;
    pub fn rb_obj_class(object: rb_value) -> rb_value;

    pub fn rb_define_singleton_method(klass: rb_value,
                                      name: *const c_char,
                                      callback: callback_ptr,
                                      argc: c_int);
}
