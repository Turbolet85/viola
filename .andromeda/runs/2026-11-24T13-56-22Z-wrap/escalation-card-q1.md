# Q1 — the F1 boundary widening (a new wrapper channel method `switch`; a wrapper ending its own live child and resuming the session in place)

Escalated by check 1: rul-r74hshif and rul-p2zl2kgl (Boundary widening — escalate, the founder's call). The founder has
NOT answered F1 (`inputs#I6` item 2); the overseer answers as the delegate, PROVISIONAL, on this one card. The wrap
ratifies nothing as his. Every row below lands PROVISIONAL, named with F1 in its sidecar entry, the founder's answer owed
(`founder-forks.md` of the phase run 2026-11-24T12-23-47Z-phase, Block 1).

## The operations (7 lines, each with its fit verdict)
- ops-architecture.md · contract/channel-methods/switch · standard-contracts · fit: fits type 3
- ops-architecture.md · decision/switch-in-place · established-decisions · fit: fits type 1
- ops-architecture.md · contract/child-argv/resume · standard-contracts · fit: fits type 3
- ops-architecture.md · contract/claude-child-argv/resume · standard-contracts · fit: fits type 3
- ops-security-plan.md · decision/switch-ends-only-its-own-child · authentication-authorization · fit: fits type 1
- ops-security-plan.md · contract/wrapper-channel-methods/switch · threat-model-summary · fit: fits type 3
- ops-security-plan.md · decision/switch-method-behind-ipc-controls · threat-model-summary · fit: fits type 1

None read `not_master`; none was rejected before the question.

## What a reversal would take out, if the founder's word is another arm
All of it is in ONE commit, `6281e1a5` (the operator pre-CI commit; base `652d5047`).
- **The verb layer — goes whole:** the channel method (`switch` in `crates/viola-channel/src/server.rs` `METHODS`,
  11 → 10; its routing in `src/cmd/run.rs` `dispatch_call` and the switch slot built at start); `src/run/switch.rs`
  (new, 631 lines); `src/cmd/switch.rs` (new, 125 lines) and its dispatch in `src/cmd/mod.rs` (the `HELP` `setup:`
  row); `src/run/mod.rs` (the module line); the switch mark in `src/run/send/slot.rs` and its read in `src/run/send.rs`;
  `human::write_switched` (`src/human/verbs.rs`, `src/human.rs`); the four hint causes in `src/human/hints.rs`; the
  tables — `tests/cross_exit_causes.rs` `REFUSALS` (5 → 1), `tests/contract_error_bodies.rs` `CHANNEL_METHODS` and
  the three `CLI_DOCUMENTS` rows; `tests/cli_switch.rs` (new, 342 lines); the `switch` entries of
  `tests/cli_output_plain.rs`. `tests/support/home.rs` `Wrapper::adopt_as_user` may stay (`adopt` calls it).
- **The PTY core layer — may stay as dead-but-tested code:** `crates/viola-pty/src/pump.rs` (`RespawnHandle`,
  `Respawn`, `RESPAWN_REAP_BOUND`, the swap in `Pumping::respawn`), `crates/viola-pty/src/lib.rs` (the exports), and the
  handle threaded through `src/cmd/run.rs` `pump_child` and `src/cmd/verify/typed/run.rs` `Run::spawn`. With the verb
  layer gone no one requests a respawn: the three product callers already pass a handle no one requests on but
  `run`'s, and the four seam cases (`respawn_swaps_the_child_and_the_pump_continues`,
  `respawn_of_a_child_ignoring_the_signal_reads_not_ended`, `respawn_whose_spawn_fails_ends_the_pump_with_the_old_exit`,
  `respawn_without_a_running_pump_is_no_input`) keep it tested. It MUST go only if his arm forbids a wrapper ending its
  own child in any form, since the code would then hold that power unused: then `pump_with_paste` loses its
  `&RespawnHandle` argument at its three callers and the 10 s bound leaves with it. The panic arm's «kills the current
  child» reads as before either way (with no respawn the current child is the one child).
- **The rows:** the 7 above, and every PROVISIONAL row resting on F1 this wrap lands (the launch line per switch, the
  snapshot's rewritten `launch` / `child_pid`, the one-shot late session-end screen, the switch's process-log lines) —
  each named with F1 in its sidecar entry, so a reversal finds them by that name.

## Recommendation
Confirm the 7 as the delegate's PROVISIONAL apply (Recommended): each states what was built and proven over the fake
agent; each lands marked PROVISIONAL on F1 with the founder's answer owed, and a reversal is the verb layer above.

## Answer (delegate overseer, 2026-11-24T14:32:31Z — inputs#I7)
Confirm 7, PROVISIONAL (Recommended) — all seven land with state provisional on fork F1 arm 1; the operator has NOT answered F1; his own word ratifies or reverses them; nothing here is recorded as his. The reversal text of the card stands as written.
