use std::convert::From;

use binding::class;
use binding::global::ValueType;
use binding::util as binding_util;
use types::Value;
use util;

use {AnyObject, Class, VerifiedObject};

/// `Object`
///
/// Trait consists methods of Ruby `Object` class. Every struct like `Array`, `Hash` etc implements
/// this trait.
///
/// `class!` macro automatically implements this trait for custom classes.
pub trait Object: From<Value> {
    /// Returns internal `value` of current object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::types::Value;
    /// use ruru::Object;
    ///
    /// struct Array {
    ///     value: Value
    /// }
    ///
    /// impl From<Value> for Array {
    ///     fn from(value: Value) -> Self {
    ///         Array {
    ///             value: value
    ///         }
    ///     }
    /// }
    ///
    /// impl Object for Array {
    ///     fn value(&self) -> Value {
    ///         self.value
    ///     }
    /// }
    /// ```
    #[inline]
    fn value(&self) -> Value;

    /// Returns a `Class` struct of current object.
    ///
    /// # Examples
    /// ```
    /// use ruru::{Array, Object, VM};
    /// # VM::init();
    ///
    /// assert_eq!(Array::new().class(), Array::new().class());
    /// ```
    fn class(&self) -> Class {
        let class = class::object_class(self.value());

        Class::from(class)
    }

