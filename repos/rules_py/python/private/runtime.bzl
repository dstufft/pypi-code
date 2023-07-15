"""
"""

# TODO: Determine if we can use the built in PyRuntimeInfo
# TODO: Think up a name that doesn't collide with PyRuntimeInfo as much.
# TODO: Implement an init function to validate values?
PythonRuntimeInfo = provider(
    doc = "Contains information about a Python Runtime",
    fields = {
        "interpreter": "The actual interpreter that this describes.",
        "implementation": "The name of the interpreter implmentatation (CPython, PyPy)",
        "version": "The version of the Python interpreter",
        # TODO: Since Bazel is handling our linking, do we actually need to know
        #       about shared/static linking or can we just let Bazel do it's
        #       thing?
        "shared": "Whether the Python Runtime supports dynamic linking",
        "static": "Whether the Python Runtime supports static linking",
        # TODO: AFAIK Python's pointer width is always a property of whether it
        #       is a 32bit or 64bit Python, this also feels like something we
        #       should be able to let Bazel handle or at least infer from Bazel,
        #       or possibly it should be part of the Platform?
        "pointer_width": "The width of pointers.",
    },
)

def _python_runtime_impl(ctx):
    version_parts = ctx.attr.version.split(".")[:2]
    version = struct(major = version_parts[0], minor = version_parts[1])

    return [
        PythonRuntimeInfo(
            interpreter = ctx.attr.interpreter,
            implementation = ctx.attr.implementation,
            version = version,
            shared = ctx.attr.shared,
            static = ctx.attr.static,
            pointer_width = ctx.attr.pointer_width,
        ),
    ]

python_runtime = rule(
    implementation = _python_runtime_impl,
    attrs = {
        "interpreter": attr.label(
            doc = "The interpreter that this runtime provides",
            executable = True,
            mandatory = True,
            cfg = "target",
        ),
        # TODO: We can infer these values in some (many? all?) cases, and while
        #       we should continue to support manually setting them, it would be
        #       a lot better experience if we could support infering them, but
        #       we'll need to be careful because our interpreter is for the
        #       target platform, which may be different than the exec platform
        #       so we can't execute the interpreter itself to determine this.
        #
        #       Another option is to support loading this from a static file
        #       that an interpreter can emit when it's being built-- which may
        #       even be able to be the Makefile that sysconfig itself uses.
        "implementation": attr.string(
            doc = "The implementation type for this runtime",
            mandatory = True,
            values = ["CPython", "PyPy"],
        ),
        "version": attr.string(
            doc = "The version of the Python runtime",
            mandatory = True,
        ),
        "shared": attr.bool(
            doc = "Whether this Python runtime supports shared linking",
        ),
        "static": attr.bool(
            doc = "Whether this Python runtime supports static linking",
        ),
        "pointer_width": attr.int(
            doc = "The width of pointers in the Python runtime.",
        ),
    },
    provides = [PythonRuntimeInfo],
)

# buildifier: disable=unused-variable
def _unconfigured_python_runtime_impl(ctx):
    return [
        PythonRuntimeInfo(
            interpreter = "UNKNOWN",
            implementation = "UNKNOWN",
            version = struct(major = "UNKNOWN", minor = "UNKNOWN"),
            shared = False,
            static = False,
            pointer_width = 64,
        ),
    ]

unconfigured_python_runtime = rule(
    implementation = _unconfigured_python_runtime_impl,
    provides = [PythonRuntimeInfo],
)

def _pyo3_config_impl(ctx):
    runtime = ctx.attr.runtime[PythonRuntimeInfo]

    pyo3_config = ctx.actions.declare_file(ctx.attr.name)
    ctx.actions.expand_template(
        template = ctx.file._pyo3_build_config,
        output = pyo3_config,
        substitutions = {
            "{{implementation}}": runtime.implementation,
            "{{version.major}}": str(runtime.version.major),
            "{{version.minor}}": str(runtime.version.minor),
            # TODO: This logic is wrong, both shared and static could be false?
            "{{shared}}": "false" if runtime.static else "true",
            "{{executable}}": runtime.interpreter.files.to_list()[0].path if runtime.interpreter != "UNKNOWN" else "UNKNOWN",
            "{{pointer_width}}": str(runtime.pointer_width),
        },
    )

    return [
        DefaultInfo(files = depset([pyo3_config])),
    ]

pyo3_config = rule(
    implementation = _pyo3_config_impl,
    attrs = {
        "runtime": attr.label(
            default = "//python:runtime",
            providers = [PythonRuntimeInfo],
        ),
        "_pyo3_build_config": attr.label(
            default = "@rules_py//python/private:_pyo3-build-config.tmpl.txt",
            allow_single_file = True,
        ),
    },
)
