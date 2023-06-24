"""
"""

PY_TOOLCHAIN = "@bazel_tools//tools/python:toolchain_type"
SH_TOOLCHAIN = "@bazel_tools//tools/sh:toolchain_type"

def _py_binary_rule_imp(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)
    runtime = ctx.toolchains["@bazel_tools//tools/python:toolchain_type"].py3_runtime

    files = [
        ctx.file._bash_runfile_helper,
        runtime.interpreter,
        ctx.file.main,
    ]
    files.extend(ctx.files.srcs)

    runfiles = ctx.runfiles(files = files, transitive_files = runtime.files)

    entrypoint_path = "{workspace_name}/{entrypoint_path}".format(
        workspace_name = ctx.workspace_name,
        entrypoint_path = ctx.file.main.short_path,
    )

    interpreter_path = runtime.interpreter.short_path.replace("../", "")

    substitutions = {
        "{entrypoint_path}": entrypoint_path,
        "{interpreter_path}": interpreter_path,
    }

    ctx.actions.expand_template(
        template = ctx.file._bash_runner_tpl,
        output = executable,
        substitutions = substitutions,
    )

    return [
        DefaultInfo(
            executable = executable,
            runfiles = runfiles,
        ),
    ]

_attrs = dict({
    "main": attr.label(
        allow_single_file = True,
        mandatory = True,
    ),
    "srcs": attr.label_list(
        allow_files = True,
        doc = "Source files to compile",
    ),
    "deps": attr.label_list(),
    "_bash_runner_tpl": attr.label(
        default = "//rules/python/private:_bash_runner.tpl",
        allow_single_file = True,
    ),
    "_bash_runfile_helper": attr.label(
        default = "@bazel_tools//tools/bash/runfiles",
        doc = "Label pointing to bash runfile helper",
        allow_single_file = True,
    ),
})

py_base = struct(
    implementation = _py_binary_rule_imp,
    attrs = _attrs,
    toolchains = [
        SH_TOOLCHAIN,
        PY_TOOLCHAIN,
    ],
)

py_binary = rule(
    implementation = py_base.implementation,
    attrs = py_base.attrs,
    toolchains = py_base.toolchains,
    executable = True,
)
