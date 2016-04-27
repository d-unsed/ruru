#[macro_export]

/// Creates callbacks for Ruby methods
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate ruru;
///
/// use ruru::{AnyObject, Boolean, Class, Fixnum, RString, VM};
/// use ruru::types::Argc;
/// use ruru::traits::Object;
///
/// // Creates `string_is_blank` and `string_length_equals` functions
/// methods!(
///     RString, // type of `self` object
///     itself, // name of `self` object which will be used in methods
///
///     string_is_blank() -> Boolean {
///         Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
///     }
///
///     string_length_equals(expected_length: Fixnum) -> Boolean {
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
macro_rules! methods {
    (
        $itself_class: ty,
        $itself_name: ident,
        $(
            $method_name: ident
            ($($arg_name: ident: $arg_type: ty),*) -> $return_type: ident $body: block
        )*
    ) => {
        $(
            #[no_mangle]
            pub extern fn $method_name(argc: Argc,
                                       argv: *const AnyObject,
                                       mut $itself_name: $itself_class) -> $return_type {
                let arguments = VM::parse_arguments(argc, argv);
                let mut i = 0;

                $(
                    let $arg_name = arguments[i].to::<$arg_type>();
                    i += 1;
                )*

                $body
            }
        )*
    }
}
