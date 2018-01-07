use std::ops::Deref;
use std::ptr::null_mut;
use cesu8::to_cesu8;
use ffi::*;
use errors::*;
use Context;
use serde::ser::{Serialize, Serializer};
/*
/// Translates Rust values into JavaScript values.
pub struct Encoder {
    /// An internal `Context` object, for convenience.  We own this,
    /// because if we use a reference to somebody else's, the lifetimes
    /// make it very hard to work with &Encodable references.
    ctx: Context
}

impl Encoder {
    /// Create a new encoder which pushes values to `ctx`.  If you create
    /// one of these, you're responsible for making sure it gets used
    /// safely.
    pub unsafe fn new(ctx: duk_context) -> Encoder {
        Encoder{ctx: Context::from_borrowed_mut_ptr(ctx)}
    }
}

type EncodeResult = DuktapeResult<()>;
*/
/*
pub fn serialize<S: Serialize>(ctx: duk_context, data: S) {
    let mut serializer = Encoder::new(ctx);
    data.serialize(&mut serializer);
}

impl Serializer for Encoder {


}*/