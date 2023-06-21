load("@rules_foreign_cc//foreign_cc:defs.bzl", "cmake")

filegroup(
    name = "srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

cmake(
    name = "zlib",
    cache_entries = select({
        "@platforms//os:linux": {
            "CMAKE_C_FLAGS": "${CMAKE_C_FLAGS:-} -fPIC",
        },
        "//conditions:default": {},
    }),
    generate_args = select({
        "@platforms//os:windows": ["-GNinja"],
        "//conditions:default": [],
    }),
    lib_source = ":srcs",
    out_shared_libs = select({
        "@platforms//os:windows": ["z.dll"],
        "//conditions:default": [
            "libz.so",
            "libz.so.1",
            "libz.so.1.2.13",
        ],
    }),
    out_static_libs = select({
        "@platforms//os:windows": ["z.lib"],
        "//conditions:default": ["libz.a"],
    }),
    visibility = ["//visibility:public"],
)
