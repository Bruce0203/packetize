use fast_collections::{
    const_transmute_unchecked,
    generic_array::ArrayLength,
    typenum::{Integer, Len, Unsigned},
    Cursor, PushTransmute, PushTransmuteUnchecked, String,
};

use crate::Encode;

pub const fn bits_len_to_bytes(len: usize) -> usize {
    len / 8 + 1
}

impl<N, StrLen> Encode<N> for String<StrLen>
where
    N: ArrayLength + Len,
    StrLen: ArrayLength + Len,
    [(); bits_len_to_bytes(<StrLen as Len>::Output::USIZE)]:,
    [(); StrLen::USIZE]:,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
        let compact_len = unsafe {
            const_transmute_unchecked::<
                usize,
                [u8; bits_len_to_bytes(<StrLen as Len>::Output::USIZE)],
            >(self.len())
        };
        write_cursor.push_transmute(compact_len)?;
        println!("{:?}", compact_len);
        write_cursor.push_transmute(*self.as_vec().as_array())?;
        Ok(())
    }

    unsafe fn encode_unchecked(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
        let compact_len = unsafe {
            const_transmute_unchecked::<
                usize,
                [u8; bits_len_to_bytes(<StrLen as Len>::Output::USIZE)],
            >(self.len())
        };
        write_cursor.push_transmute_unchecked(compact_len);
        write_cursor.push_transmute_unchecked(self.as_vec().as_array());
        Ok(())
    }
}
