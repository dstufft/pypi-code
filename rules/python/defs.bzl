"""
"""

load("//rules/python/private:py_binary.bzl", _py_binary = "py_binary")

def py_binary(name, srcs = [], main = None, imports = ["."], **kwargs):
    if not main and not len(srcs):
        fail("When 'main' is not specified, 'srcs' must be non-empty")

    _py_binary(
        name = name,
        srcs = srcs,
        main = main if main != None else srcs[0],
        # imports = imports,
        **kwargs
    )
