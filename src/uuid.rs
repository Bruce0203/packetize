use fast_collections::Cursor;
use uuid::Uuid;

use crate::{Decode, Encode};

impl Encode for Uuid {
    fn encode<const N: usize>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
        write_cursor.push_transmute(*self.as_bytes())
    }
}

impl Decode for Uuid {
    fn decode<const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()> {
        Ok(Uuid::from_bytes(
            *read_cursor.read_transmute::<[u8; 16]>().ok_or_else(|| ())?,
        ))
    }
}
