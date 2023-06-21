"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repositories():
    http_archive(
        name = "xz",
        build_file = "//:third_party/xz/xz.BUILD",
        sha256 = "92177bef62c3824b4badc524f8abcce54a20b7dbcfb84cde0a2eb8b49159518c",
        strip_prefix = "xz-5.4.3",
        urls = [
            "https://github.com/tukaani-project/xz/releases/download/v5.4.3/xz-5.4.3.tar.xz",
        ],
    )
