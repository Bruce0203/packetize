#![feature(generic_const_exprs)]

use std::fmt::write;

use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Len, U10, U100, U1000, U5},
    Cursor, CursorReadTransmute, PushTransmute, PushTransmuteUnchecked, String,
};
use packetize::{Decode, Encode};
use packetize_derive::Packetize;

#[test]
fn test() {
    let mut value = MyComponent {
        value: 14,
        value2: unsafe {
            fast_collections::const_transmute_unchecked(String::<U10>::from_array(*b"ABCDE     "))
        },
        value3: String::from_array(*b"123"),
        value4: 123,
    };
    *unsafe { value.value2.as_vec_mut().len_mut() } = 5;
    let mut cursor = Cursor::<u8, U100>::new();
    value.value2.encode(&mut cursor).unwrap();
    println!("{:?}", cursor.filled());
    let decoded: String<U100> = Decode::decode(&mut cursor).unwrap();
    assert_eq!(value.value2.len(), decoded.len());

    {
        #[repr(u8)]
        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
        enum TestEnum {
            VALUE1,
            VALUE2,
        }

        impl<N: ArrayLength> Encode<N> for TestEnum {
            fn encode(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
                PushTransmute::push_transmute(write_cursor, Clone::clone(self))
            }
        }

        impl<N: ArrayLength> Decode<N> for TestEnum {
            fn decode(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()> {
                CursorReadTransmute::read_transmute(read_cursor)
                    .map(|v| *v)
                    .ok_or_else(|| ())
            }
        }
        let mut cursor: Cursor<u8, U100> = Cursor::new();
        TestEnum::VALUE1.encode(&mut cursor).unwrap();
        assert_eq!(
            cursor.read_transmute::<TestEnum>().unwrap(),
            &TestEnum::VALUE1
        );
        assert_eq!(cursor.filled_len(), cursor.pos());
        TestEnum::VALUE2.encode(&mut cursor).unwrap();
        assert_eq!(TestEnum::decode(&mut cursor).unwrap(), TestEnum::VALUE2);
    }

    {
        println!("asdf");
        println!("{}", String::<U100>::from_array(*b"123").len());
        let mut cursor: Cursor<u8, U1000> = Cursor::new();
        MyComponent {
            value: 123,
            value2: String::from_array(*b"ABCD"),
            value3: String::from_array(*b"A"),
            value4: 42,
        }
        .encode(&mut cursor)
        .unwrap();
        println!("{:?}", cursor.filled());
        let component = MyComponent::decode(&mut cursor).unwrap();
        assert_eq!(component.value, 123);
        assert_eq!(component.value4, 42);
    }
}

#[derive(Packetize)]
pub struct MyComponent {
    value: u8,
    value2: String<U100>,
    value3: String<U100>,
    value4: u8,
}

struct A;

fn a() {
    let a = A {};
}
