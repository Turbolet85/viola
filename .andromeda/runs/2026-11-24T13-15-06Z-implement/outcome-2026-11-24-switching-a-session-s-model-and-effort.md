# Outcome — 2026-11-24-switching-a-session-s-model-and-effort (run 2026-11-24T13-15-06Z-implement)

**Outcome: green + smoke.** Next → the OPERATOR PASS (plan entries `bash scripts/agent-run.sh pre-push`, the hygiene
entry, the push entry, the `ci.py conclusion` entry), then `/andromeda-wrap-session`.

## Layers (directive `inputs#I5`)
- **The PTY core, its own layer, landed and proven first:** `crates/viola-pty/src/pump.rs`, `crates/viola-pty/src/lib.rs`,
  and the two callers threaded with a respawn handle no one requests on (`src/cmd/run.rs` `pump_child`,
  `src/cmd/verify/typed/run.rs` `Run::spawn`). Its own cases: `respawn_swaps_the_child_and_the_pump_continues`,
  `respawn_of_a_child_ignoring_the_signal_reads_not_ended`, `respawn_whose_spawn_fails_ends_the_pump_with_the_old_exit`,
  `respawn_without_a_running_pump_is_no_input` (viola-pty 44/44). On the core alone: fmt, clippy, viola-pty units,
  root units green, and the three whole layers green (unit 24.89 s, integration 74.55 s, e2e 11.83 s) BEFORE the
  channel method and the verb were written (`evidence/journal.md`).
- **The verb layer:** `src/run/switch.rs` (the `switch` method), `src/cmd/switch.rs` (the verb), `src/run/mod.rs`,
  `src/cmd/mod.rs`, `src/cmd/run.rs` (dispatch, the slot built at start, the handle handed the pump),
  `src/human/hints.rs`, `tests/cli_switch.rs`, `tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`,
  `tests/cross_exit_causes.rs`, and the six scope-record files below.

## Files changed
19 (gate.py scope: listed 13 · recorded 6). New: `src/cmd/switch.rs`, `src/run/switch.rs`, `tests/cli_switch.rs`.
Modified: `crates/viola-pty/src/pump.rs`, `crates/viola-pty/src/lib.rs`, `src/cmd/run.rs`, `src/cmd/verify/typed/run.rs`,
`src/run/mod.rs`, `src/cmd/mod.rs`, `src/human/hints.rs`, `tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`,
`tests/cross_exit_causes.rs`, and the recorded `src/run/send/slot.rs`, `src/run/send.rs`,
`crates/viola-channel/src/server.rs`, `src/human/verbs.rs`, `src/human.rs`, `tests/support/home.rs`.
Listed and unchanged: `schemas/diag-line.v1.json`.

## Deviations (with justifications)
- `schemas/diag-line.v1.json` unchanged: plan step 5 enters each new detail word «with its emitter», and no line emits
  `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed` — the CLI's refusal ends `process-exit` with no
  detail (as `send` does), the wrapper writes the channel pair only, and `channel-response` carries `detail` on a
  `-32602` alone.
- The step-2 late `session-end` screen is ONE-SHOT: it drops at most one `session-end` of the switched id, after the
  new child's `session-start`, while the new child's pid has a start time; a later `session-start` ends it. The plan
  names no lifetime; one-shot is the narrowest reading. The rule as written cannot tell the old child's late end from
  the new child's own: on a CLI whose old child fires none, the new child's first `session-end` of that id would be
  screened. Measured over the fake agent: neither child fires SessionEnd at all (old ends by SIGHUP, `exit 1
  kill-fallback`; new by Ctrl-C), so nothing was screened (`evidence/late-session-end-reading.md`). Carried with R6a.
- The rung reads come from the wrapper's own disk snapshot (`read_snapshot_or_replay`): `pending_dialog` (written by
  the wrapper before the dialog line), `agent_session_id`, `launch`, `window`. No dialog-slot code was touched.
- The `unverified-cli` rung reads the start's gate verdict (`cli_verified`): `LedgerRow::relied()` is every row,
  `resume-session` among them, so a verified CLI counts `resume-session` by construction.
- A `send` arriving while a switch stands (request → new `session-start`) is refused `turn-running` (the switch mark
  in the flight guard): `src/run/send.rs` companion edit.
