use fastbuf::{ReadBuf, WriteBuf};

pub trait Encode {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()>;
}

pub trait Decode: Sized {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()>;
}
