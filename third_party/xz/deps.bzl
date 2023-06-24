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
        sha256 = "92177bef62c3824b4badc524f8abcce54a20b7dbcfb84cde0a2eb8b49159518c",
        strip_prefix = "xz-5.4.3",
        urls = [
            "https://github.com/tukaani-project/xz/releases/download/v5.4.3/xz-5.4.3.tar.xz",
        ],
    )

deps = module_extension(implementation = _deps)
