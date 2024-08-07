use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Ident, Index, Item, ItemStruct};

pub(crate) fn encode_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item {
        Item::Enum(value) => {
            let item_name = &value.ident;
            if value.variants.len() == 1 {
                quote! {
                    impl packetize::Encode for #item_name {
                        fn encode(&self, buf: &mut impl fastbuf::WriteBuf) -> core::result::Result<(), ()> {
                            Ok(())
                        }
                    }
                }
            } else {
                quote! {
                    impl packetize::Encode for #item_name {
                        fn encode(&self, buf: &mut impl fastbuf::WriteBuf) -> core::result::Result<(), ()> {
                            let value: &[u8; core::mem::size_of::<Self>()] = unsafe { core::mem::transmute(self) };
                            buf.write(value)?;
                            Ok(())
                        }
                    }
                }

            }
                    }
        Item::Struct(item_struct) => {
            let item_name = &item_struct.ident;
            let has_field_name = item_struct.fields.iter().last().map(|field| field.ident.is_some());
            let encode_constructor = generate_encoder(&item_struct, has_field_name);
            quote! {
               impl packetize::Encode for #item_name {
                   fn encode(&self, buf: &mut impl fastbuf::WriteBuf) -> core::result::Result<(), ()> {
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

pub(crate) fn decode_derive(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item {
        Item::Enum(value) => {
            let item_name = &value.ident;
            if value.variants.len()==1 {
            let first = &value.variants.first().unwrap().ident;
             quote! {
                impl packetize::Decode for #item_name {
                    fn decode(buf: &mut impl fastbuf::ReadBuf) -> core::result::Result<Self, ()> {
                        return Ok(Self::#first)
                    }
                }
            }
            } else {
            quote! {
                impl packetize::Decode for #item_name {
                    fn decode(buf: &mut impl fastbuf::ReadBuf) -> core::result::Result<Self, ()> {
                        let slice = buf.read(size_of::<Self>());
                        let mut result = [unsafe { core::mem::MaybeUninit::uninit().assume_init() }; size_of::<Self>()];
                        result.copy_from_slice(slice);
                        Ok(unsafe { core::mem::transmute::<_, Self>(result) })
                    }
                }
            }
            }
        }
        Item::Struct(item_struct) => {
            let item_name = &item_struct.ident;
            let has_field_name = item_struct.fields.iter().last().map(|field| field.ident.is_some());
            let decode_constructor = generate_decoder(&item_struct, has_field_name);
            quote! {
               impl packetize::Decode for #item_name
               {
                   fn decode(buf: &mut impl fastbuf::ReadBuf) -> Result<Self, ()> {
                       Ok(#decode_constructor)
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
    let decode = quote!(packetize::Decode::decode(buf)?);
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
) -> Option<proc_macro2::TokenStream> {
    if let Some(has_field_name) = has_field_name {
        Some(if has_field_name {
            let fields = map_fields_to_idents(&item_struct.fields);
            quote! {
                #(packetize::Encode::encode(&self.#fields, buf)?;)*
            }
        } else {
            let fields = (0..item_struct.fields.len()).map(|i| Index::from(i));
            quote! {
                #(packetize::Encode::encode(&self.#fields, buf)?;)*
            }
        })
    } else {
        None
    }
}

fn map_fields_to_idents(fields: &Fields) -> Vec<Ident> {
    fields
        .iter()
        .map(|field| field.ident.clone().unwrap())
        .collect()
}
