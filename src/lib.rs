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

mod traits;
pub use traits::*;

///TODO Warning that if packet is only one in a state, than packet struct must not unit struct
#[cfg(feature = "stream")]
pub use packetize_derive::streaming_packets;
