use std::ptr;

use ruby_sys::thread;

use types::{CallbackPtr, c_void, RawFd, Value};
use util;

use ::Object;

pub fn create<F, R>(func: F) -> Value
    where F: FnOnce() -> R,
          R: Object
{
    let fnbox = Box::new(func) as Box<FnOnce() -> R>;

    let closure_ptr = Box::into_raw(Box::new(fnbox)) as *mut c_void;

    unsafe { thread::rb_thread_create(thread_create_callbox::<R>, closure_ptr) }
}

pub fn wait_fd(fd: RawFd) {
    unsafe { thread::rb_thread_wait_fd(fd) };
}

pub fn call_without_gvl<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where F: FnOnce() -> R,
          G: FnOnce()
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            thread::rb_thread_call_without_gvl(thread_call_callbox as CallbackPtr,
                                               util::closure_to_ptr(func),
                                               thread_call_callbox as CallbackPtr,
                                               util::closure_to_ptr(ubf))
        } else {
            thread::rb_thread_call_without_gvl(thread_call_callbox as CallbackPtr,
                                               util::closure_to_ptr(func),
                                               ptr::null() as CallbackPtr,
                                               ptr::null() as *const c_void)
        };

        util::ptr_to_data(ptr)
    }
}

pub fn call_without_gvl2<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where F: FnOnce() -> R,
          G: FnOnce()
{
    unsafe {
        let ptr = if let Some(ubf) = unblock_func {
            thread::rb_thread_call_without_gvl2(thread_call_callbox as CallbackPtr,
                                                util::closure_to_ptr(func),
                                                thread_call_callbox as CallbackPtr,
                                                util::closure_to_ptr(ubf))
        } else {
            thread::rb_thread_call_without_gvl2(thread_call_callbox as CallbackPtr,
                                                util::closure_to_ptr(func),
                                                ptr::null() as CallbackPtr,
                                                ptr::null() as *const c_void)
        };

        util::ptr_to_data(ptr)
    }
}

pub fn call_with_gvl<F, R>(func: F) -> R
    where F: FnOnce() -> R
{
    unsafe {
        let ptr = thread::rb_thread_call_with_gvl(thread_call_callbox as CallbackPtr,
                                                  util::closure_to_ptr(func));

        util::ptr_to_data(ptr)
    }

}

extern "C" fn thread_create_callbox<R>(boxptr: *mut c_void) -> Value
    where R: Object
{
    let mut fnbox: Box<Box<FnMut() -> R>> =
        unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> R>) };

    fnbox().value()
}

extern "C" fn thread_call_callbox(boxptr: *mut c_void) -> *const c_void {
    let mut fnbox: Box<Box<FnMut() -> *const c_void>> =
        unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> *const c_void>) };

    fnbox()
}
