`#[pyclass]` can be used with the following parameters:

|  Parameter  |  Description |
| :-  | :- |
| <span style="white-space: pre">`crate = "some::path"`</span>  | Path to import the `pyo3` crate, if it's not accessible at `::pyo3`. |
| `dict` | Gives instances of this class an empty `__dict__` to store custom attributes. |
| <span style="white-space: pre">`extends = BaseType`</span>  | Use a custom baseclass. Defaults to [`PyAny`][params-1] |
| <span style="white-space: pre">`freelist = N`</span> |  Implements a [free list][params-2] of size N. This can improve performance for types that are often created and deleted in quick succession. Profile your code to see whether `freelist` is right for you.  |
| <span style="white-space: pre">`frozen`</span> | Declares that your pyclass is immutable. It removes the borrowchecker overhead when retrieving a shared reference to the Rust struct, but disables the ability to get a mutable reference. |
| `mapping` |  Inform PyO3 that this class is a [`Mapping`][params-mapping], and so leave its implementation of sequence C-API slots empty. |
| <span style="white-space: pre">`module = "module_name"`</span> |  Python code will see the class as being defined in this module. Defaults to `builtins`. |
| <span style="white-space: pre">`name = "python_name"`</span> | Sets the name that Python sees this class as. Defaults to the name of the Rust struct. |
| `sequence` |  Inform PyO3 that this class is a [`Sequence`][params-sequence], and so leave its C-API mapping length slot empty. |
| `subclass` | Allows other Python classes and `#[pyclass]` to inherit from this class. Enums cannot be subclassed. |
| <span style="white-space: pre">`text_signature = "(arg1, arg2, ...)"`</span> |  Sets the text signature for the Python class' `__new__` method. |
| `unsendable` | Required if your struct is not [`Send`][params-3]. Rather than using `unsendable`, consider implementing your struct in a threadsafe way by e.g. substituting [`Rc`][params-4] with [`Arc`][params-5]. By using `unsendable`, your class will panic when accessed by another thread.|
| `weakref` | Allows this class to be [weakly referenceable][params-6]. |

All of these parameters can either be passed directly on the `#[pyclass(...)]` annotation, or as one or
more accompanying `#[pyo3(...)]` annotations, e.g.:

```rust,ignore
// Argument supplied directly to the `#[pyclass]` annotation.
#[pyclass(name = "SomeName", subclass)]
struct MyClass { }

// Argument supplied as a separate annotation.
#[pyclass]
#[pyo3(name = "SomeName", subclass)]
struct MyClass { }
```

[params-1]: https://docs.rs/pyo3/latest/pyo3/struct.PyAny.html
[params-2]: https://en.wikipedia.org/wiki/Free_list
[params-3]: https://doc.rust-lang.org/std/marker/trait.Send.html
[params-4]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[params-5]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[params-6]: https://docs.python.org/3/library/weakref.html
[params-mapping]: https://pyo3.rs/latest/class/protocols.html#mapping--sequence-types
[params-sequence]: https://pyo3.rs/latest/class/protocols.html#mapping--sequence-types
