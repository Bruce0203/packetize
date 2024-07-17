#![feature(generic_arg_infer)]
#![feature(panic_internals)]
#[cfg(feature = "stream")]
mod test {
    use fast_collections::Cursor;
    use packetize::{streaming_packets, Decode, Encode, SimplePacketStreamFormat};
    #[allow(dead_code)]
    pub enum PacketStreamState {
        HandShake,
        Login,
    }
    pub enum HandShakeS2cPackets {
        HandShakeS2c,
    }
    #[automatically_derived]
    impl ::core::default::Default for HandShakeS2cPackets {
        #[inline]
        fn default() -> HandShakeS2cPackets {
            Self::HandShakeS2c
        }
    }
    impl From<HandShakeS2c> for ClientBoundPacket {
        fn from(value: HandShakeS2c) -> Self {
            ClientBoundPacket::HandShakeS2c(value)
        }
    }
    impl From<ClientBoundPacket> for HandShakeS2c {
        fn from(value: ClientBoundPacket) -> Self {
            match value {
                ClientBoundPacket::HandShakeS2c(value) => value,
                _ => {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
    }
    impl packetize::Packet<PacketStreamState> for HandShakeS2c {
        fn id(state: &PacketStreamState) -> Option<u32> {
            match state {
                PacketStreamState::HandShake => {
                    if std::mem::size_of::<HandShakeS2cPackets>() == 0 {
                        None
                    } else {
                        Some(HandShakeS2cPackets::HandShakeS2c as u32)
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not implemented: {0}",
                        format_args!(
                            "There is no id for \'{0}\' packet in {1}",
                            std::any::type_name::<Self>(),
                            "HandShake",
                        ),
                    ));
                }
            }
        }
        fn is_changing_state() -> Option<PacketStreamState> {
            None
        }
    }
    pub enum LoginS2cPackets {
        LoginRequestS2c,
    }
    #[automatically_derived]
    impl ::core::default::Default for LoginS2cPackets {
        #[inline]
        fn default() -> LoginS2cPackets {
            Self::LoginRequestS2c
        }
    }
    impl From<LoginRequestS2c> for ClientBoundPacket {
        fn from(value: LoginRequestS2c) -> Self {
            ClientBoundPacket::LoginRequestS2c(value)
        }
    }
    impl From<ClientBoundPacket> for LoginRequestS2c {
        fn from(value: ClientBoundPacket) -> Self {
            match value {
                ClientBoundPacket::LoginRequestS2c(value) => value,
                _ => {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
    }
    impl packetize::Packet<PacketStreamState> for LoginRequestS2c {
        fn id(state: &PacketStreamState) -> Option<u32> {
            match state {
                PacketStreamState::HandShake => {
                    if std::mem::size_of::<LoginS2cPackets>() == 0 {
                        None
                    } else {
                        Some(LoginS2cPackets::LoginRequestS2c as u32)
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not implemented: {0}",
                        format_args!(
                            "There is no id for \'{0}\' packet in {1}",
                            std::any::type_name::<Self>(),
                            "HandShake",
                        ),
                    ));
                }
            }
        }
        fn is_changing_state() -> Option<PacketStreamState> {
            None
        }
    }
    pub enum ClientBoundPacket {
        HandShakeS2c(HandShakeS2c),
        LoginRequestS2c(LoginRequestS2c),
    }
    impl PacketStreamState {
        pub fn decode_client_bound_packet<const N: usize>(
            &mut self,
            read_cursor: &mut fast_collections::Cursor<u8, N>,
        ) -> Result<ClientBoundPacket, ()> {
            #[allow(unreachable_code)]
            Ok(
                match self {
                    PacketStreamState::HandShake => {
                        match <SimplePacketStreamFormat as packetize::PacketStreamFormat>::read_packet_id::<
                            HandShakeS2cPackets,
                            _,
                        >(read_cursor)? {
                            HandShakeS2cPackets::HandShakeS2c => {
                                <HandShakeS2c as packetize::Decode>::decode(read_cursor)?
                                    .into()
                            }
                        }
                    }
                    PacketStreamState::Login => {
                        match <SimplePacketStreamFormat as packetize::PacketStreamFormat>::read_packet_id::<
                            LoginS2cPackets,
                            _,
                        >(read_cursor)? {
                            LoginS2cPackets::LoginRequestS2c => {
                                <LoginRequestS2c as packetize::Decode>::decode(read_cursor)?
                                    .into()
                            }
                        }
                    }
                },
            )
        }
        pub fn encode_client_bound_packet<const N: usize>(
            &mut self,
            packet: &ClientBoundPacket,
            write_cursor: &mut fast_collections::Cursor<u8, N>,
        ) -> Result<(), ()> {
            #[allow(unreachable_code)]
            match packet {
                ClientBoundPacket::HandShakeS2c(p) => {
                    <SimplePacketStreamFormat as packetize::PacketStreamFormat>::write_packet_with_id::<
                        Self,
                        _,
                        _,
                    >(self, p, write_cursor)?
                }
                ClientBoundPacket::LoginRequestS2c(p) => {
                    <SimplePacketStreamFormat as packetize::PacketStreamFormat>::write_packet_with_id::<
                        Self,
                        _,
                        _,
                    >(self, p, write_cursor)?
                }
            }
            Ok(())
        }
    }
    pub enum ServerBoundPacket {}
    impl PacketStreamState {
        pub fn decode_server_bound_packet<const N: usize>(
            &mut self,
            read_cursor: &mut fast_collections::Cursor<u8, N>,
        ) -> Result<ServerBoundPacket, ()> {
            #[allow(unreachable_code)]
            Ok(match self {
                PacketStreamState::HandShake => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not implemented: {0}",
                        format_args!("there is no {0} packets for {1}", "C2s", "HandShake",),
                    ));
                }
                PacketStreamState::Login => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not implemented: {0}",
                        format_args!("there is no {0} packets for {1}", "C2s", "Login",),
                    ));
                }
            })
        }
        pub fn encode_server_bound_packet<const N: usize>(
            &mut self,
            packet: &ClientBoundPacket,
            write_cursor: &mut fast_collections::Cursor<u8, N>,
        ) -> Result<(), ()> {
            #[allow(unreachable_code)]
            Ok(())
        }
    }
    pub struct HandShakeS2c {}
    impl packetize::Encode for HandShakeS2c {
        fn encode<const N: usize>(
            &self,
            write_cursor: &mut fast_collections::Cursor<u8, N>,
        ) -> core::result::Result<(), ()> {
            Ok(())
        }
    }
    impl packetize::Decode for HandShakeS2c {
        fn decode<const N: usize>(
            read_cursor: &mut fast_collections::cursor::Cursor<u8, N>,
        ) -> Result<Self, ()> {
            Ok(Self {})
        }
    }
    pub struct LoginRequestS2c {}
    impl packetize::Encode for LoginRequestS2c {
        fn encode<const N: usize>(
            &self,
            write_cursor: &mut fast_collections::Cursor<u8, N>,
        ) -> core::result::Result<(), ()> {
            Ok(())
        }
    }
    impl packetize::Decode for LoginRequestS2c {
        fn decode<const N: usize>(
            read_cursor: &mut fast_collections::cursor::Cursor<u8, N>,
        ) -> Result<Self, ()> {
            Ok(Self {})
        }
    }
}
