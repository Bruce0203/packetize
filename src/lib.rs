#![feature(generic_arg_infer)]
#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub use packetize_derive::{Decode, Encode};

pub mod impls;
#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "nonmax")]
pub mod nonmax;

#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
pub use crate::stream::*;
#[cfg(feature = "stream")]
pub use packetize_derive::streaming_packets;

use fast_collections::Cursor;

pub trait Encode {
    fn encode<const N: usize>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>;
}

pub trait Decode: Sized {
    fn decode<const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()>;
}
