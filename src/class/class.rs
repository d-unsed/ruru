use std::convert::From;

use binding::class;
use binding::global::rb_cObject;
use binding::util as binding_util;
use types::{Callback, Value};
use util;

use AnyObject;
use traits::Object;

/// `Class`
#[derive(Debug, PartialEq)]
pub struct Class {
    value: Value,
}

impl Class {
    // TODO: replace rb_cObject with optional superclass
    /// Creates a new `Class`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Class, VM};
    /// # VM::init();
    ///
    /// let class = Class::new("Hello");
    ///
    /// assert_eq!(class, Class::from_existing("Hello"));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Hello
    /// end
    ///
    /// # or
    ///
    /// Hello = Class.new
    /// ```
    pub fn new(name: &str) -> Self {
        Class { value: class::define_class(name, rb_cObject) }
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
        Class { value: binding_util::get_constant(name, rb_cObject) }
    }

    /// Creates a new instance of `Class`
    ///
    /// Arguments must be passed as a vector of `AnyObject` (see example).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Class, Fixnum};
    /// use ruru::traits::Object;
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

    /// Wraps calls to a class.
    ///
    /// Mostly used to have Ruby-like class definitions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Class, Fixnum, RString};
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
    /// use ruru::{AnyObject, Boolean, Class, RString};
    ///
    /// use ruru::traits::Object;
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
    /// use ruru::{AnyObject, Boolean, Class, RString, VM};
    ///
    /// #[no_mangle]
    /// pub extern fn string_eq(argc: Argc, argv: *const AnyObject, itself: RString) -> Boolean {
    ///     let argv = VM::parse_arguments(argc, argv);
    ///     let other_string = argv[0].to::<RString>();
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
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::types::Argc;
    /// use ruru::{AnyObject, Class, RString, Symbol, VM};
    ///
    /// #[no_mangle]
    /// pub extern fn symbol_from_string(argc: Argc,
    ///                                  argv: *const AnyObject,
    ///                                  itself: Class) -> Symbol {
    ///     let argv = VM::parse_arguments(argc, argv);
    ///     let string = argv[0].to::<RString>();
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

    /// An alias for `define_method` (similar to Ruby syntax `def some_method`).
    pub fn def<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        self.define_method(name, callback);
    }

    /// An alias for `define_singleton_method` (similar to Ruby `def self.some_method`).
    pub fn def_self<I: Object, O: Object>(&mut self, name: &str, callback: Callback<I, O>) {
        self.define_singleton_method(name, callback);
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
