# Operator pass — 2026-10-27-pinned-copy-race-and-reproducible-builds

Run on the operator's own request in the implement session (2026-10-27, «through its operator pass and the CI
read»), the entries fired in their plan's operator forms; lines copied verbatim, headers left out.

## Entry 23 — hygiene, fired as written, before the pre-CI commit

First firing (2026-10-27T02:31Z):

```
P1 .andromeda/runs/2026-10-27T02-08-46-implement/journal.md:9 ×1 · tmp
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 528 (runs 513 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The journal's line 9 named step 3's scratch by its temp-dir path; rewritten as «a `viola-race-*` dir under the OS
temp dir». Re-fired:

```
hygiene: clean — read 528 (runs 513 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## Entry 22 — pre-push, re-fired before the push (its note)

```
 22 probe       green · exit 0 · 100.64s · 422769 B → 22.2.log · bash scripts/agent-run.sh pre-push
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

## The pre-CI commit and entry 24 — the push

Pre-CI commit `6c77eadd` (`chore(2026-10-27-pinned-copy-race-and-reproducible-builds): operator pre-CI commit, for the
run this chunk's verdict reads`), the tree clean after it. Entry 24 through `gate.py run --plan … --operator 24`:

```
operator entry 24 · history tripwire: git
 24 probe       green · exit 0 · 3.25s · 92 B → 24.log · history moved: refs/remotes/origin/HEAD 99977116→6c77eadd; refs/remotes/origin/build/viola-0.1.0 99977116→6c77eadd · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

## Entry 25 — the CI read of `6c77eadd`, fired as written — green

```
6c77eadd4db2 verdict: green · checks 15/15 · wall 746 s · runs ci#40439188053 completed/success
runs: ci#40439188053 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

The OS-sensitive witnesses (the entry's note), read from each test job's log (`gh run view 40439188053 --job <id>
--log`):

| case | test (ubuntu-latest), job 121975124223 | test (macos-latest), job 121975124225 | test (windows-2025), job 121975124210 |
|---|---|---|---|
| `fs::tests::place_private_new_placed_copy_execs_at_once` | PASS 0.009 s | PASS 0.024 s (no red leg: no `ETXTBSY`) | — (`cfg(unix)`) |
| `fs::tests::place_private_new_placed_copy_runs_at_once` | — (`cfg(windows)`) | — | PASS 0.077 s (red leg unmeasured) |
| `pin::tests::place_absent_*` (3 rows) | PASS ×3 | PASS ×3 | PASS ×3 (`MoveFileExW` without replace) |
| `strict::win::tests::strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` | — | — | PASS 0.020 s (image `windows-2025-vs2026` 20261021.618, region centralus) |
| `mcp_tools_path7_unverified_cli_answer` (the default pair, unstamped) | PASS 0.839 s | PASS 0.743 s | PASS 0.620 s |
| `path1_run_start_one_home_in_order` | PASS 8.219 s (against 13.468 s green `ci#40403389629`, 20.089 s red `ci#40425866096`) | PASS 6.899 s | PASS 6.306 s |
| Summary | `3227 tests run: 3227 passed (5 slow), 0 skipped` | `3181 tests run: 3181 passed (6 slow), 0 skipped` | `3182 tests run: 3182 passed (5 slow), 0 skipped` |

The lint jobs (each OS) green: the `cfg`-gated cases compile clean there.
