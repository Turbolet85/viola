# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A concurrent-first-start witness that spawns `viola` children must keep them in coverage: any test that clears the child env must re-add `LLVM_PROFILE_FILE`.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A race control that kills an instrumented child (a racing start, an agent whose hook spawn failed) must not land the kill inside that child's exit-time profile write, or the coverage merge goes red on a short profile.
  > One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A two-instance concurrent boot test that kills its wrappers is kill-shaped: its hooks must not be cut off in their profile write by the PTY hangup.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none;
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A new repeated-run race case can be read for refused profiles through the census's `--case` form, a dev-host witness only, never a CI, pre-push or harness gate.
  > The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, with a second form `--case <binary stem> <test name> <runs> <workers> [<name>]` over one integration case that offers each profile alone to `llvm-profdata merge` and counts the refused ones, a dev-host script (architecture §Occupied Resources → Repository): no CI job, `pre-push` stage or harness command runs it, and it has no unit test, only its recorded one-shot controls (must-fail on the untouched test, must-pass, and two planted readings).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) This chunk carries no mutation gate of its own; a mutation run on the `pin.rs` / `fs.rs` change is on demand only, never a chunk acceptance gate.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) Any Windows-only arm the fix adds to the exe's absent-copy write (a `cfg(windows)` sharing-violation wait) is a mutant the Linux host cannot reach: recorded owed by coordinate, never equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught;
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:17) `viola_state::fs`'s retry decision stays passed in by the caller: a change to `replace_private` / `replace_private_shared` must keep that restatement, which made the site reachable on Linux and holds its Epoch 4 kills.
  > 7 of them after a behaviour-preserving restatement that made the site reachable on Linux (`viola_state::fs`'s retry decision passed in by the caller, `viola send`'s issue-line decision a named function), each shown red under its hand-applied mutant and green restored with no mutation run made;
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:19) A race reproduction case runs under nextest's CI profile unchanged: it must finish inside the 120 s kill with no retry.
  > Both run on ubuntu and windows; `[profile.ci]`'s 120 s kill and `retries = 0` are unchanged.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The perf session boots through `run`'s start, the `pin_bytes` absent-copy arm included: a boot the changed pin write fails turns suite `perf` red, never a skipped row.
  > A failed build (`build`), boot (`boot: <reason>`), row (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or skipped row.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) The timed hooks spawn from the pinned path: the newest reading is the one the chunk's `run --perf` must stay under `SPINE_DEADLINE` against.
  > with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).
- (.andromeda/test-plan.md:816) The new pin and fs code (and any Windows arm) must hold the Comprehensive thresholds on each OS's own coverage job.
  > - Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:854) The race witness must be deterministic: a once-red concurrent case is the chunk's red until its root cause is fixed in it, never retried or `#[ignore]`d.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:856) The chunk's build fails on any test failure on any OS or a coverage breach on any OS.
  > **Build failure conditions:**
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
- (.andromeda/test-plan.md:861) The reproducible-build measurement goes through the release build, which must still carry no test-only binary.
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary
- (.andromeda/test-plan.md:760) The measurement's clean release builds are the `release-check.sh` build, a `--release` build of `viola` alone with no test-only feature; it is no upload and no gate step.
  > - **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
- (.andromeda/test-plan.md:795) Any build change taken on the measurement's basis (a linker flag, a path remap, a profile key) must keep both release-check verdict lines.
  > - Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
- (.andromeda/test-plan.md:214) The hash-before-reuse rule stands: a tampered present copy still exits 1 after the absent-copy write's concurrency changes.
  >     - pinned exe hash mismatch → exit 1
- (.andromeda/test-plan.md:620) The pinned-exe content hash stays pinned to its library by literal vectors: a key change taken on the measurement's basis moves those vectors only by the founder's act.
  > and the pinned-exe content hash is pinned to its library by literal vectors.
- (.andromeda/test-plan.md:876) The Windows reading of the exe's absent-copy race is owed to the `windows-2025` runner; a Linux reproduction is no proof for Windows or macOS.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:922) A concurrent-start case races inside its own per-test home, never by serialising the suite.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:932) No retry policy may make the race witness or its single-instance control pass.
  > - NEVER add retry-once policies for flaky tests (masks real failures)
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
  > - NEVER skip quality gates "just this once"
- (.andromeda/test-plan.md:946) The two-sided witness must open its race window by an explicit signal, not by a test-owned sleep or elapsed-time verdict.
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.

## Bindings
- **Coverage profile channel ↔ §11 Integration**: the category's `LLVM_PROFILE_FILE` rule (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) is the same ban as §11's `.env_clear()` line; a concurrent-start case spawning `viola` honours both (.andromeda/test-plan.md:816).
- **Kill-shaped profile mechanism ↔ §7 Fake agent**: the hangup mechanism (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) is closed by the fake agent's own process-group and SIGHUP exit rules in §7; a two-instance race case leans on them, not on a new seam (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9).
- **Mutation records ↔ §3 `run` step 4 (Verdict)**: a `cfg(windows)` arm in the exe pin is left out as host-excluded by the harness or recorded owed (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16); the `fs` retry restatement keeps its Epoch 4 kills (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:17).
- **Perf boot ↔ `run`'s start (`pin_bytes`)**: suite `perf` boots through the pin write the chunk changes (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25), its rows read against the newest reading (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26).
- **Release-build measurement ↔ §9 CI Integration and security-plan §Dependencies and CI**: the clean builds are `release-check.sh`'s (.andromeda/test-plan.md:760), and its verdict lines stand as build-failure conditions (.andromeda/test-plan.md:795, .andromeda/test-plan.md:861).
- **Hash-before-reuse ↔ security-plan §Local trust boundary (Home)**: the tampered-copy exit 1 (.andromeda/test-plan.md:214) and the hash's literal vectors (.andromeda/test-plan.md:620) are the test side of the re-hash rule the scope keeps unchanged.
- **Zero-flakiness ↔ the scope's two-sided witness**: the 6-of-24 race must read red deterministically before the fix and clean after it, under `retries = 0` and the 120 s kill (.andromeda/test-plan.md:854, .andromeda/test-plan-detail/quality-gates-coverage-targets.md:19, .andromeda/test-plan.md:946).
