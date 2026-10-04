# Session Handoff

**Last Updated:** 2026-10-04T21:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD `79ec57c`, ci#37232840791 green 15/15); this wrap made no commit
**Status:** tests-failing
**Last Commit:** `79ec57c` — operator fix after CI run 37227518624 (F-W3 pinned); the wrap of 2026-10-04-the-wheel is HALTED at P7.1

## Position
- In flight: **2026-10-04-the-wheel** — master record still `pending`. The wrap
  (`.andromeda/runs/2026-10-04T20-44-01-wrap/`) ran P1–P6 and halted at P7.1, the light gate.
- **Light gate:** 16/20 green; 3 operator legs re-read from `evidence/operator-pass.md`. Entry 16 is **RED**.
  - The guard is `! (git diff eb53a582c8dc -- '*.rs' … | grep -E '^\+.*(#\[ignore|retries *=|test\.skip|std::env::var)' | grep -v VIOLA_NAME)`.
  - It hit `+ if let Ok(mode @ ("reads" | "reads-win32")) = std::env::var(CHILD_MODE)` in `crates/viola-pty/src/lib.rs`.
  - That line came from the operator pass's `849588b`. `CHILD_MODE` is `PTY_SEAM_TEST_MODE`, the cfg(test)
    self-exec child mode.
- **Remedy (overseer, founder-delegated):** an operator pass moves the new test-child mode off the env read into
  argv. The guard stays as written and is not corrected. Commit it as a fix, then run CI on all three OSes.
- **Next:** resume this wrap at **P7.1** in run dir `2026-10-04T20-44-01-wrap` (its `resume.md`). Then the drift
  gate, P7.3, the flip, the commit and the push.
  - P7.3 is the `matrix.py refine` of v1-32 plus notes on v1-31 / v1-40; their payloads are staged in the run dir.

## Work done (uncommitted, in the tree)
- P2: **37 amendments, 1 escalation resolved** — architecture 10 · security-plan 8 · obs-plan 9 · a11y-plan 5 ·
  test-plan 4 · layout-templates 1. Sidecars are appended and the cascade swept.
  - E1, the seventh dated gap F-W1, is recorded as the founder's live ruling on the operator's word.
- P3: T2 1 (testing.md) · T3 1 · the `--e2e` one-test example corrected at its source (test-plan §3).
- P5: **Running-turn refusal** minted at `working-route.md:82` (the founder's live ruling), ahead of First live test
  (now `:84`).
  - CARRY pins: `:84` (F-W3's real-terminal mouse report) and `:111` / `:113` (the seventh gap's owners).
  - `:82` also carries a WATCH on the `.profraw` red and a CARRY for two test comments (`:82` → `:84`).
  - Route citations ≥ `:82` were renumbered +2 across masters, leaves and the matrix: 71 sites
    (`renumber-manifest.md`). Sidecars, archives and source were untouched.
- P6: `state.yaml` session 40; this handoff.

## Notes
- **Held widening (founder morning, 2026-10-05), still HELD:** the PTY typed-input `viola verify` probe, the live
  recording, and the signature / quiet-period / max-wait ledger rows (`:84`).
- **Epoch 3** has 10 entries now and stays unsplit (founder ruling 2026-09-29, re-affirmed 2026-10-04).
- **`host-win32.md`** still describes the retired Windows host; its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing.
- **`claude` on the dev host:** mise installed 2.1.288; running sessions are on 2.1.287. Stamping 2.1.288 is the operator's.
- **Operator desk:** the stray recording home `~/.viola-record-20261004T142325Z` (founder desk queue).
- **Deferred learnings** (carried): `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a
  heredoc to a file — recurred twice more in this chunk's two windows); the "not measured here" vocabulary; PID 1 as
  the cleanup-deadline target; the PTY master close needing no held clone; let a red CI run finish before folding its
  fix; the doubled-backslash guard recurrence.
- **Last failed command:** light gate entry 16 (`gate.py run`, log `/tmp/andromeda-gate/2026-10-04-the-wheel/wrap-2026-10-04T20-44-01/16.log`) — a red guard, not a command to retry; the remedy is the argv move above.
