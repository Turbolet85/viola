# Session Handoff

**Last Updated:** 2026-10-04T08:59Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the operator pass pushed `306c4ae`; this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-04-confirmed-send-with-cl-1-records — the wrap commit of Confirmed send with CL-1 records

## Position
- Done: **2026-10-04-confirmed-send-with-cl-1-records**.
  - `viola send` types the text as one bracketed paste behind the readiness gate.
  - It is confirmed by the relabelled `prompt-submitted{origin:"driver"}` within 10 s, else `not-delivered`.
  - It writes CL-1 records, the `[RB]` mirror and typed exits, and allows one send in flight.
  - The paste validation runs on both sides; the feed queue is bounded at 256.
  - The run-level feed-panic chaos test passes. v1-12 is verified.
- Next: **wait and last** (`working-route.md:76`) → `/andromeda-phase`. It now carries the `[inferred]` `src/main.rs`
  panic-line gap (a panic in `viola send` prints no `error: internal error`; `wait` / `last` are `cli` verbs too).

## Work done
- Code: `src/cmd/send.rs`, `src/run/send.rs`, `src/run/gate.rs` (bounded feed + `Gate`), viola-pty `PasteHandle`,
  viola-core `RefusalReason` / `NotDelivered` / `validate_paste_text`, viola-state `append_event_at`, viola-channel
  `Call` / `dispatch_call` / `InvalidParams`, the `paste_text` fuzz target, and 3 new root test files.
- CI: ci#37183365088 on `306c4ae` green 15/15, with the send tests passing on all three OSes. The wrap's E3 fold
  (the `--json` wrapper-fault `detail` object, `src/cmd/send.rs`) postdates that run: the wrap's light gate covers it
  locally, and CI on this wrap commit is its three-OS witness.

## Drift resolved
- **48 amendments, 3 escalations resolved:**
  - E1 **F3**: the fourth dated gap, `viola send` before server verification, ratified by the founder live until
    Epoch 6 (`:109` / `:111`).
  - E2 **F4**: the chaos home outside G2, G4 and the secret scan, accepted by the founder as the second test-data
    carve-out.
  - E3: the wrapper-fault `--json` shape, the CLI fixed to the arch contract (overseer, founder-delegated).
- By master: architecture 19 (+2 key files), security-plan 10, obs-plan 8, test-plan 10 (+2 key files).
  3 proposals were rejected: A9 / A10 (the code fix) and O3 (the wheel field, owned by `:80`).
- Route pins: `:76` main.rs gap · `:80` wheel field + `human-typing` · `:82` F2 local-command rows · `:109` / `:111`
  the F3 gap's owners · `:115` the `-32602` cause split.

## Notes
- **Held widening (founder morning, 2026-10-05), still HELD:** the PTY typed-input `viola verify` probe, the live
  2.1.287 recording, and the signature / quiet-period / max-wait ledger rows. F2's local-command rows now ride it.
  Owed to `:82` or the founder's ruling.
- **Epoch 3 holds 9 entries,** near the ~10 growth valve. A split is the operator's call.
- **`host-win32.md`** still describes the retired Windows host. Its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing.
- **Installed `claude` is 2.1.287.** Fixtures exist only for 2.1.283, so it stays unverified until `viola verify` runs.
- **Deferred learnings:**
  - `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a heredoc to a file; hit three times
    this session) — this one is new;
  - carried from earlier: the "not measured here" vocabulary; PID 1 as the cleanup-deadline target; the PTY master
    close needing no held clone; let a red CI run finish before folding its fix; the doubled-backslash guard
    recurrence.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-04 11:55:04
