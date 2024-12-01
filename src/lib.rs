#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub use packetize_derive::packet_stream;

pub trait Packet<T> {
    fn get_id(&self, state: &T) -> Option<u32>;
    fn is_changing_state(&self) -> Option<T>;
}
