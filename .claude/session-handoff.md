# Session Handoff

**Last Updated:** 2026-10-04T17:20Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap (the operator pass pushed `ca69e84`; this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-04-dialog-answers-by-dialog-id — the wrap commit of dialog answers by dialog_id

## Position
- Done: **2026-10-04-dialog-answers-by-dialog-id**.
  - Question, permission and plan dialogs reach the wrapper through PreToolUse / PermissionRequest (`hook.dialog`),
    each with a wrapper-assigned `dialog_id`, one pending at a time, and a 60 s PROVISIONAL deadline.
  - `viola answer <name> <dialog_id>` answers a dialog. An unknown id exits 13; an unverified CLI exits 12. A plan
    revise flows through the PermissionRequest repeat.
  - `run`'s stamps read is strict now. A home created outside `%USERPROFILE%` gets the protected user + SYSTEM DACL.
  - The dialog fixtures are relayed prototype captures (R1). Decisions flow on the six-row stamp until `:82` (R2).
- Next: **The wheel** (`working-route.md:80`) → `/andromeda-phase`. It now also carries `answer`'s `human-typing`
  slot and the `null` answer on a wheel move.

## Work done
- Code: `src/run/dialog.rs` (DialogSlot), `src/cmd/answer.rs`, `viola_agent_claude::dialog`, `viola_state::strict`,
  `fs::create_private_dir`'s protected DACL, `hooks.json` 7→9, `DIALOG_DEADLINE`, a fifth perf row; tests `cli_answer`,
  insta bodies, the relayed 2.1.287 fixtures.
- CI: three Windows reds folded by the operator pass (`2080e3f`, `2484b77`, `ca69e84`) → ci#37218087331 green 15/15.

## Drift resolved
- **70 amendments, 2 escalation groups resolved** (architecture 29 · security-plan 13 · test-plan 21 · obs-plan 7 ·
  layout-templates 1 · a11y-plan 1; 1 detector proposal rejected).
  - E1, the sixth dated gap (`answer` / `hook.dialog` frames after the liveness-only / shape check), and E2, decisions
    on the six-row stamp until `:82`: both recorded as the founder's live rulings (F1, R2), relayed by the overseer,
    and ratified by the operator at this wrap.
- Route pins: `:80` the `human-typing` slot · `:82` the S3/S7/S8/concurrency rows + re-probe + effect half (closing
  R2), `v1-15`, the `permission` e2e + its `v1-30` wake, the question-by-PermissionRequest body · `:109` / `:111` the
  sixth gap's owners · `:111` lost the creation half and `run`'s stamps read (premise-corrected).

## Notes
- **Held widening (founder morning, 2026-10-05), still HELD:** the PTY typed-input `viola verify` probe, the live
  recording, and the signature / quiet-period / max-wait ledger rows (`:82`).
- **Epoch 3** stays unsplit (founder ruling 2026-09-29).
- **`host-win32.md`** still describes the retired Windows host; its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing.
- **`claude` on the dev host:** mise installed 2.1.288; running sessions are on 2.1.287 (stamped, fixtures 2.1.287).
  Stamping 2.1.288 is the operator's.
- **Operator desk:** the stray recording home `~/.viola-record-20261004T142325Z` (founder desk queue);
  `~/.viola-record-20261004T142437Z` (run 2's home) is left as it is.
- **Code-graph:** `rust ok 3556/17037`, `ts ok 7/1`.
- **Deferred learnings** (carried): `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a
  heredoc to a file — recurred again at this wrap); the "not measured here" vocabulary; PID 1 as the cleanup-deadline
  target; the PTY master close needing no held clone; let a red CI run finish before folding its fix; the
  doubled-backslash guard recurrence.
- **Last failed command:** none.
