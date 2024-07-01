use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Len, U10, U100, U5},
    Cursor, String,
};
use packetize::{Encode, SizedEncode};

#[test]
fn test() {
    let value = MyComponent {
        value: 123,
        value2: String::from_array(*b"bruce"),
    };
    let mut write_cursor = Cursor::<u8, U100>::new();
    value.encode(&mut write_cursor).unwrap();
    println!("{:?}", write_cursor.filled());
}

pub struct MyComponent {
    value: u32,
    value2: String<U5>,
}

impl SizedEncode for MyComponent {}

impl<N> Encode<N> for MyComponent
where
    N: ArrayLength + Len,
{
    fn encode(&self, write_cursor: &mut fast_collections::Cursor<u8, N>) -> Result<(), ()> {
        //FIXME use unchecked_add rather than add_assign
        //if core::mem::size_of::<MyComponent>() + write_cursor.pos() < N::USIZE {

        self.value.encode(write_cursor)?;
        self.value2.encode(write_cursor)?;
        Ok(())
    }

    unsafe fn encode_unchecked(
        &self,
        write_cursor: &mut fast_collections::Cursor<u8, N>,
    ) -> Result<(), ()> {
        self.value.encode_unchecked(write_cursor)?;
        self.value2.encode_unchecked(write_cursor)?;
        Ok(())
    }
}
