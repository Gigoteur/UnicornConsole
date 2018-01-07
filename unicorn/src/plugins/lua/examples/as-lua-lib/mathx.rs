// The MIT License (MIT)
//
// Copyright (c) 2014 J.C. Moyer
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

#![crate_name="mathx"]
#![crate_type="dylib"]

extern crate libc;
extern crate lua;

// import low level lua_State
use lua::ffi::lua_State;

// import high level lua_State wrapper and a type we'll need for library
// registration
use lua::{State, Function};

// import c_int since we need this to interface with Lua
use libc::c_int;

// simple binding to Rust's sin function
#[allow(non_snake_case)]
unsafe extern "C" fn sin(L: *mut lua_State) -> c_int {
  let mut state = State::from_ptr(L);
  // convert the value on top of the stack to a number
  let num = state.to_number(-1);
  // push the sine of that number onto the stack
  state.push_number(num.sin());
  // return one value to Lua
  1
}

// simple binding to Rust's cos function
#[allow(non_snake_case)]
unsafe extern "C" fn cos(L: *mut lua_State) -> c_int {
  let mut state = State::from_ptr(L);
  let num = state.to_number(-1);
  state.push_number(num.cos());
  1
}

// simple binding to Rust's tan function
#[allow(non_snake_case)]
unsafe extern "C" fn tan(L: *mut lua_State) -> c_int {
  let mut state = State::from_ptr(L);
  let num = state.to_number(-1);
  state.push_number(num.tan());
  1
}

// mapping of function name to function pointer
const MATHX_LIB: [(&'static str, Function); 3] = [
  ("sin", Some(sin)),
  ("cos", Some(cos)),
  ("tan", Some(tan)),
];

// the format of this function name is defined by the Lua manual; the Lua
// interpreter will call into this when you require() this library
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn luaopen_mathx(L: *mut lua_State) -> c_int {
  // construct a state wrapper object from the pointer we were given
  let mut state = State::from_ptr(L);
  // create a new table and set fields for each function defined in MATHX_LIB
  state.new_lib(&MATHX_LIB);
  // return it on the stack
  1
}
