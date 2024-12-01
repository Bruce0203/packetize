use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Ident, ItemEnum, Meta, Type, TypePath, Variant, Visibility,
};

struct Bound {
    suffix: &'static str,
    bound_packet_ident: &'static str,
}

const CLIENT_BOUND: Bound = Bound {
    suffix: "S2c",
    bound_packet_ident: "ClientBoundPacket",
};

const SERVER_BOUND: Bound = Bound {
    suffix: "C2s",
    bound_packet_ident: "ServerBoundPacket",
};

struct PacketStream<'a> {
    ident: &'a Ident,
    attrs: &'a Vec<Attribute>,
    vis: &'a Visibility,
    states: Vec<PacketStreamState<'a>>,
    packets: Vec<Packet<'a>>,
}

struct PacketStreamState<'a> {
    attrs: &'a Vec<Attribute>,
    ident: &'a Ident,
    packets: Vec<Packet<'a>>,
}

#[derive(Clone)]
struct Packet<'a> {
    ident: &'a TypePath,
    changing_state: Option<proc_macro2::TokenStream>,
    enforced_id: Option<proc_macro2::TokenStream>,
}

#[proc_macro_attribute]
pub fn packet_stream(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let packet_stream = packet_stream_by_inputs(&input);
    let client_bound_generated = generate_by_bound(&packet_stream, CLIENT_BOUND);
    let server_bound_generated = generate_by_bound(&packet_stream, SERVER_BOUND);
    let main_body_generated = generate_main_enum_body(&packet_stream);
    quote! {
        #main_body_generated
        #client_bound_generated
        #server_bound_generated
    }
    .into()
}

fn generate_main_enum_body(packet_stream: &PacketStream) -> proc_macro2::TokenStream {
    let vis = packet_stream.vis;
    let packet_stream_ident = packet_stream.ident;
    let state_idents = idents_by_states(&packet_stream.states);
    let attrs = packet_stream.attrs;
    let state_attrs = attrs_by_states(&packet_stream.states);
    quote! {
        #(#attrs)*
        #[allow(dead_code)]
        #vis enum #packet_stream_ident {
            #(#(#state_attrs)* #state_idents,)*
        }
    }
}

