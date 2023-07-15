load("//python/private:runtime.bzl", "pyo3_config", "unconfigured_python_runtime")

exports_files(["_pyo3-build-config.tmpl.txt"])

pyo3_config(
    name = "pyo3-build-config.txt",
    visibility = ["//visibility:public"],
)

unconfigured_python_runtime(name = "no-runtime")
