#![feature(generic_arg_infer)]

#[cfg(feature = "stream")]
mod test {
    use fast_collections::Cursor;
    use packetize::{streaming_packets, Packetize, SimplePacketStreamFormat};

    #[streaming_packets(SimplePacketStreamFormat)]
    pub enum PacketStreamState {
        HandShake(HandShakeS2c),
        Login(LoginRequestS2c),
    }

    #[derive(Packetize)]
    pub struct HandShakeS2c {}

    #[derive(Packetize)]
    pub struct LoginRequestS2c {}

    #[test]
    fn asdf() {
        let mut cursor: Cursor<u8, 100> = Cursor::new();
        let mut state = PacketStreamState::HandShake;
        state
            .encode_client_bound_packet(&HandShakeS2c {}.into(), &mut cursor)
            .unwrap();
    }
}
