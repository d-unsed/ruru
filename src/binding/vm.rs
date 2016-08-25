use ruby_sys::vm;

use binding::util::{unwrap_data_from_ptr, wrap_closure_to_ptr};
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


pub fn thread_call_without_gvl<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where F: FnMut() -> R,
          G: FnMut()
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            vm::rb_thread_call_without_gvl(callbox as CallbackPtr,
                                           wrap_closure_to_ptr(func),
                                           callbox as CallbackPtr,
                                           wrap_closure_to_ptr(ubf))
        } else {
            vm::rb_thread_call_without_gvl(callbox as CallbackPtr,
                                           wrap_closure_to_ptr(func),
                                           0 as CallbackPtr,
                                           0 as *const c_void)
        };
        unwrap_data_from_ptr(ptr as *mut _)
    }
}

pub fn thread_call_without_gvl2<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where F: FnMut() -> R,
          G: FnMut()
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            vm::rb_thread_call_without_gvl2(callbox as CallbackPtr,
                                            wrap_closure_to_ptr(func),
                                            callbox as CallbackPtr,
                                            wrap_closure_to_ptr(ubf))
        } else {
            vm::rb_thread_call_without_gvl2(callbox as CallbackPtr,
                                            wrap_closure_to_ptr(func),
                                            0 as CallbackPtr,
                                            0 as *const c_void)
        };
        unwrap_data_from_ptr(ptr as *mut _)
    }
}

pub fn thread_call_with_gvl<F, R>(func: F) -> R
    where F: FnMut() -> R
{
    unsafe {
        let ptr = vm::rb_thread_call_with_gvl(callbox as CallbackPtr, wrap_closure_to_ptr(func));
        unwrap_data_from_ptr(ptr as *mut _)
    }

}

extern "C" fn callbox(boxptr: *mut c_void) -> *const c_void {
    let mut fnbox: Box<Box<FnMut() -> *const c_void>> =
        unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> *const c_void>) };
    fnbox()
}


pub fn protect<F>(func: F) -> Result<Value, c_int>
    where F: FnMut()
{
    let mut state = 0;
    let value = unsafe {
        vm::rb_protect(callbox as CallbackPtr,
                       wrap_closure_to_ptr(func),
                       &mut state as *mut c_int)
    };
    if state == 0 {
        Ok(value)
    } else {
        Err(state)
    }
}
