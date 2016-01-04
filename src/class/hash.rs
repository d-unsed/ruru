use std::convert::From;

use binding::hash::{aset, aref, new};
use types::rb_value;

use super::object::Object;
use super::traits::RawObject;

pub struct Hash {
    value: rb_value
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            value: new()
        }
    }

    pub fn at<T: RawObject>(&self, key: T) -> Object {
        let value = aref(self.value(), key.value());

        Object::from(value)
    }

    pub fn store<K: RawObject, V: RawObject>(&mut self, key: K, value: V) -> Object {
        let value = aset(self.value(), key.value(), value.value());

        Object::from(value)
    }
}

impl From<rb_value> for Hash {
    fn from(value: rb_value) -> Self {
        Hash {
            value: value
        }
    }
}

impl RawObject for Hash {
    fn value(&self) -> rb_value {
        self.value
    }
}
