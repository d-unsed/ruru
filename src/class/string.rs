use std::convert::From;

use binding::string;
use types;

use super::traits::RawObject;

pub struct RString {
    value: types::rb_value
}

impl RString {
    pub fn new(string: &str) -> Self {
        RString {
            value: string::new(string)
        }
    }

    pub fn to_string(&self) -> String {
        string::from_value(self.value)
    }
}

impl From<types::rb_value> for RString {
    fn from(value: types::rb_value) -> Self {
        RString {
            value: value
        }
    }
}

impl RawObject for RString {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
