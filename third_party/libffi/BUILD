load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "libffi",
    configure_in_place = True,
    configure_options = [
        # We don't need the documentation
        "--disable-docs",
        # We don't need or want the shared library.
        "--disable-shared",
    ],
    install_prefix = "bazel",
    lib_source = "@upstream//:srcs",
    visibility = ["//visibility:public"],
)
