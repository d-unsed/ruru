use types::{rb_signed_value, rb_value};
use unsafe_binding::fixnum::{rb_int2inum, rb_num2int};

pub fn int_to_num(num: i64) -> rb_value {
    unsafe {
        rb_int2inum(num as rb_signed_value)
    }
}

pub fn num_to_int(num: rb_value) -> i64 {
    unsafe {
        rb_num2int(num)
    }
}
