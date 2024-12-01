use packetize::packet_stream;
use serialization::Serializable;

#[packet_stream]
pub enum ConnState {
    Test(TestPacketS2c, TestPacketC2s),
}

#[derive(Debug, Serializable)]
pub struct TestPacketC2s;

#[derive(Debug, Serializable)]
pub struct TestPacketS2c;
