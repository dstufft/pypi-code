load("@rules_py//python:defs.bzl", "py_binary", "python_runtime")

# TODO: Move this into third_party/python
python_runtime(
    name = "runtime",
    interpreter = "@python//:interpreter",
    pointer_width = 64,
    static = True,
    version = "3.11",
    implementation = "CPython",
)

py_binary(
    name = "demo",
    runtime = ":runtime",
)
