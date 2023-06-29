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
        # We don't need or want the shared library.
        "--disable-shared",
    ],
    lib_source = "@upstream//:srcs",
    visibility = ["//visibility:public"],
)
