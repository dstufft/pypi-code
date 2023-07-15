use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use pyo3::prelude::*;
use pyo3::types::PyList;

fn iter_list(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 100_000;
        let list = PyList::new(py, 0..LEN);
        let mut sum = 0;
        b.iter(|| {
            for x in list.iter() {
                let i: u64 = x.extract().unwrap();
                sum += i;
            }
        });
    });
}

fn list_new(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 50_000;
        b.iter(|| PyList::new(py, 0..LEN));
    });
}

fn list_get_item(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 50_000;
        let list = PyList::new(py, 0..LEN);
        let mut sum = 0;
        b.iter(|| {
            for i in 0..LEN {
                sum += list.get_item(i).unwrap().extract::<usize>().unwrap();
            }
        });
    });
}

#[cfg(not(Py_LIMITED_API))]
fn list_get_item_unchecked(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: usize = 50_000;
        let list = PyList::new(py, 0..LEN);
        let mut sum = 0;
        b.iter(|| {
            for i in 0..LEN {
                unsafe {
                    sum += list.get_item_unchecked(i).extract::<usize>().unwrap();
                }
            }
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter_list", iter_list);
    c.bench_function("list_new", list_new);
    c.bench_function("list_get_item", list_get_item);
    #[cfg(not(Py_LIMITED_API))]
    c.bench_function("list_get_item_unchecked", list_get_item_unchecked);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
