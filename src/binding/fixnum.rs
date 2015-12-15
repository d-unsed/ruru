use types;
use unsafe_binding::fixnum;

pub fn int_to_num(num: i64) -> types::rb_value {
    unsafe {
        fixnum::rb_int2inum(num as types::rb_signed_value)
    }
}

pub fn num_to_int(num: types::rb_value) -> i64 {
    unsafe {
        fixnum::rb_num2int(num)
    }
}
