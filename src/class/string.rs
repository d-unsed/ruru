use std::convert::From;

use binding::string::{from_value, new};
use types::Value;

use super::traits::Object;

pub struct RString {
    value: Value
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

impl From<Value> for RString {
    fn from(value: Value) -> Self {
        RString {
            value: value
        }
    }
}

impl Object for RString {
    fn value(&self) -> Value {
        self.value
    }
}
