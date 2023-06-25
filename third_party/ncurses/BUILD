load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

configure_make(
    name = "ncurses",
    configure_in_place = True,
    configure_options = [
        "CFLAGS=-fPIC",
        # Disable unneeded stuff
        "--without-manpages",
        "--without-progs",
        "--without-tack",
        "--without-tests",
        "--without-ada",
        # Build shared libraries as well
        "--with-shared",
        # Enable wide characters (e.g. UTF-8)
        "--enable-widec",
        # 	Use only built-in data.  The ncurses libraries normally read terminfo
        # 	and termcap data from disk.  You can configure ncurses to have a
        # 	built-in database, aka "fallback" entries.  Embedded applications may
        # 	have no need for an external database.  Some, but not all of the
        # 	programs are useful in this configuration, e.g., tset and tput versus
        # 	infocmp and tic.
        "--disable-database",
        "--with-fallbacks=linux,vt100,xterm,xterm-256color",
        # 	The $HOME/.terminfo directory is normally added to ncurses' search
        # 	list for reading/writing terminfo entries, since that directory is
        # 	more likely writable than the system terminfo database.  Use this
        # 	option to disable the feature altogether.
        "--disable-home-terminfo",
        # Suppress the "w", "t" or "tw" suffixes which normally would be added
        # to the library names for the --enable-widec and --with-pthread options.
        "--disable-lib-suffixes",
        # 	Extend the cchar_t structure to allow more than 16 colors to be
        # 	encoded.  This applies only to the wide-character (--enable-widec)
        # 	configuration.
        "--enable-ext-colors",
        # Link with POSIX threads, set --enable-reentrant.  The use_window() and
        # use_screen() functions will use mutex's, allowing rudimentary support
        # for multithreaded applications.
        "--with-pthread",
        # Taken from Debian's package
        "--without-profile",
        "--without-debug",
        "--disable-rpath",
        "--enable-echo",
        "--enable-const",
        "--disable-setuid-environ",
        "--disable-root-acces",
        "--disable-termcap",
        "--enable-overwrite",
    ],
    # This is a hack to fix a bug in the configure script.
    configure_prefix = "unset MAKE && ",
    lib_name = "libncurses",
    lib_source = "@upstream//:srcs",
    out_shared_libs = select({
        # "@platforms//os:windows": ["zlib.dll"],
        # "@platforms//os:osx": ["zlib.dylib"],
        "//conditions:default": ["libncurses.so"],
    }),
    visibility = ["//visibility:public"],
)
