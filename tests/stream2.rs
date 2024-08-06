#![no_std]

#[cfg(feature = "stream")]
mod test {
    use core::panic;

    use fast_collections::{Cursor, Vec};
    use packetize::{
        streaming_packets, ClientBoundPacketStream, Decode, Encode, SimplePacketStreamFormat,
    };

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
        vec: Vec<u16, 20>,
    }

    #[derive(Encode, Decode)]
    pub struct LoginRequestS2c {}

    #[derive(Encode, Decode)]
    pub struct LoginSuccessC2s {}

    #[test]
    fn test_change_state() {
        let cursor = &mut Cursor::<u8, 100>::new();
        let mut state = PacketStreamState::HandShake;
        let mut vec = Vec::uninit();
        vec.push(123).unwrap();
        state
            .encode_client_bound_packet(&HandShakeS2c { vec }.into(), cursor)
            .unwrap();
        assert_eq!(state, PacketStreamState::Login);
        state = PacketStreamState::HandShake;
        let decoded = state.decode_client_bound_packet(cursor).unwrap();
        match decoded {
            ClientBoundPacket::HandShakeS2c(HandShakeS2c { vec }) => {
                assert_eq!(vec.get(0).unwrap(), &123);
                assert_eq!(vec.len(), 1);
            }
            ClientBoundPacket::LoginRequestS2c(_) => panic!(),
        }
    }
}
