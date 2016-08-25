use ruby_sys::vm;

use types::*;
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


pub fn thread_call_without_gvl<F, G>(func: F, unblock_func: Option<G>)
    where F: Fn(),
          G: Fn()
{
    unsafe {
        vm::rb_thread_call_without_gvl(callbox as CallbackPtr,
                                       Box::into_raw(Box::new(Box::new(func) as Box<Fn()>)) as *const c_void,
                                       callbox as CallbackPtr,
                                       Box::into_raw(
                                           Box::new(
                                               unblock_func.map(|f| Box::new(f) as Box<Fn()>).unwrap_or(Box::new(|| {}) as Box<Fn()>)
                                            )
                                        ) as *const c_void)
    }
}

pub fn thread_call_without_gvl2<F, G>(func: F, unblock_func: Option<G>)
    where F: Fn(),
          G: Fn()
{
    unsafe {
        vm::rb_thread_call_without_gvl2(callbox as CallbackPtr,
                                       Box::into_raw(Box::new(Box::new(func) as Box<Fn()>)) as *const c_void,
                                       callbox as CallbackPtr,
                                       Box::into_raw(
                                           Box::new(
                                               unblock_func.map(|f| Box::new(f) as Box<Fn()>).unwrap_or(Box::new(|| {}) as Box<Fn()>)
                                            )
                                        ) as *const c_void)
    }
}

pub fn thread_call_with_gvl<F>(func: F)
    where F: Fn()
{
    unsafe {
        vm::rb_thread_call_with_gvl(callbox as CallbackPtr,
                                    Box::into_raw(Box::new(Box::new(func) as Box<Fn()>)) as *const c_void);
    }
}

extern "C" fn callbox(boxptr: *mut c_void) {
    let fnbox: Box<Box<Fn()>> = unsafe { Box::from_raw(boxptr as *mut Box<Fn()>) };
    fnbox();
}


pub fn protect<F>(func: F) -> Result<(), ()>
    where F: Fn()
{
    let mut state = 0;
    unsafe {
        vm::rb_protect(callbox as CallbackPtr,
                       Box::into_raw(Box::new(Box::new(func) as Box<Fn()>)) as *const c_void,
                       &mut state as *mut i32);
    }
    if state == 0 {
        Ok(())
    } else {
        Err(())
    }
}