use types;
use unsafe_binding::util;

pub fn int_to_num(num: i64) -> types::rb_value {
    unsafe {
        util::rb_int2inum(num as types::rb_signed_value)
    }
}

pub fn num_to_int(num: types::rb_value) -> i64 {
    unsafe {
        util::rb_num2int(num)
    }
}

pub fn internal_id(string: &str) -> types::rb_id {
    unsafe {
        util::rb_intern(::util::str_as_ptr(string))
    }
}

pub fn call_method(receiver: types::rb_value,
                   method: &str,
                   arguments: Vec<types::rb_value>) -> types::rb_value {
    let method_id = internal_id(method);
    let argc = arguments.len() as types::c_int;
    let argv = arguments.as_ptr();

    unsafe {
        util::rb_funcallv(receiver, method_id, argc, argv)
    }
}
