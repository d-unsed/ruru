use std::convert::From;

use binding::array;
use types;

use super::{object, string};
use super::traits::RawObject;

pub struct Array {
    value: types::rb_value
}

impl Array {
    pub fn new() -> Self {
        Array {
            value: array::new()
        }
    }

    pub fn at(&self, index: i64) -> object::Object {
        let value = array::entry(self.value(), index);

        object::Object::from(value)
    }

    pub fn join(&self, separator: string::RString) -> string::RString {
        let value = array::join(self.value(), separator.value());

        string::RString::from(value)
    }

    pub fn push<T: RawObject>(&mut self, item: T) -> &mut Self {
        array::push(self.value(), item.value());

        self
    }

    pub fn store<T: RawObject>(&mut self, index: i64, item: T) -> object::Object {
        let value = array::store(self.value(), index, item.value());

        object::Object::from(value)
    }
}

impl From<types::rb_value> for Array {
    fn from(value: types::rb_value) -> Self {
        Array {
            value: value
        }
    }
}

impl RawObject for Array {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
