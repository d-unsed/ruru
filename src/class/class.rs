use std::convert::From;

use binding::class;
use binding::global::rb_cObject;
use binding::util as binding_util;
use types::{Callback, Value, ValueType};
use util;

use {AnyObject, Object, VerifiedObject};

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

        Class { value: class::define_class(name, superclass) }
    }

    // TODO: replace rb_cObject with optional class
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
        Class { value: binding_util::get_constant(name, rb_cObject) }
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
    /// # Arguments
    ///
    /// - `name` is name of the Ruby method
    ///
    /// - `callback` is the function which will be called by MRI when the method is called inside
    ///    Ruby
    ///
    /// ## Callback
    ///
    /// Use `methods!` macro instead of manually creating callbacks.
    ///
    /// `callback` must have the following signature:
    ///
    /// `pub type Callback<I: Object, O: Object> = extern fn(Argc, *const AnyObject, I) -> O;`
    ///
    /// The function must also have `#[no_mangle]` attribute.
    ///
    /// - First argument `argc: Argc` will receive the number of arguments passed to method
    ///
    /// - Second argument `argv: *const AnyObject` will receive the arguments passed to the method
    ///   as `AnyObject`s
    ///
    /// - Third value `itself: I` will receive the object which got the method call (Ruby `self`).
    ///   Can be any type which implements `Object` trait
    ///
    /// - Return type can be any type which implements `Object` trait
    ///
    /// If you need to receive and use arguments which are passed to the method, you can use
    /// `VM::parse_arguments()` function which processes a pointer to array (`*const AnyObject`)
    /// to a vector of `AnyObject`s (`Vec<AnyObject>`), see examples.
    ///
    /// # Examples
    ///
    /// ## Method receives no arguments
    ///
    /// In this case `argc` and `argv` can be ignored.
    ///
    /// Famous `String#blank?` example
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Boolean, Class, Object, RString};
    ///
    /// #[no_mangle]
    /// pub extern fn string_blank(_: Argc, _: *const AnyObject, itself: RString) -> Boolean {
    ///     Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
    /// }
    ///
    /// fn main() {
    ///     Class::from_existing("String").define_method("blank?", string_blank);
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
    /// ## Method receives arguments
    ///
    /// Arguments should be processed to vector using `VM::parse_arguments()`.
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Boolean, Class, Object, RString, VM};
    ///
    /// #[no_mangle]
    /// pub extern fn string_eq(argc: Argc, argv: *const AnyObject, itself: RString) -> Boolean {
    ///     let argv = VM::parse_arguments(argc, argv);
    ///     let other_string = argv[0].try_convert_to::<RString>().unwrap();
    ///
    ///     Boolean::new(itself.to_string() == other_string.to_string())
    /// }
    ///
    /// fn main() {
    ///     Class::from_existing("String").define_method("==", string_eq);
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class String
    ///   def ==(other_string)
    ///     # simplified
    ///     self.chars == other_string.chars
    ///   end
    /// end
    /// ```
    pub fn define_method<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        class::define_method(self.value, name, callback);
    }

    /// Defines a class method for given class.
    ///
    /// Function has the same requirements as `define_method`.
    /// Also the same rules are applied for `callback` (see above).
    ///
    /// Use `methods!` macro instead of manually creating callbacks.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Class, Object, RString, Symbol, VM};
    ///
    /// #[no_mangle]
    /// pub extern fn symbol_from_string(argc: Argc,
    ///                                  argv: *const AnyObject,
    ///                                  itself: Class) -> Symbol {
    ///     let argv = VM::parse_arguments(argc, argv);
    ///     let string = argv[0].try_convert_to::<RString>().unwrap();
    ///
    ///     Symbol::new(&string.to_string())
    /// }
    ///
    /// fn main() {
    ///     Class::from_existing("Symbol")
    ///         .define_singleton_method("from_string", symbol_from_string);
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Symbol
    ///   def self.from_string(string)
    ///     # simplified
    ///     string.to_sym
    ///   end
    /// end
    /// ```
    pub fn define_singleton_method<I: Object, O: Object>(&mut self,
                                                         name: &str,
                                                         callback: Callback<I, O>) {
        class::define_singleton_method(self.value, name, callback);
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
    ///
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Outer
    ///   class Inner
    ///   end
    /// end
    /// ```
    pub fn define_nested_class(&mut self, name: &str, superclass: Option<&Class>) -> Self {
        let superclass = Self::superclass_to_value(superclass);

        Class { value: class::define_nested_class(self.value(), name, superclass) }
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
