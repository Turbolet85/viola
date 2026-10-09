# Curation — 2026-10-09-epoch-3-cleanup

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A master, a key file or a leaf names a working-route entry by its title, never by a bare route line number" (confidence 0.7)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extension only (below)
  Filters: 1 dup · 4 below threshold · 0 conflict · 0 deferred by the cap · 1 recurrence (→ handoff)
  Extended: T3/session-learnings.md: "2026-09-29 — A gate's freshness check must name a file, never a directory" + "an entry that only reads a file takes no `artifact` key"
```

## Applied
- **Tier 1 — route entries by title.** Signals: the operator's direction, given twice ("the stale bare route
  numbers are cited by title, every site found": the wrap invocation, `inputs#I9`, and the P4 answer, `inputs#I3`)
  +0.4; a repeated pattern, separate events +0.3 (the a11y key file at the 2026-10-09 0-pending wrap; this wrap's
  `:93`, the `:125` / `:127` pair in three masters, and the two `working-route.md:109` sites). 0.7. Tier 1 over
  Tier 3: it binds every write to a master or a leaf, no rule file is scoped to those paths, and it is one sentence
  (215 B).
  Proof: this wrap's `cascade-dispositions.md` (the `route-125-127`, `route-109` and `route-93` patterns: 5 master
  and key-file sites amended, 5 leaf sites re-derived); `citation-dispositions.md` (the sweep read none of them, a
  bare `:N` being no citation).
- **Tier 3 extension — a reader takes no `artifact` key.** Filter 1: the matched entry is 2026-09-29's on the
  `artifact` key; the facet names a failure mode it lacks. Signals: verified by measurement +0.4 (a real gate red
  that stopped implement and had the plan revised), a specific technical detail +0.2, and, at exactly 0.6, no other
  durable home +0.2 (no master, no route annotation, no playbook rule and no hook carries it). 0.8.
  Proof: the stopped run's gate trail (`.andromeda/runs/2026-10-09T15-46-10-implement/`, entries 22 and 23:
  exit 0, `true`, `artifact STALE`); the revision, `inputs#I7`; both entries green at this wrap's implement run
  without the key.

## Filtered
- duplicate: a relayed founder ruling is not written into master text (the Tier-1 entry on relayed claims covers
  it; applied at Validate, not a recurrence of a defect).
- below threshold (0.2 to 0.6): the private directory of a hand-driven rig is recorded nowhere for a re-entry ·
  a bounded-repetition window search over a multi-KB master line stalls the host's grep (the documented
  `cascade.py window` already is its home) · `py_compile` leaves a bytecode cache in `evidence/` (the hygiene read
  already refuses it) · a plan's gate-block order fixes the order of the steps that write a probed file.
- recurrence, to the handoff: a time typed into a record ahead of the clock (the report's date, 6 minutes ahead;
  the stamp hook refused it and one edit after a clock read corrected it). It matches the carried deferred
  learning "a time written into a record ahead of the clock".
