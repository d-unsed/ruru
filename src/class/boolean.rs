use std::convert::From;

use types::Value;
use util::{bool_to_value, value_to_bool};

use super::traits::Object;

pub struct Boolean {
    value: Value
}

impl Boolean {
    pub fn new(state: bool) -> Self {
        Boolean {
            value: bool_to_value(state)
        }
    }

    pub fn to_bool(&self) -> bool {
        value_to_bool(self.value)
    }
}

impl From<Value> for Boolean {
    fn from(value: Value) -> Self {
        Boolean {
            value: value
        }
    }
}

impl Object for Boolean {
    fn value(&self) -> Value {
        self.value
    }
}
