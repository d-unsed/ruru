use libc::{c_void, intptr_t, uintptr_t};

pub use libc::{c_char, c_int, c_long};
pub use value::{Value, ValueType};

pub type Id = uintptr_t;
pub type InternalValue = uintptr_t;
pub type SignedValue = intptr_t;

pub type Argc = c_int;
pub type CallbackPtr = *const c_void;
