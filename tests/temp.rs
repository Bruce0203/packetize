#![feature(generic_const_exprs)]

use std::fmt::write;

use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Len, U10, U100, U1000, U4, U5},
    Cursor, CursorReadTransmute, PushTransmute, PushTransmuteUnchecked, String,
};
use packetize::{Decode, Encode};
use packetize_derive::Packetize;

#[test]
fn test() {
    let mut value = MyComponent {
        value: 14,
        value3: String::from_array(*b"123"),
        value4: 123,
    };
    let mut cursor = Cursor::<u8, U100>::new();
    value.value3.encode(&mut cursor).unwrap();
    println!("{:?}", cursor.filled());
    let decoded: String<U100> = Decode::decode(&mut cursor).unwrap();
    assert_eq!(value.value3.len(), decoded.len());

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
        let mut cursor: Cursor<u8, U1000> = Cursor::new();
        MyComponent {
            value: 123,
            value3: String::from_array(*b"ABCA"),
            value4: 42,
        }
        .encode(&mut cursor)
        .unwrap();
        let component = MyComponent::decode(&mut cursor).unwrap();
        assert_eq!(component.value, 123);
        assert_eq!(component.value4, 42);
    }
}

#[derive(Packetize)]
pub struct MyComponent {
    value: u16,
    value3: String<U4>,
    value4: u16,
}

struct A;

fn a() {
    let a = A {};
    let value = Identifier(String::new());
    let value = value.0;
}

#[derive(Packetize)]
pub struct Identifier(String<U5>);

#[cfg(feature = "uuid")]
#[test]
fn test_uuid() {
    use std::hint::black_box;

    use uuid::Uuid;

    #[derive(packetize_derive::Packetize, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct TestStruct {
        value: usize,
        value2: Uuid,
        value3: usize,
    }
    let mut cursor: Cursor<u8, U100> = Cursor::new();
    let value = TestStruct {
        value: 123,
        value2: Uuid::from_u128(123),
        value3: 123123,
    };
    value.encode(&mut cursor).unwrap();
    println!("{:?}", cursor.filled());
    let test_struct = TestStruct::decode(&mut cursor).unwrap();
    assert_eq!(test_struct, value);
    black_box(cursor);
}

#[test]
fn asdf() {
    #[derive(Packetize)]
    struct A {
        value: String<U4>,
    }
    let a = A {
        value: String::from_array(*b"123123"),
    };
    let mut cursor: Cursor<u8, U100> = Cursor::new();
    a.encode(&mut cursor).unwrap()
}
