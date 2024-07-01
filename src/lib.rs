#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]

use ::fast_collections::{generic_array::ArrayLength, typenum::Len, Cursor};

mod fast_collections;

mod impls;

pub trait Encode<N>
where
    N: ArrayLength + Len,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>;

    unsafe fn encode_unchecked(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>;
}

pub trait Decode<N>
where
    N: ArrayLength + Len,
{
    fn decode(read_cursor: &mut Cursor<u8, N>) -> Self;
    unsafe fn decode_unchecked(read_cursor: &mut Cursor<u8, N>) -> Self;
}

pub trait SizedEncode {}
