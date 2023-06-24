load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "libuuid",
    configure_in_place = True,
    configure_options = [
        # We only need a few things from util-linux, so we'll disable everything
        # by default.
        "--disable-all-programs",
        # Turn on the things we actually want
        "--enable-libuuid",
    ],
    lib_source = "@upstream//:srcs",
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": [
            "libuuid.so",
            "libuuid.so.1",
            "libuuid.so.1.3.0",
        ],
    }),
    visibility = ["//visibility:public"],
)
