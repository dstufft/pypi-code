"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def _deps(_ctx):
    http_archive(
        name = "upstream",
        build_file_content = """
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)
        """,
        sha256 = "f0925a5adf4b1bf116ee19766b7daa766917aec198747943b1c4edf67a4be2bb",
        strip_prefix = "libedit-20221030-3.1",
        urls = [
            "https://www.thrysoee.dk/editline/libedit-20221030-3.1.tar.gz",
        ],
    )

deps = module_extension(implementation = _deps)
