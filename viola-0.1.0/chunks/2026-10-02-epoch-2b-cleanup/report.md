# Report — 2026-10-02-epoch-2b-cleanup

**Chunk:** Epoch 2b cleanup. The in-repo suite red is diagnosed (cause outside the repository; M2 stays OPEN), coverage
re-taken, eight mutation survivors disposed, test leaks measured, the L4/P6/P2 project halves and P5 child survival. M3
and the WSL leg were split off at P1.
**Date:** 2026-10-03
**Commits:** `e848944` chore(2026-10-02-epoch-2b-cleanup): operator pre-CI commit, for the run this chunk's verdict
reads · `9e3b850` fix(2026-10-02-epoch-2b-cleanup): import remove_owned only in the Windows-only case that uses it
(base `e0fbc72`)

## Changes (structured — detectors read this)
- **Files:** `crates/viola-pty/src/sideload.rs` · `crates/viola-state/src/pin.rs` · `crates/viola-e2e/src/harness/run.rs` ·
  `crates/viola-e2e/tests/cli.rs` · `tests/cli_instance_state.rs` · `tests/contract_lints.rs` · `tests/run_cli.rs` ·
  `tests/support/home.rs` · `.claude/docs/gotchas.md` · chunk `evidence/` (9 files). Basis: `git diff --name-only e0fbc72`
  minus run dirs and evidence; `gate.py scope` clean, changed 8, listed 8.
- **Symbols / APIs:** **no product code changed.** Each `src/` diff starts below its file's `#[cfg(test)]` line (sideload.rs
  first hunk :121 vs `#[cfg(test)]` :62; pin.rs :280 vs :189; harness run.rs :610 vs :608). Test-side:
  - `tests/support/home.rs`: new `pub fn remove_owned(dir) -> bool` removes an owned dir with its `owner.json` last.
    `TestHome`'s drop (non-keep branch) and `sweep_gone_owners` now call it in place of `TempDir`'s drop and
    `remove_dir_all` respectively. Callers: those two, plus `cli_instance_state.rs`'s new case.
  - New tests:
    - `sideload::tests::search_restricted_reads_whether_the_restriction_took_effect` (viola-pty), with a
      `search-restricted` case added to the existing `sideload_child_entry` rig;
    - `pin::tests::pin_exe_refuses_a_copy_it_cannot_read_and_leaves_it` (every OS; **replaces** the Unix-only
      `pin_exe_refuses_an_unreadable_copy_and_leaves_it`) and `pin_companions_refuses_a_companion_it_cannot_read_and_leaves_it`
      (Windows), with a test-only `Unreadable` guard. On Windows it spawns `%SystemRoot%\System32\icacls.exe` directly
      (`/deny <USERDOMAIN>\<USERNAME>:(RD)`, `/remove:d`); on Unix it sets mode 000 / 0600. These test-only env reads
      (`SystemRoot`, `USERDOMAIN`, `USERNAME`) are in `#[cfg(test)]`;
    - `harness::run::tests::throwaway_repo_leaves_no_dir_after_drop` (viola-e2e, over the unchanged `git_repo()`);
    - `cli_instance_state.rs`: `remove_owned_keeps_the_owner_record_while_a_file_is_held` (Windows) and
      `run_refuses_a_frozen_wrapper_as_stale_on_windows` (Windows; a test-only `Frozen` guard calling
      `DebugActiveProcess` / `DebugSetProcessKillOnExit(0)` / `WaitForDebugEvent` / `DebugActiveProcessStop` from the
      root's existing `windows-sys` features);
    - `run_cli.rs`: `run_child_ends_when_its_wrapper_is_terminated` (Windows; `OpenProcess` + `TerminateProcess`);
    - `contract_lints.rs`: `test_deadlines_sit_below_the_nextest_kill_line` and
      `past_the_line_fires_at_the_kill_line_and_passes_below_it`.
  - Renamed: `crates/viola-e2e/tests/cli.rs` `unbuilt_selectors_and_unknown_commands_are_usage` →
    `unknown_selectors_and_unknown_commands_are_usage`, whose case table now uses `run --no-such-selector` and
    `boot --no-such-flag` in place of `run --e2e` / `boot --ui`.
  - No IPC method, endpoint, port, socket, env var or CLI flag added in product.
- **Crates / modules:** none added or removed.
- **Dependencies:** none (Cargo.toml / Cargo.lock untouched: `git diff --quiet e0fbc72 -- Cargo.toml Cargo.lock`).
- **Schema / config:** none. `.config/nextest.toml` is untouched; the new lint only reads it.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - Root package test count 367 → 371 (C: copy, `cargo nextest run -p viola --features fake-agent` under the `mutants`
    profile; the HEAD copy read 367).
  - Coverage-suite test count on the C: copy: 994 (`run --coverage`).
  - Windows coverage at the Epoch 2b boundary: lines 97.59 % (2 427/2 487), functions 97.67 % (293/300), regions
    97.50 % (4 447/4 561), from the C: copy's `llvm-cov-summary.json`. This corrects the code audit's `coverage.line` "not
    measured" (`.andromeda/runs/2026-10-01T09-18-50-code-audit/`). The in-repo D: run read 94.41 / 94.33 / 94.17 with 89
    failed.
- **Dev-tool versions:** none — cargo-mutants, nextest 0.9.146 and claude 2.1.283 were read unchanged.
- **Harness / gate surface:** none changed (`scripts/agent-run.*`, the harness and CI are untouched). New test-side lint:
  `test_deadlines_sit_below_the_nextest_kill_line`. It reads `.config/nextest.toml`'s `[profile.mutants]` (5 s × 2 =
  10 s) and the `package(viola-e2e)` override (15 s × 2 = 30 s) and asserts every test-side `Duration` constant (the whole
  of `tests/`, `crates/*/tests/`; the `#[cfg(test)] mod tests` tail of `src/`, `crates/*/src/`) is below its package's kill
  and cargo-mutants' 20 s floor. It refuses an empty population and has a planted two-sided case.
