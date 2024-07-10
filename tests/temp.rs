#![feature(generic_const_exprs)]

use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Len, U10, U100, U5},
    Cursor, String,
};
use packetize::{Decode, Encode};

#[test]
fn test() {
    let mut value = MyComponent {
        value: 14,
        value2: unsafe {
            fast_collections::const_transmute_unchecked(String::<U10>::from_array(*b"ABCDE     "))
        },
    };
    *unsafe { value.value2.as_vec_mut().len_mut() } = 5;
    let mut cursor = Cursor::<u8, U100>::new();
    value.value2.encode(&mut cursor).unwrap();
    println!("{:?}", cursor.filled());
    let decoded: String<U100> = Decode::decode(&mut cursor).unwrap();
    assert_eq!(value.value2.len(), decoded.len());
}

pub struct MyComponent {
    value: u8,
    value2: String<U100>,
}

impl<N> Encode<N> for MyComponent
where
    N: ArrayLength,
    [(); N::USIZE]:,
{
    fn encode(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
        //FIXME use unchecked_add rather than add_assign
        //if core::mem::size_of::<MyComponent>() + write_cursor.pos() < N::USIZE {
        self.value.encode(write_cursor)?;
        self.value2.encode(write_cursor)?;
        Ok(())
    }
}
