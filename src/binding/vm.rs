use std::ptr;

use ruby_sys::{thread, vm};

use types::{c_int, c_void, CallbackPtr, Value};
use util;

pub fn block_proc() -> Value {
    unsafe { vm::rb_block_proc() }
}

pub fn is_block_given() -> bool {
    let result = unsafe { vm::rb_block_given_p() };

    util::c_int_to_bool(result)
}

pub fn init() {
    unsafe {
        vm::ruby_init();
    }
}

pub fn require(name: &str) {
    let name = util::str_to_cstring(name);

    unsafe {
        vm::rb_require(name.as_ptr());
    }
}

// "evaluation can raise an exception."
pub fn eval_string(string: &str) -> Value {
    let s = util::str_to_cstring(string);

    unsafe {
        vm::rb_eval_string(s.as_ptr())
    }
}

pub fn eval_string_protect(string: &str) -> Result<Value, c_int> {
    let s = util::str_to_cstring(string);
    let mut state = 0;
    let value = unsafe {
        vm::rb_eval_string_protect(
            s.as_ptr(),
            &mut state as *mut c_int
        )
    };
    if state == 0 {
        Ok(value)
    } else {
        Err(state)
    }
}

pub fn raise(exception: Value, message: &str) {
    let message = util::str_to_cstring(message);

    unsafe {
        vm::rb_raise(exception, message.as_ptr());
    }
}

pub fn errinfo() -> Value {
    unsafe { vm::rb_errinfo() }
}

pub fn set_errinfo(err: Value) {
    unsafe { vm::rb_set_errinfo(err) }
}

pub fn thread_call_without_gvl<F, R, G>(func: F, unblock_func: Option<G>) -> R
where
    F: FnOnce() -> R,
    G: FnOnce(),
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            thread::rb_thread_call_without_gvl(
                callbox as CallbackPtr,
                util::closure_to_ptr(func),
                callbox as CallbackPtr,
                util::closure_to_ptr(ubf),
            )
        } else {
            thread::rb_thread_call_without_gvl(
                callbox as CallbackPtr,
                util::closure_to_ptr(func),
                ptr::null() as CallbackPtr,
                ptr::null() as *const c_void,
            )
        };

        util::ptr_to_data(ptr)
    }
}

pub fn thread_call_without_gvl2<F, R, G>(func: F, unblock_func: Option<G>) -> R
where
    F: FnOnce() -> R,
    G: FnOnce(),
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            thread::rb_thread_call_without_gvl2(
                callbox as CallbackPtr,
                util::closure_to_ptr(func),
                callbox as CallbackPtr,
                util::closure_to_ptr(ubf),
            )
        } else {
            thread::rb_thread_call_without_gvl2(
                callbox as CallbackPtr,
                util::closure_to_ptr(func),
                ptr::null() as CallbackPtr,
                ptr::null() as *const c_void,
            )
        };

        util::ptr_to_data(ptr)
    }
}

pub fn thread_call_with_gvl<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    unsafe {
        let ptr =
            thread::rb_thread_call_with_gvl(callbox as CallbackPtr, util::closure_to_ptr(func));

        util::ptr_to_data(ptr)
    }
}

extern "C" fn callbox(boxptr: *mut c_void) -> *const c_void {
    let mut fnbox: Box<Box<FnMut() -> *const c_void>> =
        unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> *const c_void>) };

    fnbox()
}

pub fn protect<F, R>(func: F) -> Result<Value, c_int>
where
    F: FnOnce() -> R,
{
    let mut state = 0;
    let value = unsafe {
        vm::rb_protect(
            callbox as CallbackPtr,
            util::closure_to_ptr(func),
            &mut state as *mut c_int,
        )
    };
    if state == 0 {
        Ok(value)
    } else {
        Err(state)
    }
}
