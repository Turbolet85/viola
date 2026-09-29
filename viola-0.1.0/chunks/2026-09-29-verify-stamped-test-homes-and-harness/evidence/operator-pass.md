# Operator pass — 2026-09-29-verify-stamped-test-homes-and-harness

## Entry 21 — hygiene
First read: `hygiene: refused 10 files — P1 9 · P2 0 · P3 1`, every row in the phase run dir
`.andromeda/runs/2026-09-29T06-43-45-phase/` (none in this chunk's implement run or evidence).

Operator + overseer word (2026-09-29): "Delete uncited, rewrite log", overseer note "uncited copies are noise in
the tree (clean as you go); the cited job log and p5-dryrun get placeholder rewrites with line counts kept". And
"Rename to .rs.txt", overseer note "agreed, same precedent as the hook-perf control file".

Acted on exactly the refused rows:
- **Removed**, uncited by any chunk document: 7 copied detail files of other CI test homes, which were panic and
  backtrace records carrying runner paths. They are the builder instance's `detail-run.ndjson` in the kept homes
  `viola-test-{5V8cOK,5jYZ4Q,8pSD9w,Qy04ij,UJUlE8}` and its `detail-hook.ndjson` in `viola-test-{CMCydS,Dd3M6U}`,
  all under `ci/diag-windows-2025/`. The originals stay in ci#36532038635's
  `diag-windows-2025` artifact. The kept home research.md cites, `viola-test-B3yvcs`, was not refused and is
  untouched.
- **Rewritten**, path roots only, in place:
  - Files: `ci/job-109287650444.log` (cited by research.md §Item 8 and scope.md) and `p5-dryrun.txt`.
  - Placeholders: `<runner-workspace>` (the runner checkout), `<runner-temp>`, `<runner-home>`,
    `<program-files>`, `<host-home>`.
  - Every spelling was rewritten: single backslash, doubled backslash, forward slash and MSYS.
  - The job log kept its line count (1567) and CRLF terminators (1563), and so did `p5-dryrun.txt` (30 / 30).
    Lines 514-534 still hold the `case_08_unreachable_endpoint` red at `hook_fail_open.rs:238`, as cited.
- **Renamed**: `baseline/control/writer.rs` → `baseline/control/writer.rs.txt`, content unchanged. It is the P5
  known-positive control plan.md's gate-8 baseline cites as "run dir baseline/control/writer.rs". It tripped P3
  only by its extension.

Re-read: `hygiene: clean — read 133 (runs 131 · evidence 2)`.

## Before the push
`bash scripts/agent-run.sh pre-push` on the tree after the hygiene actions: exit 0, `"ok":true`,
`stage:"windows-tests"`, i.e. both the Linux (WSL) and host legs green.

## Entry 22 — push
The pre-CI commit `702a3b4` ("chore(2026-09-29-verify-stamped-test-homes-and-harness): operator pre-CI commit, …")
was pushed with the entry's exact form: `31dd995..702a3b4  HEAD -> build/viola-0.1.0`. There was one push and
no measurement push.

## Entry 23 — CI read
`ci.py conclusion --sha HEAD --wait 1800`: `702a3b466ef3 verdict: green · checks 15/15 · wall 248 s · runs
ci#36538832471 completed/success` (polled 9× over 250 s).

Per-OS reads from that run's job logs (`gh run view 36538832471 --log`), all PASS:

| test | ubuntu-latest | windows-2025 | macos-latest |
|---|---|---|---|
| `hook_fail_open …::case_08_unreachable_endpoint` | 0.186 s | 0.314 s | 0.287 s |
| `channel_endpoint stop_wait_holds_while_the_endpoint_answers` | 0.147 s | 0.095 s | 0.025 s |
| `contract_ledger_probes contract_ledger_probes_pass_over_every_recorded_set` | 0.258 s | 0.280 s | 0.281 s |

No CI red to fold. Final HEAD run id: ci#36538832471.

## Entry 6's freshness check
Recorded as a plan-target defect for the wrap to retarget, not accepted as a standing red:
`entry-6-freshness-plan-defect.md`.
