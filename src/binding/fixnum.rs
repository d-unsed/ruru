use types::{SignedValue, Value};
use ruby_sys::fixnum::{rb_int2inum, rb_num2int};

pub fn int_to_num(num: i64) -> Value {
    unsafe {
        rb_int2inum(num as SignedValue)
    }
}

pub fn num_to_int(num: Value) -> i64 {
    unsafe {
        rb_num2int(num)
    }
}
