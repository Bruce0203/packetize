use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Len, U10, U100, U5},
    Cursor, String,
};
use packetize::{Encode, SizedEncode};

#[test]
fn test() {
    let mut value = MyComponent {
        value: 14,
        value2: String::from_array(*b"ABCDE     "),
    };
    *unsafe { value.value2.as_vec_mut().len_mut() } = 5;
    let mut write_cursor = Cursor::<u8, U100>::new();
    unsafe { value.encode_unchecked(&mut write_cursor) };
    println!("{:?}", write_cursor.filled());
}

pub struct MyComponent {
    value: u8,
    value2: String<U10>,
}

impl SizedEncode for MyComponent {}

impl<N> Encode<N> for MyComponent
where
    N: ArrayLength + Len,
{
    fn encode(self, write_cursor: &mut fast_collections::Cursor<u8, N>) -> Result<(), ()> {
        //FIXME use unchecked_add rather than add_assign
        //if core::mem::size_of::<MyComponent>() + write_cursor.pos() < N::USIZE {
        self.value.encode(write_cursor)?;
        self.value2.encode(write_cursor)?;
        Ok(())
    }

    unsafe fn encode_unchecked(self, write_cursor: &mut fast_collections::Cursor<u8, N>) {
        self.value.encode_unchecked(write_cursor);
        self.value2.encode_unchecked(write_cursor);
    }
}
