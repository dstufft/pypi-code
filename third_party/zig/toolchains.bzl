"""
"""

load("@hermetic_cc_toolchain//toolchain:defs.bzl", zig_toolchains = "toolchains")

def toolchains():
    zig_toolchains()
