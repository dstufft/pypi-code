"""
"""

load("//rules/zip:toolchains.bzl", "ZipInfo")

def _zip_toolchain_impl(ctx):
    toolchain_info = platform_common.ToolchainInfo(
        zinfo = ZipInfo(
            zip_target = ctx.attr.zip_target,
        ),
    )
    return [toolchain_info]

zip_toolchain = rule(
    implementation = _zip_toolchain_impl,
    attrs = {
        "zip_target": attr.label(
            mandatory = True,
        ),
    },
)

_platforms = dict({
    "x86_64-unknown-linux": {
        "exec": [
            "@platforms//os:linux",
            "@platforms//cpu:x86_64",
        ],
    },
    "x86_64-apple-darwin": {
        "exec": [
            "@platforms//os:macos",
            "@platforms//cpu:x86_64",
        ],
    },
    "arm64-apple-darwin": {
        "exec": [
            "@platforms//os:macos",
            "@platforms//cpu:arm64",
        ],
    },
})

# buildifier: disable=unnamed-macro
def declare_toolchains():
    for trip in ["x86_64-unknown-linux", "x86_64-apple-darwin", "arm64-apple-darwin"]:
        zip_toolchain(
            name = "zip-{}-toolchain".format(trip),
            zip_target = "@zip-{}//file".format(trip),
        )

        native.toolchain(
            name = trip,
            exec_compatible_with = _platforms[trip]["exec"],
            toolchain = ":zip-{}-toolchain".format(trip),
            toolchain_type = "//rules/zip:toolchain_type",
        )
