use std::convert::From;

use binding::array::{entry, join, new, push, store};
use types::rb_value;

use super::object::Object;
use super::string::RString;

use super::traits::RawObject;

pub struct Array {
    value: rb_value
}

impl Array {
    pub fn new() -> Self {
        Array {
            value: new()
        }
    }

    pub fn at(&self, index: i64) -> Object {
        let value = entry(self.value(), index);

        Object::from(value)
    }

    pub fn join(&self, separator: RString) -> RString {
        let value = join(self.value(), separator.value());

        RString::from(value)
    }

    pub fn push<T: RawObject>(&mut self, item: T) -> &mut Self {
        push(self.value(), item.value());

        self
    }

    pub fn store<T: RawObject>(&mut self, index: i64, item: T) -> Object {
        let value = store(self.value(), index, item.value());

        Object::from(value)
    }
}

impl From<rb_value> for Array {
    fn from(value: rb_value) -> Self {
        Array {
            value: value
        }
    }
}

impl RawObject for Array {
    fn value(&self) -> rb_value {
        self.value
    }
}
