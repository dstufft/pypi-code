use crate::object;
use crate::{PyObject, Py_ssize_t};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

// skipped _Py_NewReference
// skipped _Py_ForgetReference
// skipped _Py_GetRefTotal

// skipped _Py_Identifier

// skipped _Py_static_string_init
// skipped _Py_static_string
// skipped _Py_IDENTIFIER

#[cfg(not(Py_3_11))] // moved to src/buffer.rs from Python
mod bufferinfo {
    use crate::Py_ssize_t;
    use std::os::raw::{c_char, c_int, c_void};
    use std::ptr;

    #[cfg(PyPy)]
    const Py_MAX_NDIMS: usize = 36;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Py_buffer {
        pub buf: *mut c_void,
        /// Owned reference
        pub obj: *mut crate::PyObject,
        pub len: Py_ssize_t,
        pub itemsize: Py_ssize_t,
        pub readonly: c_int,
        pub ndim: c_int,
        pub format: *mut c_char,
        pub shape: *mut Py_ssize_t,
        pub strides: *mut Py_ssize_t,
        pub suboffsets: *mut Py_ssize_t,
        pub internal: *mut c_void,
        #[cfg(PyPy)]
        pub flags: c_int,
        #[cfg(PyPy)]
        pub _strides: [Py_ssize_t; Py_MAX_NDIMS],
        #[cfg(PyPy)]
        pub _shape: [Py_ssize_t; Py_MAX_NDIMS],
    }

    impl Py_buffer {
        pub const fn new() -> Self {
            Py_buffer {
                buf: ptr::null_mut(),
                obj: ptr::null_mut(),
                len: 0,
                itemsize: 0,
                readonly: 0,
                ndim: 0,
                format: ptr::null_mut(),
                shape: ptr::null_mut(),
                strides: ptr::null_mut(),
                suboffsets: ptr::null_mut(),
                internal: ptr::null_mut(),
                #[cfg(PyPy)]
                flags: 0,
                #[cfg(PyPy)]
                _strides: [0; Py_MAX_NDIMS],
                #[cfg(PyPy)]
                _shape: [0; Py_MAX_NDIMS],
            }
        }
    }

    pub type getbufferproc = unsafe extern "C" fn(
        arg1: *mut crate::PyObject,
        arg2: *mut Py_buffer,
        arg3: c_int,
    ) -> c_int;
    pub type releasebufferproc =
        unsafe extern "C" fn(arg1: *mut crate::PyObject, arg2: *mut Py_buffer);

    /// Maximum number of dimensions
    pub const PyBUF_MAX_NDIM: c_int = 64;

    /* Flags for getting buffers */
    pub const PyBUF_SIMPLE: c_int = 0;
    pub const PyBUF_WRITABLE: c_int = 0x0001;
    /* we used to include an E, backwards compatible alias */
    pub const PyBUF_WRITEABLE: c_int = PyBUF_WRITABLE;
    pub const PyBUF_FORMAT: c_int = 0x0004;
    pub const PyBUF_ND: c_int = 0x0008;
    pub const PyBUF_STRIDES: c_int = 0x0010 | PyBUF_ND;
    pub const PyBUF_C_CONTIGUOUS: c_int = 0x0020 | PyBUF_STRIDES;
    pub const PyBUF_F_CONTIGUOUS: c_int = 0x0040 | PyBUF_STRIDES;
    pub const PyBUF_ANY_CONTIGUOUS: c_int = 0x0080 | PyBUF_STRIDES;
    pub const PyBUF_INDIRECT: c_int = 0x0100 | PyBUF_STRIDES;

    pub const PyBUF_CONTIG: c_int = PyBUF_ND | PyBUF_WRITABLE;
    pub const PyBUF_CONTIG_RO: c_int = PyBUF_ND;

    pub const PyBUF_STRIDED: c_int = PyBUF_STRIDES | PyBUF_WRITABLE;
    pub const PyBUF_STRIDED_RO: c_int = PyBUF_STRIDES;

    pub const PyBUF_RECORDS: c_int = PyBUF_STRIDES | PyBUF_WRITABLE | PyBUF_FORMAT;
    pub const PyBUF_RECORDS_RO: c_int = PyBUF_STRIDES | PyBUF_FORMAT;

