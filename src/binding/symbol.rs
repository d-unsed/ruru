use types::{Id, Value};
use unsafe_binding::symbol::{rb_id2name, rb_id2sym, rb_sym2id};
use util::cstr_as_string;

pub fn id_to_sym(id: Id) -> Value {
    unsafe {
        rb_id2sym(id)
    }
}

pub fn id_to_name(id: Id) -> String {
    unsafe {
        let str = rb_id2name(id);

        cstr_as_string(str)
    }
}

pub fn sym_to_id(sym: Value) -> Id {
    unsafe {
        rb_sym2id(sym)
    }
}
