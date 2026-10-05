# Session Handoff

**Last Updated:** 2026-10-05T00:14Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the adaptation commit pushes after this file)
**Status:** clean
**Last Commit:** no chunk wrapped — `chore(route): operator-requested adaptation — 0-pending wrap` (split `:84`, R-S2)

## Position
- Done: **2026-10-04-running-turn-refusal** (last complete). This session ran a 0-pending route adaptation only.
- Next: **Real-CLI verify probes** (`working-route.md:84`, minted this wrap) → `/andromeda-phase`.
  - After it comes **First live test and self-drive** (`:86`), the last entry of Epoch 3. Epoch 3 now has 11 entries
    and stays unsplit.

## Work done
- Relay `split84-route-adaptation.md`, founder rulings R-S1 / R-S2 / R-S3, applied through a four-item dialogue. The
  overseer gave every answer, each the recommended option.
- The old `:84` (11 CARRYs) is now (A) `:84` Real-CLI verify probes (9 CARRYs, incl. a stamp-2.1.288 CARRY) and (B)
  `:86` First live test and self-drive (6 CARRYs).
- Route citations were renumbered by manifest: 46 rewritten, 28 kept at `:84`, 3 excluded; sidecars, archive and
  source files kept their bytes. Record: `.andromeda/runs/2026-10-05T00-09-13-wrap/`.

## Drift resolved
- 1 amendment, 0 escalations: architecture `:48` / `:49` / `:80` / `:91`.
  - HELD → ratified (R-S2). The owner of the signature/timing, local-command, prefix and R8 rows is now
    "Real-CLI verify probes".
  - Leaf re-derived: `security-summary.md:70`.
- The decision-effect half is split. (A) gets the decision taking effect, through the re-probe. (B) gets "the dialog
  never renders", which is v1-31, measured live.

## Notes
- **The held widening is RATIFIED** (founder, live, 2026-10-05 ~00:00Z, R-S2, relayed by the overseer):
  - What: a PTY-driven typed-input `viola verify` probe against the live installed `claude`.
  - Limits: local on the dev host only, with no Claude credential on a CI runner. It stamps the version it runs.
  - It is owned by `:84`.
- **The live proof's host** (B, `:86`) stays the founder's call at its phase. The relay's "this Linux host now" gloss
  overreached, per the overseer's correction at this wrap.
- **R-S3:** Upgrade U02 (`.claude/settings.json` hooks · bash pre-cd) and the `host-win32.md` regenerate run at the
  Epoch 3 boundary, with the boundary ritual (`/andromeda-setup-project`).
- **`claude` on the dev host:** mise installed 2.1.288 (2026-10-04 14:19Z), and running sessions are on 2.1.287.
  Stamping 2.1.288 is now a CARRY on `:84`.
- **Operator desk:** the stray recording home `~/.viola-record-20261004T142325Z` (founder desk queue).
- **Deferred learnings** (carried):
  - `recurrence-despite-learning: host-win32.md` — the zero-is-healthy count probe under pipefail.
  - `recurrence-despite-learning: testing.md` — bounded mutant-reachable waits.
  - Also carried: `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a heredoc to a file);
    the "not measured here" vocabulary; PID 1 as the cleanup-deadline target; the PTY master close needing no held
    clone; let a red CI run finish before folding its fix; the doubled-backslash guard recurrence.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-05 11:17:00
