use std::convert::From;

use binding::hash;
use types;

use super::object;
use super::traits::RawObject;

pub struct Hash {
    value: types::rb_value
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            value: hash::new()
        }
    }

    pub fn at<T: RawObject>(&self, key: T) -> object::Object {
        let value = hash::aref(self.value(), key.value());

        object::Object::from(value)
    }

    pub fn store<T: RawObject>(&mut self, key: T, value: T) -> object::Object {
        let value = hash::aset(self.value(), key.value(), value.value());

        object::Object::from(value)
    }
}

impl From<types::rb_value> for Hash {
    fn from(value: types::rb_value) -> Self {
        Hash {
            value: value
        }
    }
}

impl RawObject for Hash {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
