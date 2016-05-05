use binding::util::get_type;
use types::{Value, ValueType};

use super::traits::Object;

/// Representation of any Ruby object while its type is unknown
///
/// As Ruby is a dynamically typed language, at some points Ruru does not know the exact Ruby type
/// of the object. It happens in the following cases:
///
/// - Retrieving an object from array
///
/// - Retrieving an object from hash
///
/// - Receiving arguments to callback
///
/// - Initializing new instance of a class which is not present in Ruru
///
/// - and some other places you can find in Ruru documentation
///
/// In these cases you should cast `AnyObject` to the type to which the object belongs
/// using functions below.
///
/// # Examples
///
/// ## Retrieving an object from `Array`
///
/// ```
/// use ruru::{Array, Fixnum, VM};
/// # VM::init();
///
/// let array = Array::new().push(Fixnum::new(1));
/// let value = array.at(0).to::<Fixnum>(); // `Array::at()` returns `AnyObject`
///
/// assert_eq!(value.to_i64(), 1);
/// ```
///
/// ## Retrieving an object from `Hash`
///
/// ```no_run
/// use ruru::{Fixnum, Hash, Symbol, VM};
/// # VM::init();
///
/// let mut hash = Hash::new();
///
/// hash.store(Symbol::new("key"), Fixnum::new(1));
///
/// // `Hash::at()` returns `AnyObject`
/// let value = hash.at(Symbol::new("key")).to::<Fixnum>();
///
/// assert_eq!(value, Fixnum::new(1));
/// ```
///
/// ## Receiving arguments from Ruby to Ruru callback
///
/// ```no_run
/// use ruru::types::Argc;
/// use ruru::{AnyObject, Boolean, Class, RString, VM};
///
/// #[no_mangle]
/// pub extern fn string_eq(argc: Argc, argv: *const AnyObject, itself: RString) -> Boolean {
///     // `VM::parse_arguments()` returns `Vec<AnyObject>`
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
#[derive(Clone)]
pub struct AnyObject {
    value: Value,
}

impl AnyObject {
    /// Casts `AnyObject` to the specified Ruby type
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{AnyObject, Fixnum, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let fixnum_as_any_object = Fixnum::new(1).to_any_object();
    ///
    /// let fixnum = fixnum_as_any_object.to::<Fixnum>();
    ///
    /// assert_eq!(fixnum.to_i64(), 1);
    /// ```
    pub fn to<T: Object>(&self) -> T {
        T::from(self.value)
    }

    /// Determines the value type of the object
    ///
    /// # Example
    ///
    /// ```
    /// use ruru::{AnyObject, Fixnum, VM};
    /// use ruru::traits::Object;
    /// use ruru::types::ValueType;
    /// # VM::init();
    ///
    /// let any_object = Fixnum::new(1).to_any_object();
    ///
    /// assert_eq!(any_object.ty(), ValueType::Fixnum);
    /// ```
    pub fn ty(&self) -> ValueType {
        get_type(self.value())
    }
}

impl From<Value> for AnyObject {
    fn from(value: Value) -> Self {
        AnyObject { value: value }
    }
}

impl Object for AnyObject {
    fn value(&self) -> Value {
        self.value
    }
}
