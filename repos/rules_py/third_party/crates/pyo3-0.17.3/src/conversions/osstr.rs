use crate::types::PyString;
#[cfg(windows)]
use crate::PyErr;
use crate::{
    ffi, AsPyPointer, FromPyObject, IntoPy, PyAny, PyObject, PyResult, PyTryFrom, Python,
    ToPyObject,
};
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
#[cfg(not(windows))]
use std::os::raw::c_char;

impl ToPyObject for OsStr {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        // If the string is UTF-8, take the quick and easy shortcut
        if let Some(valid_utf8_path) = self.to_str() {
            return valid_utf8_path.to_object(py);
        }

        // All targets besides windows support the std::os::unix::ffi::OsStrExt API:
        // https://doc.rust-lang.org/src/std/sys_common/mod.rs.html#59
        #[cfg(not(windows))]
        {
            #[cfg(target_os = "wasi")]
            let bytes = std::os::wasi::ffi::OsStrExt::as_bytes(self);
            #[cfg(not(target_os = "wasi"))]
            let bytes = std::os::unix::ffi::OsStrExt::as_bytes(self);

            let ptr = bytes.as_ptr() as *const c_char;
            let len = bytes.len() as ffi::Py_ssize_t;
            unsafe {
                // DecodeFSDefault automatically chooses an appropriate decoding mechanism to
                // parse os strings losslessly (i.e. surrogateescape most of the time)
                let pystring = ffi::PyUnicode_DecodeFSDefaultAndSize(ptr, len);
                PyObject::from_owned_ptr(py, pystring)
            }
        }

        #[cfg(windows)]
        {
            let wstr: Vec<u16> = std::os::windows::ffi::OsStrExt::encode_wide(self).collect();

            unsafe {
                // This will not panic because the data from encode_wide is well-formed Windows
                // string data
                PyObject::from_owned_ptr(
                    py,
                    ffi::PyUnicode_FromWideChar(wstr.as_ptr(), wstr.len() as ffi::Py_ssize_t),
                )
            }
        }
    }
}

// There's no FromPyObject implementation for &OsStr because albeit possible on Unix, this would
// be impossible to implement on Windows. Hence it's omitted entirely

impl FromPyObject<'_> for OsString {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let pystring = <PyString as PyTryFrom>::try_from(ob)?; // Cast PyAny to PyString

        #[cfg(not(windows))]
        {
            // Decode from Python's lossless bytes string representation back into raw bytes
            let fs_encoded_bytes = unsafe {
                crate::Py::<crate::types::PyBytes>::from_owned_ptr(
                    ob.py(),
                    ffi::PyUnicode_EncodeFSDefault(pystring.as_ptr()),
                )
            };

            // Create an OsStr view into the raw bytes from Python
            #[cfg(target_os = "wasi")]
            let os_str: &OsStr = std::os::wasi::ffi::OsStrExt::from_bytes(
                fs_encoded_bytes.as_ref(ob.py()).as_bytes(),
            );
            #[cfg(not(target_os = "wasi"))]
            let os_str: &OsStr = std::os::unix::ffi::OsStrExt::from_bytes(
                fs_encoded_bytes.as_ref(ob.py()).as_bytes(),
            );

            Ok(os_str.to_os_string())
        }

        #[cfg(windows)]
        {
            // Take the quick and easy shortcut if UTF-8
            if let Ok(utf8_string) = pystring.to_str() {
                return Ok(utf8_string.to_owned().into());
            }

            // Get an owned allocated wide char buffer from PyString, which we have to deallocate
            // ourselves
            let size =
                unsafe { ffi::PyUnicode_AsWideChar(pystring.as_ptr(), std::ptr::null_mut(), 0) };
            if size == -1 {
                return Err(PyErr::fetch(ob.py()));
            }

            let mut buffer = vec![0; size as usize];
            let bytes_read =
                unsafe { ffi::PyUnicode_AsWideChar(pystring.as_ptr(), buffer.as_mut_ptr(), size) };
            assert_eq!(bytes_read, size);

            // Copy wide char buffer into OsString
            let os_string = std::os::windows::ffi::OsStringExt::from_wide(&buffer);

            Ok(os_string)
        }
    }
}

impl IntoPy<PyObject> for &'_ OsStr {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for Cow<'_, OsStr> {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        (self as &OsStr).to_object(py)
    }
}

impl IntoPy<PyObject> for Cow<'_, OsStr> {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl ToPyObject for OsString {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        (self as &OsStr).to_object(py)
    }
}

impl IntoPy<PyObject> for OsString {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl<'a> IntoPy<PyObject> for &'a OsString {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::PyString, IntoPy, PyObject, Python, ToPyObject};
    use std::fmt::Debug;
    use std::{
        borrow::Cow,
        ffi::{OsStr, OsString},
    };

    #[test]
    #[cfg(not(windows))]
    fn test_non_utf8_conversion() {
        Python::with_gil(|py| {
            #[cfg(not(target_os = "wasi"))]
            use std::os::unix::ffi::OsStrExt;
            #[cfg(target_os = "wasi")]
            use std::os::wasi::ffi::OsStrExt;

            // this is not valid UTF-8
            let payload = &[250, 251, 252, 253, 254, 255, 0, 255];
            let os_str = OsStr::from_bytes(payload);

            // do a roundtrip into Pythonland and back and compare
            let py_str: PyObject = os_str.into_py(py);
            let os_str_2: OsString = py_str.extract(py).unwrap();
            assert_eq!(os_str, os_str_2);
        });
    }

    #[test]
    fn test_topyobject_roundtrip() {
        Python::with_gil(|py| {
            fn test_roundtrip<T: ToPyObject + AsRef<OsStr> + Debug>(py: Python<'_>, obj: T) {
                let pyobject = obj.to_object(py);
                let pystring: &PyString = pyobject.extract(py).unwrap();
                assert_eq!(pystring.to_string_lossy(), obj.as_ref().to_string_lossy());
                let roundtripped_obj: OsString = pystring.extract().unwrap();
                assert_eq!(obj.as_ref(), roundtripped_obj.as_os_str());
            }
            let os_str = OsStr::new("Hello\0\n🐍");
            test_roundtrip::<&OsStr>(py, os_str);
            test_roundtrip::<Cow<'_, OsStr>>(py, Cow::Borrowed(os_str));
            test_roundtrip::<Cow<'_, OsStr>>(py, Cow::Owned(os_str.to_os_string()));
            test_roundtrip::<OsString>(py, os_str.to_os_string());
        });
    }

    #[test]
    fn test_intopy_roundtrip() {
        Python::with_gil(|py| {
            fn test_roundtrip<T: IntoPy<PyObject> + AsRef<OsStr> + Debug + Clone>(
                py: Python<'_>,
                obj: T,
            ) {
                let pyobject = obj.clone().into_py(py);
                let pystring: &PyString = pyobject.extract(py).unwrap();
                assert_eq!(pystring.to_string_lossy(), obj.as_ref().to_string_lossy());
                let roundtripped_obj: OsString = pystring.extract().unwrap();
                assert!(obj.as_ref() == roundtripped_obj.as_os_str());
            }
            let os_str = OsStr::new("Hello\0\n🐍");
            test_roundtrip::<&OsStr>(py, os_str);
            test_roundtrip::<OsString>(py, os_str.to_os_string());
            test_roundtrip::<&OsString>(py, &os_str.to_os_string());
        })
    }
}
