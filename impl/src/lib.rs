use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Index, Item, ItemStruct};

#[proc_macro_derive(Packetize)]
pub fn packetize_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item {
        Item::Enum(value) => {
            let item_name = &value.ident;
            quote! {
                impl packetize::Encode for #item_name {
                    fn encode<N: fast_collections::generic_array::ArrayLength>
                        (&self, write_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> core::result::Result<(), ()> 
                    where
                        [(); <N as fast_collections::typenum::Unsigned>::USIZE]:,
                    {
                        fast_collections::PushTransmute::push_transmute(write_cursor, Clone::clone(self))
                    }
                }

                impl packetize::Decode for #item_name {
                    fn decode<N: fast_collections::generic_array::ArrayLength>
                        (read_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> core::result::Result<Self, ()> 
                    where
                        [(); <N as fast_collections::typenum::Unsigned>::USIZE]:,
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
                   fn decode<N: fast_collections::generic_array::ArrayLength>
                       (read_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> Result<Self, ()>
                    where 
                        [(); <N as fast_collections::typenum::Unsigned>::USIZE]:,
                   {
                       Ok(#decode_constructor)
                   }
               }

               impl packetize::Encode for #item_name
               {
                   fn encode<N: fast_collections::generic_array::ArrayLength>
                       (&self, write_cursor: &mut fast_collections::cursor::Cursor<u8, N>) -> Result<(), ()> 
                    where
                        [(); <N as fast_collections::typenum::Unsigned>::USIZE]:,
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
                #(packetize::Encode::encode::<N>(&self.#fields, write_cursor)?;)*
            }
        } else {
            let fields = (0..item_struct.fields.len()).map(|i| Index::from(i));
            quote! {
                #(packetize::Encode::encode::<N>(&self.#fields, write_cursor)?;)*
            }
        }
    } else {
        quote! {}
    }
}
