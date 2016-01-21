use std::convert::From;

use binding::array::{entry, join, new, push, store};
use types::Value;

use super::any_object::AnyObject;
use super::string::RString;

use super::traits::Object;

/// `Array`
pub struct Array {
    value: Value
}

impl Array {
    /// Creates a new instance of empty `Array`
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Array, VM};
    /// # VM::init();
    /// Array::new();
    /// ```
    pub fn new() -> Self {
        Array {
            value: new()
        }
    }

    /// Retrieves an `AnyObject` from element at `index` position
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    /// let array = Array::new().push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).as_fixnum(), Fixnum::new(1));
    /// ```
    pub fn at(&self, index: i64) -> AnyObject {
        let value = entry(self.value(), index);

        AnyObject::from(value)
    }

    /// Joins all elements of `Array` to Ruby `String`
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Array, RString, VM};
    /// # VM::init();
    /// let array =
    ///     Array::new()
    ///         .push(RString::new("Hello"))
    ///         .push(RString::new("World!"));
    ///
    /// let joined_string = array.join(RString::new(", ")).to_string();
    ///
    /// assert_eq!(joined_string, "Hello, World!".to_string());
    pub fn join(&self, separator: RString) -> RString {
        let value = join(self.value(), separator.value());

        RString::from(value)
    }

    /// Pushes an object to `Array`
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    /// let mut array = Array::new();
    ///
    /// array.push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).as_fixnum(), Fixnum::new(1));
    /// ```
    pub fn push<T: Object>(&mut self, item: T) -> Self {
        let value = push(self.value(), item.value());

        Array::from(value)
    }

    /// Stores an object at `index` position
    ///
    /// # Examples
    ///
    /// ```
    /// # use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// array.store(0, Fixnum::new(2));
    ///
    /// assert_eq!(array.at(0).as_fixnum(), Fixnum::new(2));
    pub fn store<T: Object>(&mut self, index: i64, item: T) -> AnyObject {
        let value = store(self.value(), index, item.value());

        AnyObject::from(value)
    }
}

impl From<Value> for Array {
    fn from(value: Value) -> Self {
        Array {
            value: value
        }
    }
}

impl Object for Array {
    fn value(&self) -> Value {
        self.value
    }
}
