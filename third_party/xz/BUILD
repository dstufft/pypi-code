load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

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
        # We don't need or want the shared library.
        "--disable-shared",
    ],
    lib_name = "liblzma",
    lib_source = "@upstream//:srcs",
    visibility = ["//visibility:public"],
)
