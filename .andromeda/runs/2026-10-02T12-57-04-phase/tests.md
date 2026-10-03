# tests extract

## Relevance
relevant: the chunk is test infrastructure end to end. It covers the in-repo suite red (M2), the coverage re-take, the mutation survivors (M1), the tempdir leak, the P6 harness-grammar test, the P2 deadline lint and the P5 process-lifetime measurement.

## Constraints
- **Open red, zero-flakiness.** Per test-plan §10 Zero-flakiness budget, a flake is quarantined by keeping the chunk red until its root cause is fixed. Nextest `retries = 0` and Playwright `retries: 0` hold, and `#[ignore]` / `test.skip` are never a parking place. M2 cannot close by re-run, retry or `#[ignore]`. Raising a wait bound is not a §10-sanctioned fix; it is the open-red question the scope already frames.
- **Coverage re-take.** The re-take uses the §3 `run` `--coverage` arm (5-command-implementation.md, `run`). That is the single `coverage` suite under `cargo llvm-cov nextest`, followed by the `doctest` suite, against the §10 Comprehensive floors (lines ≥ 85, functions ≥ 95, regions ≥ 80) per OS. The `COVERAGE_IGNORE` regex stays the four trees named in §10 Stack adjustments.
- **Mutation verdict.** Per test-plan §10 Mutation gate and §3 `run` step 4 Verdict, the verdict is `missed == 0 && timeout == 0 && unviable <= caught`, counted from the run's own `outcomes.json`. On a Windows host that file is under `<repo parent>/viola-mutants-scratch/mutants.out/`. The exit code alone is never trusted. A mutant caught only by a doctest counts as missed, so it needs a nextest unit test (§10).
- **Owning-package kill rule.** Per §3 `run` step 4 Test scope (`--test-workspace` unset), each M1 survivor must be killed by its owning package's own tests:
  - the `sideload.rs` and `lib.rs` survivors by viola-pty tests
  - the `fs.rs` and `pin.rs` survivors by viola-state tests
  - the `src/cmd/run.rs` `refuse_stale` survivor by root-package tests

  A root integration test or a viola-e2e scenario never counts as the kill for a crate mutant. Whether the existing crate-level tests already reach these effects is research's question.
- **Tempdir cleanup.** Per test-plan test-data-bootstrap.md Cleanup:
  - `TempDir` drop removes per-test homes.
  - A runner-killed test's home is reclaimed only through its `owner.json` record (pid dead, or a different start time). Reclamation is never by age or name, and never touches a dir without a record.
  - `AGENT_RUN_KEEP_HOMES=0` is forced under `run --mutants`.

  The `.tmp*` leak fix must stay inside this rule; a sweep by name pattern is not allowed. Where the 545 dirs come from is research's question. Candidates include tempfile uses outside `TestHome`, the `<temp dir>/viola-root-watch/` reports kept on failure or runner kill (§3 `run` step 2), and the cargo-mutants copy under the host scratch.
- **Deadline below the kill line.** Per §3 `run` step 2 (5-command-implementation.md), the shared root wait `WITHIN` (7 s, `tests/support/watch.rs`) sits strictly below the nextest `mutants` profile's 10 s kill and below cargo-mutants' 20 s auto-timeout floor. A stuck wait therefore fails the test itself and grades a mutant as caught, not Timeout. `[profile.ci]` keeps its 120 s kill (§10 Mutation gate). The P2 lint must assert that relation against the profile values it reads, not against copied literals. Which constants and profiles are in scope is P3's question.
- **Harness grammar.** Per test-plan §3 Exit codes, a flag or selector whose surface is not built yet is a usage error until its chunk lands, never a vacuous pass. The new set is closed (§3 Closed enums, a new value needs a Decisions Log entry). The P6 rework must keep a witness of that rule for the selectors that are still unbuilt, while no longer going red as each one gets built.

## Patterns to follow
- Use `scripts/agent-run.sh run --mutants --file <path>` (repeatable) for the M1 inner fix-loop. It yields `verdict:"scoped"`, and survivors stay red, per §3 `run` Test selection and Output format. The counted, boundary-wide run stays `/andromeda-code-audit`'s, per §10.
- Name tests `<subject>_<condition>_<expected>` (§2 Test function naming).
  - Root and the sync crates use table cases with rstest `#[case::label]` (§4 Conventions).
  - viola-e2e has no `[dev-dependencies]` and uses a labelled `(label, …)` case table inside one `#[test]` (§4 Conventions). This applies to the P6 test in `crates/viola-e2e/tests/cli.rs`.
