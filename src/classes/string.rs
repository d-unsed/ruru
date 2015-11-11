use bindings::string;
use types;

use super::traits;
use super::traits::RawObject;

pub struct RString {
    value: types::rb_value
}

impl RString {
    pub fn new(string: &str) -> Self {
        RString {
            value: string::string_new(string)
        }
    }

    pub fn to_string(&self) -> String {
        string::string_from_value(self.value)
    }
}

impl traits::RawObject for RString {
    fn value(&self) -> types::rb_value {
        self.value
    }

    fn from_value(value: types::rb_value) -> Self {
        RString {
            value: value
        }
    }
}
