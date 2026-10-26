# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan.md:813) The new server-verification code in every client (CLI verbs, `viola hook`, `viola mcp`'s Tokio client) and its OS arms must hold the coverage gate on each OS job, its `cfg(windows)` / `cfg(unix)` / macOS arms covered on their own runner.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:854) A mismatch test that passes on Linux but fails or under-covers on `windows-2025` or `macos-latest` fails the build.
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) The new tests that drive `viola` children (CLI verbs, hooks, `viola mcp` against a mismatched or verified server) must keep their profiles in the coverage run; a test that clears the env must re-add the profile variable.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A new case that kills a wrapper so that the hook's or a verb's server check runs against a gone or replaced server must not reopen the refused-profile red; the fake agent's process-group discipline must hold for it.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none;
- (.andromeda/test-plan.md:921) A test-only stand-in server or helper the mismatch tests need must not be reached by widening the coverage exclusion.
  > NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:834) The hook client's server verification (peer pid, the `sysinfo` start-time read, the snapshot comparison) must fit inside the hook's spine budget, and its connect bound must stay below the gate the harness imports.
  > **Spine deadline:** the gate reads the product constant `viola_core::SPINE_DEADLINE` (1.0 s), imported by `viola-harness` (`gate.rs` `perf()`), never an edited literal in `ci.yml` or this table. The hook's own connect deadline is the crate-private, provisional `CONNECT_DEADLINE` = 750 ms in the root bin's `src/cmd/hook.rs`, below the gate and asserted below it by `connect_deadline_is_below_the_spine_deadline`.
- (.andromeda/test-plan.md:840) Every timed hook row (the spine hooks' `hook.event` and `pre-tool-use`'s `hook.dialog`) now verifies the server before its frame and must still read `max` under the spine deadline.
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
  > | `viola hook pre-tool-use` on an unverified CLI (`hook.dialog` → `null` at once), timed by `run --perf` and required by `perf::ROWS` | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The perf rows must time the verified path, so the verification has to run on the real logging and channel path the perf session drives.
  > hyperfine runs with the repo root as cwd and `VIOLA_NAME=builder` / `VIOLA_DIR=<perf home>/instances/builder` set, so every timed hook takes the real logging and channel path, never the no-instance no-op (obs-plan §10 obs-overhead row)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) A verification that makes a perf row fail, or panic inside the hook, must turn suite `perf` red by name, never drop the row.
  > A failed build (`build`), boot (`boot: <reason>`), row (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or skipped row.
- (.andromeda/test-plan.md:833) If the added verification breaches a hook's `max`, the chunk must fix it in the product or escalate it, not re-run the gate.
  > There is no rerun, no outlier trimming and no second attempt. A breach is fixed in the product or escalated through a Decisions Log entry, never retried away.
- (.andromeda/test-plan.md:835) The hook's new fail-open-on-mismatch case (exit 0, no body, no frame) must hold the same unit-speed spine bound as the rest of the fail-open matrix.
  > The fail-open matrix's `Instant` bound (`hook_fails_open_silently_within_the_spine_bound`, each case `< 1.0 s`) still asserts the spine bound at unit speed as well.
- (.andromeda/test-plan.md:842) The pid + start-time match the clients make must agree with the liveness classifier's boundary rule: a pid or start-time mismatch is `gone` at any heartbeat age.
  > | Heartbeat staleness flip (`viola-state` `classify`, injected age) | n/a | same process: 5.0 s → live, 5.1 s → stale; pid or start-time mismatch → gone at any age (exact boundary test) |
- (.andromeda/test-plan.md:851) Any timing-sensitive mismatch case (CARRY 2's Windows hook reaching an exiting wrapper's pipe included) must be deterministic: a flake keeps the chunk red until its root cause is fixed, with no retry and no `#[ignore]`.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:929) The chunk must not lower a threshold or skip a gate to land the verification.
  > - NEVER add retry-once policies for flaky tests (masks real failures)
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
  > - NEVER skip quality gates "just this once"
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) This chunk has no mutation gate of its own; its code is scored at the Epoch 6 boundary audit.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) The chunk's Windows-only verification arm (the pipe's server process id) will be scored at that audit by this rule, recorded by coordinate as owed, never as equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught;

## Bindings
- **gate-coverage ↔ body §1 Coverage triggers (IPC)**: the per-OS coverage gate (.andromeda/test-plan.md:813) covers the negative tests §1 requires for this chunk, "server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body" (.andromeda/test-plan.md:184).
- **gate-coverage per OS ↔ body §1 multi-os-compat and §9 matrix**: the macOS euid + dir-only server verification is an OS-branch test (.andromeda/test-plan.md:250) on `macos-latest` (.andromeda/test-plan.md:767), gated per OS (.andromeda/test-plan.md:813).
- **gate-perf ↔ obs-plan §10 obs-overhead row**: the perf session's real logging and channel path (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) is the path the hook's verification is added to, and its timing is bounded by `SPINE_DEADLINE` / `CONNECT_DEADLINE` (.andromeda/test-plan.md:834).
- **spine deadline ↔ the hook fail-open invariant (CLAUDE.md, security-plan)**: the hook's mismatch case exits 0 with no body inside the unit-speed bound (.andromeda/test-plan.md:835), so the fail-open matrix gains the mismatch case.
- **zero-flakiness ↔ test-plan §3 `run` step 2 and scope CARRY 2**: tests wait for the endpoint to be gone, and any mismatch case on the exiting-wrapper window must be deterministic (.andromeda/test-plan.md:851).
- **gate-mutation ↔ body §3 `run` step 4 (Verdict)**: the host-excluded records and the owed-by-coordinate rule for `cfg(windows)` arms (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) apply at the Epoch 6 boundary audit, not in this chunk (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15).