- **Cross-project / external claims:**
  - **Host D: volume** (this dev host, not the repo): a ReFS Dev Drive on a file-backed virtual disk, per-operation
    metadata latency mkdir 74–113 ms · small write 149–227 ms · rename 348–500 ms, against 0.2–0.3 ms on C: NVMe
    (`evidence/m2-diagnosis.md`; python probe, same minute). The operator's `cargo clean` removed 80.1 GiB, 241 256 files
    in 5 h 48 min (~86 ms/file).
  - **CI runs:**
    - ci#37106821284 on `e848944`: failure, `lint (ubuntu-latest)` + `lint (macos-latest)` (one error: an unused
      `remove_owned` import), 13/15 green;
    - **ci#37107107417 on `9e3b850`: success, 15/15, wall 290 s** (`ci.py conclusion --sha HEAD --wait 1800`).
  - **claude CLI 2.1.283** (P5 leg 2): the `claude.exe` child ends 0.50 s / 1.27 s after `Stop-Process` on its wrapper;
    `claude agents --json --all` rows naming the session went 1 → 0 (`evidence/p5-child-survival.md`).
- **Reverted / negative API facts:**
  - A `Throwaway` drop guard (clearing the read-only attribute before removing a throwaway git repo) was written in a
    scratch copy, read green with the guard removed, and was **not shipped** (`evidence/leaks.md`).
  - `wait_bounded` waiting out its `child.kill()` (`tests/support/verify.rs`) was written, measured with no effect on
    Leak B (2·3·2 new ownerless vs 1·2·0·3), and **reverted**; byte-equal to HEAD.
- **Insufficient fixes (written, kept, not the remedy):** `remove_owned` (owner record last). It closes the
  listing-order hazard its own case witnesses (red without, green with). It does **not** close the ownerless remnants
  D:'s deadline-failing runs leave (0–3 per run with and without it; `evidence/leaks.md` "Leak B on D:'s red runs").
  Owner of the remainder: M2's closure (founder decision); the remnants appear only with D:'s failures and are 0 on the
  C: copies.
