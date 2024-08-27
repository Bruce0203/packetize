#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub use packetize_derive::packet_stream;

mod stream;
pub use crate::stream::*;
