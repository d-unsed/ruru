use types;

use super::{class, array};

pub trait RawObject {
    fn from_value(value: types::rb_value) -> Self;

    fn value(&self) -> types::rb_value;

    fn as_class(&self) -> class::Class {
        class::Class::from_value(self.value())
    }

    fn as_array(&self) -> array::Array {
        array::Array::from_value(self.value())
    }
}
