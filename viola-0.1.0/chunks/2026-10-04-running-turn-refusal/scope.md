# Scope — 2026-10-04-running-turn-refusal

**Working entry (`working-route.md:82`, Epoch 3 — Windows slice II: driving verbs and live proof):** Running-turn
refusal — prompt-submitted marks a turn running, turn-ended/session-start/session-end end it, automated send refused
turn-running meanwhile, release clears it.

Minted on the founder's live ruling (2026-10-04, relayed by the overseer through the operator at the wheel's wrap),
ahead of First live test (`:84`); Epoch 3 stays unsplit (founder ruling 2026-09-29, re-affirmed 2026-10-04 and in
this take-up's directive).

## What it builds
- **A running-turn state in the wrapper**, distinct from the in-flight `send` slot (`src/run/send.rs` `SendSlot`). As
  built, `turn-running` covers only that slot: a `send` arriving while another is in flight. Once a send is
  confirmed, its slot empties and nothing records that its turn is still running (architecture [Human Takeover /
  Wheel], amended at the wheel's wrap: "as built, no running-turn state exists beyond the in-flight `send` slot").
- **Turn start:** a `prompt-submitted` line marks a turn running, whatever its `origin`: a `driver` prompt (the
  confirmed send's own, relabelled), a `human` prompt or a `harness` prompt (`<task-notification>`,
  `<agent-message from=`, either cross-session tag).
- **Turn end:** `turn-ended`, `session-start` or `session-end` ends it.
- **The refusal:** while a turn is running, an automated `send` is refused `not-delivered` with detail
  `turn-running`. It is not queued and nothing is typed for it. Its place is the documented refusal order (architecture
  §Conventions, the `send` order): after `control-character` and `human-typing`, at the existing `turn-running` rung
  (a turn running, or another `send` in flight), before the readiness gate and confirmation. The driver calls `wait`
  first.
- **`release` clears it.** A turn can end with no `turn-ended`, because Claude Code's Stop hook does not fire on a user
  interrupt. A human interrupt is human input, so it moves the wheel to the human, and the `viola release` that returns
  the wheel is to clear the running-turn state. `release` still never empties the in-flight `send` slot, which belongs
  to a live send.
- **The hypothesis, measured:** the folded CARRY states as unmeasured that a driver `send` during a turn the driver
  did not start (a harness `<task-notification>` turn, or one left running after a wheel move) is typed into that
  turn. The chunk's witness drives that case and reads `turn-running` with nothing typed.

## Boundaries
- Gates only `send`. `answer` (a dialog held during a running turn) and `hook.dialog` are not gated by the
  running-turn state: the architecture's `answer` refusal order names no `turn-running` rung, and `answer` /
  `hook.dialog` route to `DialogSlot` (`src/cmd/run.rs:128-129`; verified at P3).
- `budget-paused` (the rung between `human-typing` and `turn-running`) is not built here; it is the Budget governor
  (`:93`).
- Which hook, if any, fires for a turn that ends on an API error is a ledger row, not measured here. If one does,
  `viola-agent-claude` registers it and maps it to `turn-ended` (architecture). This chunk adds no ledger row.
- The held widening stays held (founder morning, 2026-10-05): the PTY typed-input `viola verify` probe, the live
  recording, and the signature / quiet-period / max-wait ledger rows (`:84`).
- The U02 setup upgrade (`.claude/settings.json` hooks · `bash pre-cd`) is NOT taken here. It is founder-timed with the
  `host-win32.md` regenerate (this take-up's directive).

## Surfaces and contracts touched
- `src/run/send.rs` — the `send` method's refusal order, and `append_hook_event`, the wrapper's one entry for every
  hook line. `prompt-submitted`, `turn-ended`, `session-start` and `session-end` all reach it through `hook.event`
  (`src/cmd/run.rs:78-84`, `:111-121`; verified at P3). The one exception: a SessionEnd the channel did not take is
  appended directly by the hook (`src/cmd/hook.rs:174-175`), unseen by the wrapper. Its child is ending, so no
  driver send depends on it.
- `src/run/wheel.rs` — `release` (`WheelSlot`). At HEAD a `release` on a driver-held wheel moves nothing
  (`next()`, `src/run/wheel.rs:44-45`), matching architecture's "changes nothing and appends nothing". Whether such a
  release also clears the turn state is P4's lean (research: a stuck turn under a driver-held wheel is recoverable
  without a keystroke by `viola pause` then `viola release`).
- The running-turn state rides neither `snapshot.json` nor any event: it is not among the snapshot's closed fields or
  replay's log-derived fields, so it lives in memory only (verified at P3). No new event kind, detail or field.
- Architecture [Human Takeover / Wheel]'s "as built" clause becomes false at this chunk's wrap (a wrap amendment).
- Tests: wrapper unit cases beside the existing `turn-running` cases in `src/run/send.rs` (and `src/run/wheel.rs`
  for `release`), and root integration cases through the fake agent (`tests/cli_send.rs`, `tests/tui_wheel.rs`;
  `--e2e` selects nothing at HEAD, so they ride `run --integration`). Two existing tests change meaning (verified at
  P3). `path5_harness_turns_never_take_the_wheel` (`tests/tui_wheel.rs:240-258`) asserts a send exits 0 during an
  unended harness turn. `path2_send_confirms_with_cl1_events` (`tests/cli_send.rs:151`) sends twice with no
  `turn-ended` between. The fake agent's interactive submit fires no Stop (`src/bin/viola-fake-agent.rs:288-307`).
  Added at P4, validation-1 intent-incomplete: `tests/cli_wheel.rs` witnesses the recovery of a turn stuck under a
  driver-held wheel (a bare `release` leaves it; `pause` then `release` clears it). `tests/cli_controls_not_disableable.rs`
  gains the turn-running row, since no setting may disable a control (CLAUDE.md invariant 5).

## Folded freight (`working-route.md:82`, 3 blocks; `route.py pins`)
- **CARRY (chunk 2026-10-04-the-wheel, the founder's ruling, live, 2026-10-04, relayed by the overseer through the
  operator at its wrap; minted ahead of `:84`, Epoch 3 kept unsplit):** as built, `turn-running` covers only the
  in-flight `send` slot (`src/run/send.rs`), and `release` clears nothing (architecture [Human Takeover / Wheel],
  amended at that wrap). This is the chunk's substance, folded into §What it builds.
  - Mechanism claim, verbatim: "hypothesis, unmeasured: a driver `send` during a turn the driver did not start (a
    harness `<task-notification>` turn, one left after a wheel move) is typed into that turn". VERIFIED at P3, measured
    at HEAD: the rung's only `turn-running` condition is an occupied flight slot (`src/run/send.rs:239-243`), and
    `path5_harness_turns_never_take_the_wheel` (`tests/tui_wheel.rs:240-258`) asserts exactly that send exits 0
    (confirmed, so pasted) after a harness `prompt-submitted` with no `turn-ended`, green on three CI OSes.
- **CARRY (chunk 2026-10-04-the-wheel, the operator's word at its wrap, the overseer's guards):** this entry's insertion
  moved every later route line by +2, and that wrap renumbered the spec / leaf / matrix citations
  (`.andromeda/runs/2026-10-04T20-44-01-wrap/renumber-manifest.md`). A wrap edits no source, so two test comments still
  cite First live test as `:82`. Correct both to `:84` in this entry's diff. Coordinates re-verified at HEAD
  `eb37914`:
  - `tests/tui_wheel.rs:267`: "A mouse report from a real Windows terminal is measured live at `:82`."
  - `tests/cli_answer.rs:9`: "owed to the live test (working-route `:82`)".
  These are the only `:82` citations under `tests/` and `src/`.
- watch: the coverage `llvm-profdata merge` refusal of a truncated `.profraw` in a local pre-push (1450/1450 tests
  passed; cause not proven). Hypothesis: a harness `cleanup` Ctrl-C reached a wrapper mid profile write; the exit flush
  now precedes the raw-mode restore. (0/3; since 2026-10-04-the-wheel; retires on a recurrence, or 3 green runs.)
  - The handoff (2026-10-04T21:31Z) records "one green pre-push of the subject so far, at the operator pass (not yet
    tallied)" (verified against the handoff text at P3). The entry reads 0. The tally is the wrap's, not this chunk's.

## CI read at take-up (Setup 5a)
- `eb37914` (the last wrap's flip = HEAD): **green** · checks 15/15 · wall 341 s · ci#37236396020 push
  completed/success. No red and nothing `not green`, so nothing folds.
