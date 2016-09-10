use std::convert::From;

use binding::class;
use binding::global::rb_cObject;
use binding::util as binding_util;
use types::{Value, ValueType};
use util;

use {AnyObject, Array, Object, VerifiedObject};

/// `Class`
///
/// Also see `def`, `def_self`, `define` and some more functions from `Object` trait.
///
/// ```rust
/// #[macro_use] extern crate ruru;
///
/// use ruru::{Class, Fixnum, Object, VM};
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
/// );
///
/// fn main() {
///     # VM::init();
///     Class::from_existing("Fixnum").define(|itself| {
///         itself.def("pow", pow);
///     });
/// }
/// ```
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
    ///
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
    /// use ruru::{Class, Object, VM};
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
    /// use ruru::{Class, Object, VM};
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

    /// Defines an `attr_reader` for class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, Object, VM};
    /// # VM::init();
    ///
    /// Class::new("Test", None).define(|itself| {
    ///     itself.attr_reader("reader");
    /// });
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Test
    ///   attr_reader :reader
    /// end
    /// ```
    pub fn attr_reader(&mut self, name: &str) {
        class::define_attribute(self.value(), name, true, false);
    }

    /// Defines an `attr_writer` for class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, Object, VM};
    /// # VM::init();
    ///
    /// Class::new("Test", None).define(|itself| {
    ///     itself.attr_writer("writer");
    /// });
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Test
    ///   attr_writer :writer
    /// end
    /// ```
    pub fn attr_writer(&mut self, name: &str) {
        class::define_attribute(self.value(), name, false, true);
    }

    /// Defines an `attr_accessor` for class
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, Object, VM};
    /// # VM::init();
    ///
    /// Class::new("Test", None).define(|itself| {
    ///     itself.attr_accessor("accessor");
    /// });
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Test
    ///   attr_accessor :accessor
    /// end
    /// ```
    pub fn attr_accessor(&mut self, name: &str) {
        class::define_attribute(self.value(), name, true, true);
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
