use fast_collections::{
    generic_array::{typenum, ArrayLength, IntoArrayLength},
    Cursor, String, Vec,
};

use crate::{Decode, Encode};

impl<CursorLen, VecLen> Encode<CursorLen> for Vec<u8, VecLen>
where
    CursorLen: ArrayLength,
    VecLen: ArrayLength,
    [(); CursorLen::USIZE]:,
    [(); VecLen::USIZE]:,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, CursorLen>) -> Result<(), ()> {
        let len = self.len();
        unsafe {
            let encoded_len =
                integer_encoding::VarInt::encode_var(len, write_cursor.unfilled_mut());
            let filled_len_mut = write_cursor.filled_len_mut();
            let filled_len_1 = (*filled_len_mut).unchecked_add(encoded_len);
            let filled_len_2 = filled_len_1.unchecked_add(len);
            if filled_len_2 >= CursorLen::USIZE {
                return Err(());
            }
            *filled_len_mut = filled_len_2;
            write_cursor.as_array()[filled_len_1..filled_len_2]
                .copy_from_slice(&self.as_array()[..len]);
        }
        Ok(())
    }
}

impl<CursorLen, VecLen> Decode<CursorLen> for Vec<u8, VecLen>
where
    VecLen: ArrayLength,
    CursorLen: ArrayLength,
    [(); VecLen::USIZE]:,
{
    fn decode(read_cursor: &mut Cursor<u8, CursorLen>) -> Result<Self, ()> {
        let mut vec = Vec::<u8, VecLen>::uninit();
        let filled = read_cursor.filled();
        let (length, read_length) =
            <u32 as integer_encoding::VarInt>::decode_var(filled).ok_or_else(|| ())?;
        let length = length as usize;
        let read_length_plus_length = unsafe { read_length.unchecked_add(length) };
        let new_pos = read_cursor.pos() + read_length_plus_length;
        if filled.len() < read_length_plus_length {
            return Err(());
        }
        vec.as_array_mut()[..length].copy_from_slice(&filled[read_length..read_length_plus_length]);
        *unsafe { read_cursor.pos_mut() } = new_pos;
        *unsafe { vec.len_mut() } = length;
        Ok(vec)
    }
}

impl<CursorLen, StrLen> Encode<CursorLen> for String<StrLen>
where
    CursorLen: ArrayLength,
    StrLen: ArrayLength,
    [(); CursorLen::USIZE]:,
    [(); StrLen::USIZE]:,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, CursorLen>) -> Result<(), ()> {
        let vec: &Vec<u8, StrLen> = unsafe { fast_collections::const_transmute_unchecked(self) };
        Encode::encode(vec, write_cursor)
    }
}

impl<CursorLen, StrLen> Decode<CursorLen> for String<StrLen>
where
    typenum::Const<{ StrLen::USIZE }>: IntoArrayLength<ArrayLength = StrLen>,
    StrLen: ArrayLength,
    CursorLen: ArrayLength,
    [(); CursorLen::USIZE]:,
{
    fn decode(read_cursor: &mut Cursor<u8, CursorLen>) -> Result<Self, ()> {
        let vec: Vec<u8, StrLen> = Decode::decode(read_cursor)?;
        Ok(unsafe { fast_collections::const_transmute_unchecked(vec) })
    }
}
