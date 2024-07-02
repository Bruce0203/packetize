#![feature(const_mut_refs)]

use std::hint::black_box;

use criterion::Criterion;
use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Len, U10000, U1000000},
    Clear, GetTransmute,
};
use fast_collections::{Cursor, String};
use packetize::Encode;

fn criterion_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Ts");
    group.throughput(criterion::Throughput::Elements(1000));
    group.bench_function("Test", |b| {
        b.iter(|| {
            let mut write_cursor = Cursor::<u8, U1000000>::new();
            let value = const {
                let mut value = MyComponent {
                    value: 14,
                    value2: String::from_array(unsafe {
                        fast_collections::const_transmute_unchecked::<[u8; 5], [u8; 10000]>(
                            *b"ABCDE",
                        )
                    }),
                };
                *unsafe { value.value2.as_vec_mut().len_mut() } = 5;
                value
            };
            black_box(value);
            //unsafe { value.encode_unchecked(&mut write_cursor) };
            //black_box(write_cursor.get_transmute::<u8>(0));
            //write_cursor.clear();
        });
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, criterion_bench);

pub struct MyComponent {
    value: u8,
    value2: String<U10000>,
}
impl<N> Encode<N> for MyComponent
where
    N: ArrayLength + Len,
{
    fn encode(self, write_cursor: &mut fast_collections::Cursor<u8, N>) -> Result<(), ()> {
        //FIXME use unchecked_add rather than add_assign
        //if core::mem::size_of::<MyComponent>() + write_cursor.pos() < N::USIZE {
        self.value.encode(write_cursor)?;
        self.value2.encode(write_cursor)?;
        Ok(())
    }

    #[inline(always)]
    unsafe fn encode_unchecked(self, write_cursor: &mut fast_collections::Cursor<u8, N>) {
        self.value.encode_unchecked(write_cursor);
        self.value2.encode_unchecked(write_cursor);
    }
}
