///! TODO remove code dups..
use arrayvec::{ArrayString, ArrayVec};
use fastbuf::{ReadBuf, WriteBuf};
use fastvarint::VarInt;

use crate::{Decode, Encode};

impl<const CAP: usize> Encode for ArrayVec<u8, CAP> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        let len = self.len() as u32;
        len.encode_var(buf)?;
        for ele in self.iter() {
            ele.encode(buf)?;
        }
        Ok(())
    }
}

impl<const CAP: usize> Decode for ArrayVec<u8, CAP> {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayVec::<u8, CAP>::new();
        let vec_len = u32::decode_var(buf)? as usize;
        if buf.remaining() < vec_len {
            Err(())?
        }
        if CAP < vec_len {
            #[cfg(debug_assertions)]
            dbg!(CAP < vec_len, CAP, vec_len);
            Err(())?
        }
        unsafe { vec.set_len(vec_len) };
        vec.as_mut_slice().copy_from_slice(buf.read(vec_len));
        Ok(vec)
    }
}

impl<const CAP: usize> Encode for ArrayString<CAP> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        (self.len() as u32).encode_var(buf)?;
        buf.write(self.as_bytes());
        Ok(())
    }
}

impl<const CAP: usize> Decode for ArrayString<CAP> {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut string = ArrayString::<CAP>::new();
        let string_len = u32::decode_var(buf)? as usize;
        if buf.remaining() < string_len {
            Err(())?
        }
        if CAP < string_len {
            #[cfg(debug_assertions)]
            dbg!(CAP < string_len, CAP, string_len);
            Err(())?
        }
        unsafe { string.set_len(string_len) };
        unsafe { string.as_bytes_mut().copy_from_slice(buf.read(string_len)) };
        Ok(string)
    }
}

impl<T: Encode, const CAP: usize> Encode for ArrayVec<T, CAP> {
    default fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        let vec_len = self.len();
        (vec_len as u32).encode_var(buf)?;
        if CAP < vec_len {
            #[cfg(debug_assertions)]
            dbg!(CAP < vec_len, CAP, vec_len);
            Err(())?
        }
        for ele in self.iter() {
            ele.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Decode, const CAP: usize> Decode for ArrayVec<T, CAP> {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut vec = ArrayVec::<T, CAP>::new();
        let vec_len = u32::decode_var(buf)? as usize;
        if CAP < vec_len {
            #[cfg(debug_assertions)]
            dbg!(CAP < vec_len, CAP, vec_len);
            Err(())?
        }
        unsafe { vec.set_len(vec_len) };
        for i in 0..vec_len {
            *unsafe { vec.get_unchecked_mut(i) } = T::decode(buf)?;
        }
        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use arrayvec::ArrayString;
    use fastbuf::{Buffer, WriteBuf};

    use crate::Decode;

    #[test]
    fn test_arraystring_decode() {
        let mut buf = Buffer::<100>::new();
        buf.write(&[3, 65, 65, 65]);
        let decoded = ArrayString::<255>::decode(&mut buf).unwrap();
        assert_eq!(decoded, ArrayString::<255>::from_str("AAA").unwrap());
    }
}
