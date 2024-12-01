#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub use packetize_derive::packet_stream;

pub trait Packet<T> {
    fn get_id(&self, state: &T) -> Option<u32>;
    fn is_changing_state(&self) -> Option<T>;
}

pub trait EncodePacket<T> {
    fn encode_packet<E: serialization::Encoder>(
        &self,
        encoder: E,
        state: &mut T,
    ) -> Result<(), E::Error>;
}

pub trait DecodePacket<T>: Sized {
    fn decode_packet<'de, D: serialization::Decoder<'de>>(
        decoder: D,
        state: &mut T,
    ) -> Result<Self, D::Error>;
}
