"""Assembles a Linux sysroot from pinned Debian packages.

toolchains_llvm needs a sysroot for Linux targets. The prebuilt sysroots that
Chromium publishes are still Debian bullseye (glibc 2.31, libstdc++ from
GCC 10), which is too old for the static Swift 6.4 runtime, and nobody
publishes a newer one. Rather than host a tarball somewhere, this repository
rule downloads the handful of Debian packages a C/C++ sysroot needs, verifies
each against its sha256, and unpacks them in place. The result is reproducible
from MODULE.bazel alone.

Each .deb is an `ar` archive holding a `data.tar.*` with the installed files.
Debian ships a few absolute symlinks (e.g. /lib64/ld-linux-x86-64.so.2 ->
/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2); those are rewritten to relative
links so they resolve inside the sysroot instead of on the host.
"""

_BUILD = """\
filegroup(
    name = "sysroot",
    srcs = glob(["**"]),
    visibility = ["//visibility:public"],
)
"""

_RELATIVIZE_SYMLINKS = """\
import os
import sys

root = os.path.abspath(sys.argv[1])
for dirpath, dirnames, filenames in os.walk(root):
    for name in dirnames + filenames:
        path = os.path.join(dirpath, name)
        if not os.path.islink(path):
            continue
        target = os.readlink(path)
        if not target.startswith("/"):
            continue
        # Bazel already rebases absolute link targets from a tarball onto the
        # extraction directory, so a target may point into `root` or at the
        # bare archive path. Normalise both to a path inside the sysroot.
        if not (target == root or target.startswith(root + "/")):
            target = root + target
        os.unlink(path)
        os.symlink(os.path.relpath(target, dirpath), path)

# Dev packages ship symlinks to runtime libraries that aren't part of this
# sysroot (libitm.so -> libitm.so.1, ...). Drop them rather than leave them
# dangling.
for dirpath, dirnames, filenames in os.walk(root):
    for name in dirnames + filenames:
        path = os.path.join(dirpath, name)
        if os.path.islink(path) and not os.path.exists(path):
            os.unlink(path)
"""

def _debian_sysroot_impl(rctx):
    for filename, sha256 in rctx.attr.packages.items():
        deb = "_debs/" + filename.rsplit("/", 1)[-1]
        rctx.download(
            url = [mirror.rstrip("/") + "/" + filename for mirror in rctx.attr.mirrors],
            output = deb,
            sha256 = sha256,
        )
        unpacked = deb + ".unpacked"
        rctx.extract(archive = deb, output = unpacked, type = "ar")
        data = None
        for ext in ["xz", "zst", "gz", "bz2"]:
            candidate = rctx.path(unpacked + "/data.tar." + ext)
            if candidate.exists:
                data = candidate
                break
        if not data:
            fail("{}: no data.tar.* member found in the package".format(filename))
        rctx.extract(archive = data, output = ".")
    rctx.delete("_debs")

    for path in rctx.attr.prune:
        rctx.delete(path)

    python = rctx.which("python3")
    if not python:
        fail("debian_sysroot needs python3 on PATH to rewrite absolute symlinks")
    result = rctx.execute([python, "-c", _RELATIVIZE_SYMLINKS, "."])
    if result.return_code != 0:
        fail("rewriting sysroot symlinks failed: " + result.stderr)

    rctx.file("BUILD.bazel", _BUILD)

debian_sysroot = repository_rule(
    implementation = _debian_sysroot_impl,
    doc = "Builds a sysroot by unpacking pinned Debian packages.",
    attrs = {
        "mirrors": attr.string_list(
            doc = "Base URLs that the package paths are appended to, tried in order.",
            mandatory = True,
        ),
        "packages": attr.string_dict(
            doc = "Pool-relative .deb paths mapped to their sha256.",
            mandatory = True,
        ),
        "prune": attr.string_list(
            doc = "Directories to delete after unpacking (docs, binaries, etc.).",
            default = ["etc", "usr/bin", "usr/sbin", "usr/share", "var"],
        ),
    },
)
