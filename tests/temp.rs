#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::iter::Map;

use fast_collections::{cursor, Cursor, CursorReadTransmute, Push, PushTransmute, String, Vec};
use packetize::{Decode, Encode};

#[test]
fn test() {
    let value = MyComponent {
        value: 14,
        value3: String::from_array(*b"123"),
        value4: 123,
    };
    let mut cursor = Cursor::<u8, 100>::new();
    value.value3.encode(&mut cursor).unwrap();
    println!("{:?}", cursor.filled());
    let decoded: String<100> = Decode::decode(&mut cursor).unwrap();
    assert_eq!(value.value3.len(), decoded.len());

    {
        #[repr(u8)]
        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
        enum TestEnum {
            VALUE1,
            VALUE2,
        }

        impl Encode for TestEnum {
            fn encode<const N: usize>(&self, write_cursor: &mut Cursor<u8, N>) -> Result<(), ()> {
                PushTransmute::push_transmute(write_cursor, Clone::clone(self))
            }
        }

        impl Decode for TestEnum {
            fn decode<const N: usize>(read_cursor: &mut Cursor<u8, N>) -> Result<Self, ()> {
                CursorReadTransmute::read_transmute(read_cursor)
                    .map(|v| *v)
                    .ok_or_else(|| ())
            }
        }
        let mut cursor: Cursor<u8, 100> = Cursor::new();
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
        let mut cursor: Cursor<u8, 1000> = Cursor::new();
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

#[derive(Encode, Decode)]
pub struct MyComponent {
    value: u16,
    value3: String<4>,
    value4: u16,
}

#[derive(Encode, Decode)]
pub struct Identifier(String<5>);

#[cfg(feature = "uuid")]
#[test]
fn test_uuid() {
    use std::hint::black_box;

    use uuid::Uuid;

    #[derive(Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct TestStruct {
        value: usize,
        value2: Uuid,
        value3: usize,
    }
    let mut cursor: Cursor<u8, 100> = Cursor::new();
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
    #[derive(Encode, Decode)]
    struct A {
        value: String<4>,
    }
    let a = A {
        value: String::from_array(*b"123123"),
    };
    let mut cursor: Cursor<u8, 100> = Cursor::new();
    a.encode(&mut cursor).unwrap()
}

#[test]
fn asdf2() {
    #[derive(Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum A {
        V1,
        V2,
        V3,
    }
    let value = A::V2;
    let mut cursor: Cursor<u8, 100> = Cursor::new();
    value.encode(&mut cursor).unwrap();
    let decoded = A::decode(&mut cursor).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn asdf3() {
    #[derive(Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum A {
        B,
    }
    let mut cursor: Cursor<u8, 100> = Cursor::new();
    A::B.encode(&mut cursor).unwrap();
}

#[test]
fn test222() {
    let vec: Vec<u16, 20> = Vec::uninit();
    let mut cursor: Cursor<u8, 100> = Cursor::new();
    vec.encode(&mut cursor).unwrap();
    println!("test222: {:?}", &cursor.filled()[cursor.pos()..]);
    let decoded = Vec::<u16, 20>::decode(&mut cursor).unwrap();
    assert_eq!(decoded.len(), 0);
    assert_eq!(cursor.remaining(), 0);
}
