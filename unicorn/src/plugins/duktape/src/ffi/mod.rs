#![allow(non_camel_case_types, non_snake_case, dead_code)]

use libc::{c_void, c_int, c_char, c_short, c_ushort, c_uint, c_uchar, c_double};

pub type duk_context = *mut c_void;
pub type duk_errcode_t = i32;
pub type duk_idx_t = i32;
pub type duk_int_t = i32;
pub type duk_ret_t = i32;
pub type duk_uint_t = u32;
pub type duk_bool_t = i32;
pub type duk_uarridx_t = u32;
pub type duk_codepoint_t = i32;
pub type duk_ucodepoint_t = u32;
pub type duk_size_t = u64;
pub type duk_ptrdiff_t = i64;
pub type duk_int32_t = i32;
pub type duk_uint32_t = u32;
pub type duk_uint16_t = u16;
pub type duk_double_t = c_double;

pub const DUK_VARARGS: duk_int_t = -1;

pub const DUK_ERR_NONE: duk_errcode_t = 0;
pub const DUK_ERR_ERROR: duk_errcode_t = 1;
pub const DUK_ERR_EVAL_ERROR: duk_errcode_t = 2;
pub const DUK_ERR_RANGE_ERROR: duk_errcode_t = 3;
pub const DUK_ERR_REFERENCE_ERROR: duk_errcode_t = 4;
pub const DUK_ERR_SYNTAX_ERROR: duk_errcode_t = 5;
pub const DUK_ERR_TYPE_ERROR: duk_errcode_t = 6;
pub const DUK_ERR_URI_ERROR: duk_errcode_t = 7;

pub const DUK_RET_ERROR: duk_ret_t = -1;
pub const DUK_RET_EVAL_ERROR: duk_ret_t = -2;
pub const DUK_RET_RANGE_ERROR: duk_ret_t = -3;
pub const DUK_RET_REFERENCE_ERROR: duk_ret_t = -4;
pub const DUK_RET_SYNTAX_ERROR: duk_ret_t = -5;
pub const DUK_RET_TYPE_ERROR: duk_ret_t = -6;
pub const DUK_RET_URI_ERROR: duk_ret_t = -7;

pub const DUK_EXEC_SUCCESS: duk_int_t = 0;
pub const DUK_EXEC_ERROR: duk_int_t = 1;

pub const DUK_COMPILE_EVAL: duk_uint_t = (1 << 3);    /* compile eval code (instead of global code) */
pub const DUK_COMPILE_FUNCTION: duk_uint_t = (1 << 4);    /* compile function code (instead of global code) */
pub const DUK_COMPILE_STRICT: duk_uint_t = (1 << 5);    /* use strict (outer) context for global, eval, or function code */
pub const DUK_COMPILE_SHEBANG: duk_uint_t = (1 << 6);    /* allow shebang ('#! ...') comment on first line of source */
pub const DUK_COMPILE_SAFE: duk_uint_t = (1 << 7);    /* (internal) catch compilation errors */
pub const DUK_COMPILE_NORESULT: duk_uint_t = (1 << 8);   /* (internal) omit eval result */
pub const DUK_COMPILE_NOSOURCE: duk_uint_t = (1 << 9);    /* (internal) no source string on stack */
pub const DUK_COMPILE_STRLEN: duk_uint_t = (1 << 10);   /* (internal) take strlen() of src_buffer (avoids double evaluation in macro) */
pub const DUK_COMPILE_NOFILENAME: duk_uint_t = (1 << 11);   /* (internal) no filename on stack */
pub const DUK_COMPILE_FUNCEXPR : duk_uint_t = (1 << 12);

pub const DUK_TYPE_MIN: duk_int_t = 0;
pub const DUK_TYPE_NONE: duk_int_t = 0;    /* no value, e.g. invalid index */
pub const DUK_TYPE_UNDEFINED: duk_int_t = 1;    /* Ecmascript undefined */
pub const DUK_TYPE_NULL: duk_int_t = 2;   /* Ecmascript null */
pub const DUK_TYPE_BOOLEAN : duk_int_t = 3;   /* Ecmascript boolean: 0 or 1 */
pub const DUK_TYPE_NUMBER: duk_int_t = 4; /* Ecmascript number: double */
pub const DUK_TYPE_STRING: duk_int_t = 5; /* Ecmascript string: CESU-8 / extended UTF-8 encoded */
pub const DUK_TYPE_OBJECT: duk_int_t = 6; /* Ecmascript object: includes objects, arrays, functions, threads */
pub const DUK_TYPE_BUFFER: duk_int_t = 7; /* fixed or dynamic, garbage collected byte buffer */
pub const DUK_TYPE_POINTER: duk_int_t = 8;/* raw void pointer */
pub const DUK_TYPE_LIGHTFUNC: duk_int_t = 9;/* lightweight function pointer */
pub const DUK_TYPE_MAX: duk_int_t = 9;

