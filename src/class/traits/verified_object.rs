use traits::Object;

/// Conversion
///
pub trait VerifiedObject: Object {
    fn is_correct_type<T: Object>(object: &T) -> bool;
}
