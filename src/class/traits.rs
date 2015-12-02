use binding::util;
use types;

use super::{array, class, object, string};

pub trait RawObject {
    fn from_value(value: types::rb_value) -> Self;

    fn value(&self) -> types::rb_value;

    fn as_array(&self) -> array::Array {
        array::Array::from_value(self.value())
    }

    fn as_class(&self) -> class::Class {
        class::Class::from_value(self.value())
    }

    fn as_string(&self) -> string::RString {
        string::RString::from_value(self.value())
    }

    fn send<T: RawObject>(&self, method: &str, arguments: Vec<T>) -> object::Object {
        let arguments = arguments.iter().map(|object| object.value()).collect();
        let result = util::call_method(self.value(), method, arguments);

        object::Object::from_value(result)
    }
}
