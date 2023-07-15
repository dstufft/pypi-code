#![cfg(feature = "anyhow")]

#[test]
fn test_anyhow_py_function_ok_result() {
    use pyo3::{py_run, pyfunction, wrap_pyfunction, Python};

    #[pyfunction]
    #[allow(clippy::unnecessary_wraps)]
    fn produce_ok_result() -> anyhow::Result<String> {
        Ok(String::from("OK buddy"))
    }

    Python::with_gil(|py| {
        let func = wrap_pyfunction!(produce_ok_result)(py).unwrap();

        py_run!(
            py,
            func,
            r#"
            func()
            "#
        );
    });
}

#[test]
fn test_anyhow_py_function_err_result() {
    use pyo3::{pyfunction, types::PyDict, wrap_pyfunction, Python};

    #[pyfunction]
    fn produce_err_result() -> anyhow::Result<String> {
        anyhow::bail!("error time")
    }

    Python::with_gil(|py| {
        let func = wrap_pyfunction!(produce_err_result)(py).unwrap();
        let locals = PyDict::new(py);
        locals.set_item("func", func).unwrap();

        py.run(
            r#"
            func()
            "#,
            None,
            Some(locals),
        )
        .unwrap_err();
    });
}
