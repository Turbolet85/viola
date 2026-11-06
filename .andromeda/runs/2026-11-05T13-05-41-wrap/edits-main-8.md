@fact id=fct-p7rzex2i expect_rev=1
old: portable-pty 0.8.1 calls `CreateProcessW` with `bInheritHandles = 0`, and interprocess listeners stay `inheritable(false)`, so channel handles never leak into the `claude` child.
new: channel handles never leak into the `claude` child: on Linux the property rests on close-on-exec descriptors (recorded, not established; its measurement is owed to the working-route entry "Version done-check").

@fact id=fct-iftwwapx expect_rev=1
old: NEVER make channel handles inheritable, and NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`.
new: NEVER make channel handles inheritable.

@fact id=fct-5q3bjmyd expect_rev=1
old: (`CARGO_TARGET_DIR` for its builds and test runs), because a running `target/debug/viola-harness.exe` cannot be relinked on Windows.
new: (`CARGO_TARGET_DIR` for its builds and test runs).

@fact id=fct-4573rvoy expect_rev=1
old: run in their own per-OS `perf` job;
new: run in their own `perf` job;

@fact id=fct-fruln4ot expect_rev=1
old: in the per-OS `perf` job,
new: in the `perf` job,

@fact id=fct-2geebg2r expect_rev=1
old: PIDs are recycled, quickly on Windows.
new: PIDs are recycled.