- `switch` added to `viola-channel`'s closed method-label list, so the switch's channel pair names its method.
- The respawned child is spawned at the host's current size (the pump overrides the request's `size`).
- Tests beyond the plan's names: `switch_with_neither_flag_is_usage` is a three-row table (neither, a bad effort, a bad
  model) beside `switch_with_a_bad_effort_is_usage`; `switch_mid_turn_is_refused_turn_running` boots over
  `fixtures/fake-scripts/path3.json` (a gated turn) because an unscripted booted wrapper refused the setup send;
  extra unit cases `switch_pair_cases`, `switch_params_refused_as_invalid`, `switch_params_take_either_half`,
  `switch_respawn_outcomes_are_refusals`, `switch_with_a_pending_dialog_respawns_nothing`,
  `switch_refuses_a_manual_pause_with_its_detail`, `switch_refuses_a_session_id_of_another_shape_as_no_session`,
  `switch_request_refuses_an_unverified_server_inside_its_dispatch_span`.
- Scope record (gate.py scope: `clean — changed 19 · listed 13 · recorded 6 (companion 3 · mechanical 0 · in-intent 3
  · widening 0)`):
  - companion: `src/run/send.rs` (serves step 2) · `crates/viola-channel/src/server.rs` (serves src/run/switch.rs) ·
    `src/human.rs` (serves src/cmd/switch.rs) — each `self`;
  - in-intent: `src/run/send/slot.rs` (serves src/run/switch.rs) · `src/human/verbs.rs` (serves src/cmd/switch.rs) ·
    `tests/support/home.rs` (serves tests/cli_switch.rs) — each `self`.

## Gates
- `cargo fmt --all --check` green · `cargo clippy … -D warnings` green (one red first: `type_complexity` on the
  spawner seam, factored into `Spawned`) · `cargo nextest run -p viola-pty --no-fail-fast` green 44/44 ·
  `cargo nextest run -p viola --features fake-agent --bins --no-fail-fast` green 1401/1401 · `… -E 'test(/::switch_/)'`
  green 27 · `… --workspace … -E 'test(/^switch_/)'` green 12 · the readers' six binaries green 168 · hostwatch
  `recorded` QUIET (5 s stalled) · `run --unit` green 3347 · `run --integration` green 698 · `run --e2e` green 361.
- Controls (`gate.py control … --timeout 900`): sw-turn, sw-dialog, sw-launch-first, pty-reap-bound — `4 holds · 0
  timeout · 0 does not hold · 0 unreproduced · clone removed` (`evidence/control-1.txt`).
- Smoke (operator entries, boot path changed): boot green, status `ready`, cleanup green `processes_gone:true`
  (`evidence/smoke-trio.txt`).
- No `gate.py guard` entry in the plan.

## Unproven (stated plainly)
- R6a / R6b (carried to «The fleet on the founder's host», `cry-s4qhj2e3`): nothing here shows a REAL claude resumed on
  the new pair takes it for its next turn, that a real idle child ends on SIGHUP within 10 s with its transcript
  whole, whether a real CLI fires SessionEnd on SIGHUP (the screen above), or a real switch refused mid-turn. Every
  reading is over the fake agent.
- F1 (in-place restart by a new channel method, a wrapper ending its own child), F3 (refused mid-turn, mid-dialog,
  while the human types), F5 (CLI-only, no MCP tool) and F6 (`RESPAWN_REAP_BOUND` 10 s) are built PROVISIONAL on the
  delegate's word; the founder's answers are owed.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the smoke session's supervisor, two wrappers, two fake children (`switch-smoke`) | a leg (operator entry 12) | terminated (entry 14: `processes_gone:true`) |
| test wrappers and fake agents of the gate runs | this run (nextest, tied) | terminated (census: none under `projects/viola/target`) |
| 5 `viola` processes of other trees (the bridge's `mcp`, viola-lab's `run`/`wait`, the product's `wait`) | NOT DERIVED — not this run's | left running: not this run's |

`gate.py census --comm viola --comm viola-fake-agen --comm viola-harness --args projects/viola/target`: `census: 5
running`, none of them this repository's build.

## Decisions & corrections
- Two record timestamps were typed estimated (journal `13:34:00Z`, evidence `13:35Z`) and corrected against `date -u`
  (13:33:42Z) — a recurrence of CLAUDE.md's timestamp learning.
- The bash guard refused `rec.py query … | cut` (a pipeline tool into a filter); re-issued bare once.
- The directive's order held: the PTY core landed and stood green on all three whole layers before the verb.
