use std::mem::{transmute, MaybeUninit};

use fastbuf::{ReadBuf, WriteBuf};

use crate::{Decode, Encode};

macro_rules! impl_encoder_and_decoder {
    ($($name:ident),*) => {
        $(
            impl Encode for $name {
                #[inline(always)]
                fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
                    buf.try_write(&self.to_be_bytes())?;
                    Ok(())
                }
            }
            impl Decode for $name {
                #[inline(always)]
                fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
                    let slice = buf.read(size_of::<Self>());
                    #[allow(invalid_value)]
                    let mut result = [unsafe { MaybeUninit::uninit().assume_init() }; size_of::<Self>()];
                    result.copy_from_slice(slice);
                    Ok(Self::from_be_bytes(result))
                }
            }
        )*
    };
}

impl Encode for bool {
    #[inline(always)]
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        buf.try_write(&[*self as u8])?;
        Ok(())
    }
}

impl Decode for bool {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        buf.read(size_of::<Self>())
            .get(0)
            .map(|v| match v {
                0 => false,
                _ => true,
            })
            .ok_or(())
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

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        buf.try_write(&[self.is_some() as u8]).map_err(|_| ())?;
        if let Some(value) = self {
            value.encode(buf)?;
        }
        Ok(())
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let header = buf.read(1);
        if header.is_empty() {
            return Err(());
        }
        Ok(if header[0] == 0 {
            None
        } else {
            Some(T::decode(buf)?)
        })
    }
}

impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let header = buf.read(1);
        if header.is_empty() {
            return Err(());
        }
        Ok(if header[0] == 0 {
            Err(E::decode(buf)?)
        } else {
            Ok(T::decode(buf)?)
        })
    }
}

impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        buf.try_write(&[self.is_ok() as u8])?;
        match self {
            Ok(value) => value.encode(buf)?,
            Err(value) => value.encode(buf)?,
        }
        Ok(())
    }
}

impl<T: Encode, T2: Encode> Encode for (T, T2) {
    fn encode(&self, write_cursor: &mut impl WriteBuf) -> Result<(), ()> {
        self.0.encode(write_cursor)?;
        self.1.encode(write_cursor)?;
        Ok(())
    }
}

impl<T: Encode, T2: Encode, T3: Encode> Encode for (T, T2, T3) {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        self.0.encode(buf)?;
        self.1.encode(buf)?;
        self.2.encode(buf)?;
        Ok(())
    }
}

impl<T: Encode, T2: Encode, T3: Encode, T4: Encode> Encode for (T, T2, T3, T4) {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        self.0.encode(buf)?;
        self.1.encode(buf)?;
        self.2.encode(buf)?;
        self.3.encode(buf)?;
        Ok(())
    }
}

impl<T: Decode, T2: Decode> Decode for (T, T2) {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        Ok((T::decode(buf)?, T2::decode(buf)?))
    }
}

impl<T: Decode, T2: Decode, T3: Decode> Decode for (T, T2, T3) {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        Ok((T::decode(buf)?, T2::decode(buf)?, T3::decode(buf)?))
    }
}

impl<T: Decode, T2: Decode, T3: Decode, T4: Decode> Decode for (T, T2, T3, T4) {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        Ok((
            T::decode(buf)?,
            T2::decode(buf)?,
            T3::decode(buf)?,
            T4::decode(buf)?,
        ))
    }
}

impl Encode for () {
    fn encode(&self, _buf: &mut impl WriteBuf) -> Result<(), ()> {
        Ok(())
    }
}

impl Decode for () {
    fn decode(_buf: &mut impl ReadBuf) -> Result<Self, ()> {
        Ok(())
    }
}

impl<T: Encode> Encode for Box<T> {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        T::encode(self, buf)
    }
}

impl<T: Decode> Decode for Box<T> {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        Ok(Box::new(T::decode(buf)?))
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        buf.try_write(self)
    }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let data = buf.read(N);
        #[allow(invalid_value)]
        let mut slice = [unsafe { MaybeUninit::<u8>::uninit().assume_init() }; N];
        slice.copy_from_slice(data);
        Ok(slice)
    }
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    default fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let mut slice: [T; N] = unsafe { transmute(MaybeUninit::<[T; N]>::uninit().assume_init()) };
        for i in 0..N {
            slice[i] = T::decode(buf)?;
        }
        Ok(slice)
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    default fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        for data in self.iter() {
            data.encode(buf)?;
        }
        Ok(())
    }
}
