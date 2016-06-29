use types::{Argc, InternalValue, Value};
use binding::global::RubySpecialConsts;
use ruby_sys::rproc::rb_proc_call_with_block;

pub fn call(rproc: Value, argc: Argc, argv: *const Value) -> Value {
    unsafe {
        rb_proc_call_with_block(rproc,
                                argc,
                                argv,
                                Value::from(RubySpecialConsts::Nil as InternalValue))
    }
}
