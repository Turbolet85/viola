# Cascade compare — test-plan · phase · 2026-10-25-windows-endpoint-admission
E items 194 · held 29 · lost at classify 6 · lost at strong 2 · beside a placed fact - · cascade only 20 · body 134 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.7 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:34
- (.andromeda/test-plan-detail/ci-integration.md:34) If the chunk adds a Windows-gated file, the dispatch-only Windows mutation leg runs per package over its Windows-gated files, so the new file must be within that leg's scope for the boundary audit.
  > The audit's Windows leg is `windows-mutants.yml`: `workflow_dispatch` only, no inputs, ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item (the count of its `- package:` items; the root package `viola` is five of them, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs` since chunk 2026-10-13-interrupted-verify-cleanup), each running `scripts/agent-run.ps1 run --mutants --package <member> --file …` over that item's Windows-gated files, report-only, no cache or upload

### integration-test-strategy#3.1 · lost at strong · .andromeda/test-plan-detail/integration-test-strategy.md:42
- (.andromeda/test-plan-detail/integration-test-strategy.md:42) The chunk's channel-level listener tests belong in the crate or root channel suites.
  > `send`/`wait`/`last`/`answer`/`pause`/`release`/`link`/`unlink`/`hook.dialog`/`hook.event` via `crates/viola-channel/tests/` and root `tests/channel_*.rs`

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:19
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:19) The CARRY's fix and any new Windows test take no retry: the CI profile's kill and zero retries stand.
  > `[profile.ci]`'s 120 s kill and `retries = 0` are unchanged.

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) A mutant in the chunk's `cfg(windows)` listener code is unreachable on the Linux dev host and is recorded as owed by coordinate if an on-demand run meets it, never equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught; `missed == 0` then reads over the measurable set (the overseer's ruling, 2026-10-04).

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:18
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:18) The chunk's Windows-only code is scored at the Epoch 6 boundary by the dispatch-only Windows mutation workflow, so its new tests must be able to kill mutants there.
  > The boundary audit measures the `cfg(windows)` code by dispatching `windows-mutants.yml` (founder ruling C2, 2026-10-04; §9 Mutation row): `run --mutants --package <member> --file …` per matrix item on `windows-2025` (§9 Mutation row), verdict `package`, report-only and never a gate.

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:25
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The per-OS perf job times hooks through the real channel path, so any change to the Windows listener's open or admission path lands under the hook deadline gate on `windows-2025`.
  > hyperfine runs with the repo root as cwd and `VIOLA_NAME=builder` / `VIOLA_DIR=<perf home>/instances/builder` set, so every timed hook takes the real logging and channel path, never the no-instance no-op

### test-data-fixtures#1.3 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:12
- (.andromeda/test-plan-detail/test-data-fixtures.md:12) The CARRY's `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` fix must keep its `budget.json` coming from the writer path (statusline stdin through `viola hook statusline` in a booted-wrapper test; a reader unit case only altering `write_budget`'s own bytes), never a hand-planted file.
  > | `budget.json` | Generated statusline stdin piped into `viola hook statusline`, in every booted-wrapper test. The reader's own unit cases (`budget_read_*`, `crates/viola-state/src/budget.rs`) take the bytes `write_budget` wrote and pad, replace or re-version them, because no writer produces an over-cap, unparseable or `v` 2 file | per-test |

### test-scope-summary#6.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:80
- (.andromeda/test-plan-detail/test-scope-summary.md:80) The harness `run`'s first step is the workspace suite under plain `cargo test`, the runner under which the folded CARRY's `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 went red; the case must pass there, not only under nextest.
  >       1. the workspace unit and integration tests (`cargo test`)

### test-scope-summary#6.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:84
- (.andromeda/test-plan-detail/test-scope-summary.md:84) The harness `run` is green only when every suite passes, so a red case_4 under `cargo test` keeps the chunk red.
  >     - Exit 0 only if every suite passes and no mutant survives. Non-zero otherwise, with a JSON summary on stdout (suite, passed, failed, survived).

### test-scope-summary#6.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:92
- (.andromeda/test-plan-detail/test-scope-summary.md:92) Any change to the listener's creation must keep `cleanup`'s check that the pipe no longer exists after the wrappers stop.
  >     - Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.

### test-strategy#1.7 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:28
- (.andromeda/test-plan-detail/test-strategy.md:28) The chunk carries no mutation acceptance criterion: mutation runs only at the epoch boundary or on demand.
  > No per-chunk, pre-push or CI mutation gate.

### unit-test-strategy#1.2 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:22
- (.andromeda/test-plan-detail/unit-test-strategy.md:22) The precedent for a Windows case a hosted runner cannot provision: synthetic security descriptors built from SDDL strings fed to a check function; a DACL-effect case (c) that cannot be measured on `windows-2025` follows this stub form or is owed as a Windows-only live measurement.
  > Windows strict-modes feeds synthetic security descriptors built from SDDL strings (via windows-sys) to the descriptor-check function. This covers a foreign-owner SID, a NULL DACL, an inherit-only GENERIC_WRITE ACE, a read grant on `diagnostics/`, and `CREATOR OWNER` treated as the user. These are the stubs for the cases hosted runners cannot provision (test-scope Sec 1).

## Held

- ci-integration#1.1
- ci-integration#1.6
- e2e-test-strategy#1.4
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#2.1
- e2e-test-strategy#2.2
- e2e-test-strategy#2.3
- e2e-test-strategy#2.4
- e2e-test-strategy#9.2
- integration-test-strategy#1.4
- integration-test-strategy#2.0
- mocking-stubbing-discipline#1.1
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.4
- quality-gates-coverage-targets#1.1
- test-data-fixtures#1.1
- test-scope-summary#1.1
- test-scope-summary#5.2
- test-scope-summary#8.1
- test-strategy#1.1
- test-strategy#1.2
- test-strategy#2.1
- test-strategy#3.1
- test-strategy#3.13
- test-strategy#3.18
- unit-test-strategy#1.1
- unit-test-strategy#2.1

## Cascade only

- ci-integration#1.12
- ci-integration#1.2
- ci-integration#1.3
- ci-integration#1.4
- ci-integration#1.5
- e2e-test-strategy#1.3
- e2e-test-strategy#1.6
- e2e-test-strategy#7.1
- e2e-test-strategy#7.2
- e2e-test-strategy#7.3
- integration-test-strategy#4.1
- mocking-stubbing-discipline#2.2
- test-data-fixtures#1.12
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.1
- test-scope-summary#7.1
- test-scope-summary#9.2
- test-strategy#1.3
- test-strategy#3.8
