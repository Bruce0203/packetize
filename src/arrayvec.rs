///! TODO remove code dups..
use arrayvec::{ArrayString, ArrayVec};
use fastbuf::{ReadBuf, WriteBuf};
use fastvarint::VarInt;

use crate::{Decode, Encode};

impl<const CAP: usize> Encode for ArrayVec<u8, CAP> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        let len = self.len() as u32;
        len.encode_var(|v| {
            buf.write(&[v]);
            Ok(())
        })?;
        buf.write(self.as_slice());
        Ok(())
    }
}

impl<const N: usize> Decode for ArrayVec<u8, N> {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayVec::<u8, N>::new();
        if u32::MAX_VAR_INT_SPACE > buf.remaining() {
            Err(())?
        }
        let buffer = buf.get_continuous(u32::MAX_VAR_INT_SPACE);
        let (len, read_len) = u32::decode_var(|i| Ok(unsafe { *buffer.get_unchecked(i) }))?;
        let len = len as usize;
        //TRY to advance bulk, and get_continuous with slicing
        buf.advance(read_len);
        vec.as_mut_slice().copy_from_slice(buf.get_continuous(len));
        unsafe { vec.set_len(len) };
        Ok(vec)
    }
}

impl<const N: usize> Encode for ArrayString<N> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        if self.len() + u32::MAX_VAR_INT_SPACE >= buf.remaining_space() {
            Err(())?
        }
        (self.len() as u32)
            .encode_var(|b| {
                buf.write(&[b]);
                Ok(())
            })
            .unwrap();
        buf.write(self.as_bytes());
        Ok(())
    }
}

impl<const N: usize> Decode for ArrayString<N> {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayString::<N>::new();
        let buffer = buf.get_continuous(u32::MAX_VAR_INT_SPACE);
        let (len, read_len) = u32::decode_var(|i| Ok(*unsafe { buffer.get_unchecked(i) })).unwrap();
        buf.advance(read_len);
        let len: u32 = len;
        let len = len as usize;
        unsafe { vec.set_len(len) };
        unsafe { vec.as_bytes_mut().copy_from_slice(buf.get_continuous(len)) };
        buf.advance(len);
        Ok(vec)
    }
}

impl<T: Encode, const N: usize> Encode for ArrayVec<T, N> {
    default fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        if self.len() + u32::MAX_VAR_INT_SPACE > buf.remaining_space() {
            Err(())?
        }
        (self.len() as u32)
            .encode_var(|b| {
                buf.write(&[b]);
                Ok(())
            })
            .unwrap();
        for ele in self.iter() {
            ele.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Decode, const N: usize> Decode for ArrayVec<T, N> {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayVec::<T, N>::new();
        let buffer = buf.get_continuous(u32::MAX_VAR_INT_SPACE);
        let (len, read_len) = u32::decode_var(|i| Ok(*unsafe { buffer.get_unchecked(i) })).unwrap();
        buf.advance(read_len);
        let len: u32 = len;
        let len = len as usize;
        unsafe { vec.set_len(len) };
        for i in 0..len {
            *unsafe { vec.get_unchecked_mut(i) } = T::decode(buf)?;
        }
        Ok(vec)
    }
}
