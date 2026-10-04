# Session Handoff

**Last Updated:** 2026-10-04T22:38Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read after the operator fix `334ee7f` was pushed (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-04-running-turn-refusal — the running-turn refusal (the wrap commit, after the operator fix `334ee7f`)

## Position
- Done: **2026-10-04-running-turn-refusal**.
  - A `prompt-submitted` of any origin marks a turn running. `turn-ended` / `session-start` / `session-end` end it.
  - Meanwhile `send` is refused `turn-running`, with nothing typed.
  - Only a wheel-returning `release` clears it. Recovery under a driver-held wheel is `pause` then `release`.
- Next: **First live test and self-drive** (`working-route.md:84`) → `/andromeda-phase`.

## Work done
- Code: the running-turn state in `WheelSlot` and the rung in `send` (`src/run/{wheel,send}.rs`), plus witnesses in
  `tui_wheel`, `cli_wheel`, `cli_send` and `cli_controls_not_disableable`.
- The fake agent now quiesces its hooks before exiting (`src/bin/viola-fake-agent.rs`). This was a widening on the
  overseer's founder-delegated word.
- CI green 15/15 at `93a5cbf` (ci#37239689446) and at the final code `334ee7f` (ci#37241137053).

## Drift resolved
- 10 amendments, 0 escalations: architecture 2 · test-plan 7 · obs-plan 1.
  - Architecture: the [Human Takeover / Wheel] "as built, no running-turn state" clause is retired.
  - test-plan: Path 5 / Path 2 / §5 controls / §7 fake-agent exit.
  - obs-plan: §4 `turn-running`'s two causes.
- The plan's "turn's life" acceptance line is amended as measured (operator directive 2): a human-origin turn reads
  `human-typing` first.
- `.profraw` WATCH CLOSED (directive 3).
  - Cause: the fake agent's PTY-session-leader exit hung up an in-flight `viola hook`.
  - Fixed; 4 green pre-push runs in a row after the fix.
- Residual recorded (directive 4, `.andromeda/residuals.md`): a turn starting during the readiness gate's wait
  (≤ 5 s) is not refused.

## Notes
- **Held widening (founder morning, 2026-10-05), still HELD:** the PTY typed-input `viola verify` probe, the live
  recording, and the signature / quiet-period / max-wait ledger rows (`:84`).
- **Epoch 3** has 10 entries and stays unsplit (founder ruling 2026-09-29, re-affirmed 2026-10-04).
- **Upgrade U02** (`.claude/settings.json` hooks · bash pre-cd) and the `host-win32.md` regenerate are founder-timed:
  `/andromeda-setup-project`.
- **`claude` on the dev host:** mise installed 2.1.288; running sessions are on 2.1.287. Stamping 2.1.288 is the operator's.
- **Operator desk:** the stray recording home `~/.viola-record-20261004T142325Z` (founder desk queue).
- **Deferred learnings** (carried, plus two new recurrence-despite-learning entries):
  - `recurrence-despite-learning: host-win32.md` — the zero-is-healthy count probe. The plan's `git grep … | wc -l`
    gate with `exit 0` is red on its satisfied subject under the gate shell's pipefail.
  - `recurrence-despite-learning: testing.md` — bounded mutant-reachable waits. A new unit test's remove-the-guard
    control parked on a fixed-clock confirm window (120 s TIMEOUT) before the rework.
  - Carried: `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a heredoc to a file); the
    "not measured here" vocabulary; PID 1 as the cleanup-deadline target; the PTY master close needing no held clone;
    let a red CI run finish before folding its fix; the doubled-backslash guard recurrence.
- **Last failed command:** none. The wrap's first light gate read two reds; both were folded on the overseer's
  founder-delegated word, and the re-run read 17/17 (`evidence/light-gate-fix.md`).
  - A receipt-count race in the two new turn witnesses → operator fix `334ee7f`, red-before-green on a forced hold.
  - The `:82` probe's pipefail defect → a dated plan correction, with discrimination shown.
