use std::convert::From;

use binding::string::{from_value, new};
use types::rb_value;

use super::traits::RawObject;

pub struct RString {
    value: rb_value
}

impl RString {
    pub fn new(string: &str) -> Self {
        RString {
            value: new(string)
        }
    }

    pub fn to_string(&self) -> String {
        from_value(self.value)
    }
}

impl From<rb_value> for RString {
    fn from(value: rb_value) -> Self {
        RString {
            value: value
        }
    }
}

impl RawObject for RString {
    fn value(&self) -> rb_value {
        self.value
    }
}
