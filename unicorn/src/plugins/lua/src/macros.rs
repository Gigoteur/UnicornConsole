use std::mem;

use libc::c_int;

use State;
use ffi::{lua_State, lua_CFunction};

/// Wrap a `fn(&mut State) -> u32` as an ffi-suitable `Function`. The argument
/// must be a path, so that the specific `fn` is known at compile-time.
#[macro_export]
macro_rules! lua_func {
  ($func:path) => { $crate::macros::_wrap(|s| $crate::macros::_check_type($func)(s)) }
}

#[doc(hidden)]
#[inline(always)]
pub fn _check_type(f: fn(&mut State) -> c_int) -> fn(&mut State) -> c_int {
  f
}

#[doc(hidden)]
#[inline]
pub fn _wrap<F: Fn(&mut State) -> c_int>(_: F) -> lua_CFunction {
  unsafe extern fn wrapped<F: Fn(&mut State) -> c_int>(s: *mut lua_State) -> c_int {
    mem::transmute::<&(), &F>(&())(&mut State::from_ptr(s))
  }
  assert!(mem::size_of::<F>() == 0, "can only wrap zero-sized closures");
  Some(wrapped::<F>)
}