fn generate_by_bound(packet_stream: &PacketStream, bound: Bound) -> proc_macro2::TokenStream {
    let packet_stream_ident = packet_stream.ident;
    let state_quotes: Vec<_> = packet_stream
        .states
        .iter()
        .map(|state| {
            let state_ident = state.ident;
            let state_bound_packets = packets_filtered_with_suffix(&state.packets, bound.suffix);
            let state_bound_packet_paths = paths_by_packets(&state_bound_packets);
            let state = state.ident;
            let state_packets_name = format_ident!("{state_ident}{}Packets", bound.suffix);
            let vis = packet_stream.vis;
            let bound_packets = format_ident!("{}", bound.bound_packet_ident);
            let state_bound_packet_ids = ids_by_packets(&state_bound_packets);
            let repr_attr = if state_bound_packet_paths.is_empty() { None } else {
                Some(quote! { #[repr(u32)] })
            };
            let packets_enum = quote! {
                #[derive(serialization::Serializable)]
                #repr_attr
                #vis enum #state_packets_name {
                    #(#state_bound_packet_paths(#state_bound_packet_paths) #state_bound_packet_ids,)*
                }
            };
            let changing_state_stmt: Vec<_> = state_bound_packets
                .iter()
                .map(|field| {
                    if let Some(state) = &field.changing_state {
                        Some(quote! {Some(#packet_stream_ident::#state)})
                    } else {
                        Some(quote! {None})
                    }
                })
                .collect();

            quote! {
                #packets_enum

                impl From<#state_packets_name> for #bound_packets {
                    fn from(value: #state_packets_name) -> Self {
                        #bound_packets::#state_packets_name(value)
                    }
                }

                impl packetize::Packet<#packet_stream_ident> for #state_packets_name {
                    fn get_id(&self, state: &#packet_stream_ident) -> Option<u32> {
                        match self {
                            #(
                                #state_packets_name::#state_bound_packet_paths(value) => {
                                    packetize::Packet::<#packet_stream_ident>::get_id(value, state)
                                }
                            )*
                            _ => unreachable!()
                        }
                    }

                    fn is_changing_state(&self) -> Option<#packet_stream_ident> {
                        match self {
                            #(
                                #state_packets_name::#state_bound_packet_paths(value) => {
                                    <#state_bound_packet_paths as packetize::Packet::<#packet_stream_ident>>::is_changing_state(value)
                                }
                            )*
                            _ => unreachable!()
                        }
                    }
                }

                impl TryFrom<#bound_packets> for #state_packets_name {
                    type Error = ();

                    fn try_from(value: #bound_packets) -> Result<Self, Self::Error> {
                        match value {
                            #bound_packets::#state_packets_name(value) => Ok(value),
                            _ => Err(())?,
                        }
                    }
                }

                #(
                impl From<#state_bound_packet_paths> for #state_packets_name {
                    fn from(value: #state_bound_packet_paths) -> Self {
                        #state_packets_name::#state_bound_packet_paths(value)
                    }
                }

                impl From<#state_bound_packet_paths> for #bound_packets {
                    fn from(value: #state_bound_packet_paths) -> Self {
                        #bound_packets::#state_packets_name(#state_packets_name::#state_bound_packet_paths(value))
                    }
                }

                impl TryFrom<#bound_packets> for #state_bound_packet_paths {
                    type Error = ();

                    fn try_from(value: #bound_packets) -> Result<Self, Self::Error> {
                        match value {
                            #bound_packets::#state_packets_name(value) => Ok(value.try_into()?),
                            _ => Err(())?,
                        }
                    }
                }

                impl TryFrom<#state_packets_name> for #state_bound_packet_paths {
                    type Error = ();

                    fn try_from(value: #state_packets_name) -> Result<Self, Self::Error> {
                        match value {
                            #state_packets_name::#state_bound_packet_paths(value) => Ok(value),
                            _ => Err(())?,
                        }
                    }
                }

                impl packetize::Packet<#packet_stream_ident> for #state_bound_packet_paths {
                    fn get_id(&self, state: &#packet_stream_ident) -> Option<u32> {
                        match state {
                            #packet_stream_ident::#state => {
                                Some(#state_packets_name::#state_bound_packet_paths as u32)
                            },
                            _ => None,
                        }
                    }

                    fn is_changing_state(&self) -> Option<#packet_stream_ident> {
                        #changing_state_stmt
                    }
                }
                )*
            }
        })
        .collect();

    let bound_packet_ident = format_ident!("{}", bound.bound_packet_ident);
    let state_packet_names = packet_stream
        .states
        .iter()
        .map(|state| format_ident!("{}{}Packets", state.ident, bound.suffix))
        .collect::<Vec<_>>();
    let state_names = packet_stream
        .states
        .iter()
        .map(|state| state.ident)
        .collect::<Vec<_>>();
    let vis = packet_stream.vis;
    quote! {
            #(#state_quotes)*

            #[derive(serialization::Serializable)]
            #vis enum #bound_packet_ident {
                #(#state_packet_names(#state_packet_names),)*
            }

            impl packetize::Packet<#packet_stream_ident> for #bound_packet_ident {
                fn get_id(&self, state: &#packet_stream_ident) -> Option<u32> {
                    match self {
                        #(
                            #bound_packet_ident::#state_packet_names(value) => {
                                packetize::Packet::<#packet_stream_ident>::get_id(value, state)
                            }
                        )*
                        _ => unreachable!()
                    }
                }

                fn is_changing_state(&self) -> Option<#packet_stream_ident> {
                    match self {
                        #(
                            #bound_packet_ident::#state_packet_names(value) => {
                                <#state_packet_names as packetize::Packet::<#packet_stream_ident>>::is_changing_state(value)
                            }
                        )*
                        _ => unreachable!()
                    }
                }
            }

    #[cfg(feature = "serialization")]
    impl packetize::DecodePacket<#packet_stream_ident> for #bound_packet_ident {
        fn decode_packet<'de, D: serialization::Decoder<'de>>(
            decoder: D,
            state: &mut #packet_stream_ident,
        ) -> Result<Self, D::Error> {
            let result: Self = match state {
                #(
                #packet_stream_ident::#state_names =>
                    <#state_packet_names as serialization::Decode::<'de>>::decode(decoder)?.into(),
                )*
            };
            if let Some(new_state) = <Self as packetize::Packet::<#packet_stream_ident>>::is_changing_state(&result) {
                *state = new_state;
            }
            Ok(result)
        }
    }

    #[cfg(feature = "serialization")]
    impl packetize::EncodePacket<#packet_stream_ident> for #bound_packet_ident {
        fn encode_packet<E: serialization::Encoder>(
            &self,
            encoder: E,
            state: &mut #packet_stream_ident,
        ) -> Result<(), E::Error> {
            if let Some(new_state) = <Self as packetize::Packet::<#packet_stream_ident>>::is_changing_state(self) {
                *state = new_state;
            }
            match self {
                #(
                #bound_packet_ident::#state_packet_names(value) => serialization::Encode::encode(value, encoder)?,
                )*
            };
            Ok(())
        }
    }
        }
}

