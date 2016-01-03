use types::{rb_id, rb_value};
use unsafe_binding::symbol::{rb_id2name, rb_id2sym, rb_sym2id};
use util::cstr_as_string;

pub fn id_to_sym(id: rb_id) -> rb_value {
    unsafe {
        rb_id2sym(id)
    }
}

pub fn id_to_name(id: rb_id) -> String {
    unsafe {
        let str = rb_id2name(id);

        cstr_as_string(str)
    }
}

pub fn sym_to_id(sym: rb_value) -> rb_id {
    unsafe {
        rb_sym2id(sym)
    }
}
