use std::ffi::{CStr, CString};

use binding::global::RubySpecialConsts;
use class::any_object::AnyObject;
use types::{c_char, c_int, Value};

use class::traits::Object;

pub fn cstr_as_string(str: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(str).to_string_lossy().into_owned()
    }
}

pub fn bool_to_value(state: bool) -> Value {
    let value = match state {
        false => RubySpecialConsts::False,
        true => RubySpecialConsts::True
    };

    value as Value
}

pub fn value_to_bool(value: Value) -> bool {
    if value == (RubySpecialConsts::False as Value) {
        false
    } else {
        true
    }
}

pub fn create_arguments(arguments: Vec<AnyObject>) -> (c_int, *const Value) {
    (arguments.len() as c_int, arguments_as_ptr(arguments))
}

fn arguments_as_ptr(arguments: Vec<AnyObject>) -> *const Value {
    arguments
        .iter()
        .map(|object| object.value())
        .collect::<Vec<Value>>()
        .as_ptr()
}
