fn main() {
    use fastbuf::Buffer;
    use packetize::ClientBoundPacketStream;
    use packetize::{streaming_packets, Decode, Encode, SimplePacketStreamFormat};

    #[streaming_packets]
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

    let buffer = &mut Buffer::<100>::new();
    let mut state = PacketStreamState::HandShake;
    state
        .encode_client_bound_packet(
            &HandShakeS2c {
                protocol_version: 123,
            }
            .into(),
            buffer,
            &mut SimplePacketStreamFormat,
        )
        .unwrap();
    assert_eq!(state, PacketStreamState::Login);
}
