use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sierra-2-llvm-simple-test", |b| {
        b.iter(sierra_add_test);
    });
}

fn sierra_add_test() {
    std::process::Command::new("sh").arg("./test");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
