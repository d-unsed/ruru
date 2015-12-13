use std::convert::From;

use binding::global;
use types;
use util;

use super::traits::RawObject;

pub struct Boolean {
    value: types::rb_value
}

impl Boolean {
    pub fn new(state: bool) -> Self {
        Boolean {
            value: util::bool_to_value(state)
        }
    }

    pub fn to_bool(&self) -> bool {
        util::value_to_bool(self.value)
    }
}

impl From<types::rb_value> for Boolean {
    fn from(value: types::rb_value) -> Self {
        Boolean {
            value: value
        }
    }
}

impl RawObject for Boolean {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
