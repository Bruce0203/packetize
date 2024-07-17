#![feature(generic_arg_infer)]

#[cfg(feature = "stream")]
mod test {
    use fast_collections::Cursor;
    use packetize::{streaming_packets, Decode, Encode, SimplePacketStreamFormat};

    #[streaming_packets(SimplePacketStreamFormat)]
    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub enum PacketStreamState {
        #[default]
        HandShake(#[change_state_to(Login)] HandShakeS2c),
        Login(LoginRequestS2c),
    }

    #[derive(Encode, Decode)]
    pub struct HandShakeS2c {}

    #[derive(Encode, Decode)]
    pub struct LoginRequestS2c {}

    #[test]
    fn asdf() {
        let mut cursor: Cursor<u8, 100> = Cursor::new();
        let mut state = PacketStreamState::HandShake;
        state
            .encode_client_bound_packet(&HandShakeS2c {}.into(), &mut cursor)
            .unwrap();
        assert_eq!(state, PacketStreamState::Login);
        state = PacketStreamState::HandShake;
        state
            .encode_client_bound_packet(&LoginRequestS2c {}.into(), &mut cursor)
            .unwrap();
    }
}
