load("@aspect_bazel_lib//lib:output_files.bzl", "output_files")
load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")
load("@rules_python//python:defs.bzl", "py_runtime", "py_runtime_pair")

config_setting(
    name = "optimized",
    values = {"compilation_mode": "opt"},
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
    ] + select({
        ":optimized": ["--enable-optimizations"],
        "//conditions:default": [],
    }),
    # env = {
    #     "LDFLAGS_NODIST": "-Wl,-rpath,'\\$$\\$$ORIGIN/../lib/'",
    # },
    # rules_foreign_cc defaults the install_prefix to "python". This conflicts
    # with the "python" executable that is generated.
    install_prefix = "bazel",
    lib_source = "@upstream//:srcs",
    out_binaries = [
        "python3.orig",
    ],
    out_data_dirs = ["lib"],
    out_static_libs = [],
    # This is a hack, because these rules don't produce Bazel targets that are
    # marked executable, so what we do is we move the produced binary to a new
    # name, then we have a genrule that copies it to the new name.
    postfix_script = """
mv $$INSTALLDIR$$/bin/python3 $$INSTALLDIR$$/bin/python3.orig
rm $$INSTALLDIR$$/lib/libpython*.a
""",
    deps = [
        # "@libffi",
        # "@util-linux",
        # "@xz",
        "@zlib",
    ],
)

filegroup(
    name = "interpreter_fg",
    srcs = [":python"],
    output_group = "python3.orig",
)

genrule(
    name = "interpreter",
    srcs = [":interpreter_fg"],
    outs = ["python/bin/python3"],
    cmd = "cp $(location :interpreter_fg) $@",
    executable = True,
    local = True,
    output_to_bindir = True,
)

output_files(
    name = "files",
    paths = [
        "../python~override/python/lib",
        "../python~override/python/include",
    ],
    target = ":python",
)

py_runtime(
    name = "python3_runtime",
    files = [":files"],
    interpreter = ":interpreter",
    python_version = "PY3",
)

py_runtime_pair(
    name = "python_runtimes",
    py2_runtime = None,
    py3_runtime = ":python3_runtime",
)

toolchain(
    name = "toolchain",
    toolchain = ":python_runtimes",
    toolchain_type = "@bazel_tools//tools/python:toolchain_type",
)
