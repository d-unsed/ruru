use types;
use unsafe_binding::util;

pub fn internal_id(string: &str) -> types::rb_id {
    unsafe {
        util::rb_intern(::util::str_as_ptr(string))
    }
}
