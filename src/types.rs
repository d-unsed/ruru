use AnyObject;

pub use ruby_sys::types::{Argc, c_char, c_int, c_long, CallbackPtr, CallbackMutPtr, c_void, Id,
                          InternalValue, RawFd, RbDataType as DataType,
                          RbDataTypeFunction as DataTypeFunction, SignedValue, size_t, Value,
                          ValueType};

pub type Callback<I, O> = extern "C" fn(Argc, *const AnyObject, I) -> O;
