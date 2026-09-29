# Session Handoff

**Last Updated:** 2026-09-29T08:11Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit is pushed at P7)
**Status:** clean
**Last Commit:** 2026-09-29-verify-stamped-test-homes-and-harness — the chunk's wrap commit (P7)

## Position
- Done: 2026-09-29-verify-stamped-test-homes-and-harness.
  - Test homes and harness `boot` are stamped only through `viola verify` against the fake agent at 2.1.283, with
    `boot --unstamped`, readiness `events` lines 1-3, and `run --local-live` refused under `CI` (v1-14 verified).
  - `StampError` is folded into `AgentError`, and verify logs its two spawn pairs (`version-probe`, `verify-probe`).
  - The case_08 red is fixed test-side: stop waits for the endpoint to be gone. Why the pipe outlives the exit is
    not established; the collision hypothesis is falsified.
  - CI ci#36538832471 on `702a3b4`: green 15/15.
- Next: **Sideloaded ConPTY** (working-route :61, founder ruling 2026-09-29, inserted at this wrap) → `/andromeda-phase`.
  It carries the WSL `--install-deps` hardening CARRY, moved on unchanged.

## Work done
- Implement, the operator pass (entries 21-23: hygiene on the operator's word, one push, the `ci.py` read) and this
  wrap in one session. Gate 6's freshness check was retargeted from the artifacts directory to
  `junit-nextest-integration.xml` on the operator's word.

## Drift resolved
- 27 detector proposals (arch 9 · obs 8 · test-plan 10; the other four 0): 25 applied, 2 rejected (obs §1, the
  verbatim scope copy).
- Plus 2 orchestrator fixes in test-plan (the §5 H2 owner, the `gate --require` count). 0 escalations.
- 8 sidecar entries; 9 leaf sites re-derived; obs Decisions Log D-35; a test Decisions Log entry for 2026-09-29.

## Notes
- **Route:**
  - "Sideloaded ConPTY" inserted ahead of "Fake-agent drift contract" on the founder's ruling. Acceptance: the H2
    200-loop on windows-2025 with and without it.
  - CARRY on "Sanitised error surfaces": the `Refusal` enum, the one open one-enum divergence (the operator's
    "carry the Refusal-enum line").
  - CARRY on "Server verification before any frame": the dying-pipe window skips `SessionEnd`'s direct append.
- **Epoch 2b has grown to 11 entries (9 promoted):** a boundary here would restore the diagnose/audit cadence. The
  split is yours to name at a wrap.
- **The founder's product question on H2 stays open:** "First live test and self-drive" owns the real-`claude`
  measurement; `--local-live` does not claim it.
- **Curation:**
  - T3 2: an `artifact` key names a file, not a directory; hygiene reads `/home/<x>/` in prose as a user home.
  - T2 extension 1: host-win32.md, the Bash guard refuses any command with a doubled backslash.
- **Deferred learnings:**
  - `recurrence-despite-learning: host-win32.md Session Additions 2026-09-28 — "The Bash guard refuses a heredoc whose payload carries a doubled backslash"`
    (an evolve-record heredoc was refused again).
  - The operator's hygiene-disposition convention for phase-run CI copies (delete the uncited, rewrite path roots
    in the cited with line counts kept, rename a plane-source control `.txt`), 0.7, held by the max-3 cap.
- **For the operator:** 6 `viola.exe` of `additional/viola-lab/prototype` are running (other sessions'; left as
  directed).
- **Last failed command:** none.
