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



