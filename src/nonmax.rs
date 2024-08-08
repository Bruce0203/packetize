use std::mem::MaybeUninit;

use fastbuf::{ReadBuf, WriteBuf};
use nonmax::{
    NonMaxI128, NonMaxI16, NonMaxI32, NonMaxI64, NonMaxI8, NonMaxIsize, NonMaxU128, NonMaxU16,
    NonMaxU32, NonMaxU64, NonMaxU8, NonMaxUsize,
};

use crate::{Decode, Encode};

macro_rules! impl_nonmax {
    ($($name:ident = $inner:ident),*) => {
        $(
impl Encode for $name {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        buf.try_write(&self.get().to_be_bytes())?;
        Ok(())
    }
}

impl Decode for $name {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        let slice = buf.read(size_of::<Self>());
        #[allow(invalid_value)]
        let mut result = [unsafe { MaybeUninit::uninit().assume_init() }; size_of::<Self>()];
        result.copy_from_slice(slice);
        Ok(unsafe { Self::new_unchecked($inner::from_be_bytes(result)) })
    }
}

        )*
    };
}

impl_nonmax!(
    NonMaxI8 = i8,
    NonMaxU8 = u8,
    NonMaxI16 = i16,
    NonMaxU16 = u16,
    NonMaxI32 = i32,
    NonMaxU32 = u32,
    NonMaxI64 = i64,
    NonMaxU64 = u64,
    NonMaxU128 = u128,
    NonMaxI128 = i128,
    NonMaxUsize = usize,
    NonMaxIsize = isize
);
