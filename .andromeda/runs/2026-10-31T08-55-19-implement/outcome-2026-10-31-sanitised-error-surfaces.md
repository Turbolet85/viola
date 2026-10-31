# Outcome — 2026-10-31-sanitised-error-surfaces: green + smoke

Next: `/andromeda-wrap-session`. The operator pass is done: entry 23 `hygiene: clean`, pre-CI commit `fa7be1f3`, entry 24 pushed it with the three records-door commits, entry 25 `fa7be1f37746 verdict: green · checks 15/15 · wall 838 s · runs ci#41188681876 completed/success`. Its whole record: `chunks/2026-10-31-sanitised-error-surfaces/evidence/operator-pass.md`. No fix commit was needed.

## Gates
Second firing of the whole block: `entries 25 · green 21 · red 0 · recorded 1 · timeout 0 · not-run 3` (the three `operator` entries). The first firing read red at 10 and 22, one cause, under Deviations. No deferral: every entry ran.

- `pre-push`: exit 0, `"suite":"coverage","passed":3347,"failed":0`, `gate` `"breaches":[]`.
- The two new binaries: `16 tests run: 16 passed`. The pins that must not move: `294 tests run: 294 passed`.
- `run --perf`, recorded for the comparison line: 2.0 / 2.0 / 2.0 / 2.0 / 2.2 ms mean (session-start, user-prompt-submit, stop, session-end, pre-tool-use); the earlier reading is 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms. No cause is claimed for the difference.
- Path 1 ran green locally: `path1_run_start_one_home_in_order` 5.989 s in entry 10. The WATCH's tally is the CI read's.

- **A red outside the block, in the operator pass:** the first pre-push re-run read 2 failed of 3347, `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` both cases, at the 7 s wait, under a host record of `CONTENDED`, 20 s stalled on IO with an `escher` build alive. It reads green in the block, in five rounds alone and in the next pre-push, and the push waited on that green. Recorded with what is and is not measured: `chunks/2026-10-31-sanitised-error-surfaces/evidence/operator-pass.md`. For the wrap: the case's 4.3 s against its 7 s wait is a thin margin on a contended host, and no bound moved.

## Smoke
✓ By hand on an unstamped session `p-errors-hand`: boot exit 0, status `ready`, cleanup `processes_gone` and `endpoint_gone` true. The block's entries 16 to 21 hold the same with G2 and G4 over the home.

## Remove-the-guard controls
14 of 14 guards read red under their plant and green restored: `chunks/2026-10-31-sanitised-error-surfaces/evidence/remove-the-guard.md`.

## Files changed (26, all on research's lists)
New: `tests/contract_error_bodies.rs` · `tests/channel_error_bodies.rs` · `tests/cli_error_bodies.rs`.
Modified: `crates/viola-core/src/lib.rs` · `crates/viola-channel/src/lib.rs` · `crates/viola-channel/src/client.rs` · `crates/viola-channel/src/server.rs` · `schemas/diag-line.v1.json` · `src/cmd/client.rs` · `crates/viola-mcp/src/result.rs` · `crates/viola-mcp/src/lib.rs` · `crates/viola-mcp/tests/handlers.rs` · `crates/viola-agent-claude/src/lib.rs` · `src/run/mod.rs` · `src/cmd/run.rs` · `src/cmd/verify.rs` · `tests/support/events.rs` · `Cargo.toml` · `.config/nextest.toml` · `crates/viola-e2e/tests/mcp_tools.rs` · `src/cmd/hook/statusline.rs` · `src/run/version_gate.rs` · `src/run/budget.rs` · `src/run/dialog.rs` · `src/run/env.rs` · `tests/contract_bounded_readers.rs`.
Ledger: `viola-0.1.0/verification-matrix.json`, `v1-47` implemented.

Scope read: `scope: clean — changed 26 · listed 26 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 581`. There is no scope record.

## The guard entry's pass line (entry 15, whole)
`guard: allowed 26 — .config/nextest.toml, Cargo.toml, crates/viola-agent-claude/src/lib.rs, crates/viola-channel/src/client.rs, crates/viola-channel/src/lib.rs, crates/viola-channel/src/server.rs, crates/viola-core/src/lib.rs, crates/viola-e2e/tests/mcp_tools.rs, crates/viola-mcp/src/lib.rs, crates/viola-mcp/src/result.rs, crates/viola-mcp/tests/handlers.rs, schemas/diag-line.v1.json, src/cmd/client.rs, src/cmd/hook/statusline.rs, src/cmd/run.rs, src/cmd/verify.rs, src/run/budget.rs, src/run/dialog.rs, src/run/env.rs, src/run/mod.rs, src/run/version_gate.rs, tests/channel_error_bodies.rs, tests/cli_error_bodies.rs, tests/contract_bounded_readers.rs, tests/contract_error_bodies.rs, tests/support/events.rs · companions excluded 0 · changed 26 · excluded 0`
No file was excluded.

