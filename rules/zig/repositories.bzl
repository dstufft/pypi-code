"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def repositories():
    http_archive(
        name = "hermetic_cc_toolchain",
        sha256 = "57f03a6c29793e8add7bd64186fc8066d23b5ffd06fe9cc6b0b8c499914d3a65",
        urls = [
            "https://mirror.bazel.build/github.com/uber/hermetic_cc_toolchain/releases/download/v2.0.0/hermetic_cc_toolchain-v2.0.0.tar.gz",
            "https://github.com/uber/hermetic_cc_toolchain/releases/download/v2.0.0/hermetic_cc_toolchain-v2.0.0.tar.gz",
        ],
    )