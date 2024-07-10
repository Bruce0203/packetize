#![feature(generic_const_exprs)]
#![feature(const_mut_refs)]

use std::hint::black_box;

use criterion::Criterion;
use fast_collections::{
    generic_array::ArrayLength,
    typenum::{bit, Len, U10, U10000, U1000000},
    Clear, GetTransmute,
};
use fast_collections::{Cursor, String};
use integer_encoding::VarInt;
use packetize::Encode;

fn criterion_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Ts");
    group.throughput(criterion::Throughput::Elements(1000));

    let value = {
        let mut value = MyComponent {
            value: 14,
            value2: String::from_array(*b"123112iyg3riyu1g34fygakvfo2a42"),
        };
        *unsafe { value.value2.as_vec_mut().len_mut() } = 5;
        value
    };
    let write_cursor = &mut Cursor::<u8, U1000000>::new();
    group.bench_function("Test", |b| {
        b.iter(|| {
            unsafe { value.encode(write_cursor) };
            write_cursor.clear();
        });
    });
    black_box(write_cursor);
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, criterion_bench);

struct MyComponent {
    value: u8,
    value2: String<U10000>,
}

impl<N> Encode<N> for MyComponent
where
    N: ArrayLength,
    [(); N::USIZE]:,
{
    fn encode(&self, write_cursor: &mut fast_collections::Cursor<u8, N>) -> Result<(), ()> {
        self.value.encode(write_cursor)?;
        self.value2.encode(write_cursor)?;
        Ok(())
    }
}
