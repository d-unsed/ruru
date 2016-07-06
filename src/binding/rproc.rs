use ruby_sys::rproc;

use binding::global::RubySpecialConsts;
use types::{Argc, InternalValue, Value};

pub fn call(rproc: Value, argc: Argc, argv: *const Value) -> Value {
    unsafe {
        rproc::rb_proc_call_with_block(rproc,
                                       argc,
                                       argv,
                                       Value::from(RubySpecialConsts::Nil as InternalValue))
    }
}
