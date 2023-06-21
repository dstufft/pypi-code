"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repositories():
    http_archive(
        name = "python",
        build_file = "//:third_party/python/python.BUILD",
        patch_args = ["-p1"],
        patches = [
            # Python used -Wl,-h but zig cc doesn't understand that, however, it
            # does understand -Wl,-soname, so we'll patch Python to use that
            # instead.
            "//:third_party/python/patches/soname.patch",
            # Python's setup.py implicitly adds some system directories to the
            # search path, which we do not want to do, so we'll patch them out.
            # "//:third_party/python/patches/hermetic.patch",
        ],
        sha256 = "2f0e409df2ab57aa9fc4cbddfb976af44e4e55bf6f619eee6bc5c2297264a7f6",
        strip_prefix = "Python-3.11.4",
        url = "https://www.python.org/ftp/python/3.11.4/Python-3.11.4.tar.xz",
    )

    http_archive(
        name = "rules_python",
        sha256 = "84aec9e21cc56fbc7f1335035a71c850d1b9b5cc6ff497306f84cced9a769841",
        strip_prefix = "rules_python-0.23.1",
        url = "https://github.com/bazelbuild/rules_python/releases/download/0.23.1/rules_python-0.23.1.tar.gz",
    )
