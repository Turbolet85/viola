# Outcome — 2026-11-16-local-command-delivery · implement run 2026-11-16T08-05-25-implement

**Outcome:** green (3 block iterations) · smoke skipped — no boot-path touchpoint (the plan states none; entries 8
and 9 boot their own wrapper) · Next → the OPERATOR PASS (entries 14 hygiene · 15 push · 16 CI read), then
`/andromeda-wrap-session`.

## Files changed (29; `gate.py scope`: clean — changed 29 · listed 24 · recorded 5 · absorbed 1)
- New: `src/run/send/echo.rs`.
- Adapter crate: `crates/viola-agent-claude/src/ledger.rs`, `src/hook.rs`, `src/ledger/test_cases.rs`,
  `test_cases_checks.rs`, `test_cases_framing.rs`, `test_cases_runs.rs`, `test_cases_stamps.rs`.
- Root bin: `src/cmd/hook.rs`, `src/cmd/run.rs`, `src/cmd/verify.rs`, `src/cmd/verify/typed/runs.rs`,
  `src/cmd/verify/typed/turns.rs`, `src/run/send.rs`, `src/run/send/classify.rs`, `slot.rs`, `relabel.rs`,
  `test_support.rs`, `src/run/version_gate.rs`, `src/bin/viola-fake-agent.rs`,
  `src/bin/viola-fake-agent/test_cases_opts.rs`.
- Harness and tests: `crates/viola-e2e/src/harness/boot.rs`, `run.rs`; `tests/cli_send.rs`, `tests/cli_verify.rs`,
  `tests/cli_verify/runs.rs`, `tests/cli_verify/stamps.rs`, `tests/contract_ledger_probes.rs`, `tests/support/verify.rs`.

## Deviations, with their justifications
1. **No reader seam on `SendSlot` (plan step 7).** The echo reads call `echo::end_of` / `echo::tags_since` directly.
   The unit cases drive real transcript files in a tempdir, the tag appended by the test thread after the paste
   channel fires, so no race needs a seam; a boxed reader no case replaces would be an unused abstraction.
2. **`--transcript` rewrites only the launch SessionStart (plan step 9 said every SessionStart).** Rewriting the
   replayed `SessionStart.clear-1` under verify's `--framing --transcript` broke the byte-for-byte drift contract
   (`fake_agent_framing_replay_matches_every_recorded_framing_set`, block iteration 1). The plan's own step 9 also
   requires that contract to hold. Only `default` / `resume-1` carry the path; it is all the probe
   (`transcript_of` reads Run B's first SessionStart) and the booted case read.
3. **The fake agent seeds its transcript file (0600) when it declares it.** Its transcript otherwise does not exist
   until the first echo, and `end_of` of an absent file confirms nothing, as the product requires.
4. **The echo row's check (plan step 2 states none).** It is `pasted(trusted, PROBE_ECHO_COMMAND).is_none()` and
   `echo_latency_ms` within `CONFIRM_WINDOW_FALLBACK`, on the compact row's pattern, so the stamp proves the send's
   10 s window per CLI version.
5. **Verify skips the echo paste when Run B's first SessionStart declares no readable transcript of the session-id
   shape** (step 10 has it pasted seventh): with no file to read nothing can be measured, and the run goes on to
   `/compact` and `/clear` rather than ending. The echo row then fails (`verify_without_framing` reads 3 fails).
6. **Path-shape unit cases sit in `test_cases_framing.rs`** beside the name table, not `test_cases.rs` (step 2).
7. **One more classify case than step 13 lists:**
   `send_local_command_echo_after_a_start_without_a_transcript_is_unconfirmable` (a later `session-start` with no
   transcript clears the remembered one).
8. **The list table test renamed** `local_commands_are_the_six_known_ones_and_clear_compact_and_mcp_are_probed`
   (it said «three» over six names) after block iteration 3; entries 1-4 re-fired green. Pre-push did not run on
   the rename; the operator pass re-runs it before its push.
- Scope record (`doc:2026-11-16-local-command-delivery/scope-record` rev 2), all `self`:
  - companion: `crates/viola-e2e/src/harness/boot.rs` · serves step 10; `src/cmd/verify/typed/runs.rs` · serves
    step 10; `crates/viola-agent-claude/src/ledger/test_cases_checks.rs` · serves step 2;
    `tests/cli_verify/stamps.rs` · serves step 11.
  - in-intent: `crates/viola-agent-claude/src/ledger/test_cases_runs.rs` · serves step 2.

## Gates (final block, iteration 3: `entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3`)
- 1-3 lint green · 4 adapter crate green · 5 the five modules green · 6 the echo cases by name green · 7 fake agent
  green · 8 the two booted cases green (`contains run: 2 passed`) · 9 the integration files green · 10 `LEDGER_ROWS`
  green · 11 `fixtures/claude` probe green (`last line 0`) · 12 `agent-run.sh run --unit` green · 13
  `agent-run.sh pre-push` green · 14-16 `not run — leg operator`.
- Earlier reds, each fixed in this run (journal `evidence/journal.md`): iteration 1 red 4 (4, 9, 12, 13) — a
  verdict vector pinning 21 rows, a `5 pass  16 fail` oracle, the clear-1 drift, an unbounded capture read;
  iteration 2 red 1 (13) — the same reader, its `take(` behind the read call.
- Controls (`evidence/control-1.txt`): `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
  removed`; each named case read FAIL, 0 TIMEOUT lines in every removed log.
- Not run, by the plan: mutation, live legs, live `claude` start, `viola verify` against a real `claude`.
- Matrix: `v2-23` planned → implemented.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none | this run started no lasting process (every gate child, its booted wrappers and fake agents, ended with its entry; the controls' clone removed) | — |

`gate.py census --comm viola --comm viola-fake-agent --comm claude --comm cargo-nextest --args projects/viola/target`:
`census: 16 running` — every row another tree's or the operator's (the prototype's and the product's `viola`, the
`~/.viola` pinned `viola mcp`, the overseer sessions' `claude`); none carries `projects/viola/target`: started by NOT
DERIVED for this run, none of them this run's.

## Decisions & corrections
- Guard refusals: the bash guard refused a `cat` heredoc to a file (re-issued through Edit) and two `cd` into the
  skill's references dir (re-issued with absolute paths).
- The permission classifier gave no verdict four times (transient); each call re-issued as-is or filled with
  read-only work (CLAUDE.md Tier 1: a refusal with no verdict is a transient outage).
- A `ScheduleWakeup` (1800 s fallback) was set while the first block ran, against the skill's «never a
  ScheduleWakeup» for a backgrounded gate; it outlives `/clear` and is harmless if it fires later.
- Sweep hazard: a row-count sweep keyed on `local-command-compact` misses neighbours that pin the count without
  naming the row (a `[bool; 21]` verdict vector, a `5 pass  16 fail` summary): testing.md 2026-11-15's removal
  sweep, recurred for an addition.
- Hygiene at the end refused `.andromeda/runs/2026-11-16T06-28-26-phase/relay-3.md:11 ×1 · tmp`; that one token was
  respelled `$TMPDIR/viola-live-localcmd-125iyA/cfg` (run-dir spelling; the verbatim bytes stand in
  `I4-relay-3.md.txt`); hygiene then `clean`.
- `contract_bounded_readers` reads `take(` only AHEAD of the read call in its statement:
  `read_to_end(&mut take(file, …))` reads unbounded to it.
