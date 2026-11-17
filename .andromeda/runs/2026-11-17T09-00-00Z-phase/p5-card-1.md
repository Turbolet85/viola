# P5 review — 2026-11-17-booted-wrapper-does-not-outlive-its-test («A booted wrapper does not outlive its test»)

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-17-booted-wrapper-does-not-outlive-its-test/plan.md`
(plan rev 4). No plan stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the
chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-booted-wrapper-does-not-outlive-its-test`.

## Goal
1. **The CI red first** (CARRY `cry-n25qin7l`). The mechanism was measured at P3. rust-cache `6323deb`'s save strips
   every file under a non-profile dir of `target/`, so `target/e2e-home/viola-stamp-<key>/` is restored HOLLOW: the
   folders stand and no file is left. On identical source the key matches, and `stamp_dir` trusts `is_dir()` and
   copies an empty tree. The overseer's hypothesis of a `/tmp` link is CORRECTED: CI has no link, and the stamp dir
   itself comes back hollow. The fix judges a stamp by a completion file and `stamps.json`, and stamps a hollow one
   into the next generation `viola-stamp-<key>-g<n>`, removing nothing (`inputs#I2`).
2. **No booted wrapper or agent outlives its test** (v2-37). Measured at P3:
   - The outer-PTY boots END when the test is SIGKILLed (hangup).
   - Piped and direct boots SURVIVE, idle at 0 % CPU.
   - An in-test harness session's supervisor SURVIVES.
   Every long-lived root spawn goes through `tests/support/tied.rs` (the parent-death signal, SIGKILL), and the
   in-test supervisor is tied the same way. The CLI `boot` is unchanged. The carry's «99 %» is not reproduced at HEAD:
   it reads as the mutated code under test, and the fix holds whatever a wrapper loops on.
3. **The proof of «killed»**: a test-owned victim self-spawn boots by each path (piped, direct, outer PTY, frozen,
   harness session). The test SIGKILLs it and censuses its recorded tree to gone within `WITHIN`. A survivor is
   killed only on the census's failure path, and only by matched pid and start time.

## Top acceptance criteria
- This chunk's pre-CI push reads `verdict: green` on an EXACT restore of `v0-rust-test-Linux-x64-41224fa4-058865c8`
  (`full match: true`, entry 14). This is the directive's proof (`inputs#I1` item 2, `inputs#I3`), and no cache is
  deleted to pass it.
- `matrix#v2-37`: the killed-test witnesses (entries 5, 6), held by controls `tied-pdeath` and `sup-tied`.
- The hollow-stamp witnesses (entry 4), held by control `stamp-hollow`.
- The census: no root test spawns viola untied (entry 5). The full integration and E2E suites stay green (entries 8–9).

## Touchpoints
New: `tests/support/tied.rs`, `tests/cross_killed_test.rs`, `crates/viola-e2e/tests/harness_tied.rs`. Modify:
`tests/support/{home,mod,piped,cli,start,verify}.rs`, `tests/cli_verify.rs`,
`crates/viola-e2e/src/harness/{boot,session}.rs`, `crates/viola-e2e/src/bin/viola-harness.rs`, `.config/nextest.toml`,
and the derived 44 direct `Command::new(VIOLA)` sites in 24 root test files. No product code changes.

