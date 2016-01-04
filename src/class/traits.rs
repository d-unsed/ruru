use std::convert::From;

use binding::class::object_class;
use binding::util::call_method;
use types::rb_value;
use util::create_arguments;

use super::class::Class;
use super::object::Object;

pub trait RawObject : From<rb_value> {
    fn value(&self) -> rb_value;

    fn class(&self) -> Class {
        let class = object_class(self.value());

        Class::from(class)
    }

    fn send(&self, method: &str, arguments: Vec<Object>) -> Object {
        let (argc, argv) = create_arguments(arguments);

        let result = call_method(self.value(), method, argc, argv);

        Object::from(result)
    }
}
