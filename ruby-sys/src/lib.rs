extern crate libc;

pub mod types;
pub mod hash;
pub mod vm;

use types::Value;

extern "C" {
    pub static rb_cObject: Value;
}

