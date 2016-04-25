extern crate libc;

pub mod types;
pub mod hash;
pub mod string;
pub mod symbol;
pub mod util;
pub mod vm;

use types::Value;

extern "C" {
    pub static rb_cObject: Value;
}

