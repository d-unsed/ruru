pub use libc::{c_char, c_int, c_long};

use libc::uintptr_t;
use libc::intptr_t;

pub type Value = uintptr_t;
pub type SignedValue = intptr_t;
pub type Id = uintptr_t;

pub type Argc = c_int;

