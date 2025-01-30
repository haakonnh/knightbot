use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use knightbot::board::tables::{generate_rank_attack_table, test_attack_mask};

fn benchmark_generate_rank_attack_table(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_rank_attack_table");
    group.sample_size(100); // Increase the sample size for more granular measurements
    group.measurement_time(std::time::Duration::new(10, 0)); // Increase the measurement time

    group.bench_function(BenchmarkId::new("generate_rank_attack_table", "default"), |b| {
        b.iter(|| black_box(generate_rank_attack_table()))
    });

    group.finish();
}

criterion_group!(benches, benchmark_generate_rank_attack_table);
criterion_main!(benches);