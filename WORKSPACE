load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "hermetic_cc_toolchain",
    sha256 = "57f03a6c29793e8add7bd64186fc8066d23b5ffd06fe9cc6b0b8c499914d3a65",
    urls = [
        "https://mirror.bazel.build/github.com/uber/hermetic_cc_toolchain/releases/download/v2.0.0/hermetic_cc_toolchain-v2.0.0.tar.gz",
        "https://github.com/uber/hermetic_cc_toolchain/releases/download/v2.0.0/hermetic_cc_toolchain-v2.0.0.tar.gz",
    ],
)

load("@hermetic_cc_toolchain//toolchain:defs.bzl", zig_toolchains = "toolchains")

zig_toolchains()

register_toolchains(
    "@zig_sdk//toolchain:linux_amd64_gnu.2.28",
    "@zig_sdk//toolchain:linux_arm64_gnu.2.28",
    "@zig_sdk//toolchain:darwin_amd64",
    "@zig_sdk//toolchain:darwin_arm64",
    "@zig_sdk//toolchain:windows_amd64",
    "@zig_sdk//toolchain:windows_arm64",
)

http_archive(
    name = "rules_python",
    sha256 = "84aec9e21cc56fbc7f1335035a71c850d1b9b5cc6ff497306f84cced9a769841",
    strip_prefix = "rules_python-0.23.1",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.23.1/rules_python-0.23.1.tar.gz",
)

load("//rules/zip:repositories.bzl", zip_repositories = "repositories")
load("//:third_party/util-linux/repositories.bzl", util_linux_repositories = "repositories")

zip_repositories()

util_linux_repositories()

load("//rules/zip:toolchains.bzl", zip_toolchains = "toolchains")

zig_toolchains()

zip_toolchains()

load("@rules_python//python:repositories.bzl", rules_python_dependencies = "py_repositories")

rules_python_dependencies()
