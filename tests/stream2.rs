#![feature(generic_arg_infer)]

#[cfg(feature = "stream")]
mod test {
    use fast_collections::Cursor;
    use packetize::{streaming_packets, ClientBoundPacketStream, Decode, Encode, SimplePacketStreamFormat};

    #[streaming_packets(SimplePacketStreamFormat)]
    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub enum PacketStreamState {
        #[default]
        HandShake(#[change_state_to(Login)] HandShakeS2c),
        Login(LoginRequestS2c, LoginSuccessC2s),
        //...
    }

    #[derive(Encode, Decode)]
    pub struct HandShakeS2c {
        protocol_version: i32,
    }

    #[derive(Encode, Decode)]
    pub struct LoginRequestS2c {}

    #[derive(Encode, Decode)]
    pub struct LoginSuccessC2s {}

    #[test]
    fn test_change_state() {
        let cursor = &mut Cursor::<u8, 100>::new();
        let mut state = PacketStreamState::HandShake;
        state
            .encode_client_bound_packet(
                &HandShakeS2c {
                    protocol_version: 123,
                }
                .into(),
                cursor,
            )
            .unwrap();
        assert_eq!(state, PacketStreamState::Login);
    }
}
