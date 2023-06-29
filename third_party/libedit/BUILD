load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "libedit",
    configure_in_place = True,
    configure_options = [
        "--disable-docs",
        # We don't need or want the shared library.
        "--disable-shared",
    ],
    lib_source = "@upstream//:srcs",
    visibility = ["//visibility:public"],
    deps = ["@ncurses"],
)
