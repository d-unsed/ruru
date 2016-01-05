use std::slice;

use binding::vm::init;
use class::object::Object;
use types::{Argc, Value};

pub struct VM;

impl VM {
    pub fn init() {
        init();
    }

    pub fn parse_arguments(argc: Argc, arguments: *const Object) -> Vec<Object> {
        unsafe {
            slice::from_raw_parts(arguments, argc as usize).to_vec()
        }
    }

    pub fn parse_itself(itself: Value) -> Object {
        Object::from(itself)
    }
}
