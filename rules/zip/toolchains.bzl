"""
"""

ZipInfo = provider(
    doc = "Information about how to invoke zip.",
    fields = ["zip_target"],
)

# buildifier: disable=unnamed-macro
def toolchains():
    native.register_toolchains(
        "//rules/zip/toolchains:x86_64-unknown-linux",
        "//rules/zip/toolchains:x86_64-apple-darwin",
        "//rules/zip/toolchains:arm64-apple-darwin",
    )
