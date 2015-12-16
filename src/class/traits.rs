use std::convert::From;

use binding;
use binding::util;
use types;

use super::{class, object};

pub trait RawObject : From<types::rb_value> {
    fn value(&self) -> types::rb_value;

    fn class(&self) -> class::Class {
        let class = binding::class::object_class(self.value());

        class::Class::from(class)
    }

    fn send(&self, method: &str, arguments: Vec<object::Object>) -> object::Object {
        let (argc, argv) = ::util::create_arguments(arguments);

        let result = util::call_method(self.value(), method, argc, argv);

        object::Object::from(result)
    }
}
