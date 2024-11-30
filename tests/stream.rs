#![feature(negative_impls)]

use std::marker::PhantomData;

use packetize::packet_stream;
use serialization::Serializable;

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
        #[id(0x04)] TestPacketS2c<'a>,
    ),
}

#[derive(Serializable)]
pub struct HandShakeC2s;

#[derive(Serializable)]
pub struct LoginStartC2s;

#[derive(Serializable)]
pub struct LoginSuccessS2c;

#[derive(Serializable)]
pub struct EncryptionRequestC2s;

#[derive(Serializable)]
pub struct EncryptionResponseS2c;

#[derive(Serializable)]
pub struct TestPacketS2c<'a>(PhantomData<&'a ()>);
