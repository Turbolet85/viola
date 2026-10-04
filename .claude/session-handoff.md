# Session Handoff

**Last Updated:** 2026-10-04T10:43Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the operator pass pushed `3de125f`; this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-04-wait-and-last — the wrap commit of wait and last

## Position
- Done: **2026-10-04-wait-and-last**.
  - `viola wait` parks on the wrapper until the next turn end, dialog or session end at or after `--after`, at once
    when it is already logged, with a typed timeout and exit 21 when the wrapper vanishes.
  - `viola last` returns the newest turn's message and `ts`, rebuilt across a restart; human mode escapes
    control characters as `\xHH`.
  - Nothing polls: each hook line's append runs inside the wait feed's lock and wakes the parked calls.
  - `send` / `wait` / `last` print `error: internal error` once, from the catch site (CARRY 2). v1-30 is verified.
- Next: **Dialog answers by dialog_id** (`working-route.md:78`) → `/andromeda-phase`. It now carries the v1-30
  dialog-kind wait witness (the claim's condition).

## Work done
- Code: `src/run/wait.rs` (WaitFeed), `src/cmd/{wait,last,client}.rs`, `src/human.rs` escaper + writers, `src/main.rs`
  `role_of`, `viola-state::events::read_from`, `EventKind` dialog kinds + `WAIT_WAKE`, the channel's `wait` log
  fields, `frame::Params` without `from`; tests `cli_wait_last` (8), `chaos_wait_vanish`, 40 `run::wait` units.
- CI: ci#37194791251 on `db15b2b` red on macOS (`last` read the turn before one on disk) → folded as `3de125f` →
  ci#37195156240 green 15/15.

## Drift resolved
- **25 amendments, 1 escalation group resolved** (architecture 10 · security-plan 7 · layout-templates 4 · obs-plan 2
  · test-plan 1 · design-system 1, raised by the wrap).
  - The fifth dated gap (CLI `wait` / `last` after the liveness-only pre-check) is recorded as the founder's live
    ruling of 2026-10-04, relayed by the overseer (F1 at P4).
  - A non-string `from` is now the method's `-32602` (the envelope typed it as `-32600`).
- Route pins: `:78` the dialog wait witness · `:85` the landed reader · `:102` / `:129` `role_of` must name
  `mcp` / `ui` · `:109` / `:111` the fifth gap's owners.

## Notes
- **Held widening (founder morning, 2026-10-05), still HELD:** the PTY typed-input `viola verify` probe, the live
  2.1.287 recording, and the signature / quiet-period / max-wait ledger rows (`:82`).
- **Epoch 3** stays unsplit (founder ruling 2026-09-29).
- **`host-win32.md`** still describes the retired Windows host; its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing.
- **Installed `claude` is 2.1.287**; fixtures exist only for 2.1.283, so it stays unverified until `viola verify` runs.
- **Code-graph:** both planes build now (`rust ok 3228/15021`, `ts ok 7/1`; scip-typescript 0.4.0 on PATH).
- **Deferred learnings** (carried): `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a
  heredoc to a file); the "not measured here" vocabulary; PID 1 as the cleanup-deadline target; the PTY master close
  needing no held clone; let a red CI run finish before folding its fix; the doubled-backslash guard recurrence.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-04 13:48:22
