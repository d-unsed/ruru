use libc::size_t;
use std::mem;
use types::{Argc, c_char, Id, Value, ValueType};

extern "C" {
    pub fn rb_const_get(klass: Value, id: Id) -> Value;
    pub fn rb_funcallv(receiver: Value, method: Value, argc: Argc, argv: *const Value) -> Value;
    pub fn rb_intern(name: *const c_char) -> Id;
}

#[repr(C)]
struct RBasic {
    flags: Value,
    klass: Value,
}

pub enum RubySpecialConsts {
    False = 0,
    True = 0x14,
    Nil = 0x08,
    Undef = 0x34,

    ImmediateMask = 0x07,
    FixnumFlag = 0x01,
    FlonumMask = 0x03,
    FlonumFlag = 0x02,
    SymbolFlag = 0x0c,
}

pub const SPECIAL_SHIFT: usize = 8;

pub fn rb_value_is_fixnum(value: Value) -> bool {
    (value & (RubySpecialConsts::FixnumFlag as usize)) != 0
}

pub fn rb_value_is_flonum(value: Value) -> bool {
    (value & (RubySpecialConsts::FlonumMask as usize)) == (RubySpecialConsts::FlonumFlag as usize)
}

pub fn rb_value_is_immediate(value: Value) -> bool {
    (value & (RubySpecialConsts::ImmediateMask as usize)) != 0
}

pub fn rb_value_is_static_sym(value: Value) -> bool {
    (value & !((!0) << SPECIAL_SHIFT)) == (RubySpecialConsts::SymbolFlag as usize)
}

pub fn rb_test(value: Value) -> bool {
    (value & !(RubySpecialConsts::Nil as usize)) != 0
}

pub fn rb_builtin_type(value: Value) -> ValueType {
    unsafe {
        let basic: *const RBasic = mem::transmute(value);
        let masked = (*basic).flags & (ValueType::Mask as size_t);
        mem::transmute(masked as u32)
    }
}

pub fn rb_type(value: Value) -> ValueType {
    if rb_value_is_immediate(value) {
        if rb_value_is_fixnum(value) {
            ValueType::Fixnum
        } else if rb_value_is_flonum(value) {
            ValueType::Float
        } else if value == (RubySpecialConsts::True as usize) {
            ValueType::True
        } else if rb_value_is_static_sym(value) {
            ValueType::Symbol
        } else if value == (RubySpecialConsts::Undef as usize) {
            ValueType::Undef
        } else {
            rb_builtin_type(value)
        }
    } else if !rb_test(value) {
        if value == (RubySpecialConsts::Nil as usize) {
            ValueType::Nil
        } else if value == (RubySpecialConsts::False as usize) {
            ValueType::False
        } else {
            rb_builtin_type(value)
        }
    } else {
        rb_builtin_type(value)
    }
}