## Deviations from the plan, with why
1. **Step 7 boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env`.** `Wrapper` takes no environment and `tests/support/home.rs` is outside the plan's lists and the guard's allow. The planted value reaches that child through the PTY spawn's `env_set`, the same form as `tests/tui_env_strip.rs`.
2. **`tests/contract_error_bodies.rs` holds its own scanner** beside `tests/contract_bounded_readers.rs`'s: that one blanks string literals, and this census reads them. A shared helper would be a `tests/support/` file outside the allow.
3. **The `PLUMBING` table is added to step 11's tables.** It names the census lines that build or route a document a row names (`refused_document`'s head, the frame around the channel bodies), so a row stays one body.
4. **The MCP witness's session is unstamped.** No row reaches a stamped path, and it keeps the case small under the kill line.
5. **The first block's red, an oracle of this chunk:** the unknown tool's `data` was asserted `Some(Null)`. rmcp's client reads the wire's `"data":null` as `None`, so the case now reads `None`, and a held value would still read red. The wire's null is the product's `Some(Value::Null)` (`crates/viola-mcp/src/tools.rs`); the live case reads it through the client only.

## Surfaced
Nothing new. The spec corrections the plan already names stand for the wrap (its Expected amendments), the "exit 11" one included: the process exit reads 1 in `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`, and 11 is the unit child's own code in `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`.

## Read at the operator pass, for the wrap
- `v1-47`'s `secret-scan` clause: `success` on all three `test` jobs, each document `"ok":true`, `"hits":[]`. The Windows and macOS legs of every new case and the `.cmd` refusal read PASS.
- The WATCH: Path 1 green at 9.067 s on `test (ubuntu-latest)`, 3/3 by `scope.md`'s tally, its retire condition. The handoff's "1/3" predates Setup 5a's reading, so the wrap reads the tally from `scope.md`.

## Still open
- The PROVISIONAL marker on this chunk's P5 word (the delegate overseer, `.andromeda/runs/2026-10-31T07-21-45-phase/relay-1.md`) stands, owed the founder's own word.

## Process hygiene
Census: `census: 8 running`, none from this repository's `target/` (trail `.andromeda/runs/2026-10-31T08-55-19-implement/gate-2026-10-31-sanitised-error-surfaces.json`).

| process | started by | final state |
|---|---|---|
| `viola run` · `viola wait` under `~/dev/projects/additional/viola-lab/prototype/target/debug/` (6 rows) | not this run: another tree's bridge | left running: not this run's — their owner stops them |
| `~/.viola/bin/0.1.0-231c0225dac912d9/viola mcp` | not this run: another session's server | left running: not this run's — its owner stops it |
| `~/.local/viola-product/bin/viola wait viola-builder …` | not this run: the overseer pair's bridge | left running: not this run's — its owner stops it |
| the harness sessions `p-errors-smoke` and `p-errors-hand` | this run | terminated: cleanup read `processes_gone` and `endpoint_gone` true |
| the test wrappers and fake agents of the gate block | a gate entry | terminated: none in the census |

## Decisions & corrections
- The operator's direction at the invocation: no live `claude` session (none ran); a kill deadline on every spawn (the three new owned-child cases and the two channel and CLI witnesses each carry one); each long run journalled first (`journal.md`); the operator pass with its CI read.
- Stamp-ahead hook, one refusal: a journal line stamped `09:17Z` by estimate while the clock read 09:14:46Z, corrected from `date -u` to `09:14:51Z`. A recurrence of the standing learning that a timestamp is read, never estimated. The journal's earlier lines `09:08Z` and `09:11Z` were written the same way, by minute and unchecked, and the hook passed them: the gate trail's `ts` is the record of when the block fired.
- Bash guard, one refusal: a `cat` heredoc to a run-dir script, re-issued once as a Write-tool file in the scratchpad.
- `controls.py`'s last-line summary was shadowed by its own loop and printed `no summary`; the record's line is copied from its printed output instead. Its fourteenth plant did not build and proved nothing, so it was re-planted and read red.
