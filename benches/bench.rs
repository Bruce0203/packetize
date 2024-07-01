#![feature(generic_const_exprs)]

use std::hint::black_box;

use criterion::Criterion;
use fast_collections::{
    generic_array::ArrayLength,
    typenum::{Integer, Len, U100},
};
use fast_collections::{typenum::Unsigned, Cursor, PushTransmuteUnchecked, String};

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("Test", |b| {
        b.iter(|| {
            #[inline(always)]
            fn a<T: Len>()
            where
                [(); { <T as Len>::Output::USIZE / 8 + 1 }]:,
            {
                let len = [0u8; { <T as Len>::Output::USIZE / 8 + 1 }];
                black_box(len);
            }
            a::<U100>();
        });
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, criterion_bench);
