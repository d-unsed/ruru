use ruby_sys::{class, typed_data};

use binding::util as binding_util;
use typed_data::DataTypeWrapper;
use types::{c_void, Value};
use util;

pub fn define_module(name: &str) -> Value {
    let name = util::str_to_cstring(name);

    unsafe { class::rb_define_module(name.as_ptr()) }
}

pub fn define_nested_module(outer: Value, name: &str) -> Value {
    let name = util::str_to_cstring(name);

    unsafe { class::rb_define_module_under(outer, name.as_ptr()) }
}

pub fn const_get(klass: Value, name: &str) -> Value {
    unsafe { class::rb_const_get(klass, binding_util::internal_id(name)) }
}

pub fn const_set(klass: Value, name: &str, value: Value) {
    let name = util::str_to_cstring(name);

    unsafe { class::rb_define_const(klass, name.as_ptr(), value) };
}

pub fn ancestors(klass: Value) -> Value {
    unsafe { class::rb_mod_ancestors(klass) }
}

pub fn define_attribute(object: Value, name: &str, reader: bool, writer: bool) {
    let name = util::str_to_cstring(name);
    let reader = util::bool_to_c_int(reader);
    let writer = util::bool_to_c_int(writer);

    unsafe { class::rb_define_attr(object, name.as_ptr(), reader, writer) };
}

pub fn wrap_data<T>(klass: Value, data: T, wrapper: &DataTypeWrapper<T>) -> Value {
    let data = Box::into_raw(Box::new(data)) as *mut c_void;

    unsafe { typed_data::rb_data_typed_object_wrap(klass, data, wrapper.data_type()) }
}
