use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_define_class(name: *const types::c_char, superclass: types::rb_value) -> types::rb_value;
    pub fn rb_define_module(name: *const types::c_char) -> types::rb_value;

    pub fn rb_obj_class(object: types::rb_value) -> types::rb_value;

    pub fn rb_class_new_instance(argc: types::argc,
                                 argv: *const types::rb_value,
                                 klass: types::rb_value) -> types::rb_value;

    pub fn rb_define_method(klass: types::rb_value,
                            name: *const types::c_char,
                            callback: types::callback_ptr,
                            argc: types::c_int);

    pub fn rb_define_singleton_method(klass: types::rb_value,
                                      name: *const types::c_char,
                                      callback: types::callback_ptr,
                                      argc: types::c_int);
}
