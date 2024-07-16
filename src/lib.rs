#![feature(generic_arg_infer)]
#![feature(generic_const_exprs)]

use fast_collections::{generic_array::ArrayLength, Cursor};
pub use packetize_derive::*;

pub mod impls;
#[cfg(feature = "uuid")]
pub mod uuid;

pub trait Encode {
    fn encode<N: ArrayLength>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>
    where
        [(); N::USIZE]:;
}

pub trait Decode: Sized {
    fn decode<N: ArrayLength>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()>
    where
        [(); N::USIZE]:;
}
