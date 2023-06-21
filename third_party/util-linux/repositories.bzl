"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repositories():
    http_archive(
        name = "util-linux",
        build_file = "//:third_party/util-linux/BUILD.util-linux.bazel",
        sha256 = "32b30a336cda903182ed61feb3e9b908b762a5e66fe14e43efb88d37162075cb",
        strip_prefix = "util-linux-2.39",
        urls = [
            "https://mirrors.edge.kernel.org/pub/linux/utils/util-linux/v2.39/util-linux-2.39.tar.xz",
        ],
    )
