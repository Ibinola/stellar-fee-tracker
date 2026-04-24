use criterion::{criterion_group, criterion_main, Criterion};
use stellar_devkit::simulation::fee_model::{FeeModel, FeeModelConfig};

fn bench_generate_100k(c: &mut Criterion) {
    let config = FeeModelConfig {
        ledger_count: 100_000,
        ..Default::default()
    };
    c.bench_function("fee_model_generate_100k", |b| {
        b.iter(|| FeeModel::generate(&config))
    });
}

criterion_group!(benches, bench_generate_100k);
criterion_main!(benches);
