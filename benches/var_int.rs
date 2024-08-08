use std::hint::black_box;

use fastbuf::{Buffer, WriteBuf};
use packetize::VarInt;
use rand::Rng;

#[divan::bench(args = [get_model()], sample_size = 1000, sample_count = 1000)]
fn encode_var_int_to_buffer(model: u32) {
    let mut buf = Buffer::<10000>::new();
    model
        .encode_var(|b| {
            buf.write(&[b]);
            Ok(())
        })
        .unwrap();
    black_box(&buf);
}

fn get_model() -> u32 {
    rand::thread_rng().gen_range(0..5)
}

fn main() {
    divan::main()
}
