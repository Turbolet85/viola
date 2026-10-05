# Session Handoff

**Last Updated:** 2026-10-05T11:06Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-05-real-cli-verify-probes — the wrap commit (typed-input PTY verify probe, ten ledger rows, 2.1.288 stamped)

## Position
- Done: **2026-10-05-real-cli-verify-probes**. Built W1 + W5 of the founder's three-way split:
  - the typed-input PTY `viola verify` probe: Run A untrusted, Run B trusted, never a key into a CLI dialog;
  - compiled screen signatures and timing rows, so the ledger has ten rows and the full gate runs on a verified CLI;
  - the installed `claude` 2.1.288 stamped `10 pass  0 fail`.
- Next: **Dialog rows and re-probe** (`working-route.md:86`, minted this wrap) → `/andromeda-phase`.
  - Then **Local-command and paste-framing rows** (`:88`), then **First live test and self-drive** (`:90`). Epoch 3
    now has 13 entries and stays one epoch (the founder's split ruling).
  - `:86` carries a founder crossing (research M7). The capture arm answers nothing, so "the decision takes effect"
    needs an answering probe hook or the probe keying the rendered dialog. Both are beyond R-S2's words and against
    "never type into a CLI dialog" as worded, so they are a widening for the founder to rule at that phase.

## Work done
- The wrap resumed after P1, which ran in the prior window. It then ran:
  - P2: 7 detectors, 44 proposals plus 7 raised and 3 sweep folds, 3 escalations resolved;
  - P3 through P7: curation, the code-graph read, route-resolve, state and handoff, then the gates.
- Record: `.andromeda/runs/2026-10-05T10-37-44-wrap/`.

## Drift resolved
- The masters were amended across architecture, security-plan, test-plan, obs-plan, layout-templates and
  design-system (a11y-plan was unchanged: §4 P6 is generic). The facts carried:
  - screen signatures compiled and validated per version;
  - ten ledger rows (`/10`);
  - verify's two interactive runs and their four spawn pairs (`verify-pty-probe`);
  - the screen fixture class and `claude-screen.v1.json`;
  - the named `--record` refusal;
  - the owners re-pointed from `:84` to the two new entries.
- Three escalations were resolved at P2:
  - E1, the PTY probe crossing and its two dirs: ratified as the founder's live rulings (R-S2, "two runs, never
    accept", the two dirs), relayed by the overseer;
  - E2, the Run B transcript residual and the dev-host external-imports answer: accepted and recorded on the
    overseer's word;
  - E3, the named refusal: the founder ruled live at this wrap.
- Route citations renumbered by manifest: old ≥ `:86` → +4, 51 occurrences in 12 files.

## Notes
- **R-S3:** Upgrade U02 (`.claude/settings.json` hooks · bash pre-cd) and the `host-win32.md` regenerate run at the
  Epoch 3 boundary, with the boundary ritual (`/andromeda-setup-project`).
- **`claude` on the dev host:** 2.1.288 is stamped (W5). Running sessions are still on 2.1.287.
  - The repo-root external-imports flags read "No" (`~/.claude.json`, set by the overseer on the founder's ruling,
    with a backup).
  - Five Run B transcripts sit under `~/.claude/projects/` (the accepted residual).
- **`local-live`'s live firing at ten rows** is a CARRY on `:90`.
- **Operator desk:** the stray recording home `~/.viola-record-20261004T142325Z` (founder desk queue).
- **Deferred learnings** (carried):
  - `recurrence-despite-learning: session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose). It
    recurred in this chunk's operator-pass evidence: describe such a path, never spell it.
  - `recurrence-despite-learning: host-win32.md` — the zero-is-healthy count probe under pipefail.
  - `recurrence-despite-learning: testing.md` — bounded mutant-reachable waits.
  - Also carried:
    - `recurrence-despite-learning: host-win32.md 2026-09-28` (the Bash guard and a heredoc to a file);
    - the "not measured here" vocabulary;
    - PID 1 as the cleanup-deadline target;
    - the PTY master close needing no held clone;
    - let a red CI run finish before folding its fix;
    - the doubled-backslash guard recurrence.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-05 13:47:31
