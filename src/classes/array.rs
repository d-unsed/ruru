use bindings::array;
use types;

use super::traits;
use super::traits::RawObject;

pub struct Array {
    value: types::rb_value
}

impl Array {
    fn new() -> Self {
        Array {
            value: array::array_new()
        }
    }

    fn push<T: traits::RawObject>(&mut self, item: T) -> &mut Self {
        array::array_push(self.value(), item.value());

        self
    }
}

impl traits::RawObject for Array {
    fn value(&self) -> types::rb_value {
        self.value
    }

    fn from_value(value: types::rb_value) -> Self {
        Array {
            value: value
        }
    }
}