## Test commands (14 entries) and baselines
1–2 fmt and clippy (seen green). 3 unit `boot_ties_…`: red, no tests to run. 4 `cli_verify & stamped_home`: green,
3 passed. 5 `cross_killed_test`: red, no binary. 6 `harness_tied`: red, no binary. 7 `contract_lints`: green, 7
passed. 8 `--integration`: green, 505 passed. 9 `--e2e`: green, 348 passed. 10–14 operator: pre-push, hygiene,
push, `ci.py conclusion`, and the exact-hit log read.
Controls (implement's one-shot, `--timeout 600`): `stamp-hollow` (entry 4), `tied-pdeath` (entry 5), `sup-tied`
(entry 6).

## Concretization preview — v2-37
- **Current acceptance:** «After a test that booted a wrapper is killed mid-run, no wrapper or agent that test
  started is still running; a test that passes, fails or times out leaves none either.»
- **Proposed:** «After a test process that booted a wrapper on pipes, by a direct spawn, under an outer PTY, frozen
  under an outer PTY, or under an in-test harness session is SIGKILLed mid-run, every process of that test's recorded
  tree (the wrapper, its fake agent, the supervisor) is gone by pid and start time within `WITHIN`
  (`killed_test_leaves_no_{piped,direct,outer_pty,frozen}_wrapper`, `killed_test_leaves_no_harness_session`). A test
  that panics leaves none (`panicked_test_leaves_no_piped_wrapper`). No root test spawns viola outside the tied helper
  (`cross_killed_test_census_finds_no_untied_viola_spawn`). A test timed out is one the runner kills, the case the
  witnesses drive; a test that passes or fails ends its wrapper through the existing `Drop` guards.»
- Not weakened: the killed case is made concrete over every boot path the tree holds, and the timed-out case is
  driven as a kill. The pass and fail cases rest on the existing `Drop` guards plus the tie, with no new witness for
  a passing test. Say so if you want one.

## CI read (Setup 5a, and since)
- 668854a6: red, ci#44048204364, `test (ubuntu-latest)`.
- 2dc9b58d (HEAD): `in progress` at Setup (attempt 2, the overseer's one re-run). Per `inputs#I3`: with the 07:46Z
  cache deleted, attempt 2 took the 2026-11-15 cache by restore-key and read 7/7 green. The re-run then saved the key
  `…-058865c8` again (id 10651568151, 09:05:29Z), so this chunk's pre-CI push hits it EXACTLY. A green re-run does not
  close the red: the carry closes with the fix and that run.

## Provenance block (directive item 11, verbatim where a tool printed it)
- The union's per-master lines and its `rows-union` line:
  ```
  architecture: rows 948 · packs 10 · picked 139 (87 by every draw) · standing 3 · 43485 B
  security-plan: rows 606 · packs 7 · picked 96 (61 by every draw) · standing 39 · 28714 B
  design-system: rows 315 · packs 4 · picked 64 (40 by every draw) · standing 13 · 16248 B
  layout-templates: rows 222 · packs 3 · picked 19 (7 by every draw) · standing 0 · 4469 B
  test-plan: rows 1334 · packs 14 · picked 249 (127 by every draw) · standing 16 · 63360 B
  obs-plan: rows 1024 · packs 11 · picked 243 (105 by every draw) · standing 13 · 56010 B
  a11y-plan: rows 235 · packs 3 · picked 32 (13 by every draw) · standing 12 · 10471 B
  rows-union: 7 master(s) · picked 842 of 4684 · rule any
  ```
- The rows were read WHOLE, not by search: the sum is 222 757 B, under 250 000.
- Every row the plan cites stands in the union: 15 handles, each found in its master's rows file. None is cited from
  a standing list, and none was looked up by `rec.py find`.
- `[RETYPED]` lines (each a check note; no draw failed):
  ```
  arch-pick-p1.d3 · [RETYPED] line 89: decision/dialog-hold-per-instance is no row of the pack — read as decision/dialog-hold, the pack's row 89
  arch-pick-p4.d3 · [RETYPED] line 80: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 80
  tests-pick-p7 · [RETYPED] line 2: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 2
  tests-pick-p13 · [RETYPED] line 28: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 28
  obs-pick-p3 · [RETYPED] line 77: contract/run-exit-1-detail/sha-256 is no row of the pack — read as contract/run-exit-1-detail/sha-256-mismatch, the pack's row 77
  ```
- Re-spawns: one, `layouts-pick-p3`, on `[ABSENT]` at pick-p1's check (`VERDICT: 74 of 75 pass · failed: layouts-pick-p3 [ABSENT]`). Its re-spawn pick-p1-r1 read `VERDICT: 1 of 1 pass · failed: none`, yes 0 of 24.
- Non-zero exits and refusals:
  - `rec.py put --edit-file` exit 3: «`old:` «(the directive's item 2), to be tested from the code» hits 0 times,
    one is owed — nothing written». The anchor crossed a wrapped line; re-fired with a one-line anchor.
  - The Bash guard refused three calls, each re-issued once in its passing form: two `cd` calls into a run dir or the
    scratchpad («moves the session cwd»), and one `xargs` of a shell builtin.
  - A typed control of entry 14 failed at exit 2 on my own nested quoting («syntax error near unexpected token `(`»);
    it was re-fired from a script in the scratchpad.
  - Two `git -C ~/dev/projects/additional/viola-overseer` probes exit 128: not a git repository, so the directive
    snapped as a copy.
  - Baselines red by design: entry 3 (exit 4, no tests to run), entries 5 and 6 (exit 1, no binary matched).
- `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` lines: none printed in this run.
- Places a letter was wrong or silent:
  - (a) The directive asked the orphan behaviour re-measured «from the code». The code alone could not decide
    whether an orphan ends, idles or spins, so two host probes in the session scratchpad measured it, ending every
    process they started. The letters name no such probe step.
  - (b) The scope's hypothesis (a `/tmp` link) was wrong in mechanism and was corrected at P3.
  - (c) codebase-research's letter is silent on a P3 correction that adds files inside a class the pinned scope
    already names («any test that spawns one itself»). I judged it no re-run of the row read.
  - (d) `fanout-contract.md` is past the Read tool's cap and was read in two pages.

## Beyond the plan
- none

The host: per `inputs#I2`, another project's mutation run takes the whole host when this phase ends.
`/andromeda-implement` starts only on the overseer's word that the host is free.

Apply? (yes / review / cancel)
