use traits::Object;

/// Interface for save conversions between types.
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
/// use ruru::{Class, Fixnum, VM};
/// use ruru::traits::{Object, VerifiedObject};
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
/// }
/// ```
pub trait VerifiedObject: Object {
    fn is_correct_type<T: Object>(object: &T) -> bool;
    fn error_message() -> String;
}
