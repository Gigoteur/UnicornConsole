use std::error::Error;
use std::fmt;
use std::result::Result;
use ffi::*;

/// These are the standard error codes, which make it easy to return
/// pre-defined errors from duktape functions implemented in Rust.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    None = DUK_ERR_NONE,
    Error = DUK_ERR_ERROR,
    Eval = DUK_ERR_EVAL_ERROR,
    Range = DUK_ERR_RANGE_ERROR,
    Reference = DUK_ERR_REFERENCE_ERROR,
    Syntax = DUK_ERR_SYNTAX_ERROR,
    Type = DUK_ERR_TYPE_ERROR,
    Uri = DUK_ERR_URI_ERROR
}

/// A duktape API error.  The is used as both the return type of duktape of
/// functions, and also the return type of Rust functions called from
/// duktape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuktapeError {
    /// The error code, if a specific one is available, or
    /// `ErrorCode::Error` if we have nothing better.
    code: ErrorCode,

    /// Errors have some sort of internal structure, but the duktape
    /// documentation always just converts them to strings.  So that's all
    /// we'll store for now.
    message: Option<String>
}

impl DuktapeError {
    /// Create an error specifying just the error code.
    pub fn from_code(code: ErrorCode) -> DuktapeError {
        DuktapeError{code: code, message: None}
    }

    /// Create an error, specifying an error message.
    pub fn from_str(message: &str) -> DuktapeError {
        DuktapeError{code: ErrorCode::Error, message: Some(message.to_string())}
    }
}

/// Re-exported within the crate, but not outside.
pub fn err_code(err: &DuktapeError) -> ErrorCode { err.code.clone() }
pub fn err_message(err: &DuktapeError) -> &Option<String> { &err.message }

impl Error for DuktapeError {
    fn description(&self) -> &str { "script error:" }

    fn cause(&self) -> Option<&Error> { None }
}

impl fmt::Display for DuktapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.message, self.code.clone()) {
            (&Some(ref msg), _) => write!(f, "{}", msg),
            (&None, ErrorCode::Error) => write!(f, "an unknown error occurred"),
            (&None, code) => 
                write!(f, "type: {:?} code: {:?}", code, code.clone() as duk_int_t)
        }
    }
}

/// Either a return value of type `T`, or a duktape error.
pub type DuktapeResult<T> = Result<T, DuktapeError>;