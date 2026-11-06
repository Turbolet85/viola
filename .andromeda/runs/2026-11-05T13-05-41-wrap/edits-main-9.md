@fact id=fct-lj7tqt64 expect_rev=1
old: `viola-harness gate` enforces the per-OS floors
new: `viola-harness gate` enforces the coverage floors

@fact id=fct-eesxehzs expect_rev=1
old: a `cfg(windows)`/`cfg(unix)` file-pair control
new: a control file gated on the CI triple's own OS (`cfg(target_os = "linux")`)

@fact id=fct-ak4xjyyx expect_rev=2
old: (that pair fails on Windows, rust-lang/rust#54118)
new: (a layout chosen while Windows, where that pair fails (rust-lang/rust#54118), was a target, and kept)

@fact id=fct-zyth7okk expect_rev=1
old: it returns `Unsupported` on macOS, which fails the bind;
new: a rule set while macOS (where it returns `Unsupported` and fails the bind) was a target, and kept;

@fact id=fct-lqftgban expect_rev=1
old: first-execution costs, for example Windows Defender scanning a freshly built exe, and are
new: first-execution costs and are

@fact id=fct-vxnii5yh expect_rev=1
old: CI runs it in its own per-OS `perf` job
new: CI runs it in its own `perf` job

@fact id=fct-zxoe3ae7 expect_rev=1
old: separator-agnostic, because llvm-cov reports Windows paths with `\`.
new: separator-agnostic, a form kept from the Windows target, where llvm-cov reports paths with `\`.

@fact id=fct-zxaumenb expect_rev=3
old: on Windows a running `target/debug/viola-harness.exe` cannot be relinked
new: on the retired Windows host a running `target/debug/viola-harness.exe` could not be relinked

@fact id=fct-zujg4opw expect_rev=2
old: tests compiled out by `#[cfg(windows)]` / `#[cfg(unix)]` are never counted
new: tests compiled out by an OS `#[cfg(…)]` gate are never counted

@fact id=fct-3q2uf724 expect_rev=1
old: in its own per-OS job
new: in its own `perf` job

@fact id=fct-ydc7ueix expect_rev=2
old: the per-OS `perf` job's `diag-perf-<os>`
new: the `perf` job's `diag-perf-<os>`

@fact id=fct-gsrzhsh7 expect_rev=1
old: from `run --perf` in the per-OS `perf` job
new: from `run --perf` in the `perf` job

@fact id=fct-eilqg5wz expect_rev=1
old: they run in their own per-OS `perf` job
new: they run in their own `perf` job

@fact id=fct-ecerp7e3 expect_rev=1
old: runs it in the per-OS `lint` job once per lib/bin target
new: runs it in the `lint` job once per lib/bin target
old: It runs in CI's per-OS `lint` job after `Lint probes`.
new: It runs in CI's `lint` job after `Lint probes`.

@fact id=fct-h3byb6pq expect_rev=1
old: The per-OS `release` job builds
new: The `release` job builds

@fact id=fct-zhdn6mmq expect_rev=1
old: The per-OS `perf` job (job-level `contents: read`)
new: The `perf` job (job-level `contents: read`)
