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
///         let name = name
///             .map(|name| name.to_string())
///             .unwrap_or("Anonymous".to_string());
///
///         let greeting = format!("Hello dear {}!", name);
///
///         RString::new(&greeting)
///     }
/// );
///
/// fn main() {
///     # VM::init();
///     Class::new("Greeter", None).define(|itself| {
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
        #[derive(Debug, PartialEq)]
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

/// Creates unsafe callbacks for Ruby methods
///
/// This macro is unsafe, because:
///
///  - it uses automatic unsafe conversions for arguments
///     (no guarantee that Ruby objects match the types which you expect);
///  - no bound checks for the array of provided arguments
///     (no guarantee that all the expected arguments are provided);
///
/// That is why creating callbacks in unsafe way may cause panics.
///
/// Due to the same reasons unsafe callbacks are faster.
///
/// Use it when:
///
///  - you own the Ruby code which passes arguments to callback;
///  - you are sure that all the object has correct type;
///  - you are sure that all the required arguments are provided;
///  - Ruby code has a good test coverage.
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate ruru;
///
/// use ruru::{Boolean, Class, Fixnum, RString, VM};
///
/// // Creates `string_length_equals` functions
/// unsafe_methods!(
///     RString, // type of `self` object
///     itself, // name of `self` object which will be used in methods
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
macro_rules! unsafe_methods {
    (
        $itself_class: ty,
        $itself_name: ident,
        $(
            fn $method_name: ident
            ($($arg_name: ident: $arg_type: ty),*) -> $return_type: ty $body: block
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

/// Creates callbacks for Ruby methods
///
/// Unlike `unsafe_methods!`, this macro is safe, because:
///
///  - it uses safe conversions of arguments (`Object::try_convert_to()`);
///  - it checks if arguments are present;
///
/// Each argument will have type `Result<Object, String>`.
///
/// For example, if you declare `number: Fixnum` in the method definition, it will have actual
/// type `number: Result<Fixnum, String>`.
///
///
///
/// See examples below and docs for `Object::try_convert_to()` for more information.
///
/// # Examples
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
/// use ruru::{Class, Fixnum, Hash, NilClass, Symbol, VM};
/// use ruru::traits::Object;
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
                    let $arg_name =
                        _arguments
                            .get(_i)
                            .ok_or({
                                format!(
                                    "Argument {}: {} not found for method {}",
                                    stringify!($arg_name),
                                    stringify!($arg_type),
                                    stringify!($method_name)
                                )
                            }).and_then(|argument| {
                                <$crate::AnyObject as $crate::traits::Object>
                                    ::try_convert_to::<$arg_type>(argument)
                            });

                    _i += 1;
                )*

                $body
            }
        )*
    }
}
