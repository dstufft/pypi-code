use crate::ffi::{Py_ssize_t, PY_SSIZE_T_MAX};
use std::ffi::{CStr, CString};

pub struct PrivateMarker;

macro_rules! private_decl {
    () => {
        /// This trait is private to implement; this method exists to make it
        /// impossible to implement outside the crate.
        fn __private__(&self) -> crate::internal_tricks::PrivateMarker;
    };
}

macro_rules! private_impl {
    () => {
        fn __private__(&self) -> crate::internal_tricks::PrivateMarker {
            crate::internal_tricks::PrivateMarker
        }
    };
}

macro_rules! pyo3_exception {
    ($doc: expr, $name: ident, $base: ty) => {
        #[doc = $doc]
        #[repr(transparent)]
        #[allow(non_camel_case_types)]
        pub struct $name($crate::PyAny);

        $crate::impl_exception_boilerplate!($name);

        $crate::create_exception_type_object!(pyo3_runtime, $name, $base, Some($doc));
    };
}

#[derive(Debug)]
pub(crate) struct NulByteInString(pub(crate) &'static str);

pub(crate) fn extract_cstr_or_leak_cstring(
    src: &'static str,
    err_msg: &'static str,
) -> Result<&'static CStr, NulByteInString> {
    CStr::from_bytes_with_nul(src.as_bytes())
        .or_else(|_| {
            CString::new(src.as_bytes()).map(|c_string| &*Box::leak(c_string.into_boxed_c_str()))
        })
        .map_err(|_| NulByteInString(err_msg))
}

/// Convert an usize index into a Py_ssize_t index, clamping overflow to
/// PY_SSIZE_T_MAX.
pub(crate) fn get_ssize_index(index: usize) -> Py_ssize_t {
    index.min(PY_SSIZE_T_MAX as usize) as Py_ssize_t
}

/// Implementations used for slice indexing PySequence, PyTuple, and PyList
macro_rules! index_impls {
    (
        $ty:ty,
        $ty_name:literal,
        $len:expr,
        $get_slice:expr $(,)?
    ) => {
        impl std::ops::Index<usize> for $ty {
            // Always PyAny output (even if the slice operation returns something else)
            type Output = PyAny;

            #[track_caller]
            fn index(&self, index: usize) -> &Self::Output {
                self.get_item(index).unwrap_or_else(|_| {
                    crate::internal_tricks::index_len_fail(index, $ty_name, $len(self))
                })
            }
        }

        impl std::ops::Index<std::ops::Range<usize>> for $ty {
            type Output = $ty;

            #[track_caller]
            fn index(
                &self,
                std::ops::Range { start, end }: std::ops::Range<usize>,
            ) -> &Self::Output {
                let len = $len(self);
                if start > len {
                    crate::internal_tricks::slice_start_index_len_fail(start, $ty_name, len)
                } else if end > len {
                    crate::internal_tricks::slice_end_index_len_fail(end, $ty_name, len)
                } else if start > end {
                    crate::internal_tricks::slice_index_order_fail(start, end)
                } else {
                    $get_slice(self, start, end)
                }
            }
        }

        impl std::ops::Index<std::ops::RangeFrom<usize>> for $ty {
            type Output = $ty;

            #[track_caller]
            fn index(
                &self,
                std::ops::RangeFrom { start }: std::ops::RangeFrom<usize>,
            ) -> &Self::Output {
                let len = $len(self);
                if start > len {
                    crate::internal_tricks::slice_start_index_len_fail(start, $ty_name, len)
                } else {
                    $get_slice(self, start, len)
                }
            }
        }

        impl std::ops::Index<std::ops::RangeFull> for $ty {
            type Output = $ty;

            #[track_caller]
            fn index(&self, _: std::ops::RangeFull) -> &Self::Output {
                let len = $len(self);
                $get_slice(self, 0, len)
            }
        }

        impl std::ops::Index<std::ops::RangeInclusive<usize>> for $ty {
            type Output = $ty;

            #[track_caller]
            fn index(&self, range: std::ops::RangeInclusive<usize>) -> &Self::Output {
                let exclusive_end = range
                    .end()
                    .checked_add(1)
                    .expect("range end exceeds Python limit");
                &self[*range.start()..exclusive_end]
            }
        }

        impl std::ops::Index<std::ops::RangeTo<usize>> for $ty {
            type Output = $ty;

            #[track_caller]
            fn index(&self, std::ops::RangeTo { end }: std::ops::RangeTo<usize>) -> &Self::Output {
                &self[0..end]
            }
        }

        impl std::ops::Index<std::ops::RangeToInclusive<usize>> for $ty {
            type Output = $ty;

            #[track_caller]
            fn index(
                &self,
                std::ops::RangeToInclusive { end }: std::ops::RangeToInclusive<usize>,
            ) -> &Self::Output {
                &self[0..=end]
            }
        }
    };
}

// these error messages are shamelessly "borrowed" from std.

#[inline(never)]
#[cold]
#[track_caller]
pub(crate) fn index_len_fail(index: usize, ty_name: &str, len: usize) -> ! {
    panic!(
        "index {} out of range for {} of length {}",
        index, ty_name, len
    );
}

#[inline(never)]
#[cold]
#[track_caller]
pub(crate) fn slice_start_index_len_fail(index: usize, ty_name: &str, len: usize) -> ! {
    panic!(
        "range start index {} out of range for {} of length {}",
        index, ty_name, len
    );
}

#[inline(never)]
#[cold]
#[track_caller]
pub(crate) fn slice_end_index_len_fail(index: usize, ty_name: &str, len: usize) -> ! {
    panic!(
        "range end index {} out of range for {} of length {}",
        index, ty_name, len
    );
}

#[inline(never)]
#[cold]
#[track_caller]
pub(crate) fn slice_index_order_fail(index: usize, end: usize) -> ! {
    panic!("slice index starts at {} but ends at {}", index, end);
}
