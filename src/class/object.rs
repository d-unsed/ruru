use types;

use super::traits::RawObject;

pub struct Object {
    value: types::rb_value
}

impl RawObject for Object {
    fn value(&self) -> types::rb_value {
        self.value
    }

    fn from_value(value: types::rb_value) -> Self {
        Object {
            value: value
        }
    }
}
