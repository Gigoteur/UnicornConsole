use std::borrow::Cow;
use libc::c_double;

/// A value that can be passed to and from JavaScript.  This does not
/// include all the types that can be stored internally!
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    /// An undefined JavaScript value.
    Undefined,
    /// A JavaScript `null` value.
    Null,
    /// A JavaScript boolean value.
    Bool(bool),
    /// A JavaScript numeric value.
    Number(c_double),
    /// A JavaScript string value.
    String(Cow<'a, str>)
}