/// Creates Rust structure for new Ruby class
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate ruru;
///
/// use ruru::{Class, RString, VM};
///
/// class!(Greeter);
///
/// methods!(
///     Greeter,
///     itself,
///
///     fn anonymous_greeting() -> RString {
///         RString::new("Hello stranger!")
///     }
///
///     fn friendly_greeting(name: RString) -> RString {
///         let greeting = format!("Hello dear {}!", name.to_string());
///
///         RString::new(&greeting)
///     }
/// );
///
/// fn main() {
///     # VM::init();
///     Class::new("Greeter").define(|itself| {
///         itself.def("anonymous_greeting", anonymous_greeting);
///         itself.def("friendly_greeting", friendly_greeting);
///     });
/// }
/// ```
///
/// Ruby:
///
/// ```ruby
/// class Greeter
///   def anonymous_greeting
///     'Hello stranger!'
///   end
///
///   def friendly_greeting(name)
///     "Hello dear #{name}"
///   end
/// end
/// ```
#[macro_export]
macro_rules! class {
    ($class: ident) => {
        pub struct $class {
            value: $crate::types::Value,
        }

        impl From<$crate::types::Value> for $class {
            fn from(value: $crate::types::Value) -> Self {
                $class { value: value }
            }
        }

        impl $crate::traits::Object for $class {
            fn value(&self) -> $crate::types::Value {
                self.value
            }
        }
    }
}

/// Creates callbacks for Ruby methods
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate ruru;
///
/// use ruru::{Boolean, Class, Fixnum, RString, VM};
///
/// // Creates `string_is_blank` and `string_length_equals` functions
/// methods!(
///     RString, // type of `self` object
///     itself, // name of `self` object which will be used in methods
///
///     fn string_is_blank() -> Boolean {
///         Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
///     }
///
///     fn string_length_equals(expected_length: Fixnum) -> Boolean {
///         let real_length = itself.to_string().len() as i64;
///
///         Boolean::new(expected_length.to_i64() == real_length)
///     }
/// );
///
/// fn main() {
///     # VM::init();
///     Class::from_existing("String").define(|itself| {
///         itself.def("blank?", string_is_blank);
///         itself.def("length_equals?", string_length_equals);
///     });
/// }
/// ```
///
/// Ruby:
///
/// ```ruby
/// class String
///   def blank?
///     # ...
///   end
///
///   def length_equals?(expected_length)
///     # ...
///   end
/// end
/// ```
#[macro_export]
macro_rules! methods {
    (
        $itself_class: ty,
        $itself_name: ident,
        $(
            fn $method_name: ident
            ($($arg_name: ident: $arg_type: ty),*) -> $return_type: ident $body: block
        )*
    ) => {
        $(
            #[no_mangle]
            #[allow(unused_mut)]
            pub extern fn $method_name(argc: $crate::types::Argc,
                                       argv: *const $crate::AnyObject,
                                       mut $itself_name: $itself_class) -> $return_type {
                let _arguments = $crate::VM::parse_arguments(argc, argv);
                let mut _i = 0;

                $(
                    let $arg_name = unsafe {
                        <$crate::AnyObject as $crate::traits::Object>
                        ::to::<$arg_type>(&_arguments[_i])
                    };

                    _i += 1;
                )*

                $body
            }
        )*
    }
}
