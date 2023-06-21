load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

filegroup(
    name = "srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

configure_make(
    name = "libffi",
    configure_in_place = True,
    configure_options = [
        # We don't need the documentation
        "--disable-docs",
    ],
    lib_source = ":srcs",
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": [
            "libffi.so",
            "libffi.so.8",
            "libffi.so.8.1.2",
        ],
    }),
    visibility = ["//visibility:public"],
)
