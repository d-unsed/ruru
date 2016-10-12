use ruby_sys::vm;

use types::{CallbackPtr, c_int, c_void, Value};
use util;

pub fn block_proc() -> Value {
    unsafe { vm::rb_block_proc() }
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

pub fn raise(exception: Value, message: &str) {
    let message = util::str_to_cstring(message);

    unsafe {
        vm::rb_raise(exception, message.as_ptr());
    }
}

pub fn thread_call_without_gvl<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where F: FnOnce() -> R,
          G: FnOnce()
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            vm::rb_thread_call_without_gvl(callbox as CallbackPtr,
                                           util::closure_to_ptr(func),
                                           callbox as CallbackPtr,
                                           util::closure_to_ptr(ubf))
        } else {
            vm::rb_thread_call_without_gvl(callbox as CallbackPtr,
                                           util::closure_to_ptr(func),
                                           0 as CallbackPtr,
                                           0 as *const c_void)
        };

        util::ptr_to_data(ptr)
    }
}

pub fn thread_call_without_gvl2<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where F: FnOnce() -> R,
          G: FnOnce()
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            vm::rb_thread_call_without_gvl2(callbox as CallbackPtr,
                                            util::closure_to_ptr(func),
                                            callbox as CallbackPtr,
                                            util::closure_to_ptr(ubf))
        } else {
            vm::rb_thread_call_without_gvl2(callbox as CallbackPtr,
                                            util::closure_to_ptr(func),
                                            0 as CallbackPtr,
                                            0 as *const c_void)
        };

        util::ptr_to_data(ptr)
    }
}

pub fn thread_call_with_gvl<F, R>(func: F) -> R
    where F: FnOnce() -> R
{
    unsafe {
        let ptr = vm::rb_thread_call_with_gvl(callbox as CallbackPtr, util::closure_to_ptr(func));

        util::ptr_to_data(ptr)
    }

}

extern "C" fn callbox(boxptr: *mut c_void) -> *const c_void {
    let mut fnbox: Box<Box<FnMut() -> *const c_void>> =
        unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> *const c_void>) };

    fnbox()
}


pub fn protect<F>(func: F) -> Result<Value, c_int>
    where F: FnOnce()
{
    let mut state = 0;
    let value = unsafe {
        vm::rb_protect(callbox as CallbackPtr,
                       util::closure_to_ptr(func),
                       &mut state as *mut c_int)
    };
    if state == 0 { Ok(value) } else { Err(state) }
}
