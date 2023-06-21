load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

filegroup(
    name = "srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

configure_make(
    name = "xz",
    configure_in_place = True,
    configure_options = [
        # We don't need the command line tools built or installed
        "--disable-xz",
        "--disable-xzdec",
        "--disable-lzmadec",
        "--disable-lzmainfo",
        "--disable-lzma-links",
        "--disable-scripts",
        # We don't need the documentation
        "--disable-doc",
    ],
    lib_name = "liblzma",
    lib_source = ":srcs",
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": [
            "liblzma.so",
            "liblzma.so.5",
            "liblzma.so.5.4.3",
        ],
    }),
    visibility = ["//visibility:public"],
)
