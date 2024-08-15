# packetize
- Supports fast packet encoding and decoding for network packets.
- Features packet framing with macros.
- Designed for multiple packet formats (e.g., WebSocket, Minecraft, TCP)


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
