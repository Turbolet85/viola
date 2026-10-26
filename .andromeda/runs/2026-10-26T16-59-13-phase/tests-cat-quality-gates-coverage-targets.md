# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan.md:810) The chunk's new strict-modes, DACL, re-hash and plugin-rewrite code must hold the Comprehensive tier's coverage thresholds.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:813) The thresholds hold per OS job, so the chunk's `cfg(windows)` DACL / `conpty/` strict-modes code must be covered on `windows-2025` and its `cfg(unix)` mode checks on the Unix runners, each on its own runner.
  > - Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every new test that spawns `viola` (a hook strict-modes negative, a 0770 `--home` negative, a seeded-home case) and clears its env must re-add `LLVM_PROFILE_FILE`, or that run drops out of the coverage number.
  > - Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan.md:921) Coverage of the new code may not be reached by widening the ignore regex.
  > - NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:930) Coverage of the new code may not be reached by lowering a threshold.
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) The chunk's gate set carries no mutation gate: mutation is judged at the Epoch 6 boundary audit or on demand, never as a chunk gate.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The perf session's hooks must keep taking the real logging and channel path once the hook runs the strict-modes check before its snapshot read: the boot-made perf home must pass that check, or the timed rows fall to the fail-open no-op the gate forbids.
  > hyperfine runs with the repo root as cwd and `VIOLA_NAME=builder` / `VIOLA_DIR=<perf home>/instances/builder` set, so every timed hook takes the real logging and channel path, never the no-instance no-op (obs-plan §10 obs-overhead row)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) A perf boot, row, panic check or cleanup the new home checks or pinned-exe re-hash break must turn suite `perf` red with the failure named, never a skipped row.
  > A failed build (`build`), boot (`boot: <reason>`), row (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or skipped row.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) The hook's strict-modes check joins the server check in the timed path; this is the newest reading the chunk's `run --perf` restates against.
  > Every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md` (Outcome, `run --perf`).
- (.andromeda/test-plan.md:839) With the strict-modes check added before the hook's snapshot read, every timed hook row must still pass its `max` gate.
  > | `viola hook session-end` (synthetic payload via `--input`, live wrapper), per OS | mean, median, stddev | `max < 1.0 s` (`jq -e '.results[0].max < 1.0'`) |
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
  > | `viola hook pre-tool-use` on an unverified CLI (`hook.dialog` → `null` at once), timed by `run --perf` and required by `perf::ROWS` | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:833) A perf breach the added check causes is fixed in the product or escalated, never re-run away.
  > - **Verdict:** `viola-harness gate` evaluates the exported JSON once; the `jq -e` expressions below are equivalent. There is no rerun, no outlier trimming and no second attempt. A breach is fixed in the product or escalated through a Decisions Log entry, never retried away.
- (.andromeda/test-plan.md:835) The hook's new strict-modes refusal (failing open) is a fail-open case that must also hold the spine bound at unit speed.
  > The fail-open matrix's `Instant` bound (`hook_fails_open_silently_within_the_spine_bound`, each case `< 1.0 s`) still asserts the spine bound at unit speed as well.
- (.andromeda/test-plan.md:848) The hook deadline holds at unit speed inside the assert_cmd hook tests as well as under hyperfine.
  > The same deadlines are also asserted at unit speed inside the assert_cmd hook tests, with a `std::time::Instant` bound.
- (.andromeda/test-plan.md:851) The `pin_exe` race (CARRY 16: concurrent fresh-home boots failing 6 of 24) is a flake the chunk may not retry or park: the chunk stays red until its root cause is fixed within it.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:854) The chunk's Windows proof is CI's `windows-2025` leg, so a red on any OS, test or coverage, fails the build.
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)

## Bindings
- **Per-OS coverage gate ↔ §9 CI Integration**: the thresholds are read in each OS's one instrumented `run --coverage` job, on `windows-2025` after the ConPTY vendor verification, ending in `viola-harness gate --require <suites>`; the chunk's Windows DACL code is proven there (.andromeda/test-plan.md:813) (.andromeda/test-plan.md:747)
- **Perf hook rows ↔ security.md §Local trust boundary (the `hook.event` strict-modes exception closed, CARRY 3)**: the hook's new check before its snapshot read sits in the timed path and must pass on the boot-made perf home (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26)
- **Fail-open `Instant` bound ↔ §6 Security sweep**: the sweep's `--home` strict-modes refusals and the hook fail-open matrix are the cases the unit-speed spine bound covers (.andromeda/test-plan.md:835) (.andromeda/test-plan.md:598)
- **`LLVM_PROFILE_FILE` re-add ↔ §11 Integration ban**: the coverage-channel rule and the `env_clear()` ban are one requirement on every new spawning test (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) (.andromeda/test-plan.md:891)
- **Zero-flakiness budget ↔ chunk scope CARRY 16 (`pin_exe` race)**: the race is closed in the product's home copy of the binary, not by a retry or the one-instance test workaround alone (.andromeda/test-plan.md:851)
