use types::{argc, Id, rb_value};
use unsafe_binding::util::{rb_funcallv, rb_intern};
use util::str_as_ptr;

pub fn internal_id(string: &str) -> Id {
    unsafe {
        rb_intern(str_as_ptr(string))
    }
}

pub fn call_method(receiver: rb_value,
                   method: &str,
                   argc: argc,
                   argv: *const rb_value) -> rb_value {
    let method_id = internal_id(method);

    unsafe {
        rb_funcallv(receiver, method_id, argc, argv)
    }
}
