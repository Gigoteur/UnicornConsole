pub mod ffi;
pub mod types;
pub mod errors;
pub mod encoder;

extern crate libc;
extern crate cesu8;
extern crate serde;
#[macro_use] extern crate abort_on_panic;

use types::*;
use errors::*;
use encoder::*;

use std::cmp;
use std::ffi::CString;
use std::ptr::null_mut;
use std::mem::transmute;
use libc::{c_int, c_ushort, c_void, c_char};
use std::slice::from_raw_parts;
use cesu8::{to_cesu8, from_cesu8};
use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

macro_rules! assert_stack_height_unchanged {
    ($ctx:ident, $body:block) => {
        {
            let initial_stack_height = ffi::duk_get_top($ctx.C);
            let result = $body;
            assert_eq!(initial_stack_height, ffi::duk_get_top($ctx.C));
            result
        }
    }
}

pub unsafe fn from_lstring(data: *const i8, len: ffi::duk_size_t) ->
    DuktapeResult<String>
{
    let mut v: Vec<u8> = Vec::new();

    let ptr = data as *const u8;
    let bytes = from_raw_parts(ptr, len as usize);
    match from_cesu8(bytes) {
        Ok(str) => Ok(str.into_owned()),
        Err(_) => Err(DuktapeError::from_str("can't convert string to UTF-8"))
    }
}

const RUST_FN_PROP: [i8; 5] = [-1, 'r' as i8, 'f' as i8, 'n' as i8, 0];

pub trait Foo {
    fn dispatch(&mut self, &mut Context, &[Value<'static>]) -> DuktapeResult<Value<'static>>;
}

#[allow(non_snake_case)]
pub struct Context {
  C: ffi::duk_context,
  owned: bool,
}

impl Context {
    pub fn new() -> Context {
        println!("Creating new javasript context");
        unsafe {
            Context { C: ffi::_duk_create_heap_default(), owned: true }
        }
    }

    pub unsafe fn from_borrowed_mut_ptr(ptr: ffi::duk_context) -> Context {
        Context{C: ptr, owned: false}
    }

    pub fn dump_context(&mut self) -> String {
        unsafe {
            ffi::duk_push_context_dump(self.C);
            let mut len: ffi::duk_size_t = 0;
            let str = ffi::duk_safe_to_lstring(self.C, -1, &mut len);
            let result = from_lstring(str as *const i8, len)
                .unwrap_or_else(|_| "Couldn't dump context".to_string());
            ffi::duk_pop(self.C);
            result
        }
    }

    pub fn eval_string(&mut self, src: String) {
        unsafe {
            let src = CString::new(src).unwrap();
            ffi::_duk_eval_string(self.C, src.as_ptr());
        }
    }

    pub fn get_int(&mut self, idx: i32) -> i32 {
        unsafe {
            return ffi::duk_get_int(self.C, idx);
        }
    }
    
    pub fn eval_from(&mut self, filename: &str, code: &str) ->
        DuktapeResult<Value<'static>>
    {
        unsafe {
            assert_stack_height_unchanged!(self, {
                // Push our filename parameter and evaluate our code.
                ffi::duk_push_lstring(self.C, filename.as_ptr() as *const c_char,
                                      filename.len() as ffi::duk_size_t);
                let status = ffi::duk_eval_raw(self.C,
                                               code.as_ptr() as *const c_char,
                                               code.len() as ffi::duk_size_t,
                                               ffi::DUK_COMPILE_EVAL |
                                               ffi::DUK_COMPILE_NOSOURCE |
                                               ffi::DUK_COMPILE_SAFE);
                let result = self.pop_result(status);
                ffi::duk_pop(self.C);
                result
            })
        }
    }

    pub fn eval(&mut self, code: &str) -> DuktapeResult<Value<'static>> {
        self.eval_from("<eval>", code)
    }
  
    unsafe fn get(&mut self, idx: ffi::duk_idx_t) -> DuktapeResult<Value<'static>> {
        match ffi::duk_get_type(self.C, idx) {
            ffi::DUK_TYPE_UNDEFINED => Ok(Value::Undefined),
            ffi::DUK_TYPE_NULL => Ok(Value::Null),
            ffi::DUK_TYPE_BOOLEAN => {
                let val = ffi::duk_get_boolean(self.C, idx);
                Ok(Value::Bool(val != 0))
            }
            ffi::DUK_TYPE_NUMBER => {
                Ok(Value::Number(ffi::duk_get_number(self.C, idx)))
            }
            ffi::DUK_TYPE_STRING => {
                let mut len: ffi::duk_size_t = 0;
                let str = ffi::duk_get_lstring(self.C, idx, &mut len);
                Ok(Value::String(Cow::Owned(try!(from_lstring(str as *const i8, len)))))
            }
            _ => panic!("Cannot convert duktape data type")
        }
    }

