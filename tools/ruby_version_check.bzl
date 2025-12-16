"""Repository rule to verify system Ruby version."""

def _parse_version(version_str):
    """Parse version string like '3.4.1' into tuple (3, 4, 1)."""
    parts = version_str.split(".")
    result = []
    for part in parts:
        # Extract just the numeric portion
        num = ""
        for c in part.elems():
            if c.isdigit():
                num += c
            else:
                break
        if num:
            result.append(int(num))
    return result

def _version_at_least(version, minimum):
    """Check if version tuple is >= minimum tuple."""
    for i in range(len(minimum)):
        if i >= len(version):
            return False
        if version[i] > minimum[i]:
            return True
        if version[i] < minimum[i]:
            return False
    return True

def _ruby_version_check_impl(rctx):
    # Skip check in CI environments (BuildBuddy, GitHub Actions, etc.)
    # The Docker image should have the correct Ruby version
    ci_env_vars = ["CI", "BUILDBUDDY_API_KEY", "GITHUB_ACTIONS"]
    for var in ci_env_vars:
        if rctx.os.environ.get(var):
            rctx.file("BUILD.bazel", """
# Ruby version check skipped in CI environment
filegroup(
    name = "check",
    srcs = [],
    visibility = ["//visibility:public"],
)
""")
            return

    result = rctx.execute(["ruby", "--version"])
    if result.return_code != 0:
        fail("Ruby not found on PATH. Please install Ruby 3.4.1 or later")

    version_output = result.stdout.strip()

    # Parse version from output like "ruby 3.4.1 (2024-12-25 revision 48d4efcb85) ..."
    parts = version_output.split(" ")
    if len(parts) < 2:
        fail("Could not parse Ruby version from: " + version_output)

    version_str = parts[1]
    version = _parse_version(version_str)
    minimum = [3, 4, 1]

    if not _version_at_least(version, minimum):
        fail("Ruby 3.4.1 or later required, but found: " + version_output +
             "\nPlease install Ruby 3.4.1+ (e.g., via rbenv: rbenv install 3.4.1 && rbenv global 3.4.1)")

    # Create a BUILD file with a filegroup that can be depended on
    rctx.file("BUILD.bazel", """
# Ruby version check passed: {version}
filegroup(
    name = "check",
    srcs = [],
    visibility = ["//visibility:public"],
)
""".format(version = version_output))

ruby_version_check = repository_rule(
    implementation = _ruby_version_check_impl,
    local = True,
    doc = "Verifies that Ruby 3.4.1 or later is available on PATH",
)
