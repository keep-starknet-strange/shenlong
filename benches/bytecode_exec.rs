use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cairo_run_test", |b| {
        b.iter(cairo_run_test);
    });
}

fn cairo_run_test() {
    std::process::Command::new("sh")
        .arg("cairo-rs-run")
        .arg("add.json")
        .arg("--layout")
        .arg("small")
        .output()
        .expect("dab");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
