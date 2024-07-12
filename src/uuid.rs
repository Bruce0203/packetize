use fast_collections::{generic_array::ArrayLength, Cursor, CursorReadTransmute, PushTransmute};
use uuid::Uuid;

use crate::{Decode, Encode};

impl<N> Encode<N> for Uuid
where
    N: ArrayLength,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
        write_cursor.push_transmute(*self.as_bytes())
    }
}

impl<N> Decode<N> for Uuid
where
    N: ArrayLength,
{
    fn decode(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()> {
        Ok(Uuid::from_bytes(
            *read_cursor.read_transmute::<[u8; 16]>().ok_or_else(|| ())?,
        ))
    }
}
