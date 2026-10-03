# Codebase Research — 2026-10-02-epoch-2b-cleanup

## Scope
- **Depth:** deep on M1's seven sites, the root fixture chain and the leak; moderate on P2/P5/P6 · **Reads:** 19 · **Globs/Greps:** 21 · host probes (read-only): 7
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 7 Session Additions applied (2026-09-25 host-excluded misses; 2026-09-25 never pipe `boot`; 2026-09-26 judge by verdict, not counts; 2026-09-27 `test(=tests::name)`; 2026-09-29 unbuilt-selector gate is a plan defect; 2026-09-29 same-day control) · `.claude/rules/testing.md` — read in full, 20 Session Additions applied (notably 2026-09-24 deadline below the kill line, 2026-09-25 remove-the-guard run, 2026-09-27 crate-local security tests, 2026-09-28 retarget the unbuilt selector, 2026-09-28 a timing red is never fixed by raising a bound, 2026-09-29 `viola never exited` is the stamped-home verify step)
- **Platform issues consulted:** none — no runner-only bullet (every CI run Setup read is green) and no CI-reading entry outside the operator leg

## Files inspected
- `tests/support/home.rs` (full) — `TestHome::new` sweeps `target/e2e-home` by owner record before each home (:94-110); `Drop` keeps a home only under a keep flag (:122-135); `seed_conpty` (:143-180); `stamped_home` runs `viola verify` (:219-243); `Wrapper::wait_ready` / `stop_keep` (:296-365).
- `tests/support/verify.rs` (full) — `wait_bounded` kills a `viola` child still running at `WITHIN` and panics `"{what} never exited"` (:70-84); `verify()` is the stamped-home step (:130-144). So M2's top panic ("viola never exited" ×32) is a `viola verify` (or other `viola` child) running past 7 s.
- `tests/support/watch.rs` (full) — `WITHIN` = 7 s (:14); reports land in `std::env::temp_dir()/viola-root-watch/` (:24), so the watch evidence follows `TMP`/`TEMP`.
- `.config/nextest.toml` (full) — kill lines: `mutants` 5 s × 2 = 10 s (:22), `package(viola-e2e)` 15 s × 2 = 30 s (:28), `ci` 30 s × 4 = 120 s (:12); cargo-mutants' 20 s floor is stated in the comment (:17-19).
- `crates/viola-pty/src/sideload.rs` (full) — `search_restricted` (:33-35) reads the `RESTRICTED` static that `restrict_dll_search` sets (:23-30); the module's tests never call `search_restricted`.
- `crates/viola-pty/src/lib.rs` (150-390, 405-830) — `PortablePty::resize` (:194-199); `HostTerminal` enter/drop (:287-343); the child-entry rig `pty_child_entry` reports `size WxH` from a watcher and `restored=true|false` after dropping the guard (:438-483); tests that wait on both: `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` (:731-764, waits `size 120x40` :749 and `restored=true` :752), `spawn_delivers_…_not_yet_reading` (:768-787, `restored=true` :784), `spawn_reports_a_resize_to_a_child_that_reads_no_key` (:790-805, `size 120x40` :801). None is `cfg`-gated.
- `crates/viola-state/src/fs.rs` (1-145) — `restrict` is `#[cfg(unix)] set_permissions`, and on every other target `let _ = (path, mode); Ok(())` (:16-27); its Unix witness is `#[cfg(unix)] fs_private_modes_are_explicit` (:346-348).
- `crates/viola-state/src/pin.rs` (1-205) — `pin_exe`'s match (:78-85): `Ok(found)==hash` → reuse, `Ok(_)` → `HashMismatch`, `Err(NotFound)` → `replace_private_shared`, other `Err` → returned; `pin_companions`' same shape over `open_held_waiting` (:168-176).
- `src/cmd/run.rs` (225-325, 440-480) — `collision_check` calls `refuse_stale` then returns `refused("already-live")` (:236-241), so the mutant changes only the two stderr lines; `conpty_sideload` records `search_restricted` on the `run.conpty_sideload` span (:287-299).
- `tests/cli_instance_state.rs` (205-264) — the stale refusal's only witness: SIGSTOP the wrapper, back-date `heartbeat`, assert the two stderr lines (:207-249) — `kill -STOP` / `ps`, so Unix-only.
- `crates/viola-e2e/tests/cli.rs` (1-125) — `unbuilt_selectors_and_unknown_commands_are_usage` pins `run --e2e` and `boot --ui` as `usage` beside `bogus` and no command (:58-73).
- `crates/viola-e2e/src/harness/run.rs` (:500-530) and `crates/viola-e2e/src/harness/run/mutants/base.rs` (:243, :332, :389) — harness self-tests that `git init` a throwaway repo in `tempfile::tempdir()` and commit `a.rs`.
- `crates/viola-e2e/src/harness/boot.rs` (:120-140) — a harness session home is `tempfile::Builder…tempdir_in(e2e_home).keep()` (never auto-removed; `cleanup` owns it).
- `Cargo.toml` — root `[target.'cfg(windows)'.dependencies] windows-sys` (:148) with `Win32_System_Diagnostics_Debug`, `Win32_Security_Authorization`, `Win32_System_Threading` among the pinned features (:211); root `[dev-dependencies]` carries `sysinfo`, `tempfile`, `viola-channel/test-support`.
- `.andromeda/runs/2026-10-01T09-18-50-code-audit/mut.sh` — the audit drove `cargo mutants -p <unit> --test-tool=nextest` DIRECTLY with `TMP`/`TEMP` set to its session scratchpad and `NEXTEST_PROFILE=mutants`, not through `run --mutants` (no root prebuild, no `target/mutants`).

