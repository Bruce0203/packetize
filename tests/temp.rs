#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::str::FromStr;

use arrayvec::{ArrayString, ArrayVec};
use fastbuf::{Buffer, ReadBuf};
use packetize::{Decode, Encode};

#[test]
fn test() {
    let value = MyComponent {
        value: 14,
        value3: ArrayString::from_str("123").unwrap(),
        value4: 123,
    };
    let mut cursor = Buffer::<100>::new();
    value.value3.encode(&mut cursor).unwrap();
    let decoded: ArrayString<100> = Decode::decode(&mut cursor).unwrap();
    assert_eq!(value.value3.len(), decoded.len());

    {
        #[repr(u8)]
        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Encode, Decode)]
        enum TestEnum {
            VALUE1,
            VALUE2,
        }

        let mut cursor: Buffer<100> = Buffer::new();
        TestEnum::VALUE1.encode(&mut cursor).unwrap();
        assert_eq!(cursor.read(1)[0], TestEnum::VALUE1 as u8);
        assert_eq!(cursor.remaining(), 0);
        TestEnum::VALUE2.encode(&mut cursor).unwrap();
        assert_eq!(TestEnum::decode(&mut cursor).unwrap(), TestEnum::VALUE2);
    }

    {
        println!("asdf");
        let mut cursor: Buffer<1000> = Buffer::new();
        MyComponent {
            value: 123,
            value3: ArrayString::from_str("ABCA").unwrap(),
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
    value3: ArrayString<4>,
    value4: u16,
}

#[derive(Encode, Decode)]
pub struct Identifier(ArrayString<5>);

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
    let mut cursor: Buffer<100> = Buffer::new();
    let value = TestStruct {
        value: 123,
        value2: Uuid::from_u128(123),
        value3: 123123,
    };
    value.encode(&mut cursor).unwrap();
    //println!("{:?}", cursor.filled());
    let test_struct = TestStruct::decode(&mut cursor).unwrap();
    assert_eq!(test_struct, value);
    black_box(cursor);
}

#[test]
fn asdf() {
    #[derive(Encode, Decode)]
    struct A {
        value: ArrayString<4>,
    }
    let a = A {
        value: ArrayString::<4>::from_str("1233").unwrap(),
    };
    let mut cursor: Buffer<100> = Buffer::new();
    a.encode(&mut cursor).unwrap()
}

#[test]
fn asdf2() {
    #[derive(Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[allow(dead_code)]
    enum A {
        V1,
        V2,
        V3,
    }
    let value = A::V2;
    let mut cursor: Buffer<100> = Buffer::new();
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
    let mut cursor: Buffer<100> = Buffer::new();
    A::B.encode(&mut cursor).unwrap();
}

#[test]
fn test222() {
    let vec: ArrayVec<u16, 20> = ArrayVec::new();
    let mut cursor: Buffer<100> = Buffer::new();
    vec.encode(&mut cursor).unwrap();
    //println!("test222: {:?}", &cursor.filled()[cursor.pos()..]);
    let decoded = ArrayVec::<u16, 20>::decode(&mut cursor).unwrap();
    assert_eq!(decoded.len(), 0);
    assert_eq!(cursor.remaining(), 0);
}
