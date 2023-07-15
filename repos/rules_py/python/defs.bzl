"""
"""

load("@rules_rust//rust:defs.bzl", _rust_binary = "rust_binary")
load("@rules_py//python/private:py_binary.bzl", _py_binary = "py_binary", _py_binary_skeleton = "py_binary_skeleton")
load("@rules_py//python/private:runtime.bzl", _python_runtime = "python_runtime")

python_runtime = _python_runtime

# TODO: Would it be useful to turn py_binary from only emitting a rust_binary to
#       emitting both a rust_library and a rust_binary? Or maybe we need another
#       target like py_binary_library or something? The main goal would be to
#       enable embedding a Python library into another binary (C, C++, Rust,
#       whatever), without that library having to do the same work to setup a
#       Python interpreter and produce packaged resource files, etc.
#
#       This may not actually be worthwhile though, I suspect linking multiple
#       interpreters together won't actually work, so it might be a footgun that
#       stops working the moment you try to depend on two py_binary_library
#       targets.
#
#       Still, there may be useful primitives we can split out that make it
#       easier to consume python code as a C library.

# TODO: Language Runtimes should not be part of toolchains, toolchains are tools
#       that run on the exec platform, and produce output for the target
#       platform.
#
#       It's common to also bundle runtime/stdlib type information with the
#       toolchain, however strictly speaking this is somewhat of an abuse of
#       what toolchains are actually for, unless the tool *must* copy part of
#       itself into the output.
#
#       Given that, we don't actually treat Python itself as a toolchain, and
#       we instead treat it as just another input to our rules. This presents
#       some interesting opportunities and challenges:
#
#         - We easily support building Python from source, without having to do
#           anything special, since it's just another input.
#
#         - Rules to generate repository rules from a requirements.txt file or
#           similiar can't rely on toolchains to select their Python, but can
#           accept blanket targets.
#
#         - We can still provide easy to use values that use precompiled
#           binaries.
#
#         - Need to figure out a solution for setting a workspace global for all
#           of the py_* rules, that still allows overriding for specific rules.
#               - Maybe transitions can do something with this?
#               - This is one of the big benefits to platforms, that you can
#                 register them and have one selected.
#                   - We can maybe fix this by letting you register py sources
#                     and use transitions or repo rules where you import specific
#                     ones to get specific pythons.

# TODO: Figure out how to generate the PYO3_CONFIG_FILE, ideally without
#       executing Python, because if we can do it without executing Python then
#       we make it easy to build this even with cross compiling. If we have to
#       execute Python then we can only generate that file in cases where the
#       exec platform and target platform are the same--, which I think we can
#       do, but would be more ideal to not require it.
#
#         - On *nix I think most of the information we get comes from the
#           Makefile and the pure Python code just parses it, on Windows I have
#           no idea where it comes from?

# TODO: Would it make sense to have py_binary not rely on rust at all, and not
#       bring along a Python runtime, and instead have a py_launcher rule that
#       wraps the py_binary and does the rust stuff?

