use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_decode(c: &mut Criterion) {
    // 64 KiB of seeded random bytes as a stand-in for LZMA-compressed data.
    // Replace with real LZMA-encoded data once the encoder lands.
    let input: Vec<u8> = {
        let mut state: u64 = 42;
        (0..64 * 1024)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                (state >> 33) as u8
            })
            .collect()
    };

    let mut group = c.benchmark_group("decode");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function("lzma_64k", |b| {
        b.iter(|| {
            // Placeholder: just touch the data until decode is implemented.
            black_box(&input);
        });
    });
    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
