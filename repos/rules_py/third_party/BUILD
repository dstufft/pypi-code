load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_vendor")

# TODO: Figure out our locking strategy and mode, are we going to use a lockfile?
#       local? remote? what?
# TODO: Using remote is generating the wrong paths for some reason, figure out why.
crates_vendor(
    name = "crates",
    annotations = {
        "pyo3-build-config": [crate.annotation(
            build_script_data = [
                "@rules_py//python/private:pyo3-build-config.txt",
            ],
            build_script_env = {
                "PYO3_CONFIG_FILE": "$(execpath @rules_py//python/private:pyo3-build-config.txt)",
            },
        )],
    },
    mode = "local",
    packages = {
        "pyembed": crate.spec(version = "0.24.0"),
        "python-packed-resources": crate.spec(version = "0.12.0"),
    },
    repository_name = "rules_py_crates",
    tags = ["manual"],
    vendor_path = "crates",
)