## Graph impact
- Query (rust plane, trace `.andromeda/runs/2026-10-02T12-57-04-phase/tree-query-2026-10-02-epoch-2b-cleanup.json`): `calls` where `callee_name IN (search_restricted, restrict, pin_exe, pin_companions, refuse_stale, restrict_dll_search)` — 32 rows.
- **search_restricted** — 1 caller: `conpty_sideload` @ `src/cmd/run.rs:291` (graph line 290 + 1). No viola-pty caller, so no viola-pty test can see its value today — consistent with both constant-return mutants surviving under the owning-package rule.
- **restrict** — 5 callers, all inside `crates/viola-state/src/fs.rs` (`create_private_dir` :46, :48; `open_private_append` :66; `open_private_lock` :78; `replace_private_with` :111).
- **pin_exe** — product callers `run` @ `src/cmd/run.rs:21`, `pin_and_plugin` @ `src/cmd/run.rs:249`, `verify` @ `src/cmd/verify.rs:22`, `measure` @ `src/cmd/verify.rs:125`; 5 in-crate tests.
- **pin_companions** — product caller `sideload_outcome` @ `src/cmd/run.rs:305`; 5 in-crate tests (`pin_companions_reports_a_directory_where_a_file_should_be` @ :363 among them).
- **refuse_stale** — 1 caller: `collision_check` @ `src/cmd/run.rs:239`.
- No signature changes are planned: every M1 item is a test or an exemption, so no caller threading.

## Measured host facts (read-only probes, 2026-10-02)
- **`target/e2e-home/`** holds 4 804 entries — 4 768 `viola-test-*`, 33 `viola-session-*`, 3 `probe-*` (re-derived: `ls target/e2e-home | sed … | uniq -c`). **0** carry `owner.json` (`ls target/e2e-home/*/owner.json | wc -l`), so `sweep_gone_owners` can never reclaim one. By mtime day: 09-24 46 · 09-25 501 · 09-26 171 · 09-27 1 990 · 09-28 1 412 · 09-29 644 · 10-01 4 (scratchpad `sweep_probe.py`). The files left in them are the ones a live process holds: `diagnostics/run-builder.ndjson` 2 763, `bin/<key>/viola.exe` 2 239, `instances/<n>/snapshot.json(.lock)` ~1 900, `events.ndjson(.lock)` ~1 875, `heartbeat` 1 881; 0 read-only files (`home_probe.py`). Reading: a `TestHome` drop whose `remove_dir_all` failed part-way on files still open (`owner.json` went first), i.e. a `viola`/wrapper process outliving its test. Mean size 2.2 MB per home (50 sampled). No local writer of `AGENT_RUN_KEEP_HOMES=1` exists outside CI (`grep -rn AGENT_RUN_KEEP_HOMES crates scripts .github`: ci.yml :28, :211 and one child env in `crates/viola-e2e/tests/cli.rs:226`), so these are not kept-on-purpose homes.
- A sweep over that dir costs ~94 ms of failed `owner.json` opens per `TestHome::new` (single thread, `sweep_probe.py`) — a cost the copied tree, whose `target/e2e-home` is empty, never pays.
- **`%TEMP%`** holds **23 269** `.tmp*` directories, 0 files (`find "$TEMP" -maxdepth 1 -name '.tmp*' -type d | wc -l`), dated 2026-05-20 → 10-01, the bulk on mutation/coverage days (09-24 4 239 · 09-25 6 611 · 09-26 2 155 · 09-27 5 594 · 10-01 4 377). Of the 4 377 dated 10-01: 3 832 hold a git repo (`.git` + `a.rs`), 399 a throwaway cargo project (`src`, `target`), 132 are empty. A sampled git one holds exactly its two read-only object files (`find … ! -perm -u+w`). Reading: `TempDir`'s drop removed everything but the read-only git objects, which Windows refuses to delete, and then left the dir. The audit's "545 `.tmp*` dirs" was the mutation scratch's share only.
- **The copied tree (where the viola package passed 367/367) differed from the in-repo run in four measured ways:** `target/e2e-home` absent (gitignored, not copied); `TMP`/`TEMP` pointed at a fresh scratch (`mut.sh`), so `viola-root-watch` reports and every `tempfile::tempdir()` landed there instead of the 23 269-entry `%TEMP%`; `CARGO_TARGET_DIR`/profile per cargo-mutants; a different tree path (`CARGO_MANIFEST_DIR`).
- Other `viola.exe` processes run on this host from outside the repository (8 at 15:10 local, 4 with a live `claude.exe` child, started 09-26 → 10-02; `procs2.ps1`). They are the operator's sessions: P5 may stop only the exact pids it spawned.
- No job object anywhere in the product (`grep -rn -i 'JobObject|CreateJobObject|KILL_ON_JOB_CLOSE' src crates Cargo.toml`: 0 hits).

