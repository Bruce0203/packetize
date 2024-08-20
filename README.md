# packetize
- Supports fast packet encoding and decoding for network packets.
- Features packet framing with macros.
- Designed for multiple packet formats (e.g., WebSocket, Minecraft, TCP)

```rust
#[streaming_packets]
#[derive(Debug, Default)]
pub enum Mc1_21_1ConnectionState {
    #[default]
    HandShake(HandShakeC2s),
    Status(
        StatusRequestC2s,
        StatusResponseS2c,
        PingRequestC2s,
        PingResponseS2c,
    ),
    Login(
        #[id(0x00)] LoginStartC2s,
        #[id(0x00)] LoginDisconnectS2c,
        #[id(0x01)] EncryptionRequestS2c,
        #[id(0x01)] EncryptionResponseC2s,
        #[id(0x02)] LoginSuccessS2c,
        #[id(0x03)] SetCompressionS2c,
        #[change_state_to(Conf)]
        #[id(0x03)]
        LoginAckC2s,
    ),
    Conf(
        #[id(0x00)] ClientInformationC2s,
        #[id(0x02)] PluginMessageConfC2s,
        #[id(0x01)] PluginMessageConfS2c,
        #[id(0x03)] FinishConfigurationS2c,
        #[change_state_to(Play)]
        #[id(0x03)]
        FinishConfigurationAckC2s,
        #[id(0x0C)] FeatureFlagsS2c,
        #[id(0x0E)] KnownPacksS2c,
        #[id(0x07)] KnownPacksC2s,
        #[id(0x07)] RegistryDataS2c,
        #[id(0x0D)] UpdateTagsS2c,
    ),
    Play(
        #[id(0x19)] PluginMessagePlayS2c,
        #[id(0x12)] PluginMessagePlayC2s,
    ),
}

```


```rust
use fastbuf::Buffer;
use packetize::ClientBoundPacketStream;
use packetize::{streaming_packets, Decode, Encode, SimplePacketStreamFormat};

#[streaming_packets]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PacketStreamState {
    #[default]
    HandShake(#[change_state_to(Login)] HandShakeS2c),
    Login(LoginRequestS2c, #[id(1)] LoginSuccessC2s),
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

let buf = &mut Buffer::<100>::new();
let mut state = PacketStreamState::HandShake;
state
    .encode_client_bound_packet(
        &HandShakeS2c {
            protocol_version: 123,
        }
        .into(),
        buf,
        &mut SimplePacketStreamFormat,
    )
    .unwrap();
assert_eq!(state, PacketStreamState::Login);

```

