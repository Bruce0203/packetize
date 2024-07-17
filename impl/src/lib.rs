use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Ident, Index, Item, ItemEnum, ItemStruct, Meta, Path, Type};

#[proc_macro_derive(Packetize)]
pub fn packetize_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item {
        Item::Enum(value) => {
            let item_name = &value.ident;
            quote! {
                impl packetize::Encode for #item_name {
                    fn encode<const N: usize>
                        (&self, write_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> core::result::Result<(), ()> 
                    {
                        fast_collections::PushTransmute::push_transmute(write_cursor, Clone::clone(self))
                    }
                }

                impl packetize::Decode for #item_name {
                    fn decode<const N: usize>
                        (read_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> core::result::Result<Self, ()> 
                    {
                        fast_collections::CursorReadTransmute::read_transmute(read_cursor)
                            .map(|v| *v)
                            .ok_or_else(|| ())
                    }
                }
            }
        }
        Item::Struct(item_struct) => {
            let item_name = &item_struct.ident;
            let has_field_name = item_struct.fields.iter().last().map(|field| field.ident.is_some());
            let decode_constructor = generate_decoder(&item_struct, has_field_name);
            let encode_constructor = generate_encoder(&item_struct, has_field_name);
            quote! {
               impl packetize::Decode for #item_name
               {
                   fn decode<const N: usize>
                       (read_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> Result<Self, ()>
                   {
                       Ok(#decode_constructor)
                   }
               }

               impl packetize::Encode for #item_name
               {
                   fn encode<const N: usize>
                       (&self, write_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> Result<(), ()> 
                   {
                       #encode_constructor
                       Ok(())
                   }
               }
            }
        },
        _ => panic!("unimplemented item type"),
    }
    .into()
}

fn generate_decoder(
    item_struct: &ItemStruct,
    has_field_name: Option<bool>,
) -> proc_macro2::TokenStream {
    let decode = quote!(packetize::Decode::decode(read_cursor)?);
    if let Some(has_field_name) = has_field_name {
        if has_field_name {
            let fields: Vec<_> = item_struct
                .fields
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .collect();
            quote! {
                Self {
                    #(#fields: #decode,)*
                }
            }
        } else {
            let fields: Vec<_> = (0..item_struct.fields.len())
                .map(|_| decode.clone())
                .collect();
            quote! {
                Self(
                    #(#fields,)*
                )
            }
        }
    } else {
        quote! {Self {}}
    }
}

fn generate_encoder(
    item_struct: &ItemStruct,
    has_field_name: Option<bool>,
) -> proc_macro2::TokenStream {
    if let Some(has_field_name) = has_field_name {
        if has_field_name {
            let fields: Vec<_> = item_struct
                .fields
                .iter()
                .map(|field| field.ident.clone().unwrap())
                .collect();
            quote! {
                #(packetize::Encode::encode(&self.#fields, write_cursor)?;)*
            }
        } else {
            let fields = (0..item_struct.fields.len()).map(|i| Index::from(i));
            quote! {
                #(packetize::Encode::encode(&self.#fields, write_cursor)?;)*
            }
        }
    } else {
        quote! {}
    }
}

#[allow(non_upper_case_globals)]
const C2s: &'static str = "C2s";
#[allow(non_upper_case_globals)]
const S2c: &'static str = "S2c";

#[cfg(feature = "stream")]
#[proc_macro_attribute]
pub fn streaming_packets(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_enum = parse_macro_input!(input as ItemEnum);
    let state_name = &item_enum.ident;
    let state_vis = &item_enum.vis;
    let mut all_packets: Vec<Path> = Vec::new();
    let mut packets_by_states: Vec<Vec<Path>> = Vec::new();
    let states: Vec<Ident> = item_enum.variants.iter().map(|enum_variant| enum_variant.ident.clone()).collect();
    let quotes_of_states: Vec<proc_macro2::TokenStream> = item_enum.variants.iter_mut().map(|enum_variant| {
        let state = &enum_variant.ident;
        let packets: Vec<Path> = enum_variant.fields.iter().map(|field| match &field.ty {
            Type::Path(path) => path.path.clone(),
            _ => unimplemented!("type must path")
        }.clone()).collect();
        packets_by_states.push(packets.clone());
        for packet in packets.iter() {
            all_packets.push(packet.clone());
        }
        let c2s_packets = path_filter_end_with(&packets, C2s);
        let s2c_packets = path_filter_end_with(&packets, S2c);
        let c2s_packets_name = format_ident!("{state}{C2s}Packets");
        let s2c_packets_name = format_ident!("{state}{S2c}Packets");
        let changing_state_stmt: Vec<_> = enum_variant.fields.iter_mut().map(|field| {
            if let Some(attr) = find_matching_ident_from_attrs_and_remove(&mut field.attrs, "change_state_to") {
                let state = match &attr.meta {
                    Meta::List(list) => &list.tokens,
                    _ => unimplemented!("attribute may have contain next state")
                };
                Some(quote! {
                    Some(#state_name::#state)
                })
            } else {
                Some(quote! {
                    None
                })
            }
        }).collect();
        let c2s_packets_enum = if c2s_packets.is_empty() { None } else { Some(quote! {
            #[derive(Default)]
            enum #c2s_packets_name {
                #[default]
                #(#c2s_packets,)*
            }

        }) };
        let s2c_packets_enum = if s2c_packets.is_empty() { None } else { Some(quote! {
            #[derive(Default)]
            enum #s2c_packets_name {
                #[default]
                #(#s2c_packets,)*
            }
        }) };
        quote! {
            #c2s_packets_enum
            #s2c_packets_enum
            
            #(
            impl From<#c2s_packets> for ServerBoundPacket {
                fn from(value: #c2s_packets) -> Self {
                    ServerBoundPacket::#c2s_packets(value)
                }
            }

            impl From<ServerBoundPacket> for #c2s_packets {
                fn from(value: ServerBoundPacket) -> Self {
                    match value {
                        ServerBoundPacket::#c2s_packets(value) => value,
                        _ => panic!(),
                    }
                }
            }
            )*

            #(
            impl From<#s2c_packets> for ClientBoundPacket {
                fn from(value: #s2c_packets) -> Self {
                    ClientBoundPacket::#s2c_packets(value)
                }
            }

            impl From<ClientBoundPacket> for #s2c_packets {
                fn from(value: ClientBoundPacket) -> Self {
                    match value {
                        ClientBoundPacket::#s2c_packets(value) => value,
                        _ => panic!(),
                    }
                }
            }
            )*


            #(
            impl Packet<#state_name> for #c2s_packets {
                fn id(state: &#state_name) -> Option<u32> {
                    match state {
                        #state_name::#state => if std::mem::size_of::<#c2s_packets_name>() == 0 {
                            None
                        } else {
                            Some(#c2s_packets_name::#c2s_packets as u32)
                        },
                        state => unimplemented!(
                            "There is no id for '{}' packet in {state:?}",
                            std::any::type_name::<Self>(),
                        ),
                    }
                }

                fn is_changing_state() -> Option<#state_name> {
                    #changing_state_stmt
                }
            }
            )*

            #(
            impl Packet<#state_name> for #s2c_packets {
                fn id(state: &#state_name) -> Option<u32> {
                    match state {
                        #state_name::#state => if std::mem::size_of::<#s2c_packets_name>() == 0 {
                            None
                        } else {
                            Some(#s2c_packets_name::#s2c_packets as u32)
                        },
                        state => unimplemented!(
                            "There is no id for '{}' packet in {state:?}",
                            std::any::type_name::<Self>(),
                        ),
                    }
                }

                fn is_changing_state() -> Option<#state_name> {
                    #changing_state_stmt
                }
            }
            )*
        }
    }).collect();
    let client_bound_packets = path_filter_end_with(&all_packets, S2c);
    let server_bound_packets = path_filter_end_with(&all_packets, C2s);
    let format = parse_macro_input!(attr as Ident);
    let f = |suffix: &'static str| -> Vec<_> {
        let mut state_index = 0;
        states.iter().map(|state| {
        let packets_name = format_ident!("{state}{suffix}Packets");
        let packets = path_filter_end_with(&packets_by_states[state_index], suffix);
        state_index += 1;
        if !packets.is_empty() {
            quote! {
                match #format::read_packet_id::<#packets_name, _>(read_cursor)? {
                    #(
                    #packets_name::#packets => {
                        #packets::decode(read_cursor)?.into()
                    },
                    )*
                }
            }
        } else {
            quote! { unimplemented!("there is no {} packets for {}", #suffix, stringify!(#state)) }
        }
    }).collect()
    };
    let c2s_quotes_of_match_case = f(C2s);
    let s2c_quotes_of_match_case = f(S2c);
    quote! {
        #[derive(Debug)]
        #state_vis enum #state_name {
            #(#states,)*
        }

        #(#quotes_of_states)*

        #state_vis enum ServerBoundPacket {
            #(#server_bound_packets(#server_bound_packets),)*
        }

        #state_vis enum ClientBoundPacket {
            #(#client_bound_packets(#client_bound_packets),)*
        }

        impl #state_name {
            fn decode_server_bound_packet<const N: usize>(
                &mut self,
                read_cursor: &mut Cursor<u8, N>,
            ) -> Result<ServerBoundPacket, ()> {
                Ok(match self {
                    #(
                    #state_name::#states => {
                        #c2s_quotes_of_match_case
                    }
                    )*
                })
            }

            fn decode_client_bound_packet<const N: usize>(
                &mut self,
                read_cursor: &mut Cursor<u8, N>,
            ) -> Result<ClientBoundPacket, ()> {
                Ok(match self {
                    #(
                    #state_name::#states => {
                        #s2c_quotes_of_match_case
                    }
                    )*
                })
            }

            fn encode_server_bound_packet<const N: usize>(
                &mut self,
                packet: &ServerBoundPacket,
                write_cursor: &mut Cursor<u8, N>,
            ) -> Result<(), ()> {
                match packet {
                    #(
                        ServerBoundPacket::#server_bound_packets(p) => {
                            #format::write_packet_with_id::<Self, _, _>(self, p, write_cursor)?
                        }
                    )*
                }
                Ok(())
            }

            fn encode_client_bound_packet<const N: usize>(
                &mut self,
                packet: &ClientBoundPacket,
                write_cursor: &mut Cursor<u8, N>,
            ) -> Result<(), ()> {
                match packet {
                    #(
                        ClientBoundPacket::#client_bound_packets(p) => {
                            #format::write_packet_with_id::<Self, _, _>(self, p, write_cursor)?
                        }
                    )*
                }
                Ok(())
            }
        }
    }.into()
}


#[cfg(feature = "stream")]
fn path_filter_end_with<'a>(packets: &'a Vec<Path>, ends_with: &'static str) -> Vec<&'a Path>{
    packets.iter().filter(|packet| {
        packet.get_ident().unwrap().to_string().ends_with(ends_with)
    }).collect::<Vec<_>>()
}

#[cfg(feature = "stream")]
fn find_matching_ident_from_attrs_and_remove (attrs: &mut Vec<Attribute>, ident: &'static str) -> Option<Attribute> {
    let mut index = 0;
    let mut attr_index = usize::MAX;
    let res = attrs.iter().find(|attr| {
        index += 1;
        let list = match &attr.meta {
            Meta::List(list) => list,
            _ => return false,
        };
        if !list.path.is_ident(ident) {
            return false;
        }
        attr_index = index - 1;
        true
    }).map(|v| v.clone());
    if attr_index != usize::MAX {
        attrs.remove(attr_index);
    }
    res
}