    pub const PyBUF_FULL: c_int = PyBUF_INDIRECT | PyBUF_WRITABLE | PyBUF_FORMAT;
    pub const PyBUF_FULL_RO: c_int = PyBUF_INDIRECT | PyBUF_FORMAT;

    pub const PyBUF_READ: c_int = 0x100;
    pub const PyBUF_WRITE: c_int = 0x200;
}

#[cfg(not(Py_3_11))]
pub use self::bufferinfo::*;

#[cfg(Py_3_8)]
pub type vectorcallfunc = unsafe extern "C" fn(
    callable: *mut PyObject,
    args: *const *mut PyObject,
    nargsf: libc::size_t,
    kwnames: *mut PyObject,
) -> *mut PyObject;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyNumberMethods {
    pub nb_add: Option<object::binaryfunc>,
    pub nb_subtract: Option<object::binaryfunc>,
    pub nb_multiply: Option<object::binaryfunc>,
    pub nb_remainder: Option<object::binaryfunc>,
    pub nb_divmod: Option<object::binaryfunc>,
    pub nb_power: Option<object::ternaryfunc>,
    pub nb_negative: Option<object::unaryfunc>,
    pub nb_positive: Option<object::unaryfunc>,
    pub nb_absolute: Option<object::unaryfunc>,
    pub nb_bool: Option<object::inquiry>,
    pub nb_invert: Option<object::unaryfunc>,
    pub nb_lshift: Option<object::binaryfunc>,
    pub nb_rshift: Option<object::binaryfunc>,
    pub nb_and: Option<object::binaryfunc>,
    pub nb_xor: Option<object::binaryfunc>,
    pub nb_or: Option<object::binaryfunc>,
    pub nb_int: Option<object::unaryfunc>,
    pub nb_reserved: *mut c_void,
    pub nb_float: Option<object::unaryfunc>,
    pub nb_inplace_add: Option<object::binaryfunc>,
    pub nb_inplace_subtract: Option<object::binaryfunc>,
    pub nb_inplace_multiply: Option<object::binaryfunc>,
    pub nb_inplace_remainder: Option<object::binaryfunc>,
    pub nb_inplace_power: Option<object::ternaryfunc>,
    pub nb_inplace_lshift: Option<object::binaryfunc>,
    pub nb_inplace_rshift: Option<object::binaryfunc>,
    pub nb_inplace_and: Option<object::binaryfunc>,
    pub nb_inplace_xor: Option<object::binaryfunc>,
    pub nb_inplace_or: Option<object::binaryfunc>,
    pub nb_floor_divide: Option<object::binaryfunc>,
    pub nb_true_divide: Option<object::binaryfunc>,
    pub nb_inplace_floor_divide: Option<object::binaryfunc>,
    pub nb_inplace_true_divide: Option<object::binaryfunc>,
    pub nb_index: Option<object::unaryfunc>,
    pub nb_matrix_multiply: Option<object::binaryfunc>,
    pub nb_inplace_matrix_multiply: Option<object::binaryfunc>,
}