- **Spec claims disproved by measurement:**
  1. **Plan step 4 / scope §3 / research "Measured host facts":** "git writes its object files read-only and Windows
     refuses `remove_dir_all` on them, so `TempDir`'s drop leaves the dir". Measured false: a dropped committed repo with 3
     read-only objects is removed whole by a plain `TempDir` (`throwaway_repo_leaves_no_dir_after_drop` green, guard
     absent), and a 225-test viola-e2e run leaks 0 dirs (`evidence/leaks.md`). Stated in the chunk's plan/scope/research
     only: `grep -c "read-only" .andromeda/test-plan.md` = 4 hits, none about git objects (re-read: :1051-1054, the
     fixture table's "per-suite, read-only" lifetime cells); `registries/contracts/test-plan/test-data-bootstrap.md` 0 hits.
  2. **The code audit's "suspected shape" for `PortablePty::resize` / `HostTerminal::drop`** (audit proposals §M1). Both
     caught on a direct `--re`-narrowed run (P5 reading, kept). The audit record is not a master.
  3. **The P5 study's hypothesis** (`refs/session-memory-options.md` §5: "with no job object a grandchild could
     survive"). Not observed: the direct child ends in both legs and nothing holding the home survives. The hypothesis
     sits in `refs/`, not in a master.
  4. **Narrow basis, for disposition:** testing.md §Test data and the test-data contract say viola creates a test home
     "(Windows protected DACL)". `grep -rn "SetNamedSecurityInfo\|SetSecurityInfo\|SECURITY_ATTRIBUTES" crates/*/src src`
     hits only `crates/viola-channel/src/server/win.rs` (+ its test_support): viola-state's `create_private_dir` sets no
     DACL on Windows (`restrict` is a no-op there by design). The basis is a grep, not an ACL read of a created home.
- **Expected amendments (from plan):**
  1. "test-plan §3 → test-data bootstrap — the git-fixture read-only rule": **not carried** (no rule; premise falsified,
     disproved claim 1). "… whatever step 5 changes about home lifetime": **carried**, Symbols bullet (`remove_owned`).
     Site: `registries/contracts/test-plan/test-data-bootstrap.md:15` (`grep -n "owner"` 1 hit: "**Cleanup:** `TempDir`
     drop removes per-test homes…"); `architecture.md:416` (`grep -c owner.json` 1 hit, Occupied Resources `target/e2e-home`).
  2. "test-plan §3 — the deadline lint and the P6 never-planned-flag witness, if contract-level": **carried**, Harness /
     gate surface bullet (the lint) and Symbols bullet (the rename). Sites: `grep -c "kill line"` 0 in all four of arch /
     test-plan / obs / security; `"unbuilt selector"` 0 hits; `"never-planned"` 0 hits. Neither has a master site today.
  3. "architecture §Occupied Resources — the M1 direct-file runs' `target/agent-run/m1-*` output and copy dirs":
     **superseded**. The M1 runs ran in two C: scratch copies (D: baselines fail under M2), so no `target/agent-run/m1-*`
     dir was created in the repository. `target/e2e-home.m2-held` was transient and is gone. One-off probe scratch this
     run created under the gitignored tree: `target/agent-run/p5/` (P5 leg-2 homes), made by a scratch script, not by
     committed code (`grep -c "agent-run/"` architecture.md = 6 hits, none for p5 or m1).
  4. "Route (wrap route-resolve): mint the split-off entry for M3 and the WSL leg…; the host-reds CARRY retires if M2 reads
     the suite green; the WSL `--install-deps` CARRY moves on": **carried to P5**. M2 did not read the suite green; the
     cause *was* established (the CARRY's other retirement condition).
- **Coverage of new surfaces:** none in product. The new test seams are test-only: `Unreadable` (icacls / chmod),
  `Frozen` (debug attach), `remove_owned`. Each is `#[cfg(test)]` or `tests/` code, with validation n/a · instrumentation
  n/a · PII n/a · tests unit/integ · a11y n/a · tokens n/a.

## Deviations from intent
- **M2 not fixed (by cause).** The cause is the D: volume, outside the repository. Two-sided, both orders, same day: D:
  22/40 vs C: copy 62/0, twice. Done per scope §1's alternative branch ("if the cause is outside this repository, recorded
  with its evidence"). M2 stays an **open red** for the founder's decision: fix the volume, or move the test homes (a
  test-data contract change). No bound was raised and no test was skipped. Overseer, 2026-10-03.
- **Step 4 guard not written** (premise falsified, disproved claim 1); the witness test was kept. Testing.md 2026-09-24:
  a guard repeating what the call already guarantees is left out.
- **Step 5** landed `remove_owned` (a kept, insufficient fix, above). Its plan form ("make that child end before the home
  drops") was tried as `wait_bounded` waiting out the kill and reverted for no measured effect.
- **Step 11** ran from C: copies with `--in-place`. The plan's in-repo form fails its baselines under M2 (P5's reading);
  the plan form in a deep scratch copy failed with `LNK1104` (Windows path length) before any mutant. Same `--re`, same
  tests, readings: before state 2 missed / root 1 missed; after pty 4 · state 2 · root 1 caught.
- **Step 16** coverage acceptance ("`ok:true` in the repository tree") is unmet in-repo. The correction numbers come from a
  byte-equal C: copy.
- **Step 8** replaced the Unix-only `pin_exe_refuses_an_unreadable_copy_and_leaves_it` with the cross-OS case; its body
  is that case's Unix half.
- **Step 12** renamed the test to match its content.
- **Step 15** took the conditional "the child ends" branch: the pin landed.
- **Process:** code was drafted, compiled and unit-run in a C: scratch copy (C: checks 118 crates in 37 s), then ported
  byte-exact (`cmp`) to the repository.
- **Operator pass** (overseer word):
  - the 4 phase-run dry-run captures were redacted (4 host paths each, CRLF kept) to read hygiene clean;
  - the pre-push `windows-tests` red was recorded, not chased (M2 class, CI the acceptance leg);
  - the CI lint red was folded into the chunk as `9e3b850`.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 8 · listed 8).

## Decisions & corrections
- Overseer: M2 stays OPEN; the founder decides its closure (fix the volume vs move the test homes); never mark it fixed.
- Overseer: let a red CI run finish before fixing, so one fix covers every red it shows.
- Overseer: the D: pre-push `windows-tests` red is M2's, two-sided witnessed; CI is the acceptance leg; record, don't
  chase.
- Disk guard (operator): stop and report under 40 GB on D:. It fired once (39.83 GB); the operator freed 151 GB.
- Finding, cross-OS lint: a `use` of an item referenced only inside `#[cfg(windows)]` code is an unused import on every
  other target. Windows clippy (local and gate) cannot see it; CI's Linux/macOS `lint` can, and the pre-push Linux leg
  runs tests, not clippy. WSL clippy in pre-push's clone reproduced it before the push could have.
- Finding, Windows removal: std's `remove_dir_all` deletes read-only files (no attribute clearing needed). It stops at the
  first entry it cannot delete in listing order, so an owned dir's record must go last.
- Finding, cargo-mutants on Windows: its tree copy under a deep `TMP` path breaks the link step (`LNK1104`); `--in-place`
  in a disposable copy avoids it.
- Finding, guard tests: two new guard cases first read green with the guard removed (one had nothing to guard; one had
  the held file sorted before the record). A remove-the-guard run is what showed both.
- Finding: Windows PowerShell 5 reads a BOM-less UTF-8 em dash in a `.ps1` as ANSI and fails to parse.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) **M2 closed — UNMET.** The cause is named as an equality with a two-sided witness, both orders. The in-repo
  suite is not green: the cause is outside the repository → **owner: the founder's M2 decision**. The overseer has already
  ruled it an open red (no P2 escalation pending).
