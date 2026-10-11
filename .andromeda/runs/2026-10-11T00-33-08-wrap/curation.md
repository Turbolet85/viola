# Curation — wrap of 2026-10-10-session-links (Phase 3)

The conversation that ran implement, the operator pass and this wrap's Phase 1 was gone at the resume. Its
candidates were read from `resume-point.md` (eleven, each with its evidence); this session added four recurrences
of its own. No user correction and no "from now on" convention was given in either session.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "On the Linux dev host `schema-check` (G4) reads kept homes only …" (confidence 0.8)
                                              + testing.md: "`assert_eq!(v, [x.clone()])` in test code trips `clippy::cloned_ref_to_slice_refs` …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred · 7 below the threshold · 5 recurrences (→ handoff)
  No-other-home: "On the Linux dev host `schema-check` (G4) reads kept homes only" · "`clippy::cloned_ref_to_slice_refs` on a one-element array in `assert_eq!`"
```

## Applied

- **verification-harness.md** — `schema-check` on the dev host reads kept homes only (candidate 3).
  Proof: the report's "Spec claims disproved by measurement": the chunk's `schema-check` entry read 160 files and
  2220 lines, the figures the previous chunk's entry read, because a root test's home is removed when the test
  ends; the plan's note at `plan.md:349` said the entry validates the new cases' lines. Score: measured, a plan
  claim falsified (+0.4), a specific technical detail (+0.2), no other durable home at this wrap (+0.2; Validate
  check 6 disposed the claim "plan note only, nothing to amend").
- **testing.md** — `clippy::cloned_ref_to_slice_refs` (candidate 5).
  Proof: the report's "Decisions & corrections": the lint fired on `[linked.clone()]` in an `assert_eq!` of new
  test code, found by a hand lint probe before the block; the bound array passes. Score: proven by a lint that
  the block's `-D warnings` entry turns into a red (+0.4), a specific technical detail (+0.2), no other durable
  home (+0.2).

## Rejected

- Duplicate (Filter 1): candidate 4, red-first on stub bodies. `testing.md`, the 2026-10-10 extension of the
  remove-the-guard entry, already says the red-before-product reading is taken on stubs. Its facet (restore a
  scratch edit from a kept copy and read SHA-256 back, never by a reverse Edit) scored 0.2 and was not applied.
- Below the threshold (Filter 4), each at 0.6 or under with no conditional signal:
  - candidate 1, an operation placed in the unheld part of a plan whose only caller sits behind the stop (0.6;
    the route carries the fact on "The board: viola list", so the no-other-home signal does not apply);
  - candidate 2, a forced-on edit must name a handle the target holds (0.4);
  - candidate 8, a harness `run` document names its archive (0.2);
  - candidate 9, `gate.py show --n` tells a designed red from a real one (0.2);
  - candidate 10, a CI run's three test logs read in one script (0.2);
  - candidate 11, the gate tool prints nothing until its block ends (0.2);
  - the facet of candidate 4 above (0.2).
- Recurrences (→ handoff, Deferred learnings; never a third entry):
  - `host-linux.md` Transports: one inline python heredoc for a source edit at implement (candidate 6), and
    inline python heredocs for multi-line reads in this session;
  - a time written without the clock, caught before it was pasted (candidate 7);
  - `host-linux.md` Paths: one `cd` at the head of a compound in this session's first Bash call;
  - `host-linux.md` Exit codes: one verdict tool (`registry.py check`) read through a line filter;
  - `host-linux.md` Long single-line files: master lines read through a scratch script.