#[repr(C)]
#[derive(Clone)]
pub struct PySequenceMethods {
    pub sq_length: Option<object::lenfunc>,
    pub sq_concat: Option<object::binaryfunc>,
    pub sq_repeat: Option<object::ssizeargfunc>,
    pub sq_item: Option<object::ssizeargfunc>,
    pub was_sq_slice: *mut c_void,
    pub sq_ass_item: Option<object::ssizeobjargproc>,
    pub was_sq_ass_slice: *mut c_void,
    pub sq_contains: Option<object::objobjproc>,
    pub sq_inplace_concat: Option<object::binaryfunc>,
    pub sq_inplace_repeat: Option<object::ssizeargfunc>,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct PyMappingMethods {
    pub mp_length: Option<object::lenfunc>,
    pub mp_subscript: Option<object::binaryfunc>,
    pub mp_ass_subscript: Option<object::objobjargproc>,
}

#[cfg(Py_3_10)]
pub type sendfunc = unsafe extern "C" fn(
    iter: *mut PyObject,
    value: *mut PyObject,
    result: *mut *mut PyObject,
) -> object::PySendResult;

#[repr(C)]
#[derive(Clone, Default)]
pub struct PyAsyncMethods {
    pub am_await: Option<object::unaryfunc>,
    pub am_aiter: Option<object::unaryfunc>,
    pub am_anext: Option<object::unaryfunc>,
    #[cfg(Py_3_10)]
    pub am_send: Option<sendfunc>,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct PyBufferProcs {
    pub bf_getbuffer: Option<crate::getbufferproc>,
    pub bf_releasebuffer: Option<crate::releasebufferproc>,
}

pub type printfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::libc::FILE, arg3: c_int) -> c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyTypeObject {
    #[cfg(all(PyPy, not(Py_3_9)))]
    pub ob_refcnt: Py_ssize_t,
    #[cfg(all(PyPy, not(Py_3_9)))]
    pub ob_pypy_link: Py_ssize_t,
    #[cfg(all(PyPy, not(Py_3_9)))]
    pub ob_type: *mut PyTypeObject,
    #[cfg(all(PyPy, not(Py_3_9)))]
    pub ob_size: Py_ssize_t,
    #[cfg(not(all(PyPy, not(Py_3_9))))]
    pub ob_base: object::PyVarObject,
    pub tp_name: *const c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: Option<object::destructor>,
    #[cfg(not(Py_3_8))]
    pub tp_print: Option<printfunc>,
    #[cfg(Py_3_8)]
    pub tp_vectorcall_offset: Py_ssize_t,
    pub tp_getattr: Option<object::getattrfunc>,
    pub tp_setattr: Option<object::setattrfunc>,
    pub tp_as_async: *mut PyAsyncMethods,
    pub tp_repr: Option<object::reprfunc>,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: Option<object::hashfunc>,
    pub tp_call: Option<object::ternaryfunc>,
    pub tp_str: Option<object::reprfunc>,
    pub tp_getattro: Option<object::getattrofunc>,
    pub tp_setattro: Option<object::setattrofunc>,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: c_ulong,
    pub tp_doc: *const c_char,
    pub tp_traverse: Option<object::traverseproc>,
    pub tp_clear: Option<object::inquiry>,
    pub tp_richcompare: Option<object::richcmpfunc>,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: Option<object::getiterfunc>,
    pub tp_iternext: Option<object::iternextfunc>,
    pub tp_methods: *mut crate::methodobject::PyMethodDef,
    pub tp_members: *mut crate::structmember::PyMemberDef,
    pub tp_getset: *mut crate::descrobject::PyGetSetDef,
    pub tp_base: *mut PyTypeObject,
    pub tp_dict: *mut object::PyObject,
    pub tp_descr_get: Option<object::descrgetfunc>,
    pub tp_descr_set: Option<object::descrsetfunc>,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: Option<object::initproc>,
    pub tp_alloc: Option<object::allocfunc>,
    pub tp_new: Option<object::newfunc>,
    pub tp_free: Option<object::freefunc>,
    pub tp_is_gc: Option<object::inquiry>,
    pub tp_bases: *mut object::PyObject,
    pub tp_mro: *mut object::PyObject,
    pub tp_cache: *mut object::PyObject,
    pub tp_subclasses: *mut object::PyObject,
    pub tp_weaklist: *mut object::PyObject,
    pub tp_del: Option<object::destructor>,
    pub tp_version_tag: c_uint,
    pub tp_finalize: Option<object::destructor>,
    #[cfg(Py_3_8)]
    pub tp_vectorcall: Option<super::vectorcallfunc>,
    #[cfg(any(all(PyPy, Py_3_8), all(not(PyPy), Py_3_8, not(Py_3_9))))]
    pub tp_print: Option<printfunc>,
    #[cfg(PyPy)]
    pub tp_pypy_flags: std::os::raw::c_long,
    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    pub tp_allocs: Py_ssize_t,
    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    pub tp_frees: Py_ssize_t,
    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    pub tp_maxalloc: Py_ssize_t,
    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    pub tp_prev: *mut PyTypeObject,
    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    pub tp_next: *mut PyTypeObject,
}

#[cfg(Py_3_11)]
#[repr(C)]
#[derive(Clone)]
pub struct _specialization_cache {
    pub getitem: *mut PyObject,
}

#[repr(C)]
#[derive(Clone)]
pub struct PyHeapTypeObject {
    pub ht_type: PyTypeObject,
    pub as_async: PyAsyncMethods,
    pub as_number: PyNumberMethods,
    pub as_mapping: PyMappingMethods,
    pub as_sequence: PySequenceMethods,
    pub as_buffer: PyBufferProcs,
    pub ht_name: *mut object::PyObject,
    pub ht_slots: *mut object::PyObject,
    pub ht_qualname: *mut object::PyObject,
    #[cfg(not(PyPy))]
    pub ht_cached_keys: *mut c_void,
    #[cfg(Py_3_9)]
    pub ht_module: *mut object::PyObject,
    #[cfg(Py_3_11)]
    pub _ht_tpname: *mut c_char,
    #[cfg(Py_3_11)]
    pub _spec_cache: _specialization_cache,
}

impl Default for PyHeapTypeObject {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[inline]
pub unsafe fn PyHeapType_GET_MEMBERS(
    etype: *mut PyHeapTypeObject,
) -> *mut crate::structmember::PyMemberDef {
    let py_type = object::Py_TYPE(etype as *mut object::PyObject);
    let ptr = etype.offset((*py_type).tp_basicsize);
    ptr as *mut crate::structmember::PyMemberDef
}

// skipped _PyType_Name
// skipped _PyType_Lookup
// skipped _PyType_LookupId
// skipped _PyObject_LookupSpecial
// skipped _PyType_CalculateMetaclass
// skipped _PyType_GetDocFromInternalDoc
// skipped _PyType_GetTextSignatureFromInternalDoc
// skipped _PyType_GetModuleByDef

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyObject_Print")]
    pub fn PyObject_Print(o: *mut PyObject, fp: *mut ::libc::FILE, flags: c_int) -> c_int;

