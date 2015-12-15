use std::convert::From;

use types;

use super::{array, class, fixnum, hash, string};
use super::traits::RawObject;

pub struct Object {
    value: types::rb_value
}

impl Object {
    pub fn as_array(&self) -> array::Array {
        array::Array::from(self.value())
    }

    pub fn as_class(&self) -> class::Class {
        class::Class::from(self.value())
    }

    pub fn as_fixnum(&self) -> fixnum::Fixnum {
        fixnum::Fixnum::from(self.value())
    }

    pub fn as_hash(&self) -> hash::Hash {
        hash::Hash::from(self.value())
    }

    pub fn as_string(&self) -> string::RString {
        string::RString::from(self.value())
    }
}

impl From<types::rb_value> for Object {
    fn from(value: types::rb_value) -> Self {
        Object {
            value: value
        }
    }
}

impl RawObject for Object {
    fn value(&self) -> types::rb_value {
        self.value
    }
}
