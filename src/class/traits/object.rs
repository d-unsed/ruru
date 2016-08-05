use std::convert::From;

use binding::class;
use binding::global::ValueType;
use binding::util as binding_util;
use types::Value;
use util;

use AnyObject;
use Class;

/// `Object`
///
/// Trait consists methods of Ruby `Object` class. Every struct like `Array`, `Hash` etc implements
/// this trait.
pub trait Object: From<Value> {
    /// Usually this function just returns a `value` of current object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::types::Value;
    /// use ruru::traits::Object;
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
    fn value(&self) -> Value;

    /// Returns a `Class` struct of current object
    ///
    /// # Examples
    /// ```
    /// use ruru::{Array, VM};
    /// use ruru::traits::Object;
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
    /// use ruru::{Array, Fixnum, RString, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    /// let array_to_str =
    ///     array
    ///         .send("to_s", vec![])
    ///         .to::<RString>()
    ///         .to_string();
    ///
    /// assert_eq!(array_to_str, "[1]".to_string());
    /// ```
    fn send(&self, method: &str, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = util::create_arguments(arguments);

        let result = binding_util::call_method(self.value(), method, argc, argv.as_ptr());

        AnyObject::from(result)
    }

    /// Checks weather the object is `nil`
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Hash, NilClass, VM};
    /// use ruru::traits::Object;
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
    /// use ruru::{Array, Fixnum, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    /// let args = vec![Fixnum::new(1).to_any_object()];
    /// let index =
    ///     array
    ///         .send("find_index", args)
    ///         .to::<Fixnum>()
    ///         .to_i64();
    ///
    /// assert_eq!(index, 0);
    /// ```
    fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }

    /// Sets an instance variable for object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{AnyObject, Class, Fixnum, VM};
    /// use ruru::types::Argc;
    /// use ruru::traits::Object;
    ///
    /// #[no_mangle]
    /// pub extern fn counter_initialize(_: Argc,
    ///                                  _: *const AnyObject,
    ///                                  mut itself: AnyObject) -> AnyObject {
    ///     itself.instance_variable_set("@state", Fixnum::new(0))
    /// }
    ///
    /// #[no_mangle]
    /// pub extern fn counter_increment(_: Argc,
    ///                                 _: *const AnyObject,
    ///                                 mut itself: AnyObject) -> AnyObject {
    ///     let state = itself.instance_variable_get("@state").to::<Fixnum>().to_i64();
    ///
    ///     itself.instance_variable_set("@state", Fixnum::new(state + 1))
    /// }
    ///
    /// #[no_mangle]
    /// pub extern fn counter_state(_: Argc, _: *const AnyObject, itself: AnyObject) -> Fixnum {
    ///     itself.instance_variable_get("@state").to::<Fixnum>()
    /// }
    ///
    /// fn main() {
    ///     # VM::init();
    ///     let counter = Class::new("Counter").define(|itself| {
    ///         itself.def("initialize", counter_initialize);
    ///         itself.def("increment!", counter_increment);
    ///         itself.def("state", counter_state);
    ///     }).new_instance(vec![]);
    ///
    ///     counter.send("increment!", vec![]);
    ///
    ///     let new_state = counter.send("state", vec![]).to::<Fixnum>().to_i64();
    ///
    ///     assert_eq!(new_state, 1);
    /// }
    /// ```
    fn instance_variable_get(&self, variable: &str) -> AnyObject {
        let result = class::instance_variable_get(self.value(), variable);

        AnyObject::from(result)
    }

    /// Gets an instance variable of object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{AnyObject, Class, Fixnum, VM};
    /// use ruru::types::Argc;
    /// use ruru::traits::Object;
    ///
    /// #[no_mangle]
    /// pub extern fn counter_initialize(_: Argc,
    ///                                  _: *const AnyObject,
    ///                                  mut itself: AnyObject) -> AnyObject {
    ///     itself.instance_variable_set("@state", Fixnum::new(0))
    /// }
    ///
    /// #[no_mangle]
    /// pub extern fn counter_increment(_: Argc,
    ///                                 _: *const AnyObject,
    ///                                 mut itself: AnyObject) -> AnyObject {
    ///     let state = itself.instance_variable_get("@state").to::<Fixnum>().to_i64();
    ///
    ///     itself.instance_variable_set("@state", Fixnum::new(state + 1))
    /// }
    ///
    /// #[no_mangle]
    /// pub extern fn counter_state(_: Argc, _: *const AnyObject, itself: AnyObject) -> Fixnum {
    ///     itself.instance_variable_get("@state").to::<Fixnum>()
    /// }
    ///
    /// fn main() {
    ///     # VM::init();
    ///     let counter = Class::new("Counter").define(|itself| {
    ///         itself.def("initialize", counter_initialize);
    ///         itself.def("increment!", counter_increment);
    ///         itself.def("state", counter_state);
    ///     }).new_instance(vec![]);
    ///
    ///     counter.send("increment!", vec![]);
    ///
    ///     let new_state = counter.send("state", vec![]).to::<Fixnum>().to_i64();
    ///
    ///     assert_eq!(new_state, 1);
    /// }
    /// ```
    fn instance_variable_set<T: Object>(&mut self, variable: &str, value: T) -> AnyObject {
        let result = class::instance_variable_set(self.value(), variable, value.value());

        AnyObject::from(result)
    }

    /// Casts current object to the specified Ruby type
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{AnyObject, Fixnum, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let fixnum_as_any_object = Fixnum::new(1).to_any_object();
    ///
    /// let fixnum = fixnum_as_any_object.to::<Fixnum>();
    ///
    /// assert_eq!(fixnum.to_i64(), 1);
    /// ```
    fn to<T: Object>(&self) -> T {
        T::from(self.value())
    }

    /// Determines the value type of the object
    ///
    /// # Example
    ///
    /// ```
    /// use ruru::{AnyObject, Fixnum, VM};
    /// use ruru::traits::Object;
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
