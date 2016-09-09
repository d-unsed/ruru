use std::convert::From;

use binding::class;
use binding::global::rb_cObject;
use binding::util as binding_util;
use types::{Callback, Value, ValueType};
use util;

use {AnyObject, Array, Object, VerifiedObject};

/// `Class`
#[derive(Debug, PartialEq)]
pub struct Class {
    value: Value,
}

impl Class {
    /// Creates a new `Class`.
    ///
    /// `superclass` can receive the following values:
    ///
    ///  - `None` to inherit from `Object` class
    ///     (standard Ruby behavior when superclass is not given explicitly);
    ///  - `Some(&Class)` to inherit from the given class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// let basic_record_class = Class::new("BasicRecord", None);
    ///
    /// assert_eq!(basic_record_class, Class::from_existing("BasicRecord"));
    /// assert_eq!(basic_record_class.superclass(), Some(Class::from_existing("Object")));
    ///
    /// let record_class = Class::new("Record", Some(&basic_record_class));
    ///
    /// assert_eq!(record_class, Class::from_existing("Record"));
    /// assert_eq!(record_class.superclass(), Some(Class::from_existing("BasicRecord")));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class BasicRecord
    /// end
    ///
    /// class Record < BasicRecord
    /// end
    ///
    /// BasicRecord.superclass == Object
    /// Record.superclass == BasicRecord
    /// ```
    pub fn new(name: &str, superclass: Option<&Self>) -> Self {
        let superclass = Self::superclass_to_value(superclass);

        Self::from(class::define_class(name, superclass))
    }

    /// Retrieves an existing `Class` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// let class = Class::new("Record", None);
    ///
    /// assert_eq!(class, Class::from_existing("Record"));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Record
    /// end
    ///
    /// # get class
    ///
    /// Record
    ///
    /// # or
    ///
    /// Object.const_get('Record')
    /// ```
    pub fn from_existing(name: &str) -> Self {
        Self::from(binding_util::get_constant(name, rb_cObject))
    }

    /// Creates a new instance of `Class`
    ///
    /// Arguments must be passed as a vector of `AnyObject` (see example).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Class, Fixnum, Object};
    ///
    /// // Without arguments
    /// Class::from_existing("Hello").new_instance(vec![]);
    ///
    /// // With arguments passing arguments to constructor
    /// let arguments = vec![
    ///     Fixnum::new(1).to_any_object(),
    ///     Fixnum::new(2).to_any_object()
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
        let (argc, argv) = util::create_arguments(arguments);
        let instance = class::new_instance(self.value(), argc, argv.as_ptr());

