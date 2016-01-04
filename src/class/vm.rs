use std::slice;

use binding::vm::init;
use class::object::Object;
use types::{argc, rb_value};

use class::traits::RawObject;

pub struct VM;

impl VM {
    pub fn init() {
        init();
    }

    pub fn parse_arguments(argc: argc, arguments: *const Object) -> Vec<Object> {
        unsafe {
            slice::from_raw_parts(arguments, argc as usize).to_vec()
        }
    }

    pub fn parse_itself(itself: rb_value) -> Object {
        Object::from(itself)
    }
}
