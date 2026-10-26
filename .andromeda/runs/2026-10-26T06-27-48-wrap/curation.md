# Curation — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A sidecar entry payload needs a leading blank line before it is appended"
    Proof: `splice.py append --file .andromeda/architecture-amendments.md --payload-file …/architecture-entries.md --lines 8 --dry-run` read `lines 1508→1516` — the entries file's own 8 lines, no separator, the new heading under the previous `**Ref:**`; the last wrap's own appends used `{doc}-entries.append.md` (its splice trail, `.andromeda/runs/2026-10-26T00-17-51-wrap/splice-2026-10-25-windows-endpoint-admission.json`, `--lines 9` / `18` / `45`); this wrap's `.append.md` copies appended `+9` / `+16` / `+25`, each file's last line the new `**Ref:**`. Confidence 0.8: verified by measurement +0.4, specific technical detail +0.2, no other durable home +0.2 (the skill letter appends "the writer's file whole"; the fact stands nowhere in the curation homes — `grep -rn -i -E 'entries\.append|leading blank|blank line before|abut'` 0 hits).
  Filters: 0 dup · 3 task-specific · 0 conflict · 0 deferred · 7 below threshold
  No-other-home: "A sidecar entry payload needs a leading blank line before it is appended"
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B

## Decisions & corrections — dispositions
- The operator's word for this chunk's operator pass (`inputs#I6`) — rejected: Filter 2, task-specific (an authority record for one chunk; its home is the inputs entry and the evidence file).
- The operator's word for this wrap (`inputs#I7`) — rejected: Filter 2, task-specific.
- `.claude/rules/ci.md` 2026-10-13 recurred and held — not a learning: the standing entry worked as written (no defect, no recurrence-despite-learning).
- The hygiene re-read's own line recorded after the push — rejected: Filter 4, 0.2 (a specific technical detail, one event, no gate failure).
- P3 smoke recorded from the P2 block, not re-driven — rejected: Filter 4, 0.0 (one decision, paraphrased from the letter's leg rule).
- The walk-class line with no deferral to void — not a learning (no rule moved).

## From this window
- The wrap names a master's run-dir files by the doc (`architecture-outline.md`, `cascade-architecture/`), the phase by its specialty prefix — rejected: Filter 4, 0.6 exactly (measured +0.4, technical detail +0.2; no conditional signal — the skill letter already states `{doc}-outline.md` and `cascade-{doc}/`, a durable home). Friction record `2026-10-26T07:00:05Z-e`.
- A recursive `rm` is denied on this host, so a mistaken run-dir subdir stays — rejected: Filter 1, its class is `.claude/rules/host-linux.md` 2026-10-07 (a removal the permission layer denied is not done through another tool); no facet the entry lacks that a later act needs.
- The stamp-ahead hook refused an estimated `Date:` in the report — rejected: a hook already refuses the very act (Filter 4's no-other-home condition; the guard cost one re-issue).
- A `· as:` direction carrying this chunk's marker into a body line fails `classify.py check` provenance — rejected: Filter 4, 0.6 exactly (a real gate failure +0.4, technical detail +0.2; the letter's Apply rule 1 already states provenance never enters the body).
- The apply writer refuses a retirement that empties a whole detail group; main removes the group by hand — rejected: Filter 2, task-specific (one site), and it is the skill's own mechanism (recorded in the reconcile step's problem block).
