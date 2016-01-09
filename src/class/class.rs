use std::convert::From;

use binding::class::{define_class, new_instance, define_method, define_singleton_method};
use binding::global::rb_cObject;
use binding::util::get_constant;
use types::{Callback, Value};
use util::create_arguments;

use super::any_object::AnyObject;
use super::traits::Object;

pub struct Class {
    value: Value
}

impl Class {
    // TODO: replace rb_cObject with optional superclass
    pub fn new(name: &str) -> Self {
        Class {
            value: define_class(name, rb_cObject)
        }
    }

    // TODO: replace rb_cObject with optional class/module value
    pub fn from_existing(name: &str) -> Self {
        Class {
            value: get_constant(name, rb_cObject)
        }
    }

    pub fn new_instance(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = create_arguments(arguments);
        let instance = new_instance(self.value(), argc, argv);

        AnyObject::from(instance)
    }

    pub fn define<F: Fn(&Self)>(&self, f: F) -> &Self {
        f(&self);

        self
    }

    pub fn define_method<T: Object>(&self, name: &str, callback: Callback<T>) {
        define_method::<T>(self.value, name, callback);
    }

    pub fn define_singleton_method<T: Object>(&self, name: &str, callback: Callback<T>) {
        define_singleton_method(self.value, name, callback);
    }

    pub fn def<T: Object>(&self, name: &str, callback: Callback<T>) {
        self.define_method(name, callback);
    }

    pub fn def_self<T: Object>(&self, name: &str, callback: Callback<T>) {
        self.define_singleton_method(name, callback);
    }
}

impl From<Value> for Class {
    fn from(value: Value) -> Self {
        Class {
            value: value
        }
    }
}

impl Object for Class {
    fn value(&self) -> Value {
        self.value
    }
}
