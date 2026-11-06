# Curation — 2026-11-06-version-done-check (wrap 2026-11-06T20-57-05)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  corrected in place: "Prompt text reaches viola only from stdin or `--file`, never from a leading-slash argument." (the Git Bash reason struck)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A live round on this host cannot raise a permission dialog from a shell command" (confidence 0.8)
  Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred
  No-other-home: "a live round on this host cannot raise a permission dialog from a shell command"
  CLAUDE.md size: 131/200 · T1 3.4 KB, 0 over 600 B

## Applied
- T1 correction, CLAUDE.md `USER:session-learnings` (TX 521): the entry's reason «Git Bash rewrites `/skill` into a
  Windows path» is struck with `[corrected 2026-11-06: …]`; the rule stands.
  Proof: the chunk removed the Git Bash rewritten-path warning with the Windows code (report Retired; T2, the
  overseer's technical answer, founder-delegated, 2026-11-06 19:34Z, `inputs#I2`); architecture fct-onw4pywk states the
  stdin / `--file` rule alone.
- T3, `.claude/docs/session-learnings.md` (TX 522), confidence 0.8 (measured +0.4: it changed the live round's design,
  the hold driven through a question dialog; specific detail +0.2; no other home +0.2).
  Proof: implement's outcome and friction `2026-11-06T20:10:16Z-b` (`contract.premise-falsified`); the report's
  Deviations, the overseer's technical answer 20:10Z (`inputs#I5`). No master, route annotation, ledger note or hook
  carries it.

## Decisions & corrections — each entry's disposition
1. The founder's 20:53Z word (v1-27 deferred, F-06i) — not a learning: a ruling; carried by the masters' OPEN
   (architecture, owner 0.2.0's intent) and P7.3's ledger deferral.
2. The overseer's 20:10Z / 20:16Z answers — not a learning: rulings, recorded in the report and the masters.
3. The founder's 19:34Z word (Epoch 7 confirmations, deferrals, T1–T3) — not a learning: rulings, landed in the
   masters this wrap.
4. `Bash(*)` blocks a shell-raised PermissionRequest — applied, Tier 3 (above).
5. A ledger row on the account's payload reads the account's plan — rejected, Filter 4: 0.6 (measured +0.4, detail
   +0.2), no conditional signal (amended into architecture fct-qwpklckh / fct-wfge3rsz and three more masters this wrap).
6. T2's Tier-1 Git Bash entry — corrected in place (above).
7. `gate.py run --only` rejects a range with a traceback — not a learning: a pipeline tool's defect (implement's
   friction `2026-11-06T20:20:53Z-c`), the friction stream's.
8. The guard refusals — not a learning: each guard holds its own rule and enforces it.
9. The `read rule` / `\bRefused\b` sweep hazard — rejected, Filter 2: task-specific (two pattern ids of this chunk's
   sweep); its general form, a sweep needs the mark's own words, scores 0.2 (detail) with no measurement behind it.

## This window's own candidates
- A `· as:` direction carrying provenance into a body — rejected, Filter 4: 0.6 (the NO_PROVENANCE check, detail);
  no-other-home excluded, since the check refuses the very act.
- Apply writers given no key-file target; entry writers misattributing whose word ruled — not learnings: pipeline
  friction, recorded at the reconcile checkpoint.
