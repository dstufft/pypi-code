# Understanding how Bazel evaluates WORKSPACE file and correctly structuring it
# to avoid confusion can be difficult.
#
# There are some important things to know about how a WORKSPACE file is
# evaluated:
#
#    A WORKSPACE file is essentially a list of load statements, repository
#    declarations, and function calls. Bazel evaluates the file line-by-line.
#
#    A repository declaration is a call to a repository rule like http_archive
#    or go_repository. Each repository has a name and some information on how to
#    fetch it like URLs and SHA-256 sums. Repository rules are evaluated lazily:
#    at the point where a repository is declared, the repository rule's code
#    isn't actually executed.
#
#    A repository is fetched (meaning its repository rule is executed) the first
#    time a file is loaded from it. Several things can cause this while
#    WORKSPACE is being evaluated:
#
#      * A load statement that mentions a .bzl file in the repository is
#        evaluated. The load statement might appear in WORKSPACE or in another
#        .bzl file loaded from WORKSPACE.
#
#      * A different repository rule is fetched, and that repository's
#        declaration has an attribute that refers to a file. When a repository
#        is fetched, the labels in its attributes are resolved to files, which
#        may cause other repositories to be fetched. Labels may be part of
#        explicit arguments, or they may be default values for attributes.
#
#      * A different repository rule could use ctx.path to dynamically resolve a
#        label.
#
#    The important thing to understand is that a repository isn't fetched until
#    a label mentioning that repository is resolved to a file. It's difficult to
#    be sure about when that happens because there are several cases where it
#    happens implicitly within repository rule implementations.
#
#    This leads to the most confusing aspect of WORKSPACE evaluation:
#
#      A repository may be declared with the same name multiple times without
#      error. This does not create multiple instances of the repository. When a
#      repository is fetched, the latest declaration wins. After a repository is
#      fetched, all following declarations are silently ignored.
#
#    It's difficult to determine when a repository is fetched, so to avoid
#    ambiguity, you should ensure each repository is declared only once.
#
# To reduce this impact of this particular type of confusion, we have some basic
# rules for how we organize a WORKSPACE file:
#
#     1. Workspace declaration. This must appear before all other calls.
#     2. load statements for http_archive, git_repository, and repository rules
#        defined in the main workspace. These symbols are needed in the rest of
#        the file, so they must be loaded near the top.
#     3. Declarations for dependencies that provide repository rules needed
#        later. For example, bazel_gazelle is needed for go_repository.
#     4. Declarations for direct dependencies. These may appear in the WORKSPACE
#        file itself, or you might load and call a function from a .bzl file
#        somewhere in our workspace.
#     5. Declarations for indirect dependencies. To declare these, you'll
#        usually load and call functions from your direct dependencies. Check
#        that these functions won't override your direct dependencies (see below).
#
# Many projects declare indirect dependencies before direct dependencies
# (reversing 4 and 5 above). This causes problems because it limits your ability
# to depend on a specific version of a direct dependency. If a repository is
# declared by a function provided by one of your dependencies, that declaration
# may or may not override a later (direct) declaration. Your direct declaration
# will be silently ignored if the repository is fetched first.

# 1. Workspace Declaration
workspace(name = "pypi-code")

# 2. Load statements for http_archive, git_repository, and repository rules
#    defined in this Workspace.
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# 3. Declarations for dependencies that provide repository rules needed later.
http_archive(
    name = "rules_python",
    sha256 = "84aec9e21cc56fbc7f1335035a71c850d1b9b5cc6ff497306f84cced9a769841",
    strip_prefix = "rules_python-0.23.1",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.23.1/rules_python-0.23.1.tar.gz",
)

# 4. Declarations for direct dependencies.
load("//rules/zig:repositories.bzl", zig_repositories = "repositories")
load("//rules/zip:repositories.bzl", zip_repositories = "repositories")

# Several of our third party dependencies do not use Bazel natively, and instead
# use something like configure+make or CMake or similiar, so we'll load up
# rules_foreign_cc to let Bazel call into those "foreign" build systems.
http_archive(
    name = "rules_foreign_cc",
    sha256 = "059d1d1ec0819b316d05eb7f9f0e07c5cf9636e0cbb224d445162f2d0690191e",
    strip_prefix = "rules_foreign_cc-6ecc134b114f6e086537f5f0148d166467042226",
    url = "https://github.com/bazelbuild/rules_foreign_cc/archive/6ecc134b114f6e086537f5f0148d166467042226.tar.gz",
)

load("//:third_party/libffi/repositories.bzl", libffi_repositories = "repositories")
load("//:third_party/python/repositories.bzl", python_repositories = "repositories")
load("//:third_party/util-linux/repositories.bzl", util_linux_repositories = "repositories")
load("//:third_party/xz/repositories.bzl", xz_repositories = "repositories")
load("//:third_party/zlib/repositories.bzl", zlib_repositories = "repositories")

# Setup our zig repositories, which we use for creating a hermetic C/C++
# toolchain and generate the needed toolchains.

zig_repositories()

zip_repositories()

libffi_repositories()

python_repositories()

# Register Python toolchains
register_toolchains("@python//:toolchain")

util_linux_repositories()

xz_repositories()

zlib_repositories()

# Register any toolchains that we've imported
load("//rules/zig:toolchains.bzl", zig_toolchains = "toolchains")
load("//rules/zip:toolchains.bzl", zip_toolchains = "toolchains")

zig_toolchains()

zip_toolchains()

# 5. Declarations for indirect dependencies.
load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
load("@rules_python//python:repositories.bzl", rules_python_dependencies = "py_repositories")

rules_foreign_cc_dependencies()

rules_python_dependencies()
