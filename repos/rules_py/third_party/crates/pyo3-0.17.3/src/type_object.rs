// Copyright (c) 2017-present PyO3 Project and Contributors
//! Python type object information

use crate::impl_::pyclass::PyClassItemsIter;
use crate::internal_tricks::extract_cstr_or_leak_cstring;
use crate::once_cell::GILOnceCell;
use crate::pyclass::create_type_object;
use crate::pyclass::PyClass;
use crate::types::{PyAny, PyType};
use crate::{conversion::IntoPyPointer, PyMethodDefType};
use crate::{ffi, AsPyPointer, PyNativeType, PyObject, PyResult, Python};
use parking_lot::{const_mutex, Mutex};
use std::thread::{self, ThreadId};

/// `T: PyLayout<U>` represents that `T` is a concrete representation of `U` in the Python heap.
/// E.g., `PyCell` is a concrete representation of all `pyclass`es, and `ffi::PyObject`
/// is of `PyAny`.
///
/// This trait is intended to be used internally.
///
/// # Safety
///
/// This trait must only be implemented for types which represent valid layouts of Python objects.
pub unsafe trait PyLayout<T> {}

/// `T: PySizedLayout<U>` represents that `T` is not a instance of
/// [`PyVarObject`](https://docs.python.org/3.8/c-api/structures.html?highlight=pyvarobject#c.PyVarObject).
/// In addition, that `T` is a concrete representation of `U`.
pub trait PySizedLayout<T>: PyLayout<T> + Sized {}

/// Python type information.
/// All Python native types (e.g., `PyDict`) and `#[pyclass]` structs implement this trait.
///
/// This trait is marked unsafe because:
///  - specifying the incorrect layout can lead to memory errors
///  - the return value of type_object must always point to the same PyTypeObject instance
///
/// It is safely implemented by the `pyclass` macro.
///
/// # Safety
///
/// Implementations must provide an implementation for `type_object_raw` which infallibly produces a
/// non-null pointer to the corresponding Python type object.
pub unsafe trait PyTypeInfo: Sized {
    /// Class name.
    const NAME: &'static str;

    /// Module name, if any.
    const MODULE: Option<&'static str>;

    /// Utility type to make Py::as_ref work.
    type AsRefTarget: PyNativeType;

    /// Returns the PyTypeObject instance for this type.
    fn type_object_raw(py: Python<'_>) -> *mut ffi::PyTypeObject;

    /// Returns the safe abstraction over the type object.
    fn type_object(py: Python<'_>) -> &PyType {
        unsafe { py.from_borrowed_ptr(Self::type_object_raw(py) as _) }
    }

    /// Checks if `object` is an instance of this type or a subclass of this type.
    fn is_type_of(object: &PyAny) -> bool {
        unsafe { ffi::PyObject_TypeCheck(object.as_ptr(), Self::type_object_raw(object.py())) != 0 }
    }

    /// Checks if `object` is an instance of this type.
    fn is_exact_type_of(object: &PyAny) -> bool {
        unsafe { ffi::Py_TYPE(object.as_ptr()) == Self::type_object_raw(object.py()) }
    }
}

/// Legacy trait which previously held the `type_object` method now found on `PyTypeInfo`.
///
/// # Safety
///
/// This trait used to have stringent safety requirements, but they are now irrelevant as it is deprecated.
#[deprecated(
    since = "0.17.0",
    note = "PyTypeObject::type_object was moved to PyTypeInfo::type_object"
)]
pub unsafe trait PyTypeObject: PyTypeInfo {}

#[allow(deprecated)]
unsafe impl<T: PyTypeInfo> PyTypeObject for T {}

/// Lazy type object for PyClass.
#[doc(hidden)]
pub struct LazyStaticType {
    // Boxed because Python expects the type object to have a stable address.
    value: GILOnceCell<*mut ffi::PyTypeObject>,
    // Threads which have begun initialization of the `tp_dict`. Used for
    // reentrant initialization detection.
    initializing_threads: Mutex<Vec<ThreadId>>,
    tp_dict_filled: GILOnceCell<PyResult<()>>,
}

impl LazyStaticType {
    pub const fn new() -> Self {
        LazyStaticType {
            value: GILOnceCell::new(),
            initializing_threads: const_mutex(Vec::new()),
            tp_dict_filled: GILOnceCell::new(),
        }
    }

    pub fn get_or_init<T: PyClass>(&self, py: Python<'_>) -> *mut ffi::PyTypeObject {
        fn inner<T: PyClass>() -> *mut ffi::PyTypeObject {
            // Safety: `py` is held by the caller of `get_or_init`.
            let py = unsafe { Python::assume_gil_acquired() };
            create_type_object::<T>(py)
        }

        // Uses explicit GILOnceCell::get_or_init::<fn() -> *mut ffi::PyTypeObject> monomorphization
        // so that only this one monomorphization is instantiated (instead of one closure monormization for each T).
        let type_object = *self
            .value
            .get_or_init::<fn() -> *mut ffi::PyTypeObject>(py, inner::<T>);
        self.ensure_init(py, type_object, T::NAME, T::items_iter());
        type_object
    }

