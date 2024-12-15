# packetize
- Supports fast packet encoding and decoding for network packets.
- Features packet framing with macros.
- Designed for multiple packet formats (e.g., WebSocket, Minecraft, TCP)

```rust

#[packet_stream]
#[derive(Default)]
pub enum ConnectionState {
    #[default]
    HandShake(#[change_state_to(Idle)] HandShakeC2s),
    Idle(
        RoomListRequestC2s,
        RoomListResponseS2c<'_>,
        RoomJoinRequestC2s,
        RoomCreateRequestC2s,
        RoomJoinResponseS2c<'_>,
        RoomCreateResponseS2c<'_>,
        #[change_state_to(Conf)] RoomJoinedAckC2s,
    ),
    Conf(
        #[change_state_to(Disconnected)] DisconnectedConfS2c,
        ChatConfS2c,
        ChatConfC2s,
        GameStartS2c,
        #[change_state_to(Play)] GameStartAckC2s,
        LeaveRoomConfC2s,
        LeaveRoomConfS2c,
        #[change_state_to(Idle)] LeaveRoomConfAckS2c,
        #[change_state_to(Idle)] LeaveRoomConfAckC2s,
    ),
    Play(
        #[change_state_to(Disconnected)] DisconnectedPlayS2c,
        ChatPlayC2s,
        ChatPlayS2c,
        LeaveRoomPlayC2s,
        LeaveRoomPlayS2c,
        #[change_state_to(Idle)] LeaveRoomPlayAckS2c,
        #[change_state_to(Idle)] LeaveRoomPlayAckC2s,
    ),
    Disconnected,
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

