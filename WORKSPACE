# TODO: We should move rules_py into bzlmod, but that depends on rules_rust
#       becoming compatible with bzlmod first.
local_repository(
    name = "rules_py",
    path = "repos/rules_py",
)

load("@rules_py//python:repositories.bzl", "rules_py_dependencies")

rules_py_dependencies()

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    # edition = "2021",
    # NOTE: Cannot upgrade to 1.70.0 until https://github.com/uber/hermetic_cc_toolchain/issues/103
    #       is fixed.
    # TODO: File an upstream issue with zig, as this isn't really an issue with
    #       uber/hermetic_cc_toolchain.
    versions = ["1.69.0"],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(
    rust_version = "1.69.0",
)
