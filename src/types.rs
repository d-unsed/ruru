use class::any_object::AnyObject;

pub use ruby_sys::types::{Argc, c_char, c_int, c_long, CallbackPtr, CallbackMutPtr, Id,
                          InternalValue, SignedValue, Value, ValueType};

pub type Callback<I, O> = extern "C" fn(Argc, *const AnyObject, I) -> O;
