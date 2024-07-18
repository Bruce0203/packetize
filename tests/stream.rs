#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(variant_count)]
#![feature(generic_arg_infer)]

#[cfg(feature = "stream")]
#[cfg(test)]
mod test {
    use fast_collections::{Cursor, String};
    use packetize::{stream::SimplePacketStreamFormat, streaming_packets, Decode, Encode, ServerBoundPacketStream};

    #[streaming_packets(SimplePacketStreamFormat)]
    pub enum PacketStreamState {
        HandShake(#[change_state_to(Login)] HandShakeC2s),
        Login(
            LoginRequestC2s,
            LoignSuccessS2c,
            EncryptionRequestC2s,
            EncryptionResponseS2c,
        ),
    }

    #[derive(Encode, Decode)]
    pub struct HandShakeC2s {
        value: u16,
        value2: String<123>,
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
        let value = HandShakeC2s {
            value: 123,
            value2: String::<123>::from_array(*b"baba"),
        };
        let mut connection_state = PacketStreamState::HandShake;
        let mut cursor: Cursor<u8, 1000> = Cursor::new();
        connection_state
            .encode_server_bound_packet(&value.into(), &mut cursor)
            .unwrap();
        println!("{:?}", &cursor.filled()[cursor.pos()..]);
        connection_state = PacketStreamState::HandShake;
        let decoded: HandShakeC2s = connection_state
            .decode_server_bound_packet(&mut cursor)
            .unwrap()
            .into();
        assert_eq!(decoded.value, 123);
        println!("{:?}", &cursor.filled()[cursor.pos()..]);
    }
}