fn packet_stream_by_inputs<'a>(item_enum: &'a ItemEnum) -> PacketStream<'a> {
    let states: Vec<_> = item_enum
        .variants
        .iter()
        .map(|enum_variant| packet_stream_state_by_enum_variant(enum_variant))
        .collect();
    let packets: Vec<Packet> = states.iter().fold(Vec::new(), |mut acc, state| {
        acc.append(&mut state.packets.clone());
        acc
    });
    PacketStream {
        ident: &item_enum.ident,
        vis: &item_enum.vis,
        states,
        packets,
        attrs: &item_enum.attrs,
    }
}

fn idents_by_states<'a>(states: &Vec<PacketStreamState<'a>>) -> Vec<&'a Ident> {
    states.iter().map(|state| state.ident).collect()
}

fn packet_stream_state_by_enum_variant(enum_variant: &Variant) -> PacketStreamState {
    PacketStreamState {
        ident: &enum_variant.ident,
        packets: enum_variant
            .fields
            .iter()
            .map(|field| Packet {
                ident: match &field.ty {
                    Type::Path(path) => path,
                    _ => unimplemented!("type must path"),
                },
                changing_state: find_ident_in_attrs(&field.attrs, "change_state_to").map(|attr| {
                    match attr.meta {
                        syn::Meta::List(list) => list.tokens,
                        _ => panic!("attribute needs single value input"),
                    }
                }),
                enforced_id: find_ident_in_attrs(&field.attrs, "id").map(|attr| match attr.meta {
                    syn::Meta::List(list) => {
                        let tokens = list.tokens;
                        quote! { = #tokens }
                    }
                    _ => panic!("attribute needs single value input"),
                }),
            })
            .collect(),
        attrs: &enum_variant.attrs,
    }
}

fn find_ident_in_attrs<'a>(attrs: &'a Vec<Attribute>, ident: &'static str) -> Option<Attribute> {
    attrs
        .iter()
        .find(|attr| {
            let list = match &attr.meta {
                Meta::List(list) => list,
                _ => return false,
            };
            if !list.path.is_ident(ident) {
                return false;
            }
            true
        })
        .map(|v| v.clone())
}

fn paths_by_packets<'a>(packets: &Vec<&Packet<'a>>) -> Vec<&'a TypePath> {
    packets.iter().map(|packet| packet.ident).collect()
}

fn ids_by_packets<'a>(packets: &Vec<&Packet<'a>>) -> Vec<Option<proc_macro2::TokenStream>> {
    packets
        .iter()
        .map(|packet| packet.enforced_id.clone())
        .collect()
}

fn packets_filtered_with_suffix<'a>(
    packets: &'a Vec<Packet<'a>>,
    ends_with: &'static str,
) -> Vec<&'a Packet<'a>> {
    packets
        .iter()
        .filter(|packet| {
            packet
                .ident
                .to_token_stream()
                .to_string()
                .ends_with(ends_with)
        })
        .collect::<Vec<_>>()
}

fn attrs_by_states<'a>(states: &Vec<PacketStreamState<'a>>) -> Vec<&'a Vec<Attribute>> {
    states.iter().map(|state| state.attrs).collect()
}
