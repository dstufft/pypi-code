load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

filegroup(
    name = "srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

configure_make(
    name = "util-linux",
    configure_in_place = True,
    configure_options = [
        # We only need a few things from util-linux, so we'll disable everything
        # by default.
        "--disable-all-programs",
        # Turn on the things we actually want
        "--enable-libuuid",
    ],
    lib_source = ":srcs",
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": [
            "libuuid.so",
            "libuuid.so.1",
            "libuuid.so.1.3.0",
        ],
    }),
    out_static_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": [
            "libuuid.a",
        ],
    }),
    visibility = ["//visibility:public"],
    # deps = ["@zlib"],
)
