// The MIT License (MIT)
//
// Copyright (c) 2016 J.C. Moyer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

extern crate lua;
extern crate libc;

use lua::ffi::lua_State;
use lua::{State, Function};
use libc::c_int;

struct VecWrapper {
  data: Vec<i64>
}

impl VecWrapper {
  fn new() -> VecWrapper {
    VecWrapper {
      data: Vec::new()
    }
  }

  /// Constructs a new VecWrapper and pushes it onto the Lua stack.
  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_new(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    // construct new userdata in lua space and initialize it
    let v: *mut VecWrapper = state.new_userdata_typed();
    std::ptr::write(v, VecWrapper::new());
    // set the userdata's metatable so we can call methods on it
    state.set_metatable_from_registry("VecWrapper");
    // return the userdata on top of the stack
    1
  }

  /// Returns the value in the underlying Vec at index `i`. If the index is out
  /// of bounds, this function returns nil instead.
  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_get(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    let v = state.check_userdata(1, "VecWrapper") as *mut VecWrapper;
    let i = state.check_integer(2) as usize;
    // push integer if index is not out of bounds, otherwise nil
    match (*v).data.get(i) {
      Some(value) => state.push_integer(*value),
      None        => state.push_nil()
    };
    1
  }

  /// Pushes a value into the underlying Vec.
  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_push(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    let v = state.check_userdata(1, "VecWrapper") as *mut VecWrapper;
    let i = state.check_integer(2);
    (*v).data.push(i);
    1
  }

  /// Returns the length of the underlying Vec.
  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_len(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    let v = state.check_userdata(1, "VecWrapper") as *mut VecWrapper;
    state.push_integer((*v).data.len() as i64);
    1
  }

  /// Garbage collects a VecWrapper.
  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_gc(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    let v = state.check_userdata(1, "VecWrapper") as *mut VecWrapper;
    std::ptr::drop_in_place(v);
    0
  }
}

const VECWRAPPER_LIB: [(&'static str, Function); 4] = [
    ("new",  Some(VecWrapper::lua_new)),
    ("get",  Some(VecWrapper::lua_get)),
    ("push", Some(VecWrapper::lua_push)),
    ("len",  Some(VecWrapper::lua_len))
];

fn main() {
    let mut state = lua::State::new();

    state.open_libs();

    // make a VecWrapper table globally available to the lua state and register
    // our functions there:
    state.new_table();
    state.set_fns(&VECWRAPPER_LIB, 0);
    // copy reference to VecWrapper table so we can keep the original reference
    // on the stack for later
    state.push_value(-1);
    state.set_global("VecWrapper");

    // create a metatable for VecWrapper in the lua registry that refers to the
    // global VecWrapper table:
    state.new_metatable("VecWrapper");
    // copy reference to VecWrapper table
    state.push_value(-2);
    // VecWrappermetatable.__index = VecWrapper
    state.set_field(-2, "__index");
    // VecWrappermetatable.__gc = lua_gc
    state.push_fn(Some(VecWrapper::lua_gc));
    state.set_field(-2, "__gc");

    // pop metatable and VecWrapper table from the stack
    state.pop(2);

    // try it out:
    state.do_string("local v = VecWrapper.new()
                     v:push(12)
                     v:push(34)
                     -- should print 2
                     print('length of vec is', v:len())
                     -- should print 12 34 nil
                     print(v:get(0), v:get(1), v:get(2))");
}
