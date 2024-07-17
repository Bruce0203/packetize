use proc_macro::TokenStream;

mod packetize;
#[cfg(feature = "stream")]
mod packetizer;

#[proc_macro_derive(Encode)]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    packetize::encode_derive(input)
}

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    packetize::decode_derive(input)
}

#[cfg(feature = "stream")]
#[proc_macro_attribute]
pub fn streaming_packets(attr: TokenStream, input: TokenStream) -> TokenStream {
    packetizer::streaming_packets(attr, input)
}
