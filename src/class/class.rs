use std::convert::From;

use binding::class::{define_class, new_instance, define_method, define_singleton_method};
use binding::global::rb_cObject;
use binding::util::get_constant;
use types::{Callback, Value};
use util::create_arguments;

use super::any_object::AnyObject;
use super::traits::Object;

/// `Class`
#[derive(Debug, PartialEq)]
pub struct Class {
    value: Value
}

impl Class {
    // TODO: replace rb_cObject with optional superclass
    /// Creates a new `Class` inheriting from `Object` class
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Class, VM};
    /// # VM::init();
    /// let class = Class::new("Hello");
    ///
    /// assert_eq!(class, Class::from_existing("Hello"));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Hello
    ///
    /// end
    ///
    /// # or
    ///
    /// Hello = Class.new
    /// ```
    pub fn new(name: &str) -> Self {
        Class {
            value: define_class(name, rb_cObject)
        }
    }

    // TODO: replace rb_cObject with optional class
    /// Retrieves an existing `Class` object
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Class, VM};
    /// # VM::init();
    /// let class = Class::new("Hello");
    ///
    /// assert_eq!(class, Class::from_existing("Hello"));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// Hello
    ///
    /// # or
    ///
    /// Object.const_get('Hello')
    /// ```
    pub fn from_existing(name: &str) -> Self {
        Class {
            value: get_constant(name, rb_cObject)
        }
    }

    /// Creates a new instance of `Class`
    ///
    /// Arguments must be passed as a vector of `AnyObject` (see example)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruru::{Class, Fixnum, VM};
    /// # use ruru::traits::Object;
    /// # VM::init();
    /// // No arguments
    /// Class::from_existing("Hello").new_instance(vec![]);
    ///
    /// // Passing arguments to constructor
    /// let arguments = vec![
    ///     Fixnum::new(1).as_any_object(),
    ///     Fixnum::new(2).as_any_object()
    /// ];
    ///
    /// Class::from_existing("Worker").new_instance(arguments);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// Hello.new
    ///
    /// Worker.new(1, 2)
    /// ```
    pub fn new_instance(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = create_arguments(arguments);
        let instance = new_instance(self.value(), argc, argv);

        AnyObject::from(instance)
    }

    /// Wraps calls to a class.
    ///
    /// Mostly used to have Ruby-like class definitions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Class, Fixnum, RString};
    /// # use ruru::VM;
    ///
    /// #[no_mangle]
    /// pub extern fn greeting(_: Argc, _: *const AnyObject, _: AnyObject) -> RString {
    ///     RString::new("Greeting from class")
    /// }
    ///
    /// #[no_mangle]
    /// pub extern fn many_greetings(_: Argc, _: *const AnyObject, _: AnyObject) -> RString {
    ///     RString::new("Many greetings from instance")
    /// }
    ///
    /// fn main() {
    ///     # VM::init();
    ///     Class::new("Hello").define(|itself| {
    ///         itself.def_self("greeting", many_greetings);
    ///         itself.def("many_greetings", greeting);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Hello
    ///   def self.print_many_greetings
    ///     'Hello from class'
    ///   end
    ///
    ///   def print_greeting
    ///     'Hello from instance'
    ///   end
    /// end
    /// ```
    pub fn define<F: Fn(&mut Self)>(&mut self, f: F) -> &Self {
        f(self);

        self
    }

    /// Defines an instance method for given class
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Boolean, Class, RString};
    /// # use ruru::VM;
    ///
    /// use ruru::traits::Object;
    ///
    /// #[no_mangle]
    /// pub extern fn string_blank(_: Argc, _: *const AnyObject, itself: RString) -> Boolean {
    ///     Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
    /// }
    ///
    /// fn main() {
    ///     # VM::init();
    ///     Class::from_existing("String").define_method("blank?", string_blank);
    ///
    ///     assert!(RString::new("").send("blank?", vec![]).as_boolean().to_bool());
    /// }
    /// ```
    pub fn define_method<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        define_method(self.value, name, callback);
    }

    /// Defines a class method for given class
    pub fn define_singleton_method<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        define_singleton_method(self.value, name, callback);
    }

    /// An alias for `define_method` (similar to Ruby syntax `def some_method`)
    pub fn def<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        self.define_method(name, callback);
    }

    /// An alias for `define_singleton_method` (similar to Ruby `def self.some_method`)
    pub fn def_self<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
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
