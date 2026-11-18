# Outcome — 2026-11-18-browser-tools-prompt-modal (implement 2026-11-18T16-00-06Z)

**Outcome: green + smoke.** Next → the OPERATOR PASS in this same window (operator entries 18 pre-push, 19 hygiene,
20 push — the operator's, 21 `ci.py conclusion`; 15–17, the smoke, already fired once in P3 and recorded in
`evidence/smoke.txt`), then a fresh window's `/andromeda-wrap-session`.

## Files changed (20, 0 new)
crates/viola-agent-claude/src/{screen.rs, ledger.rs, ledger/test_cases.rs, ledger/test_cases_checks.rs,
ledger/test_cases_stamps.rs} · src/cmd/verify.rs · src/cmd/verify/typed/{pty.rs, turns.rs} · src/run/{gate.rs,
version_gate.rs} · src/bin/viola-fake-agent.rs · tests/{cli_send.rs, cli_verify.rs, cli_verify/runs.rs,
cli_verify/stamps.rs, contract_ledger_probes.rs} · crates/viola-e2e/src/harness/run.rs ·
crates/viola-e2e/src/rig/{mod.rs, isolate.rs} · crates/viola-e2e/tests/rig_rehearsal.rs

## Gates
- Block entries 1–7 green on first firing (fmt · clippy · agent crate · run --unit · the five binaries · run
  --integration · run --e2e) — trail `.andromeda/runs/2026-11-18T16-00-06Z-implement/gate-2026-11-18-browser-tools-prompt-modal.json`.
- Controls: bt-none, bt-modalsig, bt-literal `holds`; bt-absent `INDETERMINATE: … not run — the runner stopped at
  448/650` (entry 3 is fail-fast nextest; the removed firing read 7 failed, the guard's neighbour
  `counts_reads_pass_and_absent_on_the_browser_tools_row_alone::case_2_absent` among them) — `evidence/control-1.txt`,
  `evidence/control-2.txt`.
- Live round: `round: COMPLETE · legs fired 2/2` — BP green (browser-tools-prompt pass, B/C/D/F browser-tools,
  cache-a written, keys none, stop none), BK green (24/24, failing none, browser-tools-prompt absent, B–F box,
  cache-a absent) — `evidence/round-162034Z.txt`, `evidence/live-starts.md`, ledger `evidence/prompt-starts.ndjson`
  (2 base starts, spare unspent).
- After the report's sweep: `crates/viola-e2e/src/harness/run.rs` `LIVE_PASS` (a test constant) renumbered
  `[NN/23]` → `[NN/24]`; entries 1, 2, 4 re-fired green (`--only 1,2,4`). No product or rig code moved.
- Smoke (operator 15–17): boot ok, status ready (`cli_verified:true`), cleanup processes_gone — `evidence/smoke.txt`.

## Deviations (with justification)
- Scope record (`gate.py scope`: clean — changed 20 · listed 17 · recorded 3):
  - `tests/cli_send.rs` · in-intent · serves step 10 · self
  - `tests/cli_verify/runs.rs` · in-intent · serves step 13 · self
  - `tests/cli_verify/stamps.rs` · companion · serves step 12 · self
- `tests/support/home.rs` (step 12 named it) is unchanged: its stamped-home rows derive from `LedgerRow::relied()`
  and moved to 24 by construction; `src/cmd/verify/typed/run.rs` and `crates/viola-e2e/src/rig/screen.rs` needed no
  arm (the compiler named no `FirstScreen` match); `tests/cli_version_gate.rs` unchanged — the 23 → 24 cases sit in
  `src/run/version_gate.rs`'s inline module (the plan's either-or).
- `tests/cli_verify.rs` `steps_failing` now rewrites a step line's last verdict word (pass or absent) to `fail`, not
  only `  pass`: the 24th line ends `absent` on the plain fake.
- The step line's verdict word for a non-pass, non-absent `row_word` (a trust-record writer word) prints `fail`
  (`step_word`), as before; the stamp keeps the writer's word.
- `--start-screen` applies under `--screens` only (a trusted start's ready screen exists only there).

## Surfaced / stuck
none. Plan gap reported: the bt-absent control's entry (3) runs nextest fail-fast and names no filter, so the case
could not be reached; the reading stays INDETERMINATE.

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola verify + claude 2.1.325 children (BP, BK) | a leg (entries 11, 13) | terminated — rig `census: 0` each, round census 0 after each |
| harness supervisor, wrappers, fake agents (screen-smoke) | this run (operator 15) | terminated — cleanup `processes_gone:true` |
| block and control test processes | this run | terminated — `gate.py census`: none running |
`gate.py census --comm viola-fake-agent --comm viola-live-rig --comm viola-harness --args projects/viola/target/debug/viola --args projects/viola/target/harness/debug/viola`: `census: none running`.

## Decisions & corrections
- Guard refusal: `cd` into the skill references dir refused by the bash guard (moves the session cwd) — re-issued
  with absolute paths.
- Two python heredoc edits failed before writing (a `\u` escape in a Rust format literal; an anchor in the wrong
  file); the escape-bearing edit went through the Edit tool (host-linux.md 2026-09-28).
- Measured: BP `cache-a: written` beside `screen-b: browser-tools`; BK `cache-a: absent` — read at verify's end only.
- Measured: Run E does not start when Run B stands on the prompt (`screen-e: null`; the cause inferred).