    // skipped _Py_BreakPoint
    // skipped _PyObject_Dump
    // skipped _PyObject_IsFreed
    // skipped _PyObject_IsAbstract
    // skipped _PyObject_GetAttrId
    // skipped _PyObject_SetAttrId
    // skipped _PyObject_LookupAttr
    // skipped _PyObject_LookupAttrId
    // skipped _PyObject_GetMethod

    #[cfg(not(PyPy))]
    pub fn _PyObject_GetDictPtr(obj: *mut PyObject) -> *mut *mut PyObject;
    #[cfg(not(PyPy))]
    pub fn _PyObject_NextNotImplemented(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_CallFinalizer(arg1: *mut PyObject);
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallFinalizerFromDealloc")]
    pub fn PyObject_CallFinalizerFromDealloc(arg1: *mut PyObject) -> c_int;

    // skipped _PyObject_GenericGetAttrWithDict
    // skipped _PyObject_GenericSetAttrWithDict
    // skipped _PyObject_FunctionStr
}

// skipped Py_SETREF
// skipped Py_XSETREF

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut _PyNone_Type: PyTypeObject;
    pub static mut _PyNotImplemented_Type: PyTypeObject;
}

// skipped _Py_SwappedOp

// skipped _PyDebugAllocatorStats
// skipped _PyObject_DebugTypeStats
// skipped _PyObject_ASSERT_FROM
// skipped _PyObject_ASSERT_WITH_MSG
// skipped _PyObject_ASSERT
// skipped _PyObject_ASSERT_FAILED_MSG
// skipped _PyObject_AssertFailed
// skipped _PyObject_CheckConsistency

// skipped _PyTrash_thread_deposit_object
// skipped _PyTrash_thread_destroy_chain
// skipped _PyTrash_begin
// skipped _PyTrash_end
// skipped _PyTrash_cond
// skipped PyTrash_UNWIND_LEVEL
// skipped Py_TRASHCAN_BEGIN_CONDITION
// skipped Py_TRASHCAN_END
// skipped Py_TRASHCAN_BEGIN
// skipped Py_TRASHCAN_SAFE_BEGIN
// skipped Py_TRASHCAN_SAFE_END
