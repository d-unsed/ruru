use std::convert::From;

use binding::hash::{aset, aref, new};
use types::Value;

use super::any_object::AnyObject;
use super::traits::Object;

pub struct Hash {
    value: Value
}

impl Hash {
    pub fn new() -> Self {
        Hash {
            value: new()
        }
    }

    pub fn at<T: Object>(&self, key: T) -> AnyObject {
        let value = aref(self.value(), key.value());

        AnyObject::from(value)
    }

    pub fn store<K: Object, V: Object>(&mut self, key: K, value: V) -> AnyObject {
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

impl Object for Hash {
    fn value(&self) -> Value {
        self.value
    }
}
