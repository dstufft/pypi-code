load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")
load("@rules_python//python:defs.bzl", "py_runtime", "py_runtime_pair")

filegroup(
    name = "srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

# TODO: LTO?
# TODO: MSAN?
# TODO: UBSAN?
# TODO: --pydebug?
# TODO: OpenSSL?

configure_make(
    name = "python",
    configure_in_place = True,
    configure_options = [
        # Python contains a macro __DATE__ which would cause the build to be not
        # reproducible bazel replace these macro with 'redacted'.
        # Somehow, it would have been remplaced in this library without quote,
        # so we'll add another macro to add the quotes.
        "CFLAGS='-Dredacted=\"redacted\"'",
        # TODO: Bring our own pip?
        "--with-ensurepip=install",
        # We don't want our Python static linking
        "--enable-shared",
        "--without-static-libpython",
    ] + select({
        ":optimized": ["--enable-optimizations"],
        "//conditions:default": [],
    }),
    env = {
        "LDFLAGS_NODIST": "-Wl,-rpath,'\\$$\\$$ORIGIN/../lib/'",
    },
    # rules_foreign_cc defaults the install_prefix to "python". This conflicts
    # with the "python" executable that is generated.
    install_prefix = "py_install",
    lib_source = ":srcs",
    out_binaries = [
        "python3",
    ],
    out_data_dirs = ["lib"],
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": [
            "libpython3.so",
            "libpython3.11.so",
            "libpython3.11.so.1.0",
        ],
    }),
    deps = [
        "@libffi",
        "@util-linux",
        "@xz",
        "@zlib",
    ],
)

config_setting(
    name = "optimized",
    values = {"compilation_mode": "opt"},
)

filegroup(
    name = "interpreter",
    srcs = [":python"],
    output_group = "python3",
)

py_runtime(
    name = "py3_runtime",
    files = [":python"],
    interpreter = ":interpreter",
    python_version = "PY3",
)

py_runtime_pair(
    name = "py_runtime",
    py2_runtime = None,
    py3_runtime = ":py3_runtime",
)

toolchain(
    name = "python_toolchain",
    toolchain = ":py_runtime",
    toolchain_type = "@bazel_tools//tools/python:toolchain_type",
)
