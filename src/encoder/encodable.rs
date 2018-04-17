use std::any::{Any, TypeId};
use std::fmt::{self, Debug};
use std::result::{ Result as StdResult };
use std::sync::Arc;

use ::error::EncodingError;
use super::{EncodeHandle, Encoder};
use super::body::BodyBuffer;

/// Trait Implemented by mainly by structs representing a mail or
/// a part of it
pub trait Encodable<B: BodyBuffer> {
    //TODO potentially consume the Encoder and return a result with
    // the data as Vec<u8> ??
    fn encode( &self, encoder:  &mut Encoder<B>) -> Result<(), EncodingError>;
}


// can not be moved to `super::traits` as it depends on the
// EncodeHandle defined here
/// Trait Implemented by "components" used in header field bodies
///
/// This trait can be turned into a trait object allowing runtime
/// genericallity over the "components" if needed.
pub trait EncodableInHeader: Send + Sync + Any + Debug {
    fn encode(&self, encoder:  &mut EncodeHandle) -> Result<(), EncodingError>;

    fn boxed_clone(&self) -> Box<EncodableInHeader>;

    #[doc(hidden)]
    fn type_id( &self ) -> TypeId {
        TypeId::of::<Self>()
    }
}

//TODO we now could use MOPA or similar crates
impl EncodableInHeader {

    #[inline(always)]
    pub fn is<T: EncodableInHeader>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }


    #[inline]
    pub fn downcast_ref<T: EncodableInHeader>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some( unsafe { &*( self as *const EncodableInHeader as *const T) } )
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: EncodableInHeader>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some( unsafe { &mut *( self as *mut EncodableInHeader as *mut T) } )
        } else {
            None
        }
    }
}

impl Clone for Box<EncodableInHeader> {

    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}


pub trait EncodableInHeaderBoxExt: Sized {
    fn downcast<T: EncodableInHeader>(self) -> StdResult<Box<T>, Self>;
}

impl EncodableInHeaderBoxExt for Box<EncodableInHeader> {

    fn downcast<T: EncodableInHeader>(self) -> StdResult<Box<T>, Self> {
        if EncodableInHeader::is::<T>(&*self) {
            let ptr: *mut EncodableInHeader = Box::into_raw(self);
            Ok( unsafe { Box::from_raw(ptr as *mut T) } )
        } else {
            Err( self )
        }
    }
}

impl EncodableInHeaderBoxExt for Box<EncodableInHeader+Send> {

    fn downcast<T: EncodableInHeader>(self) -> StdResult<Box<T>, Self> {
        if EncodableInHeader::is::<T>(&*self) {
            let ptr: *mut EncodableInHeader = Box::into_raw(self);
            Ok( unsafe { Box::from_raw(ptr as *mut T) } )
        } else {
            Err( self )
        }
    }
}

#[macro_export]
macro_rules! enc_func {
    (|$enc:ident : &mut EncodeHandle| $block:block) => ({
        use $crate::error::EncodingError;
        fn _anonym($enc: &mut EncodeHandle) -> Result<(), EncodingError> {
            $block
        }
        let fn_pointer = _anonym as fn(&mut EncodeHandle) -> Result<(), EncodingError>;
        $crate::encoder::EncodeFn::new(fn_pointer)
    });
}

type _EncodeFn = for<'a, 'b: 'a> fn(&'a mut EncodeHandle<'b>) -> Result<(), EncodingError>;

#[derive(Clone, Copy)]
pub struct EncodeFn(_EncodeFn);

impl EncodeFn {
    pub fn new(func: _EncodeFn) -> Self {
        EncodeFn(func)
    }
}

impl EncodableInHeader for EncodeFn {
    fn encode(&self, encoder:  &mut EncodeHandle) -> Result<(), EncodingError> {
        (self.0)(encoder)
    }

    fn boxed_clone(&self) -> Box<EncodableInHeader> {
        Box::new(*self)
    }
}

impl Debug for EncodeFn {
    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        write!(fter, "EncodeFn(..)")
    }
}

#[macro_export]
macro_rules! enc_closure {
    ($($t:tt)*) => ({
        $crate::encoder::EncodeClosure::new($($t)*)
    });
}

pub struct EncodeClosure<FN: 'static>(Arc<FN>)
    where FN: Send + Sync +
        for<'a, 'b: 'a> Fn(&'a mut EncodeHandle<'b>) -> Result<(), EncodingError>;

impl<FN: 'static> EncodeClosure<FN>
    where FN: Send + Sync +
        for<'a, 'b: 'a> Fn(&'a mut EncodeHandle<'b>) -> Result<(), EncodingError>
{
    pub fn new(closure: FN) -> Self {
        EncodeClosure(Arc::new(closure))
    }
}

impl<FN: 'static> EncodableInHeader for EncodeClosure<FN>
    where FN: Send + Sync +
        for<'a, 'b: 'a> Fn(&'a mut EncodeHandle<'b>) -> Result<(), EncodingError>
{
    fn encode(&self, encoder:  &mut EncodeHandle) -> Result<(), EncodingError> {
        (self.0)(encoder)
    }

    fn boxed_clone(&self) -> Box<EncodableInHeader> {
        Box::new(self.clone())
    }
}

impl<FN: 'static> Clone for EncodeClosure<FN>
    where FN: Send + Sync +
        for<'a, 'b: 'a> Fn(&'a mut EncodeHandle<'b>) -> Result<(), EncodingError>
{
    fn clone(&self) -> Self {
        EncodeClosure(self.0.clone())
    }
}


impl<FN: 'static> Debug for EncodeClosure<FN>
    where FN: Send + Sync +
        for<'a, 'b: 'a> Fn(&'a mut EncodeHandle<'b>) -> Result<(), EncodingError>
{
    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        write!(fter, "EncodeClosure(..)")
    }
}