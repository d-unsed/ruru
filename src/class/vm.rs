use std::slice;

use binding::vm;
use class::object;
use types;

use class::traits::RawObject;

pub struct VM;

impl VM {
    pub fn init() {
        vm::init();
    }

    pub fn parse_arguments(argc: types::argc, arguments: *const types::rb_value) -> Vec<object::Object> {
        let arguments = unsafe {
            slice::from_raw_parts(arguments, argc as usize).to_vec()
        };

        arguments.iter().map(|value| object::Object::from(*value)).collect()
    }

    pub fn create_arguments(arguments: Vec<object::Object>) -> (types::c_int, *const types::rb_value) {
        (arguments.len() as types::c_int, Self::arguments_as_ptr(arguments))
    }

    pub fn parse_itself(itself: types::rb_value) -> object::Object {
        object::Object::from(itself)
    }

    fn arguments_as_ptr(arguments: Vec<object::Object>) -> *const types::rb_value {
        arguments
            .iter()
            .map(|object| object.value())
            .collect::<Vec<types::rb_value>>()
            .as_ptr()
    }
}
