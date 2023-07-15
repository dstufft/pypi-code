# simple-file-manifest

This crate provides a storage-agnostic interface for representing a collection
of files. It allows you to build up lists of files, which are composed of a path
name and content + metadata. The content can be backed by a referenced file
or defined in memory.
