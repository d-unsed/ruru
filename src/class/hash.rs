use std::convert::From;

use binding::hash::{aset, aref, new};
use types::Value;

use super::any_object::AnyObject;
use super::traits::RawObject;

pub struct Hash {
    value: Value
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            value: new()
        }
    }

    pub fn at<T: RawObject>(&self, key: T) -> AnyObject {
        let value = aref(self.value(), key.value());

        AnyObject::from(value)
    }

    pub fn store<K: RawObject, V: RawObject>(&mut self, key: K, value: V) -> AnyObject {
        let value = aset(self.value(), key.value(), value.value());

        AnyObject::from(value)
    }
}

impl From<Value> for Hash {
    fn from(value: Value) -> Self {
        Hash {
            value: value
        }
    }
}

impl RawObject for Hash {
    fn value(&self) -> Value {
        self.value
    }
}
