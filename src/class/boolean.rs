use std::convert::From;

use types::rb_value;
use util::{bool_to_value, value_to_bool};

use super::traits::RawObject;

pub struct Boolean {
    value: rb_value
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

impl From<rb_value> for Boolean {
    fn from(value: rb_value) -> Self {
        Boolean {
            value: value
        }
    }
}

impl RawObject for Boolean {
    fn value(&self) -> rb_value {
        self.value
    }
}
