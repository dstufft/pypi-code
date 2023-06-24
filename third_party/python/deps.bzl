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
            # Python used -Wl,-h but zig cc doesn't understand that, however, it
            # does understand -Wl,-soname, so we'll patch Python to use that
            # instead.
            "//:patches/soname.patch",
            # Python's setup.py implicitly adds some system directories to the
            # search path, which we do not want to do, so we'll patch them out.
            "//:patches/hermetic.patch",
        ],
        sha256 = "2f0e409df2ab57aa9fc4cbddfb976af44e4e55bf6f619eee6bc5c2297264a7f6",
        strip_prefix = "Python-3.11.4",
        url = "https://www.python.org/ftp/python/3.11.4/Python-3.11.4.tar.xz",
    )

deps = module_extension(implementation = _deps)
