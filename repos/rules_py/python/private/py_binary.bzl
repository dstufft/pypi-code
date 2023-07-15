"""
"""

load("@bazel_skylib//lib:paths.bzl", "paths")
load("@aspect_bazel_lib//lib:paths.bzl", "relative_file", "to_repository_relative_path")
load("@aspect_bazel_lib//lib:copy_file.bzl", "copy_file_action")
load("//python/private:runtime.bzl", "PythonRuntimeInfo")

def _py_binary_skeleton_impl(ctx):
    outfiles = []

    for file in ctx.files._wrapper_skel:
        # Determine the relative path of our file, which is a little convulted because we
        # bury it a level deeper than expected.
        rpath = relative_file(to_repository_relative_path(file), ctx.attr._wrapper_skel.label.package)
        rpath = paths.join(*rpath.split("/")[1:])

        # Create a project directory based on the name of our rule, and root our
        # skeleton files in there.
        ofile = ctx.actions.declare_file(paths.join(ctx.attr.name, rpath))

        # Our skeleton files are actually template files, so we'll expand them
        # into the output directory.
        ctx.actions.expand_template(
            template = file,
            output = ofile,
            substitutions = {},
        )

        outfiles.append(ofile)

    return [
        DefaultInfo(files = depset(outfiles)),
    ]

py_binary_skeleton = rule(
    implementation = _py_binary_skeleton_impl,
    attrs = {
        "_wrapper_skel": attr.label(
            default = "@rules_py//python/private/wrapper-skel",
            doc = "The skeleton of template files to use to generate the wrapper files",
            allow_files = True,
        ),
    },
)

def _py_binary_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)
    copy_file_action(ctx, ctx.attr.bin[0][DefaultInfo].files.to_list()[0], executable)
    return [DefaultInfo(executable = executable)]

def _py_binary_transition_impl(_settings, attr):
    return {"//python:runtime": attr.runtime}

_py_binary_transition = transition(
    implementation = _py_binary_transition_impl,
    inputs = [],
    outputs = ["//python:runtime"],
)

py_binary = rule(
    implementation = _py_binary_impl,
    attrs = {
        "bin": attr.label(
            doc = "The underlying binary rule that is actually compiling/creating this py_binary",
            mandatory = True,
            cfg = _py_binary_transition,
        ),
        "runtime": attr.label(
            providers = [PythonRuntimeInfo],
        ),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
    executable = True,
)
