use AnyObject;

pub use ruby_sys::types::{Argc, c_char, c_int, c_long, CallbackPtr, CallbackMutPtr, c_void, Id,
                          InternalValue, RbDataType as DataType,
                          RbDataTypeFunction as DataTypeFunction, SignedValue, size_t, Value,
                          ValueType};

#[cfg(unix)]
pub use ruby_sys::types::RawFd;

pub type Callback<I, O> = extern "C" fn(Argc, *const AnyObject, I) -> O;
