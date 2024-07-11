#![feature(portable_simd)]
#![feature(generic_arg_infer)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]

use ::fast_collections::{generic_array::ArrayLength, Cursor};

pub use packetize_derive::*;

mod fast_collections;
mod impls;

pub trait Encode<N>
where
    N: ArrayLength,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>;
}

pub trait Decode<N>
where
    Self: Sized,
    N: ArrayLength,
{
    fn decode(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()>;
}
