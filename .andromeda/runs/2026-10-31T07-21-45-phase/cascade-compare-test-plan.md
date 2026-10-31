# Cascade compare — test-plan · phase · 2026-10-31-sanitised-error-surfaces
E items 250 · held 50 · lost at classify 4 · lost at strong 3 · beside a placed fact - · cascade only 36 · listed 5 · body 133 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.7 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:35
- (.andromeda/test-plan-detail/ci-integration.md:35) The chunk's plan must list no mutation gate and no mutation CI job.
  > no push or pull-request job and no gate since 2026-09-28: mutation testing runs at the epoch boundary through `/andromeda-code-audit`, and on demand through `agent-run run --mutants` [`--file`] (§3 `run` step 4, §10 Mutation gate).

### ci-integration#1.9 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) The chunk's code and unit cases must compile and pass on the Rust 1.96 floor as well as on the pinned stable.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:25
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) Any error-surface or `channel-response` change that reaches a hook's logging or channel path falls inside the timed `--perf` rows and must be judged by them.
  > so every timed hook takes the real logging and channel path, never the no-instance no-op (obs-plan §10 obs-overhead row)

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) This is the perf gate's newest recorded Linux reading, the comparison line for a change in the timed path; the gate itself stays `SPINE_DEADLINE`.
  > with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).

### test-data-fixtures#1.2 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:11
- (.andromeda/test-plan-detail/test-data-fixtures.md:11) A witness that drives a path-bearing refusal by widening a home tree on Windows x64 must start that home unseeded.
  > so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded (`run_refuses_a_widened_home_tree_before_its_log`).

### test-data-fixtures#1.6 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:15
- (.andromeda/test-plan-detail/test-data-fixtures.md:15) A witness that drives a strict-modes refusal with a path in play must widen through these helpers, with the `icacls` exit asserted.
  > Set by the test on the real file, never by viola: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted).

### test-data-fixtures#1.10 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:19
- (.andromeda/test-plan-detail/test-data-fixtures.md:19) A fixed-message change that reaches a human-mode line must move the expected output in its trycmd case or test-file literal.
  > trycmd `tests/cmd/*.toml`, with snapbox redactions for timestamps, cursors and paths;

### test-data-fixtures#2.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:29
- (.andromeda/test-plan-detail/test-data-fixtures.md:29) A witness that drives free text from a server that is not the recorded wrapper must take this impostor form, the only case that writes an `InstanceSnapshot` itself.
  > - The server-identity impostor cases (Linux and Windows: `served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves, recording a held live process's pid (the fake agent's holder, below) beside an endpoint the test binds, so that a server that is not the recorded wrapper answers the frame-writing clients.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:39
- (.andromeda/test-plan-detail/integration-test-strategy.md:39) Every witness home must stay a not-yet-existing path under `target/e2e-home`, so the planted stripped `CLAUDE*` value and the error bodies stand inside the CI gates and the secret scan.
  > there is a fresh tempfile home per test (the rstest `home` fixture). Like harness homes (§3 `boot` step 2), it is a not-yet-existing `home` path inside a `tempdir_in("<workspace root>/target/e2e-home")` parent, so viola creates it and obs-plan's CI gates, secret scan and upload cover integration runs too.

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:39
- (.andromeda/test-plan-detail/integration-test-strategy.md:39) A no-absolute-path oracle must hold on the Linux dev host, where the home's path passes through this link, as well as on CI, where no link exists.
  > on the Linux dev host `target/e2e-home` is an operator-set symbolic link to the owner-only tmpfs directory `/tmp/viola-e2e-home-<uid>`, so the homes' bytes sit outside the working directory while every home class keeps its path, viola still creates the home itself, and G2, G4 and the secret scan reach it through the link.

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:40
- (.andromeda/test-plan-detail/integration-test-strategy.md:40) A witness that drives a check-guarded failure on every OS must start from `stamped_home`, not a plain home.
  > A case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).

### integration-test-strategy#3.0 · listed (lost at classify) · .andromeda/test-plan-detail/integration-test-strategy.md:40
- (.andromeda/test-plan-detail/integration-test-strategy.md:40) The outside-scan home belongs to this one test alone: no witness of this chunk may boot there, and no third carve-out from the scans is allowed.
  > `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, a home under the system temp dir (`viola-chaos-*`), not under `target/e2e-home`, because its forced vt100 feed panic writes a G2-counted `event:"panic"` line; that home is therefore outside G2 (zero panics), G4 (schema conformance) and the secret scan, and the test asserts its panic line and its `parse-rejected{vt100-feed, panicked}` line present itself.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every witness and owed case that starts a `viola` child with a cleared or hand-built environment (the planted stripped `CLAUDE*` value among them) must re-add the profile variable, so the error paths it drives count toward the coverage gate.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A witness that kills a child to drive a failure must not kill an instrumented child that exits by itself during its exit-time profile write, because the short profile reds the coverage merge.
  > an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:9
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A kill-shaped witness (a killed wrapper driving MCP `instance-unreachable` or a channel failure) must run its hooks through the fake agent's own-process-group spawn, so the hangup cuts no `viola hook` profile.
  > a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) The chunk must not add or claim a chunk-level mutation gate, and its acceptance must not require a mutation run; its product changes are scored at the epoch-boundary audit.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) Each new product error mapping or fixed message must be held by a nextest case, never by a doctest alone, or the later boundary score reads its mutant missed.
  > A mutant caught only by a doctest counts as missed under nextest, so the missing unit test must be added.

