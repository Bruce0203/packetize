use std::mem::{transmute, transmute_copy, MaybeUninit};

use fastbuf::{ReadBuf, WriteBuf};
use fastvarint::VarInt;

use crate::{Decode, Encode};

pub trait Packet<T> {
    fn id(state: &T) -> Option<u32>;
    fn is_changing_state() -> Option<T>;
}

pub trait ClientBoundPacketStream {
    type BoundPacket;

    fn decode_client_bound_packet(
        &mut self,
        buf: &mut impl ReadBuf,
    ) -> Result<Self::BoundPacket, ()>;

    fn encode_client_bound_packet(
        &mut self,
        packet: &Self::BoundPacket,
        buf: &mut impl WriteBuf,
    ) -> Result<(), ()>;
}

pub trait ServerBoundPacketStream {
    type BoundPacket;
    fn decode_server_bound_packet(
        &mut self,
        buf: &mut impl ReadBuf,
    ) -> Result<Self::BoundPacket, ()>;

    fn encode_server_bound_packet(
        &mut self,
        packet: &Self::BoundPacket,
        buf: &mut impl WriteBuf,
    ) -> Result<(), ()>;
}

pub trait PacketStreamFormat: Sized {
    fn read_packet_id<ID>(buf: &mut impl ReadBuf) -> Result<ID, ()>
    where
        ID: Default,
        [(); size_of::<ID>()]:;

    fn write_packet_with_id<T, P>(
        state: &mut T,
        packet: &P,
        buf: &mut impl WriteBuf,
    ) -> Result<(), ()>
    where
        P: Packet<T> + Encode;

    fn read_packet<T, P>(state: &mut T, buf: &mut impl ReadBuf) -> Result<P, ()>
    where
        P: Decode + Packet<T>,
    {
        if let Some(s) = P::is_changing_state() {
            *state = s;
        }
        Ok(P::decode(buf)?)
    }
}

pub struct SimplePacketStreamFormat;

impl PacketStreamFormat for SimplePacketStreamFormat {
    fn read_packet_id<ID>(buf: &mut impl ReadBuf) -> Result<ID, ()>
    where
        ID: Default,
        [(); size_of::<ID>()]:,
    {
        let buffer = buf.get_continuous(<u32 as VarInt>::MAX_VAR_INT_SPACE);
        let (data, read_length) = u32::decode_var(|i| Ok(unsafe { *buffer.get_unchecked(i) }))?;
        buf.advance(read_length);
        unsafe { transmute_copy(&data) }
    }

    fn write_packet_with_id<T, P>(
        state: &mut T,
        packet: &P,
        buf: &mut impl WriteBuf,
    ) -> Result<(), ()>
    where
        P: Packet<T> + Encode,
    {
        match P::id(state) {
            Some(id) => {
                if buf.remaining_space() < <u32 as VarInt>::MAX_VAR_INT_SPACE {
                    Err(())?
                }
                id.encode_var(|b| {
                    buf.write(&[b]);
                    Ok(())
                })?;
            }
            None => Err(())?,
        };

        if let Some(s) = P::is_changing_state() {
            *state = s;
        }
        packet.encode(buf)?;
        Ok(())
    }
}
