#![feature(generic_const_exprs)]

pub use packetize_derive::*;

pub mod fast_collections;
pub mod impls;

pub trait Encode<N>
where
    N: ::fast_collections::generic_array::ArrayLength,
{
    fn encode(&self, write_cursor: &mut ::fast_collections::Cursor<u8, N>) -> Result<(), ()>;
}

pub trait Decode<N>
where
    Self: Sized,
    N: ::fast_collections::generic_array::ArrayLength,
{
    fn decode(read_cursor: &mut ::fast_collections::Cursor<u8, N>) -> Result<Self, ()>;
}
