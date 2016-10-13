use std::convert::From;
use std::iter::{FromIterator, IntoIterator, Iterator};

use binding::array;
use types::{Value, ValueType};

use {AnyObject, RString, Object, VerifiedObject};

/// `Array`
#[derive(Debug, PartialEq)]
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
        Self::from(array::new())
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
    /// use ruru::{Array, Fixnum, Object, VM};
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).try_convert_to::<Fixnum>(), Ok(Fixnum::new(1)));
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
    /// use ruru::{Array, Fixnum, Object, VM};
    /// # VM::init();
    ///
    /// let mut array = Array::new();
    ///
    /// array.push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).try_convert_to::<Fixnum>(), Ok(Fixnum::new(1)));
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
    /// use ruru::{Array, Fixnum, Object, VM};
    /// # VM::init();
    ///
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// array.store(0, Fixnum::new(2));
    ///
    /// assert_eq!(array.at(0).try_convert_to::<Fixnum>(), Ok(Fixnum::new(2)));
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

    /// Removes and returns the last element of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new().push(Fixnum::new(1));
    /// let last_element = array.pop().try_convert_to::<Fixnum>();
    /// assert_eq!(last_element, Ok(Fixnum::new(1)));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// array[0] = 2
    /// last_element = array.pop
    /// last_element == 2
    /// ```
    pub fn pop(&mut self) -> AnyObject {
        let result = array::pop(self.value());

        AnyObject::from(result)
    }

    /// Inserts `item` at the beggining of the array
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// array.unshift(Fixnum::new(2));
    ///
    /// assert_eq!(array.at(0).try_convert_to::<Fixnum>(), Ok(Fixnum::new(2)));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// array.unshift 2
    ///
    /// array[0] == 2
    /// ```
    pub fn unshift<T: Object>(&mut self, item: T) -> AnyObject {
        let result = array::unshift(self.value(), item.value());
        AnyObject::from(result)
    }

    /// Removes the first `item` of the array and moves the rest of the items
    /// one position back
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new().push(Fixnum::new(1));
    /// array.push(Fixnum::new(2));
    ///
    /// array.shift();
    ///
    /// assert_eq!(array.at(0).try_convert_to::<Fixnum>(), Ok(Fixnum::new(2)));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// array.push 2
    /// array.shift
    ///
    /// array[0] == 2
    /// ```
    pub fn shift(&mut self) -> AnyObject {
        let result = array::shift(self.value());
        AnyObject::from(result)
    }

    /// Creates a copy of the array
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// let copy = array.dup();
    ///
    /// assert_eq!(
    ///    array.at(0).try_convert_to::<Fixnum>(),
    ///    copy.at(0).try_convert_to::<Fixnum>());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// copy = array.dup
    ///
    /// array[0] == copy[0]
    /// ```
    pub fn dup(&mut self) -> Array {
        let result = array::dup(self.value());
        Array::from(result)
    }

    /// Concatenates the array elements together without spacing
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// VM::init();
    /// let mut array = Array::new().push(Fixnum::new(1)).push(Fixnum::new(2));
    /// let string = array.to_s().to_string();
    ///
    /// assert_eq!(string, "[1, 2]".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1,2]
    /// str = array.to_s
    ///
    /// str == "[1, 2]"
    /// ```
    pub fn to_s(&mut self) -> RString {
        let result = array::to_s(self.value());
        RString::from(result)
    }

    /// Reverse the order of all of the elements in array
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new()
    ///                    .push(Fixnum::new(1))
    ///                    .push(Fixnum::new(2))
    ///                    .push(Fixnum::new(3));
    ///
    /// array.reverse();
    ///
    /// assert_eq!(
    ///    array.at(0).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(3))
    /// );
    /// assert_eq!(
    ///    array.at(1).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(2))
    /// );
    /// assert_eq!(
    ///    array.at(2).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(1))
    /// );
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1,2,3]
    /// array.reverse!
    ///
    /// array[0] == 3
    /// array[1] == 2
    /// array[2] == 1
    /// ```
    pub fn reverse(&mut self) -> Array {
        let result = array::reverse(self.value());
        Array::from(result)
    }

    /// Appends the elements of `other` to `self`
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new()
    ///                    .push(Fixnum::new(1))
    ///                    .push(Fixnum::new(2))
    ///                    .push(Fixnum::new(3));
    ///
    /// let other = Array::new()
    ///                    .push(Fixnum::new(4))
    ///                    .push(Fixnum::new(5))
    ///                    .push(Fixnum::new(6));
    /// let array = array.concat(other);
    ///
    /// assert_eq!(
    ///    array.at(3).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(4))
    /// );
    /// assert_eq!(
    ///    array.at(4).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(5))
    /// );
    /// assert_eq!(
    ///    array.at(5).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(6))
    /// );
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1,2,3]
    /// other = [4,5,6]
    ///
    /// array[3] == 4
    /// array[4] == 5
    /// array[5] == 6
    /// ```
    pub fn concat(&mut self, other: Array) -> Array {
        let result = array::concat(self.value(), other.value());
        Array::from(result)
    }

    /// Returns a new array created by sorting self
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new()
    ///                    .push(Fixnum::new(100))
    ///                    .push(Fixnum::new(-1))
    ///                    .push(Fixnum::new(5));
    ///
    /// let sorted_array = array.sort();
    ///
    /// assert_eq!(
    ///    sorted_array.at(0).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(-1))
    /// );
    /// assert_eq!(
    ///    sorted_array.at(1).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(5))
    /// );
    /// assert_eq!(
    ///    sorted_array.at(2).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(100))
    /// );
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [100,-1,5]
    /// sorted_array = array.sort
    ///
    /// sorted_array[0] == -1
    /// sorted_array[1] == 5
    /// sorted_array[2] == 100
    /// ```
    pub fn sort(&mut self) -> Array {
        let result = array::sort(self.value());
        Array::from(result)
    }

    /// Sorts array in place
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, Object, VM};
    /// VM::init();
    /// let mut array = Array::new()
    ///                    .push(Fixnum::new(100))
    ///                    .push(Fixnum::new(-1))
    ///                    .push(Fixnum::new(5));
    ///
    /// array.sort_bang();
    ///
    /// assert_eq!(
    ///    array.at(0).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(-1))
    /// );
    /// assert_eq!(
    ///    array.at(1).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(5))
    /// );
    /// assert_eq!(
    ///    array.at(2).try_convert_to::<Fixnum>(),
    ///    Ok(Fixnum::new(100))
    /// );
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [100,-1,5]
    /// array.sort!
    ///
    /// array[0] == -1
    /// array[1] == 5
    /// array[2] == 100
    /// ```
    pub fn sort_bang(&mut self) -> Array {
        let result = array::sort_bang(self.value());
        Array::from(result)
    }
}

impl From<Value> for Array {
    fn from(value: Value) -> Self {
        Array { value: value }
    }
}

impl Object for Array {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Array {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::Array
    }

    fn error_message() -> &'static str {
        "Error converting to Boolean"
    }
}

pub struct ArrayIterator {
    array: Array,
    current_index: i64,
}

impl ArrayIterator {
    fn new(array: Array) -> ArrayIterator {
        ArrayIterator {
            array: array,
            current_index: 0,
        }
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
/// use ruru::{Array, Fixnum, Object, VM};
/// # VM::init();
///
/// let mut array = Array::new().push(Fixnum::new(1));
/// array.push(Fixnum::new(2));
/// array.push(Fixnum::new(3));
/// let mut sum: i64 = 0;
///
/// for item in array.into_iter() {
///     sum += item.try_convert_to::<Fixnum>().unwrap().to_i64();
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
/// use ruru::{Array, Fixnum, Object, VM};
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
///     assert_eq!(array.at(i).try_convert_to::<Fixnum>().unwrap().to_i64(), expected_number);
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
