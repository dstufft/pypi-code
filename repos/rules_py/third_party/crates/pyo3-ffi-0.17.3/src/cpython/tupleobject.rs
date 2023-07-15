use crate::object::*;
#[cfg(not(PyPy))]
use crate::pyport::Py_ssize_t;

#[repr(C)]
pub struct PyTupleObject {
    pub ob_base: PyVarObject,
    pub ob_item: [*mut PyObject; 1],
}

// skipped _PyTuple_Resize
// skipped _PyTuple_MaybeUntrack

/// Macro, trading safety for speed

// skipped _PyTuple_CAST

#[inline]
#[cfg(not(PyPy))]
pub unsafe fn PyTuple_GET_SIZE(op: *mut PyObject) -> Py_ssize_t {
    Py_SIZE(op)
}

#[inline]
#[cfg(not(PyPy))]
pub unsafe fn PyTuple_GET_ITEM(op: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    *(*(op as *mut PyTupleObject))
        .ob_item
        .as_ptr()
        .offset(i as isize)
}

/// Macro, *only* to be used to fill in brand new tuples
#[inline]
#[cfg(not(PyPy))]
pub unsafe fn PyTuple_SET_ITEM(op: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) {
    *(*(op as *mut PyTupleObject))
        .ob_item
        .as_mut_ptr()
        .offset(i as isize) = v;
}

// skipped _PyTuple_DebugMallocStats
