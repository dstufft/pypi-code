use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use pyo3::prelude::*;
use pyo3::types::PySet;
use std::collections::{BTreeSet, HashSet};

fn iter_set(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 100_000;
        let set = PySet::new(py, &(0..LEN).collect::<Vec<_>>()).unwrap();
        let mut sum = 0;
        b.iter(|| {
            for x in set.iter() {
                let i: u64 = x.extract().unwrap();
                sum += i;
            }
        });
    });
}

fn extract_hashset(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 100_000;
        let set = PySet::new(py, &(0..LEN).collect::<Vec<_>>()).unwrap();
        b.iter(|| HashSet::<u64>::extract(set));
    });
}

fn extract_btreeset(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 100_000;
        let set = PySet::new(py, &(0..LEN).collect::<Vec<_>>()).unwrap();
        b.iter(|| BTreeSet::<u64>::extract(set));
    });
}

#[cfg(feature = "hashbrown")]
fn extract_hashbrown_set(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 100_000;
        let set = PySet::new(py, &(0..LEN).collect::<Vec<_>>()).unwrap();
        b.iter(|| hashbrown::HashSet::<u64>::extract(set));
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter_set", iter_set);
    c.bench_function("extract_hashset", extract_hashset);
    c.bench_function("extract_btreeset", extract_btreeset);

    #[cfg(feature = "hashbrown")]
    c.bench_function("extract_hashbrown_set", extract_hashbrown_set);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
