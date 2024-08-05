use fast_collections::Cursor;

pub trait Encode {
    fn encode<const N: usize>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()>;
}

pub trait Decode: Sized {
    fn decode<const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()>;
}
