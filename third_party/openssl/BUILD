load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "openssl",
    configure_command = "config",
    configure_in_place = True,
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
        "no-shared",
    ],
    env = {
        "PERL": "$$EXT_BUILD_ROOT$$/$(PERL)",
    },
    lib_source = "@upstream//:srcs",
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
    toolchains = ["@rules_perl//:current_toolchain"],
    visibility = ["//visibility:public"],
)
