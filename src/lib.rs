#![feature(generic_arg_infer)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fast_collections::Cursor;
pub use packetize_derive::*;

pub mod impls;
#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "uuid")]
pub mod uuid;

pub trait Encode {
    fn encode<const N: usize>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>;
}

pub trait Decode: Sized {
    fn decode<const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()>;
}
