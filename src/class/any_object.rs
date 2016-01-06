use std::convert::From;

use types::Value;

use super::array::Array;
use super::class::Class;
use super::fixnum::Fixnum;
use super::hash::Hash;
use super::string::RString;

use super::traits::RawObject;

#[derive(Clone)]
pub struct AnyObject {
    value: Value
}

impl AnyObject {
    pub fn as_array(&self) -> Array {
        Array::from(self.value())
    }

    pub fn as_class(&self) -> Class {
        Class::from(self.value())
    }

    pub fn as_fixnum(&self) -> Fixnum {
        Fixnum::from(self.value())
    }

    pub fn as_hash(&self) -> Hash {
        Hash::from(self.value())
    }

    pub fn as_string(&self) -> RString {
        RString::from(self.value())
    }
}

impl From<Value> for AnyObject {
    fn from(value: Value) -> Self {
        AnyObject {
            value: value
        }
    }
}

impl RawObject for AnyObject {
    fn value(&self) -> Value {
        self.value
    }
}
