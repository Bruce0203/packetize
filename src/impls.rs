use fast_collections::{generic_array::ArrayLength, CursorReadTransmute, PushTransmute};

use crate::{Decode, Encode};

macro_rules! impl_encoder_and_decoder {
    ($($name:ident),*) => {
        $(
        impl<N> Encode<N> for $name
            where N: ArrayLength,
        {
            #[inline(always)]
            fn encode(
                &self,
                write_cursor: &mut fast_collections::Cursor<u8, N>,
            ) -> Result<(), ()>
            {
                write_cursor.push_transmute(*self)?;
                Ok(())
            }
        }

        impl<N> Decode<N> for $name
        where
            N: ArrayLength,
        {
            fn decode(read_cursor: &mut fast_collections::Cursor<u8, N>) -> Result<Self, ()> {
                CursorReadTransmute::read_transmute(read_cursor)
                    .map(|v| *v)
                    .ok_or_else(|| ())
            }
        }
        )*
    };
}

impl_encoder_and_decoder! {
    usize, isize,
    u8,    i8,
    u16,   i16,
    u32,   i32,
    u64,   i64,
    u128,  i128,
    bool,
    f32, f64

}
