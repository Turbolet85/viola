# Session Handoff

**Last Updated:** 2026-10-04T21:31Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 (the wrap commit `eb37914` pushed)
**Status:** clean
**Last Commit:** 2026-10-04-the-wheel — the wrap commit of the wheel (`eb37914`; the committed copy of this file still reads the P7.1 halt's tests-failing — this rewrite is the current state)

## Position
- Done: **2026-10-04-the-wheel**.
  - A human editing key takes the wheel. Never: focus / mouse reports, terminal replies (F-W2) or a resize.
  - Windows: an injected mouse report takes the wheel (F-W3). `viola pause` / `viola release` work.
  - Refusals: `human-typing` / `manual-pause` ahead of `turn-running`. `null` to a pending dialog on a wheel move.
  - `release-from-driver` exits 20. viola's own Windows console reader keeps `^Z`.
- Next: **Running-turn refusal** (`working-route.md:82`, minted on the founder's live ruling, ahead of First live test
  `:84`) → `/andromeda-phase`.
  - It carries the WATCH on the unproven `.profraw` red. One green pre-push of the subject so far, at the operator
    pass (not yet tallied).
  - It also carries the CARRY to correct `tests/tui_wheel.rs:267` / `tests/cli_answer.rs:9` from `:82` to `:84`.

## Work done
- Code: the wheel (`src/run/wheel.rs`, `src/run/snapshot.rs`), `viola pause` / `viola release`, `host_stdin()`,
  `ProtocolError::ReleaseFromDriver`; tests `tui_wheel`, `cli_wheel`.
- The wrap's light gate first read entry 16 (the no-new-env-read guard) red. It halted on the overseer's direction.
- The operator pass's `ba36659` moved viola-pty's test-child read modes to argv. ci#37235841342 is green 15/15, and
  the re-run light gate is green 17/17 (3 operator legs re-read from evidence).

## Drift resolved
- **37 amendments, 1 escalation resolved**: architecture 10 · security-plan 8 · obs-plan 9 · a11y-plan 5 ·
  test-plan 4 · layout-templates 1.
  - E1: the seventh dated gap (F-W1, CLI `pause` / `release` after the liveness-only pre-check). It is the founder's
    live ruling, relayed by the overseer, recorded on the operator's word.
- Matrix: v1-32 is verified, its acceptance refined with F-W3's Windows clause. v1-31 / v1-40 have ledger notes.
  Version: verified 12/53.
- Route: 71 route citations renumbered +2 after the insert (`renumber-manifest.md` in the wrap run dir).

## Notes
- **Held widening (founder morning, 2026-10-05), still HELD:** the PTY typed-input `viola verify` probe, the live
  recording, and the signature / quiet-period / max-wait ledger rows (`:84`).
- **Epoch 3** has 10 entries and stays unsplit (founder ruling 2026-09-29, re-affirmed 2026-10-04).
- **`host-win32.md`** still describes the retired Windows host; its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing.
- **`claude` on the dev host:** mise installed 2.1.288; running sessions are on 2.1.287. Stamping 2.1.288 is the operator's.
- **Operator desk:** the stray recording home `~/.viola-record-20261004T142325Z` (founder desk queue).
- **Master-route** records written before this wrap cite the pre-insert numbering (immutable; e.g. dialog answers' "owed :82" is now `:84`).
- **Deferred learnings** (carried): `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a
  heredoc to a file — recurred twice more in this chunk's two windows); the "not measured here" vocabulary; PID 1 as
  the cleanup-deadline target; the PTY master close needing no held clone; let a red CI run finish before folding its
  fix; the doubled-backslash guard recurrence.
- **Last failed command:** none (the P7.1 entry-16 red was folded by `ba36659`).

## Session End Status
Completed normally at 2026-10-04 23:59:22
