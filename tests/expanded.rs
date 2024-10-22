#![feature(test)]
#![feature(coverage_attribute)]
#![feature(panic_internals)]
#![feature(prelude_import)]
use std::marker::PhantomData;
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use packetize::packet_stream;
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
pub enum ConnState {
    Test,
}
#[repr(u32)]
pub enum TestS2cPackets<'a> {
    TestPacketS2c(TestPacketS2c<'a>),
}

impl<'a> From<TestS2cPackets<'a>> for ClientBoundPacket<'a> {
    fn from(value: TestS2cPackets<'a>) -> Self {
        match value {
            TestS2cPackets::TestPacketS2c(v) => ClientBoundPacket::TestPacketS2c(v),
        }
    }
}
impl<'a> From<TestPacketS2c<'a>> for TestS2cPackets<'a> {
    fn from(value: TestPacketS2c<'a>) -> Self {
        TestS2cPackets::TestPacketS2c(value)
    }
}
impl<'a> From<TestPacketS2c<'a>> for ClientBoundPacket<'a> {
    fn from(value: TestPacketS2c<'a>) -> Self {
        ClientBoundPacket::TestPacketS2c(value)
    }
}
impl<'a> TryFrom<ClientBoundPacket<'a>> for TestPacketS2c<'a> {
    type Error = ();
    fn try_from(value: ClientBoundPacket<'a>) -> Result<Self, Self::Error> {
        match value {
            ClientBoundPacket::TestPacketS2c(value) => Ok(value),
            _ => Err(())?,
        }
    }
}
impl packetize::Packet<ConnState> for TestPacketS2c<'_> {
    fn get_id(state: &ConnState) -> Option<u32> {
        match state {
            ConnState::Test => Some(TestS2cPackets::TestPacketS2c as u32),
            _ => None,
        }
    }
    fn is_changing_state() -> Option<ConnState> {
        None
    }
}
pub enum ClientBoundPacket<'a> {
    TestPacketS2c(TestPacketS2c<'a>),
}

#[repr(u32)]
pub enum TestC2sPackets {
    TestPacketC2s(TestPacketC2s),
}
impl From<TestC2sPackets> for ServerBoundPacket {
    fn from(value: TestC2sPackets) -> Self {
        match value {
            TestC2sPackets::TestPacketC2s(v) => ServerBoundPacket::TestPacketC2s(v),
        }
    }
}
impl From<TestPacketC2s> for TestC2sPackets {
    fn from(value: TestPacketC2s) -> Self {
        TestC2sPackets::TestPacketC2s(value)
    }
}
impl From<TestPacketC2s> for ServerBoundPacket {
    fn from(value: TestPacketC2s) -> Self {
        ServerBoundPacket::TestPacketC2s(value)
    }
}
impl TryFrom<ServerBoundPacket> for TestPacketC2s {
    type Error = ();
    fn try_from(value: ServerBoundPacket) -> Result<Self, Self::Error> {
        match value {
            ServerBoundPacket::TestPacketC2s(value) => Ok(value),
            _ => Err(())?,
        }
    }
}
impl packetize::Packet<ConnState> for TestPacketC2s {
    fn get_id(state: &ConnState) -> Option<u32> {
        match state {
            ConnState::Test => Some(TestC2sPackets::TestPacketC2s as u32),
            _ => None,
        }
    }
    fn is_changing_state() -> Option<ConnState> {
        None
    }
}
pub enum ServerBoundPacket {
    TestPacketC2s(TestPacketC2s),
}

pub struct TestPacketC2s;
pub struct TestPacketS2c<'a>(PhantomData<&'a ()>);
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
