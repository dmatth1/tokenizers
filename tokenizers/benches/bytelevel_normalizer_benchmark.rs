#[macro_use]
extern crate criterion;

use std::hint::black_box;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput};
use tokenizers::normalizers::byte_level::ByteLevel;
use tokenizers::{NormalizedString, Normalizer};

/// Times only the `ByteLevel` normalizer's `normalize` call over the standard `big.txt`
/// corpus. Each line gets a fresh `NormalizedString` (built outside the timed region) so
/// we measure the byte-level transform itself, not allocation of the input. This isolates
/// the Tier 1.1 change (`apply_byte_map` vs the general transform machinery), which no
/// standard end-to-end bench exercises (every other bench wires `ByteLevel` as a
/// *pre-tokenizer*, not a *normalizer*).
fn bench_bytelevel_normalizer(c: &mut Criterion) {
    let data = std::fs::read_to_string("data/big.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let norm = ByteLevel::new();

    let mut group = c.benchmark_group("bytelevel-normalizer");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("ByteLevel normalize big.txt", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::new(0, 0);
            for _ in 0..iters {
                // Pre-build the inputs so only `normalize` is timed.
                let mut inputs: Vec<NormalizedString> =
                    lines.iter().map(|l| NormalizedString::from(*l)).collect();
                let start = Instant::now();
                for ns in inputs.iter_mut() {
                    norm.normalize(black_box(ns)).unwrap();
                }
                duration = duration.checked_add(start.elapsed()).unwrap();
                black_box(&inputs);
            }
            duration
        })
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = bench_bytelevel_normalizer
}
criterion_main!(benches);
