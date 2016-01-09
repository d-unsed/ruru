use std::convert::From;

use binding::class::{instance_variable_get, instance_variable_set, object_class};
use binding::util::call_method;
use types::Value;
use util::create_arguments;

use super::any_object::AnyObject;
use super::class::Class;

pub trait Object : From<Value> {
    fn value(&self) -> Value;

    fn class(&self) -> Class {
        let class = object_class(self.value());

        Class::from(class)
    }

    fn send(&self, method: &str, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = create_arguments(arguments);

        let result = call_method(self.value(), method, argc, argv);

        AnyObject::from(result)
    }

    fn as_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }

    fn instance_variable_get(&self, variable: &str) -> AnyObject {
        let result = instance_variable_get(self.value(), variable);

        AnyObject::from(result)
    }

    fn instance_variable_set<T: Object>(&mut self, variable: &str, value: T) -> AnyObject {
        let result = instance_variable_set(self.value(), variable, value.value());

        AnyObject::from(result)
    }
}
