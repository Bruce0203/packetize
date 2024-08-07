use std::mem::MaybeUninit;

use arrayvec::{ArrayString, ArrayVec};
use fastbuf::{ReadBuf, WriteBuf};

use crate::{Decode, Encode};

impl<const CAP: usize> Encode for ArrayVec<u8, CAP> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        let len = self.len() as u32;
        unsafe {
            #[allow(invalid_value)]
            let mut buffer: [u8; u32::BITS as usize / 8 + 1] =
                [MaybeUninit::uninit().assume_init(); u32::BITS as usize / 8 + 1];
            let encoded_len = integer_encoding::VarInt::encode_var(len, &mut buffer);
            buf.write(&buffer[..encoded_len])?;
            buf.write(self.as_slice())?;
        }
        Ok(())
    }
}

impl<const N: usize> Decode for ArrayVec<u8, N> {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayVec::<u8, N>::new();
        let (len, read_len) =
            integer_encoding::VarInt::decode_var(buf.get_continuous((i32::BITS / 8 + 1) as usize))
                .ok_or_else(|| ())?;
        let len: u32 = len;
        let len = len as usize;
        buf.advance(read_len + len);
        vec.as_mut_slice().copy_from_slice(buf.get_continuous(len));
        unsafe { vec.set_len(len) };
        Ok(vec)
    }
}

impl<const N: usize> Encode for ArrayString<N> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        let len = self.len() as u32;
        unsafe {
            #[allow(invalid_value)]
            let mut buffer: [u8; u32::BITS as usize / 8 + 1] =
                [MaybeUninit::uninit().assume_init(); u32::BITS as usize / 8 + 1];
            let encoded_len = integer_encoding::VarInt::encode_var(len, &mut buffer);
            buf.write(&buffer[..encoded_len])?;
            buf.write(self.as_bytes())?;
        }
        Ok(())
    }
}

impl<const N: usize> Decode for ArrayString<N> {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayString::<N>::new();
        let (len, read_len) =
            integer_encoding::VarInt::decode_var(buf.get_continuous(i32::BITS as usize / 8 + 1))
                .ok_or_else(|| ())?;
        let len: u32 = len;
        let len = len as usize;
        unsafe { vec.set_len(len) };
        unsafe { vec.as_bytes_mut().copy_from_slice(buf.get_continuous(len)) };
        buf.advance(read_len + len);
        Ok(vec)
    }
}

impl<T: Encode, const N: usize> Encode for ArrayVec<T, N> {
    default fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        #[allow(invalid_value)]
        let mut buffer =
            [unsafe { MaybeUninit::uninit().assume_init() }; u32::BITS as usize / 8 + 1];
        let write_len = integer_encoding::VarInt::encode_var(self.len() as u32, &mut buffer);
        buf.write(&buffer[..write_len])?;
        for ele in self.iter() {
            ele.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Decode, const N: usize> Decode for ArrayVec<T, N> {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayVec::<T, N>::new();
        let (len, read_len) =
            integer_encoding::VarInt::decode_var(buf.get_continuous((i32::BITS / 8 + 1) as usize))
                .ok_or_else(|| ())?;
        let len: u32 = len;
        let len = len as usize;
        buf.advance(read_len);
        unsafe { vec.set_len(len) };
        for i in 0..len {
            *unsafe { vec.get_unchecked_mut(i) } = T::decode(buf)?;
        }
        Ok(vec)
    }
}