pub type duk_c_function =
    ::std::option::Option<unsafe extern "C" fn(arg1: duk_context) -> duk_ret_t>;

#[no_mangle]
#[allow(non_snake_case)]
extern "C" {
    pub fn _duk_create_heap_default() -> duk_context;
    pub fn _duk_eval_string(ctx: duk_context, src: *const c_char);
    pub fn duk_push_context_dump(ctx: duk_context);
    pub fn duk_eval_raw(ctx: duk_context,
                        src_buffer: *const ::libc::c_char,
                        src_length: duk_size_t, flags: duk_uint_t) -> duk_int_t;
    pub fn duk_get_int(ctx: duk_context, idx: duk_idx_t) -> duk_int_t;
    pub fn duk_destroy_heap(ctx: duk_context);
    pub fn duk_get_top(ctx: duk_context) -> duk_idx_t;
    pub fn duk_get_pointer(ctx: duk_context, index: duk_idx_t) -> *mut ::libc::c_void;
    pub fn duk_get_prop_string(ctx: duk_context, obj_index: duk_idx_t, key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_call(ctx: duk_context, nargs: duk_idx_t);
    pub fn duk_call_method(ctx: duk_context, nargs: duk_idx_t);
    pub fn duk_call_prop(ctx: duk_context, obj_index: duk_idx_t,
                         nargs: duk_idx_t);
    pub fn duk_pcall(ctx: duk_context, nargs: duk_idx_t) -> duk_int_t;
    pub fn duk_pcall_method(ctx: duk_context, nargs: duk_idx_t) -> duk_int_t;
    pub fn duk_pcall_prop(ctx: duk_context, obj_index: duk_idx_t,
                          nargs: duk_idx_t) -> duk_int_t;
    pub fn duk_new(ctx: duk_context, nargs: duk_idx_t);
    pub fn duk_push_current_function(ctx: duk_context);
    pub fn duk_push_global_object(ctx: duk_context);
    pub fn duk_push_c_function(ctx: duk_context, func: duk_c_function, nargs: duk_idx_t) -> duk_idx_t;
    pub fn duk_push_pointer(ctx: duk_context, p: *mut ::libc::c_void);
    pub fn duk_push_string(ctx: duk_context, str: *const ::libc::c_char) -> *const ::libc::c_char;
    pub fn duk_put_prop_string(ctx: duk_context, obj_index: duk_idx_t, key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_pop(ctx: duk_context);
    pub fn duk_pop_n(ctx: duk_context, count: duk_idx_t);
    pub fn duk_pop_2(ctx: duk_context);
    pub fn duk_pop_3(ctx: duk_context);
    pub fn duk_get_type(ctx: duk_context, index: duk_idx_t) -> duk_int_t;
    pub fn duk_get_magic(ctx: duk_context, index: duk_idx_t)
     -> duk_int_t;
    pub fn duk_set_magic(ctx: duk_context, index: duk_idx_t,
                         magic: duk_int_t);
    pub fn duk_get_current_magic(ctx: duk_context) -> duk_int_t;
    pub fn duk_push_undefined(ctx: duk_context);
    pub fn duk_push_null(ctx: duk_context);
    pub fn duk_push_boolean(ctx: duk_context, val: duk_bool_t);
    pub fn duk_push_true(ctx: duk_context);
    pub fn duk_push_false(ctx: duk_context);
    pub fn duk_push_number(ctx: duk_context, val: duk_double_t);
    pub fn duk_push_nan(ctx: duk_context);
    pub fn duk_push_int(ctx: duk_context, val: duk_int_t);
    pub fn duk_push_uint(ctx: duk_context, val: duk_uint_t);

    pub fn duk_push_lstring(ctx: duk_context, str: *const ::libc::c_char,
                            len: duk_size_t) -> *const ::libc::c_char;
    pub fn duk_put_global_string(ctx: duk_context,
                                 key: *const ::libc::c_char) -> duk_bool_t;
    pub fn duk_safe_to_lstring(ctx:duk_context, index: duk_idx_t,
                               out_len: *mut duk_size_t)
     -> *const ::libc::c_char;
    pub fn duk_get_lstring(ctx: duk_context, index: duk_idx_t,
                           out_len: *mut duk_size_t) -> *const ::libc::c_char;
    pub fn duk_get_boolean(ctx: duk_context, index: duk_idx_t)
     -> duk_bool_t;
    pub fn duk_get_number(ctx: duk_context, index: duk_idx_t)
     -> duk_double_t;
    pub fn duk_to_string(ctx: duk_context, index: duk_idx_t)
     -> *const ::libc::c_char;
}