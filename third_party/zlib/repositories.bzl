"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repositories():
    http_archive(
        name = "zlib",
        build_file = "//:third_party/zlib/BUILD.zlib.bazel",
        sha256 = "d14c38e313afc35a9a8760dadf26042f51ea0f5d154b0630a31da0540107fb98",
        strip_prefix = "zlib-1.2.13",
        urls = [
            "https://github.com/madler/zlib/releases/download/v1.2.13/zlib-1.2.13.tar.xz",
            "https://zlib.net/zlib-1.2.13.tar.xz",
        ],
    )
