# Outcome — 2026-11-20-harness-prefix-and-identity-floor-ledger-rows

**Run:** 2026-11-21T06-22-40Z-implement · **Outcome:** surfaced — live S1 (`HF`) read red on two rows; a finding,
stopped before the landing and the operator pass (the directive inputs#I6, its Stops; plan step 25).

## What surfaced (the start's own words, `evidence/live-round.md`)
- `failing: cross-session-prefix,identity-floor`, `pass-count: 27/29`, `verify-exit: 1`, `record: none`.
- `cross-session-prefix: fail` · `cross-session: no-prompt`: the print-mode sender exited 0 and no UserPromptSubmit
  reached Run B within the 30 s window — the sender's prompt did not arrive. viola reads no sender text, so a model
  refusal, a missing tool and an unresolved session-id target read the one word (plan step 10; the T3 lookup-miss
  premise, plan step 9). Value/contract: architecture `decision/harness-prompts-not-human` relied on a cross-session
  arrival; 2.1.328 measured no arrival under this probe.
- `identity-floor: fail` · `env-unknown: 2` · unshaped 0: unknown names, names only — `CLAUDE_CODE_CHILD_NOTES_DIR`,
  `CLAUDE_PLUGIN_DATA` (`added` holds 13 names; entry 24). Value/contract: security-plan
  `decision/identity-floor-eleven-names` and the compiled `NON_IDENTITY_HOOK_NAMES` (3).
- Not the one red that opens `HG` (that needs `identity-floor` alone failing with `cross-session-prefix: pass`).
  No further start; `HF-2` unspent (a red reading); `HG` not opened. No `--record` refusal (no record written).
- Nothing authored: no spec, master, plan or route edit.

## Gates
- Phase A (27 rows): entries 1-7 green (second firing; first firing red on 6 `hook_capture_files_the_raw_payload_and_prints_nothing`
  (a dir count the env file moved) and 7 `boundaries_each_name_a_bounded_read_that_stands` (the census-pinned read text) — both fixed).
- Phase B: entries 8-14 green (second firing; first red on 8 fmt and 9 clippy `too_many_arguments` on `merge_stamp` — fixed).
- Controls (`evidence/control-1.txt`): kept-names, floor-unknown, prefix-match — 3 holds.
- Round: whole form refused at pre-flight (operator 22, non-leg 23-25 inside its span); fired `--live-legs --entry 21`:
  15-20 green, 21 red `contains 'verify-exit: 0' ✗`. After-reads: 22 census 0 green · 23 red `29 short` · 24 recorded
  (names above) · 25 `live-stamp: unmoved` green. Not fired: 26-38 (name diffs, landing, layers, smoke, operator pass).
- Expected reds standing until a landing: the stamped-home suites at 2.1.328 (`cross-session-prefix` fails over the
  committed set, which holds no arrival) and `env_fixtures_pass_hygiene` / `claude_framing_variants_are_walked_for_every_framing_set`.
- Matrix: no `ref` / `implemented` written — v2-25 and v2-26 are not verified (a red live reading verifies nothing).

## Files changed
34 (gate.py scope: clean — changed 34 · listed 20 · recorded 14 (companion 11 · in-intent 3)). The code of steps 1-21
and 28-29 is in the tree, uncommitted; fixtures untouched.

## Deviations (each with its why)
- `Probes.env` is the `EnvReading` (kept names removed in `measure`), not the raw document: no kept name stands in Probes.
- `ledger::env_names_fault`: the record's user-word check for an env name, `_`-bounded — the existing scrub reads `_`
  as a word char, so step 19's premise held only with it.
- The fake sender delivers only to a sibling whose argv takes inbound messages (`--framing` + the accept literal) and
  writes a `send` receipt line, not `start`: else the no-framing case waits out the 30 s window past CI's 20 s kill.
- `merge_stamp` takes the parallel-dialog measure inside `Readings` (clippy's seven-argument bound).
- Control ids shortened to the tool's 16-char form (`kept-names`, `floor-unknown`).
- Scope record (14 lines): companion — Cargo.toml, .config/nextest.toml, secret_scan.rs, auto_verify.rs, version_gate.rs,
  typed.rs, diag-line schema, contract_diag_schema.rs, test_cases_framing.rs, cli_verify/stamps.rs,
  contract_fixture_hygiene.rs; in-intent — tests/security_negatives_env_capture.rs, fake test_cases_modes.rs,
  tests/support/verify.rs; all `self`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola-live-rig (HF), its claude 2.1.328 children, the sender | a leg (21) | terminated — rig `census: 0`, entry 22 `census: 0` |
| gate entries' cargo / nextest / fake agents | this run | terminated (gate census 07:00Z: no viola-fake-agen, nextest, harness or rig) |
| 8 `claude` sessions (overseers, builders, worker) | NOT DERIVED — the host's own | left running: not this run's |

## Decisions & corrections
- The operator's word (inputs#I7): go on with the round on a QUIET read from 06:57:57Z; the read was QUIET.
- Host read 1 (06:54:14Z) CONTENDED naming escher → no start, stopped (the directive's host rule).
- Guard refusals: the bash guard refused two heredoc-with-file-target appends and one pipeline-into-grep; each re-issued once.

## Next
The overseer brings the finding to the founder (two reds: the cross-session arrival absent, two unknown names).
Any further start is the founder's; a mend goes through the phase's revision door.
