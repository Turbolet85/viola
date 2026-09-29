
## 2026-09-29-verify-stamped-test-homes-and-harness — verify-probe joins the spawn subjects; boot readiness checks the start records
**Section:** §6 Log Coverage (`process-start` field catalog; Boundary-call wrappers → Child / shell spawns); §4 Span / Trace Coverage (`verify` process-log and CI bullets; Scenario 1 product order); §12 Obs Decisions Log (D-35)
**Change:**
- `subject` is `self|claude-child|version-probe|verify-probe|agents-probe|statusline-shell` (was without `verify-probe`); the spawn set is `version-probe|verify-probe|agents-probe|statusline-shell`, verify's print-mode probe being `verify-probe`; verify logs its two spawns at the call site and `run_bounded` stays unlogged.
- `verify` with `VIOLA_NAME` writes, between its self pair, a `version-probe` pair and a `verify-probe` pair, each `process-exit` carrying `child_exit_status` and `duration_ms`; `run`'s version gate keeps one `version-probe` pair.
- In CI verify runs only against the fake agent at 2.1.283 (`tests/cli_verify.rs`, harness `boot` step 4 unless `--unstamped`, `stamped_home`; was "join with Verify-stamped test homes and harness"); the real-`claude` verify runs only through the local `run --local-live`, refused under `CI`.
- Scenario 1: the harness `boot` readiness stage `start_records` checks `events.ndjson` lines 1-3 with the missing code `<name>:events` (was "joins with … the `boot` step-4 owner").
**Why:** §6 names every child spawn as a pair and verify's two spawns wrote none (an overseer ruling at the capability-ledger wrap); a new closed value takes a Decisions Log entry.
**Kept:** §1 (the verbatim obs-scope copy) untouched, per the verbatim-scope rule; §3/§4/§6/§12 carry the fact.
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/
