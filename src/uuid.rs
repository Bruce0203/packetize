use fastbuf::{ReadBuf, WriteBuf};
use uuid::Uuid;

use crate::{Decode, Encode};

impl Encode for Uuid {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        buf.write(self.as_bytes())
    }
}

impl Decode for Uuid {
    fn decode(buf: &mut impl ReadBuf) -> Result<Self, ()> {
        Uuid::from_slice(buf.read(16)).map_err(|_| ())
    }
}
