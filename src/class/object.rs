use std::convert::From;

use types;

use super::traits::RawObject;

pub struct Object {
    value: types::rb_value
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
