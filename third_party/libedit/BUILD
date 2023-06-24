load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "libedit",
    configure_in_place = True,
    configure_options = [
        "--disable-docs",
    ],
    lib_source = "@upstream//:srcs",
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": ["libedit.so"],
    }),
    visibility = ["//visibility:public"],
    deps = ["@ncurses"],
)
