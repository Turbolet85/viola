# Operator pass — 2026-10-26-home-and-code-bearing-file-integrity

Run on the operator's own request in the implement session (2026-10-26, «through its operator pass and the CI
read»), the entries fired in their plan's operator forms; lines copied verbatim, headers left out.

## Entry 24 — hygiene, fired as written, before the pre-CI commit

First firing (2026-10-26T20:43Z):

```
P1 .andromeda/runs/2026-10-26T19-02-07-implement/journal.md:12 ×1 · tmp
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 538 (runs 532 · evidence 1 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The journal's line 12 named the e2e-home link's temp-dir target; rewritten as «the owner-only
`viola-e2e-home-<uid>` dir under the temp dir». Re-fired:

```
hygiene: clean — read 538 (runs 532 · evidence 1 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## Entry 23 — pre-push, re-fired before the push (its note)

```
 23 probe       green · exit 0 · 101.9s · 421949 B → 23.4.log · bash scripts/agent-run.sh pre-push
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

## Entry 25 — the push (pre-CI commit `af6c67ef`)

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 3.32s · 92 B → 25.log · history moved: refs/remotes/origin/HEAD 99df4536→af6c67ef; refs/remotes/origin/build/viola-0.1.0 99df4536→af6c67ef · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

## Entry 26 — the CI read of `af6c67ef`, fired as written

```
af6c67ef53b2 verdict: red · checks 15/15 · first-fail +646 s test (windows-2025) · runs ci#40398601594 in_progress/-
runs: ci#40398601594 push in_progress/-
failed 1: test (windows-2025) (failure)
running 1: oldest test (macos-latest) 654 s
run open: ci#40398601594 in_progress
```

The run then completed (`gh run view`, 2026-10-26T20:59Z): `failure`; `test (macos-latest)` success, every other
job success but `test (windows-2025)`. Its job log (job 121838141094):
`Summary [ 400.562s] 3175 tests run: 3173 passed (6 slow), 2 failed, 0 skipped` —

- `viola::cli_verify verify_under_a_regular_file_refuses_the_home` — `left: "error: internal error\n"`, `right:` the
  home pair. On Windows a path under a regular file stats `NotFound` (testing.md 2026-09-24), so the home's check
  skips it as absent and the catch-site line stands; the case now carries a per-OS oracle.
- `viola::security_negatives_strict_modes a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` —
  `no DACL in` (an empty `Get-Acl` output; its stderr was not captured). The read-back now goes through
  `icacls <home> /save <file>` (exit asserted, the file's text in the failure message).

Read from the same log, green on `windows-2025`: `strict::win::tests::strict_instance_refuses_a_plain_home_made_before_
viola_and_names_the_entry` (step 11 / CARRY 13: a home made with `create_dir_all` before viola is refused `Writable`,
the refused entry an inherited `S-1-5-32-545` write ACE), `…_tree_granted_users_write_beside_one_that_passes`,
`…_diagnostics_folder_granted_users_read`, `read_rights_are_the_named_flags`,
`pin::tests::pin_companions_refuses_a_path_another_user_may_write` (both cases),
`conpty_sideload run_on_a_companion_another_user_may_write_falls_back_to_conhost`, every `hook_fail_open`,
`hook_statusline`, `run_cli`, `handlers`, `chaos_snapshot_replay` and strict-modes case of this chunk but the two above.

## The fix push `f65a4ae8` and its CI read

Entry 23 re-fired `green · exit 0 · 100.84s`; hygiene `clean`; entry 25:

```
 25 probe       green · exit 0 · 1.94s · 92 B → 25.log · history moved: refs/remotes/origin/HEAD af6c67ef→f65a4ae8; refs/remotes/origin/build/viola-0.1.0 af6c67ef→f65a4ae8 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```

Entry 26, fired as written:

```
f65a4ae838d9 verdict: red · checks 15/15 · first-fail +630 s test (windows-2025) · runs ci#40400887151 in_progress/-
runs: ci#40400887151 push in_progress/-
failed 1: test (windows-2025) (failure)
running 1: oldest test (macos-latest) 655 s
run open: ci#40400887151 in_progress
```

The run completed `failure` (2026-10-26T21:17Z): every job green but `test (windows-2025)`, whose log (job
121846095523) reads `3175 tests run: 3173 passed (5 slow), 2 failed, 0 skipped`:

- `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` — the read-back now worked, and the reading is
  the protected owner-only DACL: `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`. Canonical SDDL spells the runner's user,
  the local Administrator account (RID 500, `S-1-5-21-…-500`), with the alias `LA`; the oracle compared the SID.
  Fixed: the oracle takes `LA` for a user SID ending `-500`. Step 13 (d)'s reading is that line.
- `run_cli run_rust_log_changes_nothing` — the traced run's child start carried `sideload_fallback`, the plain run's
  did not. The kept homes' role files (artifact `diag-windows-2025`) read, in BOTH runs, 262 child starts
  `conpty-sideload`, 3 `hash-mismatch` (the tamper tests) and 2 `strict-modes-failed`: one is
  `conpty_sideload run_on_a_companion_another_user_may_write_falls_back_to_conhost` (expected); the other starts
  inside `run_cli run_refuses_a_widened_home_tree_before_its_log::case_3_bin`'s window in both runs —
  ci#40398601594: case_3 20:54:02.247–02.574, the start 20:54:02.464 (it hit a test that compares nothing on it);
  ci#40400887151: case_3 21:10:17.012–17.498, the start 21:10:17.486 (`run_rust_log_changes_nothing`'s traced run).
  Mechanism, read in source and consistent with both windows: that case's first start was SEEDED, so its
  `bin/<key>/conpty/` companions are hard links to the one shared `target/conpty-seed/<key>/` file; `widen(bin)`
  (and case_1's `widen(home)`) grants Users write with `(OI)(CI)`, which `icacls` propagates to every existing file
  below, so the shared seed file — every seeded home's companion — carried the grant until `unwiden`, and a start
  elsewhere in that window was correctly refused by the new owner/DACL check. Not a product fault: the check did what
  step 10 asks. Fixed in the test: the case's first start goes through `run_viola_unseeded`, so the product writes
  this home's own companions and the grant stays inside the home. Confirmation owed to the next run's reading: one
  `strict-modes-failed` (the companion test's) in the kept homes.

## The second fix push `e3468500` and its CI read — green

Entry 23 re-fired `green · exit 0 · 99.05s`; hygiene `clean`; entry 25:

```
 25 probe       green · exit 0 · 2.04s · 92 B → 25.log · history moved: refs/remotes/origin/HEAD f65a4ae8→e3468500; refs/remotes/origin/build/viola-0.1.0 f65a4ae8→e3468500 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```

Entry 26, fired as written:

```
e346850044ac verdict: green · checks 15/15 · wall 715 s · runs ci#40403389629 completed/success
runs: ci#40403389629 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

`test (windows-2025)` (job 121854822279): `3175 tests run: 3175 passed (5 slow), 0 skipped`. The diagnosis's owed
confirmation, read from the kept homes (artifact `diag-windows-2025`): 264 child starts `conpty-sideload`, 3
`hash-mismatch` (the tamper tests), ONE `strict-modes-failed` at 21:26:36.167Z — inside
`run_on_a_companion_another_user_may_write_falls_back_to_conhost` (PASS at 21:26:36.217Z after 5.123 s). The
unexpected second fallback of both earlier runs is gone with the seeded first start.
