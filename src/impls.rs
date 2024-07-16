use fast_collections::{generic_array::ArrayLength, CursorReadTransmute, PushTransmute};

use crate::{Decode, Encode};
use fast_collections::{
    generic_array::{typenum, IntoArrayLength},
    Cursor, String, Vec,
};

macro_rules! impl_encoder_and_decoder {
    ($($name:ident),*) => {
        $(
        impl Encode for $name {
            #[inline(always)]
            fn encode<N: ArrayLength>(
                &self,
                write_cursor: &mut fast_collections::Cursor<u8, N>,
            ) -> Result<(), ()>
            {
                write_cursor.push_transmute(Self::to_be_bytes(*self))?;
                Ok(())
            }
        }
        impl Decode for $name
        {
            #[inline(always)]
            fn decode<N: ArrayLength>(read_cursor: &mut fast_collections::Cursor<u8, N>) -> Result<Self, ()> {
                CursorReadTransmute::read_transmute::<[u8; _]>(read_cursor)
                    .map(|v| Self::from_be_bytes(*v))
                    .ok_or_else(|| ())
            }
        }
        )*
    };
}

impl Encode for bool {
    #[inline(always)]
    fn encode<N: ArrayLength>(
        &self,
        write_cursor: &mut fast_collections::Cursor<u8, N>,
    ) -> Result<(), ()> {
        write_cursor.push_transmute(Self::from(*self))?;
        Ok(())
    }
}

impl Decode for bool {
    fn decode<N: ArrayLength>(
        read_cursor: &mut fast_collections::Cursor<u8, N>,
    ) -> Result<Self, ()> {
        CursorReadTransmute::read_transmute::<Self>(read_cursor)
            .map(|v| *v)
            .ok_or_else(|| ())
    }
}

impl_encoder_and_decoder! {
    usize, isize,
    u8,    i8,
    u16,   i16,
    u32,   i32,
    u64,   i64,
    u128,  i128,
    f32, f64
}

impl<VecLen> Encode for Vec<u8, VecLen>
where
    VecLen: ArrayLength,
    [(); VecLen::USIZE]:,
{
    fn encode<CursorLen: ArrayLength>(
        &self,
        write_cursor: &mut Cursor<u8, CursorLen>,
    ) -> Result<(), ()>
    where
        [(); CursorLen::USIZE]:,
    {
        let len = self.len();
        unsafe {
            let encoded_len =
                integer_encoding::VarInt::encode_var(len, write_cursor.unfilled_mut());
            if encoded_len == 0 {
                return Err(());
            }
            let filled_len_mut = write_cursor.filled_len_mut();
            let filled_len_0 = *filled_len_mut;
            let filled_len_1 = filled_len_0.unchecked_add(encoded_len);
            let filled_len_2 = filled_len_1.unchecked_add(len);
            if encoded_len + len >= CursorLen::USIZE - filled_len_0 {
                return Err(());
            }
            *filled_len_mut = filled_len_2;
            write_cursor.as_array()[filled_len_1..filled_len_2]
                .copy_from_slice(&self.as_array()[..len]);
        }
        Ok(())
    }
}

impl<VecLen> Decode for Vec<u8, VecLen>
where
    VecLen: ArrayLength,
    [(); VecLen::USIZE]:,
{
    fn decode<CursorLen: ArrayLength>(read_cursor: &mut Cursor<u8, CursorLen>) -> Result<Self, ()> {
        let mut vec = Vec::<u8, VecLen>::uninit();
        let pos = read_cursor.pos();
        let filled = &read_cursor.filled()[pos..];
        let (length, read_length) =
            <u32 as integer_encoding::VarInt>::decode_var(filled).ok_or_else(|| ())?;
        let length = length as usize;
        let read_length_plus_length = unsafe { read_length.unchecked_add(length) };
        let new_pos = unsafe { pos.unchecked_add(read_length_plus_length) };
        if filled.len() < read_length_plus_length {
            return Err(());
        }
        vec.as_array_mut()[..length].copy_from_slice(&filled[read_length..read_length_plus_length]);
        *unsafe { read_cursor.pos_mut() } = new_pos;
        *unsafe { vec.len_mut() } = length;
        Ok(vec)
    }
}

impl<StrLen> Encode for String<StrLen>
where
    StrLen: ArrayLength,
    [(); StrLen::USIZE]:,
{
    fn encode<CursorLen: ArrayLength>(
        &self,
        write_cursor: &mut Cursor<u8, CursorLen>,
    ) -> Result<(), ()>
    where
        [(); CursorLen::USIZE]:,
    {
        let vec: &Vec<u8, StrLen> = unsafe { fast_collections::const_transmute_unchecked(self) };
        Encode::encode(vec, write_cursor)
    }
}

impl<StrLen> Decode for String<StrLen>
where
    typenum::Const<{ StrLen::USIZE }>: IntoArrayLength<ArrayLength = StrLen>,
    StrLen: ArrayLength,
{
    fn decode<CursorLen: ArrayLength>(read_cursor: &mut Cursor<u8, CursorLen>) -> Result<Self, ()>
    where
        [(); CursorLen::USIZE]:,
    {
        let vec: Vec<u8, StrLen> = Decode::decode(read_cursor)?;
        Ok(unsafe { fast_collections::const_transmute_unchecked(vec) })
    }
}
