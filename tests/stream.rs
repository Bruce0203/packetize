#![feature(negative_impls)]

use packetize::packet_stream;
use serde::{Deserialize, Serialize};

#[test]
fn test_stream3() {
    let conn_state = ConnState::HandShake;
    let packet = HandShakeC2s;
    let packet: HandShakeC2sPackets = packet.into();
}

impl ServerBoundPacket {}

#[packet_stream]
pub enum ConnState {
    HandShake(HandShakeC2s),
    Login(
        #[id(0x00)] LoginStartC2s,
        #[id(0x01)] LoginSuccessS2c,
        #[id(0x02)] EncryptionRequestC2s,
        #[id(0x03)] EncryptionResponseS2c,
    ),
}

#[derive(Serialize, Deserialize)]
pub struct HandShakeC2s;

#[derive(Serialize, Deserialize)]
pub struct LoginStartC2s;

#[derive(Serialize, Deserialize)]
pub struct LoginSuccessS2c;

#[derive(Serialize, Deserialize)]
pub struct EncryptionRequestC2s;

#[derive(Serialize, Deserialize)]
pub struct EncryptionResponseS2c;
