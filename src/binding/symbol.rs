use types;
use unsafe_binding::symbol;
use util;

pub fn id_to_sym(id: types::rb_id) -> types::rb_value {
    unsafe {
        symbol::rb_id2sym(id)
    }
}

pub fn id_to_name(id: types::rb_id) -> String {
    unsafe {
        let str = symbol::rb_id2name(id);

        util::cstr_as_string(str)
    }
}

pub fn sym_to_id(sym: types::rb_value) -> types::rb_id {
    unsafe {
        symbol::rb_sym2id(sym)
    }
}
