use Object;

/// Interface for safe conversions between types
///
/// This trait is required by `Object::convert_to()` function.
///
/// All built-in types like `Hash`, `RString` etc implement it.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate ruru;
///
/// use ruru::{Class, Fixnum, Object, VerifiedObject, VM};
///
/// class!(Server);
///
/// impl VerifiedObject for Server {
///     fn is_correct_type<T: Object>(object: &T) -> bool {
///         object.class() == Class::from_existing("Server")
///     }
///
///     fn error_message() -> String {
///         "Error converting to Server".to_string()
///     }
/// }
///
/// fn main() {
///     # VM::init();
///     Class::new("Server", None);
///
///     let server = Class::from_existing("Server").new_instance(vec![]).to_any_object();
///
///     let server_unsafe = unsafe { server.to::<Server>() };
///     let server_safe = server.try_convert_to::<Server>();
///
///     assert_eq!(server_safe, Ok(server_unsafe));
/// }
/// ```
pub trait VerifiedObject: Object {
    fn is_correct_type<T: Object>(object: &T) -> bool;
    fn error_message() -> String;
}
