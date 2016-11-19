use ruby_sys::thread;

use types::{c_void, RawFd, Value};

use ::Object;

pub fn create<F, R>(func: F) -> Value
    where F: FnOnce() -> R,
          R: Object
{
    let fnbox = Box::new(func) as Box<FnOnce() -> R>;

    let closure_ptr = Box::into_raw(Box::new(fnbox)) as *mut c_void;

    unsafe { thread::rb_thread_create(callbox::<R>, closure_ptr) }
}

pub fn wait_fd(fd: RawFd) {
    unsafe { thread::rb_thread_wait_fd(fd) };
}

extern "C" fn callbox<R>(boxptr: *mut c_void) -> Value
    where R: Object
{
    let mut fnbox: Box<Box<FnMut() -> R>> =
        unsafe { Box::from_raw(boxptr as *mut Box<FnMut() -> R>) };

    fnbox().value()
}
