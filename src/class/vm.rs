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

    pub fn parse_arguments(argc: types::argc, arguments: *const object::Object) -> Vec<object::Object> {
        unsafe {
            slice::from_raw_parts(arguments, argc as usize).to_vec()
        }
    }

    pub fn parse_itself(itself: types::rb_value) -> object::Object {
        object::Object::from(itself)
    }
}
