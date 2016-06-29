use std::convert::From;

use binding::rproc::call;
use types::Value;
use util::create_arguments;

use super::any_object::AnyObject;
use super::traits::Object;

/// `Proc` (works with `Lambda` as well)
pub struct Proc {
    value: Value,
}

impl Proc {
    /// Calls a proc with given arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{Class, Proc, RString};
    /// use ruru::traits::Object;
    ///
    /// class!(Greeter);
    ///
    /// methods!(
    ///     Greeter,
    ///     itself,
    ///
    ///     fn greet_rust_with(greeting_template: Proc) -> RString {
    ///         let name = RString::new("Rust").to_any_object();
    ///
    ///         greeting_template.call(vec![name]).to::<RString>()
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Greeter").define(|itself| {
    ///         itself.def_self("greet_rust_with", greet_rust_with);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Greeter
    ///   def self.greet_rust_with(greeting_template)
    ///     greeting_template.call('Rust')
    ///   end
    /// end
    ///
    /// greeting_template = -> (name) { "Hello, #{name}!" }
    ///
    /// Greeter.greet_rust_with(greeting_template) # => "Hello, Rust!"
    /// ```
    pub fn call(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = create_arguments(arguments);
        let value = call(self.value, argc, argv.as_ptr());

        AnyObject::from(value)
    }
}

impl From<Value> for Proc {
    fn from(value: Value) -> Self {
        Proc { value: value }
    }
}

impl Object for Proc {
    fn value(&self) -> Value {
        self.value
    }
}
