use fast_collections::{Cursor, CursorReadTransmute};
use nonmax::{
    NonMaxI128, NonMaxI16, NonMaxI32, NonMaxI64, NonMaxI8, NonMaxIsize, NonMaxU128, NonMaxU16,
    NonMaxU32, NonMaxU64, NonMaxU8, NonMaxUsize,
};

use crate::{Decode, Encode};

macro_rules! impl_nonmax {
    ($($name:ident),*) => {
        $(
        impl Encode for $name {
            fn encode<const N: usize>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
                self.get().encode(write_cursor)
            }
        }

        impl Decode for $name {
            fn decode<const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()> {
                Ok(
                    unsafe {
                        Self::new_unchecked(*read_cursor.read_transmute().ok_or_else(|| ())?)
                    },
                )
            }
        }
        )*
    };
}

impl_nonmax!(
    NonMaxI8,
    NonMaxU8,
    NonMaxI16,
    NonMaxU16,
    NonMaxI32,
    NonMaxU32,
    NonMaxI64,
    NonMaxU64,
    NonMaxU128,
    NonMaxI128,
    NonMaxUsize,
    NonMaxIsize
);
