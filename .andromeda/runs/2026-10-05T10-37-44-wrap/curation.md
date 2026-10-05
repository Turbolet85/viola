# Curation — wrap of 2026-10-05-real-cli-verify-probes

Source: the report's *Decisions & corrections* and the resume point's carried corrections. This is a resumed wrap, so the
implementing session's conversation is gone. A correction that only that window held, and that neither the report nor
`resume-point.md` names, is not curated here.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Never give `viola verify --record` a viola home with a path component named `home` …"
  Proof: two live record rounds read red (`evidence/round-094211Z.txt`, `evidence/round-100046Z.txt`, the second naming
  `SessionStart.default.json absolute-path`). The same entries were green by hand with `--home "$h/vhome"`
  (`evidence/hand-entries-7-8.md`), a plan correction by the overseer (founder-delegated). Score: correction +0.4,
  measurement +0.4, detail +0.2 = 1.0.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters:
  - 1 dup: "describe, never spell, a `…/home/…` path in committed evidence" matches the defect record
    session-learnings 2026-09-29 "gate.py hygiene reads `/home/<x>/` in prose", so it is a recurrence
    (→ handoff `recurrence-despite-learning`);
  - 2 below threshold:
    - the `send_window_` / `verify_window_` no-test-deadline precedent scores 0.6. Its fact was amended into the
      test-plan nextest contract this wrap, so neither conditional signal applies;
    - "`has_absolute_path` reads a drive path only at a text's start" is a one-off mention and scores 0.4;
  - 0 conflict · 0 deferred.
