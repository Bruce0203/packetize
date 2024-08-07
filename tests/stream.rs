#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(variant_count)]
#![feature(generic_arg_infer)]

#[cfg(feature = "stream")]
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use arrayvec::ArrayString;
    use fastbuf::Buffer;
    use packetize::{
        stream::SimplePacketStreamFormat, streaming_packets, ClientBoundPacketStream, Decode,
        Encode, ServerBoundPacketStream,
    };

    #[streaming_packets(SimplePacketStreamFormat)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum PacketStreamState {
        HandShake(#[change_state_to(Login)] HandShakeS2c),
        Login(
            LoginRequestC2s,
            LoignSuccessS2c,
            EncryptionRequestC2s,
            EncryptionResponseS2c,
        ),
    }

    #[derive(Encode, Decode)]
    pub struct HandShakeS2c {
        value: u16,
        value2: ArrayString<123>,
    }

    #[derive(Encode, Decode)]
    pub struct LoginRequestC2s {
        value: u32,
    }

    #[derive(Encode, Decode)]
    pub struct EncryptionRequestC2s {
        value: i32,
    }

    #[derive(Encode, Decode)]
    pub struct LoignSuccessS2c {
        value: u32,
    }

    #[derive(Encode, Decode)]
    pub struct EncryptionResponseS2c {
        value: i32,
    }

    #[test]
    fn stream_test() {
        let value = HandShakeS2c {
            value: 123,
            value2: ArrayString::<123>::from_str("baba").unwrap(),
        };
        let mut connection_state = PacketStreamState::HandShake;
        let mut cursor: Buffer<1000> = Buffer::new();
        connection_state
            .encode_client_bound_packet(&value.into(), &mut cursor)
            .unwrap();
        //println!("{:?}", &cursor.filled()[cursor.pos()..]);
        println!("HIa");
        assert_eq!(connection_state, PacketStreamState::Login);
        connection_state
            .encode_server_bound_packet(&LoginRequestC2s { value: 123 }.into(), &mut cursor)
            .unwrap();
        //connection_state = PacketStreamState::HandShake;
        //let decoded: HandShakeS2c = connection_state
        //    .decode_server_bound_packet(&mut cursor)
        //    .unwrap()
        //    .into();
        //assert_eq!(decoded.value, 123);
        //println!("{:?}", &cursor.filled()[cursor.pos()..]);
    }
}
