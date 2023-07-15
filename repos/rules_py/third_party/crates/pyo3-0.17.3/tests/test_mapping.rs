#![cfg(feature = "macros")]

use std::collections::HashMap;

use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::IntoPyDict;
use pyo3::types::PyList;
use pyo3::types::PyMapping;
use pyo3::types::PySequence;

mod common;

#[pyclass(mapping)]
struct Mapping {
    index: HashMap<String, usize>,
}

#[pymethods]
impl Mapping {
    #[new]
    fn new(elements: Option<&PyList>) -> PyResult<Self> {
        if let Some(pylist) = elements {
            let mut elems = HashMap::with_capacity(pylist.len());
            for (i, pyelem) in pylist.into_iter().enumerate() {
                let elem = String::extract(pyelem)?;
                elems.insert(elem, i);
            }
            Ok(Self { index: elems })
        } else {
            Ok(Self {
                index: HashMap::new(),
            })
        }
    }

    fn __len__(&self) -> usize {
        self.index.len()
    }

    fn __getitem__(&self, query: String) -> PyResult<usize> {
        self.index
            .get(&query)
            .copied()
            .ok_or_else(|| PyKeyError::new_err("unknown key"))
    }

    fn __setitem__(&mut self, key: String, value: usize) {
        self.index.insert(key, value);
    }

    fn __delitem__(&mut self, key: String) -> PyResult<()> {
        if self.index.remove(&key).is_none() {
            Err(PyKeyError::new_err("unknown key"))
        } else {
            Ok(())
        }
    }

    fn get(&self, py: Python<'_>, key: &str, default: Option<PyObject>) -> Option<PyObject> {
        self.index
            .get(key)
            .map(|value| value.into_py(py))
            .or(default)
    }
}

/// Return a dict with `m = Mapping(['1', '2', '3'])`.
fn map_dict(py: Python<'_>) -> &pyo3::types::PyDict {
    let d = [("Mapping", py.get_type::<Mapping>())].into_py_dict(py);
    py_run!(py, *d, "m = Mapping(['1', '2', '3'])");
    d
}

#[test]
fn test_getitem() {
    Python::with_gil(|py| {
        let d = map_dict(py);

        py_assert!(py, *d, "m['1'] == 0");
        py_assert!(py, *d, "m['2'] == 1");
        py_assert!(py, *d, "m['3'] == 2");
        py_expect_exception!(py, *d, "print(m['4'])", PyKeyError);
    });
}

#[test]
fn test_setitem() {
    Python::with_gil(|py| {
        let d = map_dict(py);

        py_run!(py, *d, "m['1'] = 4; assert m['1'] == 4");
        py_run!(py, *d, "m['0'] = 0; assert m['0'] == 0");
        py_assert!(py, *d, "len(m) == 4");
        py_expect_exception!(py, *d, "m[0] = 'hello'", PyTypeError);
        py_expect_exception!(py, *d, "m[0] = -1", PyTypeError);
    });
}

#[test]
fn test_delitem() {
    Python::with_gil(|py| {
        let d = map_dict(py);
        py_run!(
            py,
            *d,
            "del m['1']; assert len(m) == 2 and m['2'] == 1 and m['3'] == 2"
        );
        py_expect_exception!(py, *d, "del m[-1]", PyTypeError);
        py_expect_exception!(py, *d, "del m['4']", PyKeyError);
    });
}

#[test]
fn mapping_is_not_sequence() {
    Python::with_gil(|py| {
        let mut index = HashMap::new();
        index.insert("Foo".into(), 1);
        index.insert("Bar".into(), 2);
        let m = Py::new(py, Mapping { index }).unwrap();

        PyMapping::register::<Mapping>(py).unwrap();

        assert!(m.as_ref(py).downcast::<PyMapping>().is_ok());
        assert!(m.as_ref(py).downcast::<PySequence>().is_err());
    });
}
