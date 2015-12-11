use types;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_define_class(name: *const types::c_char, superclass: types::rb_value) -> types::rb_value;
    pub fn rb_define_module(name: *const types::c_char) -> types::rb_value;

    pub fn rb_define_method(klass: types::rb_value,
                            name: *const types::c_char,
                            callback: extern fn(i32, *const types::rb_value, types::rb_value) -> types::rb_value,
                            argc: types::c_int);

    pub fn rb_define_singleton_method(klass: types::rb_value,
                                      name: *const types::c_char,
                                      callback: extern fn(types::rb_value) -> types::rb_value,
                                      argc: types::c_int);
}
