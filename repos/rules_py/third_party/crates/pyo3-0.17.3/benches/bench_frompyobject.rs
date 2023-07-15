use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

use pyo3::{
    prelude::*,
    types::{PyList, PyString},
};

#[derive(FromPyObject)]
enum ManyTypes {
    Int(i32),
    Bytes(Vec<u8>),
    String(String),
}

fn enum_from_pyobject(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        let obj = PyString::new(py, "hello world");
        b.iter(|| {
            let _: ManyTypes = obj.extract().unwrap();
        });
    })
}

fn list_via_cast_as(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        let any: &PyAny = PyList::empty(py).into();

        b.iter(|| {
            let _list: &PyList = black_box(any).cast_as().unwrap();
        });
    })
}

fn list_via_extract(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        let any: &PyAny = PyList::empty(py).into();

        b.iter(|| {
            let _list: &PyList = black_box(any).extract().unwrap();
        });
    })
}

fn not_a_list_via_cast_as(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        let any: &PyAny = PyString::new(py, "foobar").into();

        b.iter(|| {
            black_box(any).cast_as::<PyList>().unwrap_err();
        });
    })
}

fn not_a_list_via_extract(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        let any: &PyAny = PyString::new(py, "foobar").into();

        b.iter(|| {
            black_box(any).extract::<&PyList>().unwrap_err();
        });
    })
}

#[derive(FromPyObject)]
enum ListOrNotList<'a> {
    List(&'a PyList),
    NotList(&'a PyAny),
}

fn not_a_list_via_extract_enum(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        let any: &PyAny = PyString::new(py, "foobar").into();

        b.iter(|| match black_box(any).extract::<ListOrNotList<'_>>() {
            Ok(ListOrNotList::List(_list)) => panic!(),
            Ok(ListOrNotList::NotList(_any)) => (),
            Err(_) => panic!(),
        });
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("enum_from_pyobject", enum_from_pyobject);
    c.bench_function("list_via_cast_as", list_via_cast_as);
    c.bench_function("list_via_extract", list_via_extract);
    c.bench_function("not_a_list_via_cast_as", not_a_list_via_cast_as);
    c.bench_function("not_a_list_via_extract", not_a_list_via_extract);
    c.bench_function("not_a_list_via_extract_enum", not_a_list_via_extract_enum);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