    fn ensure_init(
        &self,
        py: Python<'_>,
        type_object: *mut ffi::PyTypeObject,
        name: &str,
        items_iter: PyClassItemsIter,
    ) {
        // We might want to fill the `tp_dict` with python instances of `T`
        // itself. In order to do so, we must first initialize the type object
        // with an empty `tp_dict`: now we can create instances of `T`.
        //
        // Then we fill the `tp_dict`. Multiple threads may try to fill it at
        // the same time, but only one of them will succeed.
        //
        // More importantly, if a thread is performing initialization of the
        // `tp_dict`, it can still request the type object through `get_or_init`,
        // but the `tp_dict` may appear empty of course.

        if self.tp_dict_filled.get(py).is_some() {
            // `tp_dict` is already filled: ok.
            return;
        }

        let thread_id = thread::current().id();
        {
            let mut threads = self.initializing_threads.lock();
            if threads.contains(&thread_id) {
                // Reentrant call: just return the type object, even if the
                // `tp_dict` is not filled yet.
                return;
            }
            threads.push(thread_id);
        }

        struct InitializationGuard<'a> {
            initializing_threads: &'a Mutex<Vec<ThreadId>>,
            thread_id: ThreadId,
        }
        impl Drop for InitializationGuard<'_> {
            fn drop(&mut self) {
                let mut threads = self.initializing_threads.lock();
                threads.retain(|id| *id != self.thread_id);
            }
        }

        let guard = InitializationGuard {
            initializing_threads: &self.initializing_threads,
            thread_id,
        };

        // Pre-compute the class attribute objects: this can temporarily
        // release the GIL since we're calling into arbitrary user code. It
        // means that another thread can continue the initialization in the
        // meantime: at worst, we'll just make a useless computation.
        let mut items = vec![];
        for class_items in items_iter {
            for def in class_items.methods {
                if let PyMethodDefType::ClassAttribute(attr) = def {
                    let key = extract_cstr_or_leak_cstring(
                        attr.name,
                        "class attribute name cannot contain nul bytes",
                    )
                    .unwrap();

                    match (attr.meth.0)(py) {
                        Ok(val) => items.push((key, val)),
                        Err(e) => panic!(
                            "An error occurred while initializing `{}.{}`: {}",
                            name,
                            attr.name.trim_end_matches('\0'),
                            e
                        ),
                    }
                }
            }
        }

        // Now we hold the GIL and we can assume it won't be released until we
        // return from the function.
        let result = self.tp_dict_filled.get_or_init(py, move || {
            let result = initialize_tp_dict(py, type_object as *mut ffi::PyObject, items);

            // Initialization successfully complete, can clear the thread list.
            // (No further calls to get_or_init() will try to init, on any thread.)
            std::mem::forget(guard);
            *self.initializing_threads.lock() = Vec::new();
            result
        });

        if let Err(err) = result {
            err.clone_ref(py).print(py);
            panic!("An error occurred while initializing `{}.__dict__`", name);
        }
    }
}

fn initialize_tp_dict(
    py: Python<'_>,
    type_object: *mut ffi::PyObject,
    items: Vec<(&'static std::ffi::CStr, PyObject)>,
) -> PyResult<()> {
    // We hold the GIL: the dictionary update can be considered atomic from
    // the POV of other threads.
    for (key, val) in items {
        let ret = unsafe { ffi::PyObject_SetAttrString(type_object, key.as_ptr(), val.into_ptr()) };
        crate::err::error_on_minusone(py, ret)?;
    }
    Ok(())
}

// This is necessary for making static `LazyStaticType`s
unsafe impl Sync for LazyStaticType {}

#[inline]
pub(crate) unsafe fn get_tp_alloc(tp: *mut ffi::PyTypeObject) -> Option<ffi::allocfunc> {
    #[cfg(not(Py_LIMITED_API))]
    {
        (*tp).tp_alloc
    }

    #[cfg(Py_LIMITED_API)]
    {
        let ptr = ffi::PyType_GetSlot(tp, ffi::Py_tp_alloc);
        std::mem::transmute(ptr)
    }
}

#[inline]
pub(crate) unsafe fn get_tp_free(tp: *mut ffi::PyTypeObject) -> ffi::freefunc {
    #[cfg(not(Py_LIMITED_API))]
    {
        (*tp).tp_free.unwrap()
    }

    #[cfg(Py_LIMITED_API)]
    {
        let ptr = ffi::PyType_GetSlot(tp, ffi::Py_tp_free);
        debug_assert_ne!(ptr, std::ptr::null_mut());
        std::mem::transmute(ptr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_type_object() {
        // Even though PyTypeObject is deprecated, simple usages of it as a trait bound should continue to work.
        use super::PyTypeObject;
        use crate::types::{PyList, PyType};
        use crate::Python;

        fn get_type_object<T: PyTypeObject>(py: Python<'_>) -> &PyType {
            T::type_object(py)
        }

        Python::with_gil(|py| {
            assert!(get_type_object::<PyList>(py).is(<PyList as crate::PyTypeInfo>::type_object(py)))
        });
    }
}
