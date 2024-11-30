use packetize::packet_stream;

#[packet_stream]
pub enum ConnState {
    Test(TestPacketS2c, TestPacketC2s),
}

pub struct TestPacketC2s;

pub struct TestPacketS2c;