        AnyObject::from(instance)
    }

    /// Returns a superclass of the current class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, Object, VM};
    /// # VM::init();
    ///
    /// assert_eq!(
    ///     Class::from_existing("Array").superclass(),
    ///     Some(Class::from_existing("Object"))
    /// );
    ///
    /// assert_eq!(Class::from_existing("BasicObject").superclass(), None);
    /// ```
    pub fn superclass(&self) -> Option<Class> {
        let superclass_value = class::superclass(self.value());

        match superclass_value.is_nil() {
            true => None,
            false => Some(Self::from(superclass_value))
        }
    }

    /// Returns a Vector of ancestors of current class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// let true_class_ancestors = Class::from_existing("TrueClass").ancestors();
    ///
    /// let expected_ancestors = vec![
    ///     Class::from_existing("TrueClass"),
    ///     Class::from_existing("Object"),
    ///     Class::from_existing("Kernel"),
    ///     Class::from_existing("BasicObject")
    /// ];
    ///
    /// assert_eq!(true_class_ancestors, expected_ancestors);
    /// ```
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// let basic_record_class = Class::new("BasicRecord", None);
    /// let record_class = Class::new("Record", Some(&basic_record_class));
    ///
    /// let ancestors = record_class.ancestors();
    ///
    /// assert!(ancestors.iter().any(|class| *class == basic_record_class));
    /// assert!(!ancestors.iter().any(|class| *class == Class::from_existing("String")));
    /// ```
    // Using unsafe conversions is ok, because MRI guarantees to return an `Array` of `Class`es
    pub fn ancestors(&self) -> Vec<Class> {
        let ancestors = Array::from(class::ancestors(self.value()));

        ancestors.into_iter()
            .map(|class| unsafe { class.to::<Self>() })
            .collect()
    }

    /// Retrieves a `Class` nested to current `Class`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// Class::new("Outer", None).define(|itself| {
    ///     itself.define_nested_class("Inner", None);
    /// });
    ///
    /// Class::from_existing("Outer").get_nested_class("Inner");
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Outer
    ///   class Inner
    ///   end
    /// end
    ///
    /// Outer::Inner
    ///
    /// # or
    ///
    /// Outer.const_get('Inner')
    /// ```
    pub fn get_nested_class(&self, name: &str) -> Self {
        Self::from(binding_util::get_constant(name, self.value()))
    }

    /// Creates a new `Class` nested into current class.
    ///
    /// `superclass` can receive the following values:
    ///
    ///  - `None` to inherit from `Object` class
    ///     (standard Ruby behavior when superclass is not given explicitly);
    ///  - `Some(&class)` to inherit from the given class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// Class::new("Outer", None).define(|itself| {
    ///     itself.define_nested_class("Inner", None);
    /// });
    ///
    /// Class::from_existing("Outer").get_nested_class("Inner");
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Outer
    ///   class Inner
    ///   end
    /// end
    ///
    /// Outer::Inner
    ///
    /// # or
    ///
    /// Outer.const_get('Inner')
    /// ```
    pub fn define_nested_class(&mut self, name: &str, superclass: Option<&Class>) -> Self {
        let superclass = Self::superclass_to_value(superclass);

        Self::from(class::define_nested_class(self.value(), name, superclass))
    }

    /// Wraps calls to a class.
    ///
    /// Used to have Ruby-like class definition DSL.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use] extern crate ruru;
    ///
    /// use ruru::{AnyObject, Class, Fixnum, RString};
    ///
    /// class!(Hello);
    /// class!(Nested);
    ///
    /// methods!(
    ///     Hello,
    ///     itself,
    ///
    ///     fn greeting() -> RString {
    ///         RString::new("Greeting from class")
    ///     }
    ///
    ///     fn many_greetings() -> RString {
    ///         RString::new("Many greetings from instance")
    ///     }
    /// );
    ///
    /// methods!(
    ///     Nested,
    ///     itself,
    ///
    ///     fn nested_greeting() -> RString {
    ///         RString::new("Greeting from nested class")
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Hello", None).define(|itself| {
    ///         itself.def_self("greeting", greeting);
    ///         itself.def("many_greetings", many_greetings);
    ///
    ///         itself.define_nested_class("Nested", None).define(|itself| {
    ///             itself.def_self("nested_greeting", nested_greeting);
    ///         });
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Hello
    ///   def self.greeting
    ///     'Greeting from class'
    ///   end
    ///
    ///   def many_greetings
    ///     'Many greetings from instance'
    ///   end
    ///
    ///   class Nested
    ///     def self.nested_greeting
    ///       'Greeting from nested class'
    ///     end
    ///   end
    /// end
    /// ```
    pub fn define<F: Fn(&mut Self)>(&mut self, f: F) -> &Self {
        f(self);

        self
    }

    /// Defines an instance method for given class.
    ///
    /// Use `methods!` macro to define a `callback`.
    ///
    /// You can also use `def()` alias for this function combined with `Class::define()` a for
    /// nicer DSL.
    ///
    /// # Examples
    ///
    /// ### The famous String#blank? method
    ///
    /// ```rust
    /// #[macro_use] extern crate ruru;
    ///
    /// use ruru::{Boolean, Class, RString, VM};
    ///
    /// methods!(
    ///    RString,
    ///    itself,
    ///
    ///    fn is_blank() -> Boolean {
    ///        Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
    ///    }
    /// );
    ///
    /// fn main() {
    ///     # VM::init();
    ///     Class::from_existing("String").define(|itself| {
    ///         itself.def("blank?", is_blank);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class String
    ///   def blank?
    ///     # simplified
    ///     self.chars.all? { |c| c == ' ' }
    ///   end
    /// end
    /// ```
    ///
    /// ### Receiving arguments
    ///
    /// Raise `Fixnum` to the power of `exp`.
    ///
    /// ```rust
    /// #[macro_use] extern crate ruru;
    ///
    /// use ruru::{Class, Fixnum, VM};
    ///
    /// methods!(
    ///    Fixnum,
    ///    itself,
    ///
    ///    fn pow(exp: Fixnum) -> Fixnum {
    ///         // `exp` is not a valid `Fixnum`, raise an exception
    ///         if let Err(ref message) = exp {
    ///             VM::raise(Class::from_existing("ArgumentError"), message);
    ///         }
    ///
    ///         // We can safely unwrap here, because an exception was raised if `exp` is `Err`
    ///         let exp = exp.unwrap().to_i64() as u32;
    ///         let result = itself.to_i64().pow(exp);
    ///
    ///         Fixnum::new(result)
    ///    }
    ///
    ///     fn pow_with_default_argument(exp: Fixnum) -> Fixnum {
    ///         let default_exp = 0;
    ///         let exp = exp.map(|exp| exp.to_i64()).unwrap_or(default_exp);
    ///         let result = itself.to_i64().pow(exp as u32);
    ///
    ///         Fixnum::new(result)
    ///     }
    /// );
    ///
    /// fn main() {
    ///     # VM::init();
    ///     Class::from_existing("Fixnum").define(|itself| {
    ///         itself.def("pow", pow);
    ///         itself.def("pow_with_default_argument", pow_with_default_argument);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Fixnum
    ///   def pow(exp)
    ///     raise ArgumentError unless exp.is_a?(Fixnum)
    ///
    ///     self ** exp
    ///   end
    ///
    ///   def pow_with_default_argument(exp)
    ///     default_exp = 0
    //      exp = default_exp unless exp.is_a?(Fixnum)
    ///
    ///     self ** exp
    ///   end
    /// end
    /// ```
    pub fn define_method<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        class::define_method(self.value(), name, callback);
    }

    /// Defines a class method for given class.
    ///
    /// Use `methods!` macro to define a `callback`.
    ///
    /// You can also use `def()` alias for this function combined with `Class::define()` a for
    /// nicer DSL.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use] extern crate ruru;
    ///
    /// use ruru::{Class, RString, Symbol, VM};
    ///
    /// methods!(
    ///     Symbol,
    ///     itself,
    ///
    ///     fn from_string(string: RString) -> Symbol {
    ///         // `string` is not a valid `String`, raise an exception
    ///         if let Err(ref message) = string {
    ///             VM::raise(Class::from_existing("ArgumentError"), message);
    ///         }
    ///
    ///         Symbol::new(&string.unwrap().to_string())
    ///     }
    /// );
    ///
    /// fn main() {
    ///     # VM::init();
    ///     Class::from_existing("Symbol").define(|itself| {
    ///         itself.def_self("from_string", from_string);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Symbol
    ///   def self.from_string(string)
    ///     raise ArgumentError unless string.is_a?(String)
    ///
    ///     # simplified
    ///     string.to_sym
    ///   end
    /// end
    /// ```
    pub fn define_singleton_method<I: Object, O: Object>(&mut self,
                                                         name: &str,
                                                         callback: Callback<I, O>) {
        class::define_singleton_method(self.value(), name, callback);
    }

    /// An alias for `define_method` (similar to Ruby syntax `def some_method`).
    pub fn def<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        self.define_method(name, callback);
    }

    /// An alias for `define_singleton_method` (similar to Ruby `def self.some_method`).
    pub fn def_self<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        self.define_singleton_method(name, callback);
    }

    fn superclass_to_value(superclass: Option<&Class>) -> Value {
        match superclass {
            Some(class) => class.value(),
            None => rb_cObject
        }
    }
}

impl From<Value> for Class {
    fn from(value: Value) -> Self {
        Class { value: value }
    }
}

impl Object for Class {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Class {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::Class
    }

    fn error_message() -> String {
        "Error converting to Class".to_string()
    }
}