    pub unsafe fn push_old(&mut self, val: &Value) {
        match val {
            &Value::Undefined => ffi::duk_push_undefined(self.C),
            &Value::Null => ffi::duk_push_null(self.C),
            &Value::Bool(v) => ffi::duk_push_boolean(self.C, if v { 1 } else { 0 }),
            &Value::Number(v) => ffi::duk_push_number(self.C, v),
            &Value::String(ref v) => {
                let encoded = to_cesu8(v);
                ffi::duk_push_lstring(self.C, encoded.as_ptr() as *const c_char,
                                      encoded.len() as ffi::duk_size_t);
            }
        }
    }

    unsafe fn get_result(&mut self, status: ffi::duk_int_t) ->
        DuktapeResult<Value<'static>>
    {        
        if status == ffi::DUK_EXEC_SUCCESS {
            self.get(-1)
        } else {
            let mut len: ffi::duk_size_t = 0;
            let str = ffi::duk_safe_to_lstring(self.C, -1, &mut len);
            let msg = try!(from_lstring(str as *const i8, len));
            Err(DuktapeError::from_str(&msg))
        }
    }

    pub unsafe fn pop_result(&mut self, status: ffi::duk_int_t) ->
        DuktapeResult<Value<'static>>
    {
        let result = self.get_result(status);
        ffi::duk_pop(self.C);

        result
    }

    pub fn register<T: Foo>(&mut self, magic_value: i32, fn_name: &str, obj: Arc<Mutex<T>>, arg_count: Option<u16>) {
        let c_arg_count =
            arg_count.map(|n| n as ffi::duk_int_t).unwrap_or(ffi::DUK_VARARGS);
        unsafe {
            assert_stack_height_unchanged!(self, {
                ffi::duk_push_global_object(self.C);

                ffi::duk_push_c_function(self.C, Some(rust_duk_callback), c_arg_count);

                let cb_obj: Box<Box<Arc<Mutex<Foo>>>> = Box::new(Box::new(obj));

                ffi::duk_push_pointer(self.C,  Box::into_raw(cb_obj) as *mut c_void);
                
                ffi::duk_set_magic(self.C, -2, magic_value);

                ffi::duk_put_prop_string(self.C, -2, RUST_FN_PROP.as_ptr() as *const c_char);

                let c_str = CString::new(fn_name.as_bytes()).unwrap();
                ffi::duk_put_prop_string(self.C, -2, c_str.as_ptr() as *const c_char);

                ffi::duk_pop(self.C);
            })
        }
    }
}

impl Drop for Context {
  fn drop(&mut self) {
    if self.owned {
        self.owned = false;
        unsafe { ffi::duk_destroy_heap(self.C) }
    }
  }
}

unsafe extern "C" fn rust_duk_callback(ctx: ffi::duk_context) -> ffi::duk_ret_t {
    assert!(ctx != null_mut());
    let mut ctx = Context::from_borrowed_mut_ptr(ctx);

    ffi::duk_push_current_function(ctx.C);

    ffi::duk_get_prop_string(ctx.C, -1, RUST_FN_PROP.as_ptr() as *const c_char);
        
    let p = ffi::duk_get_pointer(ctx.C, -1);

    ffi::duk_pop_n(ctx.C, 2);
    assert!(p != null_mut());
    let f: &mut Box<Arc<Mutex<Foo>>> = transmute(p);

    let arg_count = ffi::duk_get_top(ctx.C) as usize;
  
    let mut args = Vec::with_capacity(arg_count+1);
    args.push(Value::Number(ffi::duk_get_current_magic(ctx.C) as f64));

    for i in 0..arg_count {
        match ctx.get(i as ffi::duk_idx_t) {
            Ok(arg) => args.push(arg),
            Err(_) => return ffi::DUK_RET_TYPE_ERROR
        }
    }

    let result =
        abort_on_panic!("unexpected panic in code called from JavaScript", {
            f.lock().unwrap().dispatch(&mut ctx, &args)
        });

    match result {
        // No return value.
        Ok(Value::Undefined) => { 0 }
        // A single return value.
        Ok(ref val) => { ctx.push_old(val); 1 }
        Err(ref err) => {
            let code = err_code(err) as ffi::duk_int_t;
            match err_message(err) {
                // An error with an actual error message.
                &Some(ref _msg) => {
                    ffi::DUK_RET_ERROR
                }
                // A generic error using one of the standard codes.
                &None => { -code }
            }
        }
    }
}