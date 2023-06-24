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
        sha256 = "a993af1aaee3b6ac72d03b6815dd3f60d291c4a585a0c2db92fcdc02b2dc7893",
        strip_prefix = "ncurses-6.4-20230617",
        urls = [
            "https://invisible-mirror.net/archives/ncurses/current/ncurses-6.4-20230617.tgz",
        ],
    )

deps = module_extension(implementation = _deps)
