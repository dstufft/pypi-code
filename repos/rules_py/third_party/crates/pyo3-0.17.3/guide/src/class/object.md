# Basic object customization

Recall the `Number` class from the previous chapter:

```rust
use pyo3::prelude::*;

#[pyclass]
struct Number(i32);

#[pymethods]
impl Number {
    #[new]
    fn new(value: i32) -> Self {
        Self(value)
    }
}

#[pymodule]
fn my_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Number>()?;
    Ok(())
}
```

At this point Python code can import the module, access the class and create class instances - but
nothing else.

```python
from my_module import Number

n = Number(5)
print(n)
```

```text
<builtins.Number object at 0x000002B4D185D7D0>
```

### String representations

It can't even print an user-readable representation of itself! We can fix that by defining the
 `__repr__` and `__str__` methods inside a `#[pymethods]` block. We do this by accessing the value
 contained inside `Number`.

 ```rust
# use pyo3::prelude::*;
#
# #[pyclass]
# struct Number(i32);
#
#[pymethods]
impl Number {
    // For `__repr__` we want to return a string that Python code could use to recreate
    // the `Number`, like `Number(5)` for example.
    fn __repr__(&self) -> String {
        // We use the `format!` macro to create a string. Its first argument is a
        // format string, followed by any number of parameters which replace the
        // `{}`'s in the format string.
        //
        //                       👇 Tuple field access in Rust uses a dot
        format!("Number({})", self.0)
    }

    // `__str__` is generally used to create an "informal" representation, so we
    // just forward to `i32`'s `ToString` trait implementation to print a bare number.
    fn __str__(&self) -> String {
        self.0.to_string()
    }
}
```

### Hashing


Let's also implement hashing. We'll just hash the `i32`. For that we need a [`Hasher`]. The one
provided by `std` is [`DefaultHasher`], which uses the [SipHash] algorithm.

```rust
use std::collections::hash_map::DefaultHasher;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};

# use pyo3::prelude::*;
#
# #[pyclass]
# struct Number(i32);
#
#[pymethods]
impl Number {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}
```

> **Note**: When implementing `__hash__` and comparisons, it is important that the following property holds:
>
> ```text
> k1 == k2 -> hash(k1) == hash(k2)
> ```
>
> In other words, if two keys are equal, their hashes must also be equal. In addition you must take
> care that your classes' hash doesn't change during its lifetime. In this tutorial we do that by not
> letting Python code change our `Number` class. In other words, it is immutable.
>
> By default, all `#[pyclass]` types have a default hash implementation from Python.
> Types which should not be hashable can override this by setting `__hash__` to None.
> This is the same mechanism as for a pure-Python class. This is done like so:
>
> ```rust
> # use pyo3::prelude::*;
> #[pyclass]
> struct NotHashable { }
>
> #[pymethods]
> impl NotHashable {
>    #[classattr]
>    const __hash__: Option<Py<PyAny>> = None;
>}
> ```

### Comparisons

Unlike in Python, PyO3 does not provide the magic comparison methods you might expect like `__eq__`,
 `__lt__` and so on. Instead you have to implement all six operations at once with `__richcmp__`.
This method will be called with a value of `CompareOp` depending on the operation.

```rust
use pyo3::class::basic::CompareOp;

# use pyo3::prelude::*;
#
# #[pyclass]
# struct Number(i32);
#
#[pymethods]
impl Number {
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }
}
```

If you obtain the result by comparing two Rust values, as in this example, you
can take a shortcut using `CompareOp::matches`:

```rust
use pyo3::class::basic::CompareOp;

# use pyo3::prelude::*;
#
# #[pyclass]
# struct Number(i32);
#
#[pymethods]
impl Number {
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.0.cmp(&other.0))
    }
}
```

It checks that the `std::cmp::Ordering` obtained from Rust's `Ord` matches
the given `CompareOp`.

Alternatively, if you want to leave some operations unimplemented, you can
return `py.NotImplemented()` for some of the operations:


```rust
use pyo3::class::basic::CompareOp;

# use pyo3::prelude::*;
#
# #[pyclass]
# struct Number(i32);
#
#[pymethods]
impl Number {
    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}
```

### Truthyness

We'll consider `Number` to be `True` if it is nonzero:

```rust
# use pyo3::prelude::*;
#
# #[pyclass]
# struct Number(i32);
#
#[pymethods]
impl Number {
    fn __bool__(&self) -> bool {
        self.0 != 0
    }
}
```

### Final code

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use pyo3::prelude::*;
use pyo3::class::basic::CompareOp;

#[pyclass]
struct Number(i32);

#[pymethods]
impl Number {
    #[new]
    fn new(value: i32) -> Self {
        Self(value)
    }

    fn __repr__(&self) -> String {
        format!("Number({})", self.0)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __bool__(&self) -> bool {
        self.0 != 0
    }
}

#[pymodule]
fn my_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Number>()?;
    Ok(())
}
```

[`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[`Hasher`]: https://doc.rust-lang.org/std/hash/trait.Hasher.html
[`DefaultHasher`]: https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html
[SipHash]: https://en.wikipedia.org/wiki/SipHash