def py_binary(name, runtime = None):
    # Generally speaking, the way most of the various Python rules in other
    # projects work is they will generate some kind of a wrapper script using
    # some kind of interpreted language (rules_python uses a non-hermetic
    # python3 wrapper, other rules use a non-hermetic bash wrapper) and that
    # script is responsible for setting up the environment, and then ultimately
    # dispatching to the underlying python binary from the registered toolchain.
    #
    # To make this work, these tools often do things like setup a series of
    # directories that get fed into the PYTHONPATH environment variable, or
    # creating virtual environments (and "fixing" them on the fly for different
    # paths).
    #
    # Overall these approaches are extremely fragile, particularly whenever you
    # want to export your now built binary (and related files) out of Bazel for
    # use elsewhere.
    #
    # We take a different approach. We instead generate a Rust based wrapper
    # that produces a binary. This binary links against Python and is reponsible
    # for setting Python up and ultimately executing the included code.
    #
    # This gives us a number of benefits:
    #
    #  - We get very fine grained control over the Python interpreter startup,
    #    which means we can carefully control things like sys.path and other
    #    settings to ensure that our interpreter is started up correctly and is
    #    isolated from the system.
    #
    #  - We eliminate the startup costs of starting up a wrapper script that has
    #    to then startup the Python interpreter.
    #
    #  - We can use oxidized_importer which can avoid having to do work at
    #    runtime to locate importable files by precomputing a list of everything
    #    that should be import-able, and compiling that list into the binary.
    #    This means that rather than a bunch of I/O to discover which file to
    #    import, we only need to check in a Rust based HashMap.
    #
    #  - We can reduce or eliminate the need for runfiles, again through the use
    #    of oxidized_importer we can shift some or all of the importable items
    #    out of the filesystem and embed them into the binary itself, meaning
    #    imports never have to touch the filesystem at all, it's just reading
    #    files direct from memory.
    #
    #  - We can (when available) statically link against any C dependencies,
    #    making it possible to have a binary that no longer needs to dlopen()
    #    or dynamically link against anything, including C Extensions.
    #
    #  - One of the implications of the other benefits combined, is that we can
    #    often times produce binaries which are statically linked (other than
    #    potentionally glibc, unless musl is being used), and have no dependence
    #    on locating anything on the filesystem at all. Providing a Rust or Go
    #    like experience of a single file binary.
    #
    # This does have some downsides of course:
    #
    #  - Building requires a Rust toolchain, and we end up depending on
    #    rules_rust (and some third party crates) to make our binary wrapper.
    #
    #  - Building can take longer, since we now have to invoke rustc and linkers
    #    rather than just copying around files and generating a wrapper script.
    #
    #  - We have to worry about cross platform compiling, even for pure Python
    #    code, when previously only the Python toolchain had to worry about
    #    that.
    #
    #  - The importer based optimizations rely on using a specialized importer
    #    which is not 100% compatible with the default importer. This
    #    specialized importer does correctly implement the interfaces required
    #    by importlib, and views deviations from that spec as bugs; however the
    #    default file system based importer adds it's own behaviors ontop of the
    #    requirements of what importlib requires, and some Python code makes
    #    assumptions that those behaviors will always exist, making them
    #    incompatible with importers that don't match the filesystem importer
    #    exactly.
    #
    #    The specialized importer we use works radically different from the
    #    default filesystem importer, and as such there is not a good way to
    #    implement all of the behavior from it that is known to exist in the
    #    wild.
    #
    #    The page at https://pyoxidizer.readthedocs.io/en/stable/oxidized_importer_behavior_and_compliance.html
    #    goes into more details about the known issues that can come from using
    #    it.
    #
    #  - The typical way to import C extensions in Python is to use dlopen to
    #    load the module. However, dlopen on most (all?) platforms assumes that
    #    the .so/dll/etc begins at the start of the file, which makes it
    #    impossible to dlopen() a C extension that is bundled as data inside of
    #    a binary.
    #
    #    This means that whenever C extensions are at play (including in the
    #    Python standard library) there's only a few options available to us,
    #    each with their own downsides:
    #
    #      - Keep the C extensions on disk as individual files, possibly
    #        embedding everything else.
    #          - Pro: Everything works as expected with maximum compatiblity
    #                 with both binary wheels and arbitrary Python toolchains.
    #          - Con: Impossible to support "single file binaries", since the
    #                 C extension files have to remain on disk.
    #      - Embed the C extensions, but write them out temporarily to disk on
    #        demand and dlopen open them from there.
    #          - Pro: Everything (mostly) works as expected with maximum
    #                 compatiblity with both binaries wheels and arbitrary
    #                 Python toolchains.
    #          - Pro: Retain the ability to have a single file binary that can
    #                 be deployed by copying.
    #          - Con: Importing C extensions slow down because the binary has to
    #                 first write the C extension to disk, which can be
    #                 mitigated by using a persistent cache rather than temp
    #                 location.
    #          - Con: Mitigating import slow down with a persistent cache means
    #                 that you have to deal with ensuring that the cache matches
    #                 what you expect and hasn't been modified or isn't out of
    #                 date, etc.
    #          - Con: More fragile at runtime due to issues like read only or
    #                 no exec filesystems.
    #      - Static link the C extensions into the binary as well.
    #          - Pro: Retain the ability to have a single file binary that can
    #                 be deployed by copying.
    #          - Con: May require building Python and/or any C extension modules
    #                 from source rather than being able to use existing binary
    #                 wheels or arbitrary Python toolchains.

    # Emit the rust source files for our binary wrapper, setting up the project
    # so that it can later be compiled by rust_binary.
    #
    # This generated project is also where all of the magic happens to setup the
    # Python interpreter with everything it needs to know to run this project.
    _py_binary_skeleton(name = "%s._wrapper" % name)

    # Take our generated rust files for our binary wrapper, and feed them into
    # rust_binary to ultimately compile our binary.
    _rust_binary(
        name = "%s._bin" % name,
        crate_name = name,
        srcs = [":%s._wrapper" % name],
        edition = "2021",
        # TODO: We're hardcoding @python here, but it should come through the
        #       runtime instead.
        deps = ["@rules_py//third_party/crates:pyembed", "@python"],
    )

    # Wrap our rust_binary with a py_binary, which exists primarily to make the
    # runtime transition work, so that the runtime can be passed in as a
    # parameter, and we can transition to the provided runtime.
    _py_binary(name = name, bin = "%s._bin" % name, runtime = runtime)