- (tests) **Coverage re-taken — UNMET in-repo.** The numbers were recorded from the C: copy (97.59 / 97.67 / 97.50 ≥ 85 /
  95 / 80, its gate `ok:true`); in-repo: 89 failed, functions 94.33 → owner: M2.
- (tests) **Eight survivors disposed — MET.** Counts 4 · 2 · 1 in each run's `caught.txt`; `restrict` exemption recorded;
  kill-test entries green.
- (security) **Five security-control survivors assert the effect — MET.** No exemption claims a control unobservable.
- (design) **Stale refusal wording — MET.** The exact pair is asserted; green locally (gate entry 7) and in CI
  `test (windows-2025)` on `9e3b850`.
- (tests) **No new leak — UNMET in-repo** (Leak B on D:'s red runs, 0–3 per run). 0 on the C: copies; 0 new `%TEMP%` dirs
  → **owner: M2's decision**. The chunk that closes M2 re-reads it.
- (obs) **Keep-on-failure intact — MET.** Watch reports were kept on failure (27 after the D: arms); `AGENT_RUN_KEEP_*`
  untouched; every test home is under `target/e2e-home/`.
- (tests) **P2 lint — MET.** Passes, fails its planted case, refuses an empty population, states its scope.
- (design) **P6 usage test — MET.**
- (tests) **P5 recorded — MET.** The tab-close leg is **owed → owner: the founder** (founder-attended, the study's §5).
  Its consumer is the revive entry.
- (security) **No widening — MET.** No product env read, flag or key; `release-check --probe` exit 0; `git diff --quiet
  e0fbc72 -- scripts/wsl-provision.sh .github/workflows/ci.yml scripts/g2-zero-panics.sh` exit 0.
- (security) **Evidence floor — MET.** `gate.py hygiene` clean.
- (a11y) **TUI boundary cases — UNMET in-repo, MET in CI.** `tui_passthrough` failed 5 on D: (M2 class); CI test jobs on
  three OSes are green on `9e3b850` → owner: M2.
- (tests) **CI green — MET.** ci#37107107417, 15/15.

Gates (implement's runs, the operator pass, and pass re-runs on the final tree):
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green
  (Windows; Linux clippy in WSL also green on the fixed file).
- `bash scripts/agent-run.sh run --unit` green (1 404 s on D:).
- `… --unit --filter 'test(/search_restricted_reads_whether_the_restriction_took_effect/)'` green.
- `… --unit --filter 'test(/refuses_a_(copy|companion)_it_cannot_read_and_leaves_it/)'` green.
- `… --unit --filter 'test(/throwaway_repo_leaves_no_dir_after_drop/)'` green.
- `… --integration --filter 'binary(cli_instance_state) & test(/run_refuses_a_frozen_wrapper_as_stale_on_windows/)'` green.
- `… --integration --filter 'binary(contract_lints) & test(/test_deadlines_sit_below_the_nextest_kill_line/)'` green.
- `… --integration --filter 'package(viola-e2e) & binary(cli)'` green.
- `… --integration --filter 'binary(run_cli) | binary(hook_fail_open) | binary(cli_verify) | binary(cli_version_gate) |
  binary(cli_instance_state)'` **red — not this chunk's**. Basis: the same red on HEAD's source the same day (D1 / D2
  22/40), against the byte-equal C: copy 62/0 in both orders → owner: the M2 open-red pin (P5 of this wrap).
- `bash scripts/agent-run.sh run` **red — not this chunk's** (85 failed, every one a `watch.rs` deadline or the spine
  bound); same basis → same owner.
- `! (git diff e0fbc72 … | grep -E '^\+.*(#\[ignore|retries *=|test\.skip)')` green.
- `bash scripts/agent-run.sh run --coverage` **red — not this chunk's** (89 failed); same basis → same owner.
- `bash scripts/agent-run.sh gate --require coverage,doctest` **red — not this chunk's** (`suite-failed`, `functions 94.33
  < 95`, a consequence of the 89); same basis → same owner.
- `grep -c 'cargo metadata' .claude/docs/gotchas.md` green · `git diff --quiet e0fbc72 -- …` green ·
  `bash scripts/release-check.sh --probe` green · `cargo deny check` green.
- `bash scripts/agent-run.sh pre-push` **red — not this chunk's**: `linux-tests` green (951/0 · Playwright 1/0 · gate ok);
  `windows-tests` 87 failed, M2 class; same basis → same owner.
- Smoke: `cleanup --session p-e2bc-smoke` green · `boot --session p-e2bc-smoke --instance builder` green (425 s) ·
  `cleanup` green (`processes_gone`, `endpoint_gone` true).
- `gate.py hygiene` (leg operator): clean after the redaction (`evidence/operator-pass.md`).
- `git … push origin HEAD` (leg operator): `e0fbc72..e848944`, then `e848944..9e3b850`.
- `ci.py conclusion --sha HEAD --wait 1800` (leg operator): **green**, ci#37107107417 on `9e3b850`.

**Wrap light gate** (`.andromeda/runs/2026-10-03T07-46-03-wrap/`, the whole block, final tree):
- Green: fmt · clippy · `run --unit` · the three new unit gates · the deadline lint · the ignore probe · the gotchas probe ·
  the diff-quiet probe · `release-check` · `cargo deny` · smoke cleanup / boot (431 s) / cleanup.
- `… binary(cli_instance_state) & test(/run_refuses_a_frozen_wrapper_as_stale_on_windows/)` **red — not this
  chunk's**: it fails in the unchanged stamped-home fixture's `viola verify` ("viola never exited", 15 s) before the test
  body. That fixture step fails the same way on HEAD's source on D: (D1/D2). The case was green at /implement and in CI
  `test (windows-2025)` on `9e3b850` → owner: the M2 open-red CARRY (working-route :68).
- `… package(viola-e2e) & binary(cli)` **red — not this chunk's**: `cleanup_keeps_the_home_under_agent_run_keep_homes` and
  `status_logs_and_cleanup_drive_a_booted_session` fail at harness `boot` `readiness-timeout` (missing `builder:events`);
  neither test was changed by this chunk. **Narrower basis, named:** these two were not run on a HEAD tree on D:. Their
  basis is the M2 two-sided witness for the same failure class (D: vs a C: copy, both orders), the entry green at
  /implement, and CI green 15/15 on `9e3b850` → owner: the M2 CARRY.
- `… 5 binaries …`, `run`, `run --coverage` (89-class, 5 054 s this run), `gate --require coverage,doctest`: **red — not
  this chunk's**, M2 class as above → owner: the M2 CARRY.
- `pre-push` **timeout** at its 5 400 s bound. `linux-tests` green (951/0 · Playwright 1/0); the `windows-tests`
  instrumented build on D: was killed mid-link (exit 143); no survivors (census: no cargo, llvm-cov, link, harness, viola
  or wsl process left). Treated as M2 class on the overseer's ruling.
- Operator legs re-verified from `evidence/operator-pass.md`: hygiene clean, two pushes, CI green on `9e3b850`.
- Disposition, overseer (founder-delegated), 2026-10-03: "the M2 witness is two-sided (D: vs a C: copy, both orders) and
  CI is green 15/15 on 9e3b850, the acceptance leg per the host-reds CARRY. The founder has paused Viola so D: can be
  fixed, so no more cold D: builds now." Hence no HEAD-on-D: control for entry 9.

Watches: none folded.

Outcome basis: the operator pass ran (`e848944` pre-CI, `9e3b850` fix). The final HEAD's CI run ci#37107107417 green is
recorded in `evidence/operator-pass.md`. The operator directives between implement and this report: redact the phase
captures; record, don't chase, the pre-push red; fold the CI lint red. Implement's conversation is present (same
session).

Process hygiene: implement's census reads nothing left running that this run started: the witness, coverage, M1, leak and
gate runs finished; the P5 wrappers and the `du` walk were stopped by exact pid. The operator's viola-lab sessions and
`pulse-app` builds were never touched. Re-measured at P7.
