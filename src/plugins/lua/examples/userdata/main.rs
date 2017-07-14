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

struct Point2D {
  // using i64 for convenience since lua defaults to 64 bit integers
  x: i64,
  y: i64
}

impl Point2D {
  fn new(x: i64, y: i64) -> Point2D {
    return Point2D {
      x: x, y: y
    };
  }

  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_new(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    // takes two optional integer parameters
    let x = state.opt_integer(1, 0);
    let y = state.opt_integer(2, 0);
    // construct new userdata in lua space and initialize it
    *state.new_userdata_typed::<Point2D>() = Point2D::new(x, y);
    // set the userdata's metatable so we can call methods on it
    state.set_metatable_from_registry("Point2D");
    // return the userdata on top of the stack
    1
  }

  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_x(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    let point = state.check_userdata(1, "Point2D") as *mut Point2D;
    state.push_integer((*point).x);
    1
  }

  #[allow(non_snake_case)]
  unsafe extern "C" fn lua_y(L: *mut lua_State) -> c_int {
    let mut state = State::from_ptr(L);
    let point = state.check_userdata(1, "Point2D") as *mut Point2D;
    state.push_integer((*point).y);
    1
  }
}

const POINT2D_LIB: [(&'static str, Function); 3] = [
  ("new", Some(Point2D::lua_new)),
  ("x",   Some(Point2D::lua_x)),
  ("y",   Some(Point2D::lua_y))
];

fn main() {
  let mut state = lua::State::new();

  state.open_libs();

  // make a Point2D table globally available to the lua state and register
  // our functions there:
  state.new_table();
  state.set_fns(&POINT2D_LIB, 0);
  // copy reference to Point2D table so we can keep the original reference on
  // the stack for later
  state.push_value(-1);
  state.set_global("Point2D");

  // create a metatable for Point2D in the lua registry that refers to the
  // global Point2D table:
  state.new_metatable("Point2D");
  // copy reference to Point2D table
  state.push_value(-2);
  // Point2Dmetatable.__index = Point2D
  state.set_field(-2, "__index");

  // pop metatable and Point2D table from the stack
  state.pop(2);

  // try it out:
  state.do_string("local p = Point2D.new(12, 34)
                   print(p:x(), p:y())");
}
