use std::mem::{transmute_copy, MaybeUninit};

use fastbuf::{ReadBuf, WriteBuf};
use integer_encoding::VarInt;

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
    fn read_packet_id<ID>(read_cursor: &mut impl ReadBuf) -> Result<ID, ()>
    where
        ID: Default,
        [(); size_of::<ID>()]:;

    fn write_packet_with_id<T, P>(
        state: &mut T,
        packet: &P,
        cursor: &mut impl WriteBuf,
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
        Ok(unsafe {
            if size_of::<ID>() == 0 {
                ID::default()
            } else if size_of::<ID>() >= u8::MAX as usize {
                let (len, read_length) =
                    VarInt::decode_var(buf.get_continuous(u32::BITS as usize / 8 + 1))
                        .ok_or_else(|| ())?;
                let len: u32 = len;
                buf.advance(read_length);
                transmute_copy::<_, ID>(&len)
            } else {
                #[allow(invalid_value)]
                let mut value: [u8; size_of::<ID>()] =
                    [MaybeUninit::uninit().assume_init(); size_of::<ID>()];
                value.copy_from_slice(buf.read(size_of::<ID>()));
                transmute_copy::<_, ID>(&value)
            }
        })
    }

    fn write_packet_with_id<T, P>(
        state: &mut T,
        packet: &P,
        cursor: &mut impl WriteBuf,
    ) -> Result<(), ()>
    where
        P: Packet<T> + Encode,
    {
        Self::write_packet_id::<_, P>(state, cursor)?;
        if let Some(s) = P::is_changing_state() {
            *state = s;
        }
        packet.encode(cursor)?;
        Ok(())
    }
}

impl SimplePacketStreamFormat {
    fn write_packet_id<T, P>(state: &T, buf: &mut impl WriteBuf) -> Result<(), ()>
    where
        P: Packet<T>,
    {
        match P::id(state) {
            Some(id) => {
                if id > u8::MAX as u32 {
                    unsafe {
                        #[allow(invalid_value)]
                        let mut buffer: [u8; u32::BITS as usize / 8 + 1] =
                            [MaybeUninit::uninit().assume_init(); u32::BITS as usize / 8 + 1];
                        let write_len = VarInt::encode_var(id, &mut buffer);
                        buf.try_write(&buffer[..write_len])?;
                    }
                } else {
                    buf.try_write(&id.to_be_bytes())?;
                }
            }
            None => {}
        }
        Ok(())
    }
}
