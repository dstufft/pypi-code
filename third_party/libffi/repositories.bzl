"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repositories():
    http_archive(
        name = "libffi",
        patch_args = ["-p1"],
        patches = [
            # https://github.com/libffi/libffi/issues/760
            # https://github.com/libffi/libffi/pull/764
            "//:third_party/libffi/patches/gh-764.patch",
        ],
        build_file = "//:third_party/libffi/libffi.BUILD",
        sha256 = "d66c56ad259a82cf2a9dfc408b32bf5da52371500b84745f7fb8b645712df676",
        strip_prefix = "libffi-3.4.4",
        urls = [
            "https://github.com/libffi/libffi/releases/download/v3.4.4/libffi-3.4.4.tar.gz",
        ],
    )
