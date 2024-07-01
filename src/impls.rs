use fast_collections::{
    generic_array::ArrayLength, typenum::Len, PushTransmute, PushTransmuteUnchecked,
};

use crate::Encode;

macro_rules! impl_encoder_and_decoder {
    ($($name:ident),*) => {
        $(
        impl<N> Encode<N> for $name
            where N: ArrayLength + Len,
        {
            fn encode(
                &self,
                write_cursor: &mut fast_collections::Cursor<u8, N>,
            ) -> Result<(), ()>
            {
                write_cursor.push_transmute(&self)?;
                Ok(())
            }

            unsafe fn encode_unchecked(
                &self,
                write_cursor: &mut fast_collections::Cursor<u8, N>,
            ) -> Result<(), ()>
            {
                write_cursor.push_transmute_unchecked(*self);
                Ok(())
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