    /// Calls a given method on an object similarly to Ruby `Object#send` method
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Array, Fixnum, Object, RString, VM};
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    /// let array_to_str =
    ///     array
    ///         .send("to_s", vec![])
    ///         .try_convert_to::<RString>()
    ///         .unwrap()
    ///         .to_string();
    ///
    /// assert_eq!(array_to_str, "[1]".to_string());
    /// ```
    fn send(&self, method: &str, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = util::create_arguments(arguments);

        let result = binding_util::call_method(self.value(), method, argc, argv.as_ptr());

        AnyObject::from(result)
    }

    /// Checks whether the object responds to given method
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Object, VM};
    /// # VM::init();
    ///
    /// let array = Array::new();
    ///
    /// assert!(array.respond_to("push"));
    /// assert!(!array.respond_to("something_else"));
    /// ```
    fn respond_to(&self, method: &str) -> bool {
        class::respond_to(self.value(), method)
    }

    /// Checks whether the object is `nil`
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Hash, NilClass, Object, VM};
    /// # VM::init();
    ///
    /// assert!(NilClass::new().is_nil());
    /// assert!(!Hash::new().is_nil());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// nil.nil? == true
    /// {}.nil? == false
    /// ```
    fn is_nil(&self) -> bool {
        self.value().is_nil()
    }

    /// Converts struct to `AnyObject`
    ///
    /// See docs for `AnyObject` class for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Array, Fixnum, Object, VM};
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    /// let args = vec![Fixnum::new(1).to_any_object()];
    /// let index =
    ///     array
    ///         .send("find_index", args)
    ///         .try_convert_to::<Fixnum>();
    ///
    /// assert_eq!(index, Ok(Fixnum::new(0)));
    /// ```
    fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }

    /// Gets an instance variable of object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{AnyObject, Class, Fixnum, Object, VM};
    /// use ruru::types::Argc;
    ///
    /// class!(Counter);
    ///
    /// methods!(
    ///     Counter,
    ///     itself,
    ///
    ///     fn counter_initialize() -> AnyObject {
    ///         itself.instance_variable_set("@state", Fixnum::new(0))
    ///     }
    ///
    ///     fn counter_increment() -> AnyObject {
    ///         // Using unsafe conversion, because we are sure that `@state` is always a `Fixnum`
    ///         // and we don't provide an interface to set the value externally
    ///         let state = unsafe {
    ///             itself.instance_variable_get("@state").to::<Fixnum>().to_i64()
    ///         };
    ///
    ///         itself.instance_variable_set("@state", Fixnum::new(state + 1))
    ///     }
    ///
    ///     fn counter_state() -> Fixnum {
    ///         unsafe { itself.instance_variable_get("@state").to::<Fixnum>() }
    ///     }
    /// );
    ///
    /// fn main() {
    ///     # VM::init();
    ///     let counter = Class::new("Counter", None).define(|itself| {
    ///         itself.def("initialize", counter_initialize);
    ///         itself.def("increment!", counter_increment);
    ///         itself.def("state", counter_state);
    ///     }).new_instance(vec![]);
    ///
    ///     counter.send("increment!", vec![]);
    ///
    ///     let new_state = counter.send("state", vec![]).try_convert_to::<Fixnum>();
    ///
    ///     assert_eq!(new_state, Ok(Fixnum::new(1)));
    /// }
    /// ```
    fn instance_variable_get(&self, variable: &str) -> AnyObject {
        let result = class::instance_variable_get(self.value(), variable);

        AnyObject::from(result)
    }

    /// Sets an instance variable for object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{AnyObject, Class, Fixnum, Object, VM};
    /// use ruru::types::Argc;
    ///
    /// class!(Counter);
    ///
    /// methods!(
    ///     Counter,
    ///     itself,
    ///
    ///     fn counter_initialize() -> AnyObject {
    ///         itself.instance_variable_set("@state", Fixnum::new(0))
    ///     }
    ///
    ///     fn counter_increment() -> AnyObject {
    ///         // Using unsafe conversion, because we are sure that `@state` is always a `Fixnum`
    ///         // and we don't provide an interface to set the value externally
    ///         let state = unsafe {
    ///             itself.instance_variable_get("@state").to::<Fixnum>().to_i64()
    ///         };
    ///
    ///         itself.instance_variable_set("@state", Fixnum::new(state + 1))
    ///     }
    ///
    ///     fn counter_state() -> Fixnum {
    ///         unsafe { itself.instance_variable_get("@state").to::<Fixnum>() }
    ///     }
    /// );
    ///
    /// fn main() {
    ///     # VM::init();
    ///     let counter = Class::new("Counter", None).define(|itself| {
    ///         itself.def("initialize", counter_initialize);
    ///         itself.def("increment!", counter_increment);
    ///         itself.def("state", counter_state);
    ///     }).new_instance(vec![]);
    ///
    ///     counter.send("increment!", vec![]);
    ///
    ///     let new_state = counter.send("state", vec![]).try_convert_to::<Fixnum>();
    ///
    ///     assert_eq!(new_state, Ok(Fixnum::new(1)));
    /// }
    /// ```
    fn instance_variable_set<T: Object>(&mut self, variable: &str, value: T) -> AnyObject {
        let result = class::instance_variable_set(self.value(), variable, value.value());

        AnyObject::from(result)
    }

    /// Unsafely casts current object to the specified Ruby type
    ///
    /// This operation in unsafe, because it does not perform any validations on the object, but
    /// it is faster than `try_convert_to()`.
    ///
    /// Use it when:
    ///
    ///  - you own the Ruby code which passes the object to Rust;
    ///  - you are sure that the object always has correct type;
    ///  - Ruby code has a good test coverage.
    ///
    /// This function is used by `unsafe_methods!` macro for argument casting.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{AnyObject, Fixnum, Object, VM};
    /// # VM::init();
    ///
    /// let fixnum_as_any_object = Fixnum::new(1).to_any_object();
    ///
    /// let fixnum = unsafe { fixnum_as_any_object.to::<Fixnum>() };
    ///
    /// assert_eq!(fixnum.to_i64(), 1);
    /// ```
    unsafe fn to<T: Object>(&self) -> T {
        T::from(self.value())
    }

    /// Safely casts current object to the specified Ruby type
    ///
    /// This function is used by `methods!` macro for argument casting.
    ///
    /// See documentation for `VerifiedObject` trait to enable safe conversions
    /// for custom classes.
    ///
    /// # Examples
    ///
    /// ### Basic conversions
    ///
    /// ```
    /// use ruru::{Fixnum, Object, RString, VM};
    /// # VM::init();
    ///
    /// let fixnum_as_any_object = Fixnum::new(1).to_any_object();
    /// let converted_fixnum = fixnum_as_any_object.try_convert_to::<Fixnum>();
    ///
    /// assert_eq!(converted_fixnum, Ok(Fixnum::new(1)));
    ///
    /// let string = RString::new("string");
    /// let string_as_fixnum = string.try_convert_to::<Fixnum>();
    ///
    /// assert_eq!(string_as_fixnum, Err("Error converting to Fixnum".to_string()));
    /// ```
    ///
    /// ### Method arguments
    ///
    /// To launch a server in Rust, you plan to write a simple `Server` class
    ///
    /// ```ruby
    /// class Server
    ///   def start(address)
    ///     # ...
    ///   end
    /// end
    /// ```
    ///
    /// The `address` must be `Hash` with the following structure:
    ///
    /// ```ruby
    /// {
    ///   host: 'localhost',
    ///   port: 8080,
    /// }
    /// ```
    ///
    /// You want to extract port from it. Default port is `8080` in case when:
    ///
    ///  - `address` is not a `Hash`
    ///  - `address[:port]` is not present
    ///  - `address[:port]` is not a `Fixnum`
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{Class, Fixnum, Hash, NilClass, Object, Symbol, VM};
    ///
    /// class!(Server);
    ///
    /// methods!(
    ///     Server,
    ///     itself,
    ///
    ///     fn start(address: Hash) -> NilClass {
    ///         let default_port = 8080;
    ///
    ///         let port = address
    ///             .map(|hash| hash.at(Symbol::new("port")))
    ///             .and_then(|port| port.try_convert_to::<Fixnum>())
    ///             .map(|port| port.to_i64())
    ///             .unwrap_or(default_port);
    ///
    ///         // Start server...
    ///
    ///         NilClass::new()
    ///     }
    /// );
    ///
    /// fn main() {
    ///     # VM::init();
    ///     Class::new("Server", None).define(|itself| {
    ///         itself.def("start", start);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Server
    ///   def start(address)
    ///     default_port = 8080
    ///
    ///     port =
    ///       if address.is_a?(Hash) && address[:port].is_a?(Fixnum)
    ///         address[:port]
    ///       else
    ///         default_port
    ///       end
    ///
    ///     # Start server...
    ///   end
    /// end
    /// ```
    fn try_convert_to<T: VerifiedObject>(&self) -> Result<T, String> {
        if T::is_correct_type(self) {
            let converted_object = unsafe { self.to::<T>() };

            Ok(converted_object)
        } else {
            Err(T::error_message())
        }
    }

    /// Determines the value type of the object
    ///
    /// # Example
    ///
    /// ```
    /// use ruru::{AnyObject, Fixnum, Object, VM};
    /// use ruru::types::ValueType;
    /// # VM::init();
    ///
    /// let any_object = Fixnum::new(1).to_any_object();
    ///
    /// assert_eq!(any_object.ty(), ValueType::Fixnum);
    /// ```
    fn ty(&self) -> ValueType {
        self.value().ty()
    }
}
