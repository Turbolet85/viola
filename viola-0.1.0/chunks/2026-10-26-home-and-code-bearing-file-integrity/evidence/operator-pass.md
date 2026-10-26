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
