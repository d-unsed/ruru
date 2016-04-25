extern crate libc;

pub mod types;
pub mod vm;

use types::Value;

extern "C" {
    pub static rb_cObject: Value;
}

