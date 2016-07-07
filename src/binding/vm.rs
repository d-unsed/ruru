use ruby_sys::vm;

use types::Value;
use util;

pub fn block_proc() -> Value {
    unsafe { vm::rb_block_proc() }
}

pub fn init() {
    unsafe {
        vm::ruby_init();
    }
}

pub fn require(name: &str) {
    let name = util::str_to_cstring(name);

    unsafe {
        vm::rb_require(name.as_ptr());
    }
}
