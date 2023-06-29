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
        patch_args = ["-p1"],
        patches = [
            # The Configure script for perl assumes /usr/bin/env perl, but we
            # need to provide it using $PERL.
            "//:patches/perl.patch",
        ],
        sha256 = "b3aa61334233b852b63ddb048df181177c2c659eb9d4376008118f9c08d07674",
        strip_prefix = "openssl-3.1.1",
        urls = [
            "https://www.openssl.org/source/openssl-3.1.1.tar.gz",
        ],
    )

deps = module_extension(implementation = _deps)
