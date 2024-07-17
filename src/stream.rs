use fast_collections::{const_transmute_unchecked, Cursor, CursorReadTransmute, Push};
use integer_encoding::VarInt;

use crate::{Decode, Encode};

pub trait Packet<T> {
    fn id(state: &T) -> Option<u32>;
    fn is_changing_state() -> Option<T>;
}

pub trait PacketStreamFormat: Sized {
    fn read_packet_id<P, const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<P, ()>
    where
        P: Default,
        [(); size_of::<P>()]:;

    fn write_packet_with_id<T, P, const N: usize>(
        state: &mut T,
        packet: &P,
        cursor: &mut Cursor<u8, N>,
    ) -> Result<(), ()>
    where
        P: Packet<T> + Encode;

    fn read_packet<T, P, const N: usize>(
        state: &mut T,
        read_cursor: &mut Cursor<u8, N>,
    ) -> Result<P, ()>
    where
        P: Decode + Packet<T>,
    {
        if let Some(s) = P::is_changing_state() {
            *state = s;
        }
        Ok(P::decode(read_cursor)?)
    }
}

pub struct SimplePacketStreamFormat;

impl PacketStreamFormat for SimplePacketStreamFormat {
    fn read_packet_id<P, const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<P, ()>
    where
        P: Default,
        [(); size_of::<P>()]:,
    {
        Ok(unsafe {
            if size_of::<P>() == 0 {
                P::default()
            } else if size_of::<P>() >= u8::MAX as usize {
                let (len, read_length) =
                    VarInt::decode_var(read_cursor.unfilled_mut()).ok_or_else(|| ())?;
                *read_cursor.filled_len_mut() += read_length;
                let len: u32 = len;
                const_transmute_unchecked(len)
            } else {
                const_transmute_unchecked(
                    *read_cursor
                        .read_transmute::<[u8; size_of::<P>()]>()
                        .ok_or_else(|| ())?,
                )
            }
        })
    }

    fn write_packet_with_id<T, P, const N: usize>(
        state: &mut T,
        packet: &P,
        cursor: &mut Cursor<u8, N>,
    ) -> Result<(), ()>
    where
        P: Packet<T> + Encode,
    {
        Self::write_packet_id::<_, P, _>(state, cursor)?;
        if let Some(s) = P::is_changing_state() {
            *state = s;
        }
        packet.encode(cursor)?;
        Ok(())
    }
}

impl SimplePacketStreamFormat {
    fn write_packet_id<T, P, const N: usize>(
        state: &T,
        write_cursor: &mut Cursor<u8, N>,
    ) -> Result<(), ()>
    where
        P: Packet<T>,
    {
        match P::id(state) {
            Some(id) => {
                if id > u8::MAX as u32 {
                    unsafe {
                        let write_len = VarInt::encode_var(id, write_cursor.unfilled_mut());
                        *write_cursor.filled_len_mut() += write_len;
                    }
                } else {
                    write_cursor.push(id as u8).map_err(|_| ())?;
                }
            }
            None => {}
        }
        Ok(())
    }
}
