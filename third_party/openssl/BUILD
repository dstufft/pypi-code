load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "openssl",
    configure_command = "Configure",
    configure_options = [
        "--libdir=lib",
        "no-autoload-config",
        "no-cmp",
        "no-cms",
        "no-comp",
        "no-filenames",
        "no-gost",
        "no-tests",
        "no-ts",
        "no-idea",
        "no-mdc2",
        "no-rc5",
        "no-zlib",
        "no-ssl3",
        "no-ssl3-method",
        "no-capieng",
        "no-rdrand",
    ],
    lib_source = "@upstream//:srcs",
    # Note that for Linux builds, libssl must come before libcrypto on the linker command-line.
    # As such, libssl must be listed before libcrypto
    out_shared_libs = select({
        # "@platforms//os:macos": [
        #     "libssl.1.1.dylib",
        #     "libcrypto.1.1.dylib",
        # ],
        "//conditions:default": [
            "libssl.so",
            "libssl.so.3",
            "libcrypto.so",
            "libcrypto.so.3",
        ],
    }),
    out_static_libs = [
        "libssl.a",
        "libcrypto.a",
    ],
    targets = [
        "depend",
        "build_sw",
        "install_sw",
        "install_ssldirs",
    ],
    visibility = ["//visibility:public"],
)
