use core::simd;

use fast_collections::{
    const_transmute_unchecked,
    generic_array::ArrayLength,
    typenum::{Len, Unsigned},
    Cursor, GetTransmuteUnchecked, PushTransmute, SetTransmute, String, Vec,
};

use crate::{Decode, Encode};

pub const fn bits_len_to_bytes(len: usize) -> usize {
    len / 8 + 1
}

impl<CursorLen, StrLen> Encode<CursorLen> for String<StrLen>
where
    CursorLen: ArrayLength + Len,
    StrLen: ArrayLength + Len,
    [(); bits_len_to_bytes(<StrLen as Len>::Output::USIZE)]:,
    [(); StrLen::USIZE]:,
{
    fn encode(self, write_cursor: &mut Cursor<u8, CursorLen>) -> Result<(), ()> {
        let compact_len = unsafe {
            const_transmute_unchecked::<_, [u8; bits_len_to_bytes(<StrLen as Len>::Output::USIZE)]>(
                self.len(),
            )
        };
        write_cursor.push_transmute((compact_len, *self.as_vec().as_array()))?;
        //TODO push_transmut를 맨 마지막에 필드들을 다 모아서 튜플로 푸시하자
        Ok(())
    }

    #[inline(always)]
    unsafe fn encode_unchecked(self, write_cursor: &mut Cursor<u8, CursorLen>) {
        let len = self.len();
        let compact_len = const_transmute_unchecked::<
            _,
            [u8; bits_len_to_bytes(<StrLen as Len>::Output::USIZE)],
        >(len);
        let filled_len_mut = write_cursor.filled_len_mut();
        let filled_len = *filled_len_mut;
        *filled_len_mut = filled_len
            .unchecked_add(const { bits_len_to_bytes(<StrLen as Len>::Output::USIZE) })
            .unchecked_add(len);
        write_cursor.set_transmute_unchecked(filled_len, compact_len);
        let array: &mut [u8; StrLen::USIZE] = write_cursor.get_transmute_mut_unchecked(filled_len);
        let src: [u8; 8] = const_transmute_unchecked((compact_len, *self.as_vec().as_array()));
        let src_simd = simd::u8x8::from_slice(&src);
        src_simd.copy_to_slice(array);
    }
}

impl<CursorLen, StrLen> Decode<CursorLen> for String<StrLen>
where
    StrLen: ArrayLength + Len,
    CursorLen: ArrayLength + Len,
{
    fn decode(read_cursor: &mut Cursor<u8, CursorLen>) -> Result<Self, ()> {
        Ok(unsafe { Self::decode_unchecked(read_cursor) })
    }

    unsafe fn decode_unchecked(read_cursor: &mut Cursor<u8, CursorLen>) -> Self {
        String::new()
    }
}

impl<CursorLen, VecLen, T> Decode<CursorLen> for Vec<T, VecLen>
where
    VecLen: ArrayLength + Len,
    CursorLen: ArrayLength + Len,
{
    fn decode(read_cursor: &mut Cursor<u8, CursorLen>) -> Result<Self, ()> {
        Ok(unsafe { Self::decode_unchecked(read_cursor) })
    }

    unsafe fn decode_unchecked(read_cursor: &mut Cursor<u8, CursorLen>) -> Self {
        let vec = Vec::uninit();
        vec
    }
}
