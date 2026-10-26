# Operator pass — 2026-10-25-windows-endpoint-admission

Run by the session on the operator's word at invocation (2026-10-25T23:49Z): «run 2026-10-25-windows-endpoint-admission
through its operator pass and the CI read (supply-chain red is expected, owned by the tokio lock-bump entry; the
windows-2025 leg is this chunk's witness).»

## Entry 21 — hygiene, fired as written

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`, last read 2026-10-26T00:03Z,
right before the pre-CI commit (an earlier read at 00:01Z read the same):

```
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 427 (runs 416 · evidence 3 · inputs 8) · trails 13 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record is written after that read (plan-template §Test Commands, the `leg` key).

## pre-push, re-run before the push

`gate.py run --plan … --entry 20`, no run dir, 2026-10-26T00:01Z:

```
 20 probe       green · exit 0 · 100.13s · 392516 B → 20.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

## The pre-CI commit

`git add -A` once hygiene read clean, 2026-10-26T00:03Z: `e2bb59c8 chore(2026-10-25-windows-endpoint-admission):
operator pre-CI commit, for the run this chunk's verdict reads` (on `b6740d59`); tree clean after it.

## Entry 22 — the push, `gate.py run --plan … --operator 22` (no run dir)

```
operator entry 22 · history tripwire: git
 22 probe       green · exit 0 · 3.45s · 92 B → 22.log · history moved: refs/remotes/origin/HEAD b6740d59→e2bb59c8; refs/remotes/origin/build/viola-0.1.0 b6740d59→e2bb59c8 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

The move is the intended one: the pushed branch's remote ref, fast-forward, `b6740d59→e2bb59c8`.

## Entry 23 — `test (windows-2025)`, fired as written

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400 --name 'test (windows-2025)'`:

```
ci v1.3 · e1a0c23b
repo Turbolet85/viola (the push remote `origin`) · names test (windows-2025) · polled 23× over 683 s
e2bb59c8973d verdict: green · checks 1/15 · wall 659 s · runs ci#40247500584 completed/failure
runs: ci#40247500584 push completed/failure
checks: test (windows-2025)
```

**`verdict: green` on ci#40247500584, sha `e2bb59c8`.**

### The Windows witnesses, from the `junit-windows-2025` artifact of ci#40247500584

Read with `gh run download 40247500584 -n junit-windows-2025` into a scratch dir; `nextest-run` 2988 tests, 0
failures, 0 errors. The rows this chunk names (status · time · classname :: name):

```
passed      0.008s  viola::security_negatives_channel :: security_negative_viola_listener_rejects_remote_clients
passed      0.009s  viola::security_negatives_channel :: security_negative_control_accept_remote_pipe_reads_the_bit_clear
passed      0.009s  viola::security_negatives_channel :: security_negative_control_reject_remote_pipe_reads_the_bit_set
passed      0.009s  viola::security_negatives_channel :: security_negative_viola_listener_denies_an_anonymous_open
passed      0.009s  viola::security_negatives_channel :: security_negative_control_default_pipe_admits_an_anonymous_read
passed      0.009s  viola::security_negatives_channel :: security_negative_viola_client_grants_only_identification
passed      0.012s  viola-channel :: server::win::tests::bound_pipe_dacl_reads_back_protected_user_and_system
passed      0.011s  viola-channel :: server::win::tests::owner_only_sddl_is_protected_user_and_system
passed      0.036s  viola-channel :: server::win::tests::pipe_handles_on_both_ends_are_not_inheritable
passed      0.013s  viola-channel :: server::tests::bind_of_a_live_endpoint_is_taken_until_it_drops
passed      0.025s  viola::channel_endpoint :: channel_endpoint_second_bind_is_taken_while_the_first_lives
passed      5.318s  viola::cli_instance_state :: run_refuses_a_squatted_endpoint
passed      5.989s  viola::path1_run_start :: path1_run_start_one_home_in_order
passed      0.033s  viola::bin/viola :: run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor::case_4_both_name_five_hour
passed      0.019s  viola::bin/viola :: tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first
```

So on that runner the two controls tell the remote-client bit apart (accept-remote reads it clear, reject-remote reads
it set), and the default-descriptor pipe admits the anonymous read that the viola listener denies: neither claim is
owed to "Windows-only live measurements".

## Entry 24 — the full row, report-only, fired as written

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`:

```
ci v1.3 · e1a0c23b
repo Turbolet85/viola (the push remote `origin`) · polled 1× over 2 s
e2bb59c8973d verdict: red · checks 15/15 · first-fail +28 s supply-chain · wall 659 s · runs ci#40247500584 completed/f…
runs: ci#40247500584 push completed/failure
failed 1: supply-chain (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/viola/actions/jobs/121332079152/logs
run failed: ci#40247500584 failure
```

Recorded, not asserted. The other 14 checks read `success` (`gh run view 40247500584 --json jobs`): lint (ubuntu-latest,
macos-latest, windows-2025) · test (ubuntu-latest, macos-latest, windows-2025) · release (ubuntu-latest, macos-latest,
windows-2025) · perf (ubuntu-latest, macos-latest, windows-2025) · fuzz-replay · msrv.

`supply-chain`'s log holds one `error` diagnostic: `"code":"unsound"`, `"id":"RUSTSEC-2026-0386"` (the tokio
`AsyncFd` advisory, dated 2026-10-13), beside four `duplicate` warnings. That is the expected red, owned by the route
entry "tokio RUSTSEC-2026-0386 lock bump" (`inputs#I2`); no ignore was widened.

## The `watch:` subject (the macOS readiness stall)

`test (macos-latest)` read `success` on ci#40247500584: one more green run of the watched leg (recorded for the wrap's
tally; the retirement is the wrap's to rule).
