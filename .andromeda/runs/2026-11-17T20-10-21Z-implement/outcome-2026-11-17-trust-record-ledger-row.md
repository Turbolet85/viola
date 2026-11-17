# Outcome — 2026-11-17-trust-record-ledger-row (implement run 2026-11-17T20-10-21Z-implement)

**Outcome: green** — every non-leg gate green, the three controls hold, the live round COMPLETE, the boot smoke green.
Next → the OPERATOR PASS (entries 18 pre-push · 19 hygiene · 20 push · 21 ci.py conclusion), then
`/andromeda-wrap-session`.

## Files changed
New: `crates/viola-agent-claude/src/trust.rs` · `src/trust_record.rs` · `tests/contract_verify_home.rs`.
Modified: `crates/viola-agent-claude/src/{lib.rs,ledger.rs}` · `crates/viola-agent-claude/src/ledger/{test_cases.rs,test_cases_checks.rs,test_cases_stamps.rs}` ·
`src/main.rs` · `src/cmd/verify.rs` · `src/cmd/verify/typed.rs` · `src/cmd/verify/typed/runs.rs` · `src/run/version_gate.rs` ·
`src/bin/viola-fake-agent.rs` · `src/bin/viola-fake-agent/test_cases_opts.rs` · `tests/support/verify.rs` · `tests/cli_verify.rs` ·
`tests/cli_verify/{runs.rs,stamps.rs}` · `tests/contract_ledger_probes.rs` · `tests/contract_bounded_readers.rs` ·
`crates/viola-e2e/src/harness/{boot.rs,run.rs}` · `crates/viola-e2e/src/rig/{mod.rs,isolate.rs}` ·
`crates/viola-e2e/src/bin/viola-live-rig.rs` · `crates/viola-e2e/tests/rig_rehearsal.rs`.
Not modified though listed: `tests/support/home.rs` (`take_stamp` already goes through `verify::verify`), `tests/channel_error_bodies.rs`
and `tests/cli_version_gate.rs` (both already stamp through `support::verify::verify`; no direct verify start).

## Gates (the trail: .andromeda/runs/2026-11-17T20-10-21Z-implement/gate-2026-11-17-trust-record-ledger-row.json)
- `cargo fmt --all --check` green · `cargo clippy … -D warnings` red at the first firing (`clippy::type_complexity` on a
  test table in `trust.rs`), fixed (a type alias), re-fired green · `cargo nextest run -p viola-agent-claude` green ·
  `run --unit` green · `run --integration --filter …` green · `run --integration` green · `run --e2e` green · the
  retired-count probe green · `release-check.sh` green.
- Controls (`gate.py control`, `--timeout 900`, entry 5): `trust-nowrite` holds · `trust-noremove` holds ·
  `trust-cfgdir` holds — `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
  (evidence/control-1.txt).
- Live round: `round: COMPLETE · legs fired 1/1 · … red 0` (evidence/round-203938Z.txt). S0 reads `cli: 2.1.325 ·
  viola-build: c808ee96814fe771 · keys: none · verify-exit: 1 · trust-record: fail · pass-count: 7/23 · stop: none ·
  record-left: absent · census: 0 · credentials: removed · scratch: absent · user-config: changed-by-others`.
- Smoke (operator entries 15-17): boot `trust-smoke` ok · status `ready` (both instances `cli_verified: true`) · cleanup
  ok, `processes_gone`, `home_removed` (evidence/smoke-trust-smoke.txt).

## SURFACED for the founder (not a gate red; no plan entry moves)
The live S0 reading is not a clean measurement of the trust-record row. verify's own step lines (the rig's kept screen)
read rows 01-07 pass and 08-23 fail: every interactive run but Run A failed, including rows a live verify of 2.1.325
passed before (input-box-signature, the dialog rows, resume). Run B, C and D start in subfolders of the rig's trusted
cwd and never reached the input box; Run F's own record was written and removed (`trust-record: fail`, not a writer
word; `record-left: absent`) and the CLI still did not open on the box. HYPOTHESIS (unmeasured): under the rig's
private config the 2.1.325 CLI does not extend a trusted folder's record to its subfolders, and Run F failed for a
reason the frame did not keep. The spare S0-2 is unspent: S0 ran whole and read; it was not lost to its shape. The
trust-record row stays unverified on 2.1.325; «Session folder trusted by viola» (which writes only when this row
passes) needs this read before its S1/S2.

## Deviations (justified)
1. `--record` verify starts keep the real `HOME` and tie Run F through a kept `CLAUDE_CONFIG_DIR` naming the test-owned
   user home: verify's fixture scrub reads `HOME`, so step 7's `HOME` on a `--record` run broke every recording case.
2. A test-only fake-agent hold `--trust-record-blank` (a start the record alone trusts draws nothing): the stop case of
   step 10 would otherwise sample a ~300 ms race.
3. `without_record` also drops a `projects` object it empties: `with_record` creates `projects` in a document without
   one, and the removal must leave that document as found.
4. Control `trust-cfgdir` edits `src/cmd/verify.rs` (`trust_config_file`), where the kept-config branch lives; the plan's
   prose named `runs.rs`.
5. The new stop case carries no `#[cfg(unix)]` (Linux-only tree): no `GATES` row needed in a file outside the lists.
6. `contract_verify_home` is a `contract_` binary, which the E2E layer runs: entry 5's filter selects none of it; entry 7
   ran both cases green.
7. The rig's `V` arm counts `child: exit 1` as a whole start (a failing row is a reading); `verify-exit` records it.
8. A failed replace reads as a lost attempt (`lost` after three); a lock dir that cannot be made reads `lock-held`.
9. Stale leaf counts corrected in passing: `all twenty-four` → `twenty-three` in `tests/cli_verify.rs` and
   `tests/contract_ledger_probes.rs` headers (CARRY cry-2snj2igc's family).
Scope record: `src/run/version_gate.rs` · companion · serves step 9 · self — `scope: clean — changed 27 · listed 26 ·
recorded 1 (companion 1 …)`.

## Watch
`cli_verify runs::verify_window_without_screens_fails_every_interactive_row` reads 52.2-52.3 s (three runs) against
its 60 s `verify_window_` kill (was ~43 s): Run F adds one designed 8.5 s wait. No bound moved.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate entries 1-9 (cargo, nextest, harness) | this run | terminated (each entry's tree censused by the tool) |
| control clone firings (~/.cache/andromeda-control/a50bdecb) | this run | terminated · clone removed |
| live rig S0 and its viola verify + 7 claude starts | a leg | terminated · rig census 0 before and after |
| harness session trust-smoke (supervisor, 2 wrappers, 2 fake agents) | this run (operator entries 15-17) | terminated — cleanup processes_gone |
`gate.py census --comm claude --comm viola --comm viola-fake-agen --comm viola-live-rig --comm viola-harness --args
projects/viola/target`: 16 running, none under this repository's target (host Claude sessions, other projects'
wrappers, the product's `viola wait` / `viola mcp`) — not this run's.

## Decisions & corrections
- The Bash guard refused a `cd` out of the root (twice) and a heredoc to a file; each re-issued in the passing form.
- A `ScheduleWakeup` call was made in error and refused by the permission classifier; no wake or timer was set (the
  operator's direction: set no scheduled wake).
- The rig's live start cap: S0 used, S0-2 unspent; ledger evidence/live-starts.ndjson (start 2026-11-17T20:39:44Z).
