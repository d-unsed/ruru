use std::convert::From;

use binding;
use binding::util;
use types;

use super::{array, class, hash, object, string};

pub trait RawObject : From<types::rb_value> {
    fn value(&self) -> types::rb_value;

    fn class(&self) -> class::Class {
        let class = binding::class::object_class(self.value());

        class::Class::from(class)
    }

    fn as_array(&self) -> array::Array {
        array::Array::from(self.value())
    }

    fn as_class(&self) -> class::Class {
        class::Class::from(self.value())
    }

    fn as_hash(&self) -> hash::Hash {
        hash::Hash::from(self.value())
    }

    fn as_object(&self) -> object::Object {
        object::Object::from(self.value())
    }

    fn as_string(&self) -> string::RString {
        string::RString::from(self.value())
    }

    fn send(&self, method: &str, arguments: Vec<object::Object>) -> object::Object {
        let arguments = arguments.iter().map(|object| object.value()).collect();
        let result = util::call_method(self.value(), method, arguments);

        object::Object::from(result)
    }
}