- Make every new check two-sided: a planted input proves it red and a control proves it clean. Precedents are the fixture-hygiene violation classes and the `drift` comparator's planted swap (§7 Fixture hygiene, §6 Contract suite). The P2 lint and any M1 killing test take this shape, so neither can pass vacuously.
- Assert the observable effect. Existing resize witnesses observe a resize through the child's key-free size watcher and the `size` receipt: `spawn_reports_a_resize_to_a_child_that_reads_no_key` and `pump_forwards_a_resize_that_lands_before_its_first_look` (§5 Module ↔ PTY). The DLL-search state already has a two-sided unit test, `restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load` (§5 Module ↔ PTY). Fault seams use mockall doubles of existing seam traits (`Pty`, liveness, `Clock`) only (§4, §8).
- For P5, exit and liveness are judged by pid + start time and by `child.wait()` on the handle, never by master EOF. This follows §6 Chaos suite (the no-EOF mode, `exit_source:"handle-wait"`) and §11 E2E. Real-CLI behaviour is local-only (§1 Untestable zones, §11 Test Strategy).

## Anti-patterns to avoid
- Never reach green by masking:
  - setting nextest `retries` above 0, or using `#[ignore]` (§11 CI, §11 Quality)
  - widening the coverage `--ignore-filename-regex` (§11 CI)
  - lowering a §10 floor (§11 Quality)
  - trusting the cargo-mutants exit code instead of the `outcomes.json` counts (§11 Quality)
- Never use `sleep(N)`, a test-owned timer or an elapsed-time verdict for synchronisation (§11 E2E, §11 Universal). Never use `std::env::set_var`; set env per child with `Command::env` (§11 Integration). Never call `.env_clear()` without re-adding `LLVM_PROFILE_FILE` (§11 Integration). This matters for the M2 diagnosis, which already unsets inherited `CLAUDE*` / `VIOLA_*` / `ANTHROPIC*`.
- Never test private internal state to kill a mutant: test the public API and observable behaviour (§11 Unit). Never run the real `claude` in CI, and never treat a fake-agent or Linux-only result as proof for the real CLI or for Windows (§11 Test Strategy). This applies to P5's two legs and to the host-reds CARRY.

## Contract bindings
- **tests ↔ obs (homes root).** Every rstest and harness home lives under `target/e2e-home/`, which obs-plan §9's G2, G4, secret scan and `diag-<os>` upload read. Sources: test-plan §5 Setup/teardown lifecycle; 5-command-implementation.md `boot` step 2, `cleanup` step 6 and `logs` Retention. CI keeps homes through `AGENT_RUN_KEEP_HOMES=1`. The tempdir-leak fix and any M2 change to where homes land must keep this root and the kept-home semantics.
- **tests ↔ the arch/host scratch.** On a Windows host the mutation run's `TMP`/`TEMP` and `--output` are the host scratch `<repo parent>/viola-mutants-scratch`. A wipe failure is `scratch-wipe-failed`, which signals a leaked process holding a file (§3 `run` step 4 Command). The M1 scoped runs and the leak count read from there.
- **tests ↔ security (test seams).** Any new env-var test seam for M1 or P5 is bound to security.md's seam list and its Decisions Log rule. Today only `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC` exist, both `fake-agent`-only. A new seam is a boundary to show at P4, not one to add silently.

## Acceptance criteria contributions
- (tests) M2 is closed:
  - its cause is named with a witness
  - `scripts/agent-run.sh run --coverage` then `viola-harness gate --require coverage,doctest` read `ok:true` on the in-repo tree on this host
  - the coverage numbers (lines, functions, regions) are recorded as the code-audit correction and meet the §10 floors

  (per test-plan §3 `run` `--coverage` / `gate`, §10 Coverage thresholds)
- (tests) Each of the eight M1 survivors is either caught or exempted:
  - caught means a `run --mutants --file <touched source>` run reads `verdict:"scoped"` with that mutant `caught`, and `missed == 0`, `timeout == 0`, `unviable <= caught` counted from the scratch `outcomes.json`
  - exempted means a recorded equivalent-mutant exemption that names why

  (per test-plan §10 Mutation gate, §3 `run` step 4 Verdict / Test scope)
- (tests) One full suite run leaves no new `.tmp*` dir, shown by a before/after count. The fix reclaims only through `TempDir` drop or the `owner.json` record rule (per test-plan test-data-bootstrap.md Cleanup).
- (tests) The P2 deadline lint is two-sided: it fails on a planted test deadline at or above the nextest kill line it reads, and passes on the tree, with its scope stated (per test-plan §3 `run` step 2 `WITHIN` < `mutants` 10 s kill < 20 s cargo-mutants floor, §10 Mutation gate).
