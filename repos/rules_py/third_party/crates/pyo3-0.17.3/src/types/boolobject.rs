// Copyright (c) 2017-present PyO3 Project and Contributors
use crate::{
    ffi, AsPyPointer, FromPyObject, IntoPy, PyAny, PyObject, PyResult, PyTryFrom, Python,
    ToPyObject,
};

/// Represents a Python `bool`.
#[repr(transparent)]
pub struct PyBool(PyAny);

pyobject_native_type!(PyBool, ffi::PyObject, ffi::PyBool_Type, #checkfunction=ffi::PyBool_Check);

impl PyBool {
    /// Depending on `val`, returns `true` or `false`.
    #[inline]
    pub fn new(py: Python<'_>, val: bool) -> &PyBool {
        unsafe { py.from_borrowed_ptr(if val { ffi::Py_True() } else { ffi::Py_False() }) }
    }

    /// Gets whether this boolean is `true`.
    #[inline]
    pub fn is_true(&self) -> bool {
        self.as_ptr() == unsafe { crate::ffi::Py_True() }
    }
}

/// Converts a Rust `bool` to a Python `bool`.
impl ToPyObject for bool {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        unsafe {
            PyObject::from_borrowed_ptr(
                py,
                if *self {
                    ffi::Py_True()
                } else {
                    ffi::Py_False()
                },
            )
        }
    }
}

impl IntoPy<PyObject> for bool {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyBool::new(py, self).into()
    }
}

/// Converts a Python `bool` to a Rust `bool`.
///
/// Fails with `TypeError` if the input is not a Python `bool`.
impl<'source> FromPyObject<'source> for bool {
    fn extract(obj: &'source PyAny) -> PyResult<Self> {
        Ok(<PyBool as PyTryFrom>::try_from(obj)?.is_true())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{PyAny, PyBool};
    use crate::Python;
    use crate::ToPyObject;

    #[test]
    fn test_true() {
        Python::with_gil(|py| {
            assert!(PyBool::new(py, true).is_true());
            let t: &PyAny = PyBool::new(py, true).into();
            assert!(t.extract::<bool>().unwrap());
            assert!(true.to_object(py).is(PyBool::new(py, true)));
        });
    }

    #[test]
    fn test_false() {
        Python::with_gil(|py| {
            assert!(!PyBool::new(py, false).is_true());
            let t: &PyAny = PyBool::new(py, false).into();
            assert!(!t.extract::<bool>().unwrap());
            assert!(false.to_object(py).is(PyBool::new(py, false)));
        });
    }
}
