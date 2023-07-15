use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use pyo3::prelude::*;

use pyo3::{intern, prepare_freethreaded_python};

fn getattr_direct(b: &mut Bencher<'_>) {
    prepare_freethreaded_python();

    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();

        b.iter(|| sys.getattr("version").unwrap());
    });
}

fn getattr_intern(b: &mut Bencher<'_>) {
    prepare_freethreaded_python();

    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();

        b.iter(|| sys.getattr(intern!(py, "version")).unwrap());
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("getattr_direct", getattr_direct);
    c.bench_function("getattr_intern", getattr_intern);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
