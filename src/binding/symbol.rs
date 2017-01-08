use ruby_sys::symbol;

use types::{Id, Value};
use util;

pub fn id_to_sym(id: Id) -> Value {
    unsafe { symbol::rb_id2sym(id) }
}

pub fn id_to_name(id: Id) -> String {
    let str = unsafe { symbol::rb_id2name(id) };

    util::cstr_to_string(str)
}

pub fn sym_to_id(sym: Value) -> Id {
    unsafe { symbol::rb_sym2id(sym) }
}