### test-data-fixtures#1.3 · listed (lost at strong) · .andromeda/test-plan-detail/test-data-fixtures.md:12
- (.andromeda/test-plan-detail/test-data-fixtures.md:12) A witness that drives a stamps failure or needs a stamped home must get the file from `viola verify`, never write it.
  > `viola verify --home <home>` run against the fake agent. No test or harness code writes the file.

### test-strategy#2.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) Any new root witness for the CLI or channel surface must take its child runner, event wait and home widening from the shared helpers and define no copy of them.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.

### test-strategy#2.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:43
- (.andromeda/test-plan-detail/test-strategy.md:43) Any witness that needs the rmcp client or spans the surfaces must live in the test-only crate under its naming form.
  > **Tokio-client E2E suites and the harness binary `viola-harness`:** `crates/viola-e2e/` (publish = false). This covers the rmcp client, cross-surface parity and critical-path scenarios, as `tests/{path<N>,mcp,cross}_<topic>.rs`

## Held

- ci-integration#1.1
- ci-integration#1.11
- ci-integration#1.6
- ci-integration#1.8
- e2e-test-strategy#1.1
- e2e-test-strategy#1.4
- e2e-test-strategy#1.5
- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#19.1
- e2e-test-strategy#2.3
- e2e-test-strategy#4.3
- e2e-test-strategy#6.3
- e2e-test-strategy#7.3
- e2e-test-strategy#8.2
- e2e-test-strategy#9.1
- e2e-test-strategy#9.2
- integration-test-strategy#1.3
- integration-test-strategy#1.4
- integration-test-strategy#4.1
- integration-test-strategy#4.2
- integration-test-strategy#5.1
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.4
- mocking-stubbing-discipline#2.5
- test-data-fixtures#1.1
- test-data-fixtures#1.5
- test-data-fixtures#1.7
- test-data-fixtures#2.2
- test-data-fixtures#5.1
- test-data-fixtures#6.1
- test-data-fixtures#7.1
- test-scope-summary#2.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.2
- test-scope-summary#5.3
- test-scope-summary#7.1
- test-scope-summary#9.1
- test-strategy#1.1
- test-strategy#1.2
- test-strategy#1.7
- test-strategy#3.16
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#1.3
- unit-test-strategy#1.4
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#1.2
- e2e-test-strategy#17.1
- e2e-test-strategy#3.1
- e2e-test-strategy#3.3
- e2e-test-strategy#4.1
- e2e-test-strategy#4.2
- e2e-test-strategy#5.1
- e2e-test-strategy#5.3
- e2e-test-strategy#6.1
- e2e-test-strategy#6.2
- e2e-test-strategy#7.1
- e2e-test-strategy#7.2
- e2e-test-strategy#8.1
- e2e-test-strategy#8.3
- integration-test-strategy#1.6
- test-data-fixtures#1.9
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#8.1
- test-scope-summary#8.2
- test-scope-summary#9.2
- test-strategy#1.3
- test-strategy#1.6
- test-strategy#1.8
- test-strategy#3.1
- test-strategy#3.10
- test-strategy#3.12
- test-strategy#3.13
- test-strategy#3.14
- test-strategy#3.15
- test-strategy#3.3
- test-strategy#3.4
- test-strategy#3.5
- test-strategy#3.6
- test-strategy#3.7
- test-strategy#3.8