## Patterns detected
- **Child-entry rig** (`crates/viola-pty/src/sideload.rs:122-162`, `crates/viola-pty/src/lib.rs:438-483`): a `#[test]` that is a no-op unless an env var names a case, re-run as a child of the test binary (`--exact`), reporting to a file. The shape for a `search_restricted` case and any process-state observable.
- **Owner-record sweep** (`tests/support/home.rs:46-73`): reclaim only by pid + start time, never by name or age.
- **Two-sided probe** (`scripts/lint-probes.sh`, G2 `--probe`, `deny-probes.sh`): a gate proves it fires on a planted input and passes a control.
- **Contract lints at the root** (`tests/contract_lints.rs`): workspace-wide invariants read from files; the home for a deadline-vs-kill-line check.
- **Unix stale witness** (`tests/cli_instance_state.rs:207-249`): freeze the wrapper, back-date `heartbeat`, assert the fixed stderr pair.

## Conventions to follow
- Tests `<subject>_<condition>_<expected>`; rstest `#[case::label]` in root and sync crates; viola-e2e uses a labelled `(label, …)` table in one `#[test]` (testing.md §Naming; `crates/viola-e2e/tests/cli.rs:58-73`).
- A crate's mutant is killed by that crate's own tests (testing.md 2026-09-27; test-plan §3 `run` step 4).
- Every new guard test carries its remove-the-guard run (testing.md 2026-09-25).
- Waits detect exit with `try_wait` and stay strictly below the kill line (testing.md 2026-09-24, `tests/support/verify.rs:70-84`).
- `.env_clear()` re-adds `LLVM_PROFILE_FILE`; never `std::env::set_var` (testing.md §Determinism).

## New files to create
- `viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/` — M2's diff and cause, the coverage correction, the M1 scoped runs and exemptions, the leak before/after counts, the P5 measurement

## Files to modify
- `crates/viola-pty/src/sideload.rs` — a viola-pty case that observes `search_restricted` both ways
- `crates/viola-pty/src/lib.rs` — only if the scoped re-run shows the resize/restore tests do not catch their mutants
- `crates/viola-state/src/pin.rs` — cases for the `pin_exe` and `pin_companions` NotFound guards, or their recorded exemptions
- `crates/viola-state/src/fs.rs` — none expected (the `restrict` survivor is a Windows-equivalent mutant), listed for its exemption note
- `tests/cli_instance_state.rs` — a Windows stale-refusal case for `refuse_stale`
- `crates/viola-e2e/src/harness/run.rs` — the throwaway git repos cleaned on drop
- `crates/viola-e2e/src/harness/run/mutants/base.rs` — the same for its git fixtures
- `tests/support/home.rs` — the home leak, as M2's diagnosis finds it
- `crates/viola-e2e/tests/cli.rs` — P6, the unbuilt-selector case made build-independent
- `tests/contract_lints.rs` — P2, the deadline-below-kill-line lint
- `scripts/lint-probes.sh` — P2's planted-violation probe, if the lint is a probe-carrying gate
- `.claude/docs/gotchas.md` — L4, crate sources through `cargo metadata`
- `tests/support/` — the root fixture chain M2's cause may touch (provisional until the cause is named)
- `tests/run_cli.rs` — P5's pinning case, only if the fake-agent leg reads that the child ends

## Open questions
- What makes the in-repo run red (80 of 985) while the copied tree is green? The measured differences are `target/e2e-home` (4 768 ownerless homes), `TMP`/`TEMP` (23 269 leaked dirs vs a fresh scratch) and the tree path; none is yet shown to be the cause → blocks: implementation-scope (M2's fix files stay provisional)
- Why did the audit grade `PortablePty::resize → Ok(())` and `HostTerminal::drop → ()` missed when `crates/viola-pty/src/lib.rs:749/801` and `:752/:784` wait on exactly those effects? The audit ran cargo-mutants directly (`mut.sh`), not `run --mutants`; a scoped `run --mutants --file crates/viola-pty/src/lib.rs` reproduces or clears it → blocks: implementation-scope
- May this chunk remove the leftovers it diagnoses — 4 768 `target/e2e-home/viola-test-*` and 23 269 `%TEMP%/.tmp*` — once M2 has diffed against them, or is that the operator's own cleanup? → blocks: plan-decision
