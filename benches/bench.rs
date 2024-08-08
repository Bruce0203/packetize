use std::{fmt::Debug, hint::black_box, str::FromStr};

use arrayvec::ArrayString;
use fastbuf::{Buffer, WriteBuf};
use packetize::Encode;

fn get_model() -> MyComponent {
    MyComponent {
        value: 14,
        value2: ArrayString::from_str("123112iyg3riyu1g34fygakvfo2a42").unwrap(),
    }
}

struct MyComponent {
    value: u8,
    value2: ArrayString<10000>,
}

impl Debug for MyComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyComponent").finish()
    }
}

impl Encode for MyComponent {
    fn encode(&self, buf: &mut impl WriteBuf) -> Result<(), ()> {
        self.value.encode(buf)?;
        self.value2.encode(buf)?;
        Ok(())
    }
}

#[divan::bench(args = [get_model()], sample_size = 1000, sample_count = 1000)]
fn encode_something(model: &MyComponent) {
    let mut buf = Buffer::<10000>::new();
    model.encode(&mut buf);
    black_box(&buf);
}

fn main() {
    divan::main()
}
