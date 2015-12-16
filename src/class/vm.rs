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

    pub fn parse_itself(itself: types::rb_value) -> object::Object {
        object::Object::from(itself)
    }
}
