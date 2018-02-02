use ruby_sys::class;

use binding::global::rb_cObject;
use binding::util as binding_util;
use types::Value;
use util;

pub fn define_module(name: &str) -> Value {
    let name = util::str_to_cstring(name);

    unsafe { class::rb_define_module(name.as_ptr()) }
}

pub fn define_nested_module(outer: Value, name: &str) -> Value {
    let name = util::str_to_cstring(name);

    unsafe { class::rb_define_module_under(outer, name.as_ptr()) }
}

pub fn include_module(klass: Value, module: &str) {
    let object_module = unsafe { rb_cObject };

    let module_value = binding_util::get_constant(module, object_module);

    unsafe { class::rb_include_module(klass, module_value) };
}

pub fn prepend_module(klass: Value, module: &str) {
    let object_module = unsafe { rb_cObject };

    let module_value = binding_util::get_constant(module, object_module);

    unsafe { class::rb_prepend_module(klass, module_value) };
}
