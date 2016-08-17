use std::convert::From;
use std::iter::{FromIterator, IntoIterator, Iterator};

use binding::array;
use types::{Value, ValueType};

use AnyObject;
use RString;
use traits::{Object, VerifiedObject};

/// `Array`
pub struct Array {
    value: Value,
}

impl Array {
    /// Creates a new instance of empty `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, VM};
    /// # VM::init();
    ///
    /// Array::new();
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// []
    /// ```
    pub fn new() -> Self {
        Array { value: array::new() }
    }

    /// Retrieves the length of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    ///
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// assert_eq!(array.length(), 1);
    ///
    /// array.push(Fixnum::new(2));
    ///
    /// assert_eq!(array.length(), 2);
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// array.length == 1
    ///
    /// array << 2
    /// array.length == 2
    /// ```
    pub fn length(&self) -> usize {
        array::len(self.value()) as usize
    }

    /// Retrieves an `AnyObject` from element at `index` position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).to::<Fixnum>(), Fixnum::new(1));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    ///
    /// array[0] == 1
    /// ```
    pub fn at(&self, index: i64) -> AnyObject {
        let result = array::entry(self.value(), index);

        AnyObject::from(result)
    }

    /// Joins all elements of `Array` to Ruby `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, RString, VM};
    /// # VM::init();
    ///
    /// let array =
    ///     Array::new()
    ///         .push(RString::new("Hello"))
    ///         .push(RString::new("World!"));
    ///
    /// let joined_string = array.join(RString::new(", ")).to_string();
    ///
    /// assert_eq!(joined_string, "Hello, World!".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = ['Hello', 'World!']
    ///
    /// array.join(', ') == 'Hello, World!'
    /// ```
    pub fn join(&self, separator: RString) -> RString {
        let result = array::join(self.value(), separator.value());

        RString::from(result)
    }

    /// Pushes an object to `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let mut array = Array::new();
    ///
    /// array.push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).to::<Fixnum>(), Fixnum::new(1));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = []
    /// array << 1
    ///
    /// array[0] == 1
    /// ```
    pub fn push<T: Object>(&mut self, item: T) -> Self {
        let result = array::push(self.value(), item.value());

        Array::from(result)
    }

    /// Stores an object at `index` position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// use ruru::traits::Object;
    /// # VM::init();
    ///
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// array.store(0, Fixnum::new(2));
    ///
    /// assert_eq!(array.at(0).to::<Fixnum>(), Fixnum::new(2));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// array[0] = 2
    ///
    /// array[0] == 2
    /// ```
    pub fn store<T: Object>(&mut self, index: i64, item: T) -> AnyObject {
        let result = array::store(self.value(), index, item.value());

        AnyObject::from(result)
    }
}

impl From<Value> for Array {
    fn from(value: Value) -> Self {
        Array { value: value }
    }
}

impl Object for Array {
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Array {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::Array
    }
}

pub struct ArrayIterator {
    array: Array,
    current_index: i64,
}

impl ArrayIterator {
    fn new(array: Array) -> ArrayIterator {
        ArrayIterator { array: array, current_index: 0 }
    }
}

impl Iterator for ArrayIterator {
    type Item = AnyObject;

    fn next(&mut self) -> Option<AnyObject> {
        let item = if (self.current_index as usize) < self.len() {
            Some(self.array.at(self.current_index))
        } else {
            None
        };

        self.current_index += 1;

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total = self.len() as usize;
        (total, Some(total))
    }
}

impl ExactSizeIterator for ArrayIterator {
    fn len(&self) -> usize {
        self.array.length() as usize
    }
}

/// Allows Arrays to be iterable in Rust.
///
/// # Examples
///
/// ```
/// use ruru::{Array, Fixnum, VM};
/// use ruru::traits::Object;
/// # VM::init();
///
/// let mut array = Array::new().push(Fixnum::new(1));
/// array.push(Fixnum::new(2));
/// array.push(Fixnum::new(3));
/// let mut sum: i64 = 0;
///
/// for item in array.into_iter() {
///     sum += item.to::<Fixnum>().to_i64();
/// }
///
/// assert_eq!(sum, 6);
/// ```
impl IntoIterator for Array {
    type Item = AnyObject;
    type IntoIter = ArrayIterator;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIterator::new(self)
    }
}

/// Converts an iterator into `Array`.
///
/// # Examples
///
/// ```
/// use ruru::{Array, Fixnum, VM};
/// use ruru::traits::Object;
/// # VM::init();
///
/// let array: Array = (1..6)
///     .map(|num| num * 2)
///     .map(|num| Fixnum::new(num).to_any_object())
///     .collect();
///
/// assert_eq!(array.length(), 5);
///
/// for i in 0..5 {
///     let expected_number = (i + 1) * 2;
///
///     assert_eq!(array.at(i).to::<Fixnum>().to_i64(), expected_number);
/// }
/// ```
impl FromIterator<AnyObject> for Array {
    fn from_iter<I: IntoIterator<Item = AnyObject>>(iter: I) -> Self {
        let mut array = Array::new();

        for i in iter {
            array.push(i);
        }

        array
    }
}
