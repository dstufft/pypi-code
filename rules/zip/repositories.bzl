"""
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

def repositories():
    http_file(
        name = "zip-x86_64-unknown-linux",
        sha256 = "5e0f3779c211718e7ce73badea42ef1a2af9735b3d4a9473552d3b9db6acb2bd",
        urls = [
            "https://github.com/timo-reymann/deterministic-zip/releases/download/2.1.0/deterministic-zip_linux-amd64",
        ],
        executable = True,
    )

    http_file(
        name = "zip-x86_64-apple-darwin",
        sha256 = "69a3bf22434ce4cb2f4f3b6b0314045e29bc954d18084a461dec0ae4612e421d",
        urls = [
            "https://github.com/timo-reymann/deterministic-zip/releases/download/2.1.0/deterministic-zip_darwin-amd64",
        ],
        executable = True,
    )

    http_file(
        name = "zip-arm64-apple-darwin",
        sha256 = "a07da2af885e82552b2529bd82f06431e24f9c7a5357de2ef2404959546e6346",
        urls = [
            "https://github.com/timo-reymann/deterministic-zip/releases/download/2.1.0/deterministic-zip_darwin-arm64",
        ],
        executable = True,
    )
