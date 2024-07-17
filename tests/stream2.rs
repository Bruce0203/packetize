#![feature(generic_arg_infer)]

#[cfg(feature = "stream")]
mod test {
    use packetize::{streaming_packets, Packetize, SimplePacketStreamFormat};

    #[streaming_packets(SimplePacketStreamFormat)]
    pub enum PacketStreamState {
        HandShake(HandShakeS2c),
        Login(LoginRequestC2s),
    }

    #[derive(Packetize)]
    pub struct HandShakeS2c {}

    #[derive(Packetize)]
    pub struct LoginRequestC2s {}
}
