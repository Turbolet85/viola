# Curation — 2026-10-09-inner-cr-and-crlf-in-a-sent-text

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + ci.md: "`gh run list --commit` takes the full 40-character sha; an abbreviated one prints an empty list with exit 0" (confidence 0.8)
      Proof: at the operator pass, 2026-10-09T19:47Z, the list for the 12-character sha of `308099b` printed `[]` with exit 0 while `ci#37981185305` stood on it; the full sha listed it (`evidence/operator-pass.md`, "Entry 23"; the report's Decisions & corrections).
    + verification-harness.md: "a build's file hash is not a proof that two builds load the same code; read the loaded sections" (confidence 1.0)
      Proof: `evidence/live-preconditions.md`, builds 1 to 6 (the hash moved with comment text alone, 2 against 4; build 4 did not give build 1's hash back) and the three section readings of 19:14:28Z, 19:19:17Z and 19:32:06Z, six of six equal; the plan's revision replaced step 12's file-hash equality with the section reading on the operator's word (`inputs#I5`).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred
  No-other-home: "`gh run list --commit` takes the full 40-character sha"
  CLAUDE.md size: read at P7 from `health.py check`

Scoring:
- the `gh run list` entry: verified by measurement +0.4, a specific technical detail with context +0.2, total 0.6; the next-entry signal did not fire; reached no other durable home this wrap +0.2 (it stands only in this chunk's evidence and report); 0.8.
- the loaded-sections entry: verified by measurement that changed the chunk's design +0.4, the operator's correction of step 12 +0.4, a specific technical detail +0.2; 1.0. The unexplained difference itself is owned by a route CARRY (Phase 5); the entry carries the method, not that fact.

Rejected:
- duplicate: "a red where no assertion failed on a value is read against the backing first, never re-run for green" (a standing rule: CLAUDE.md's session learnings, "A red stays open until its cause is known", and the handoff's standing rules).
- duplicate: "the implementer drives the operator pass only on the operator's word for that chunk; the final sha needs green on its first attempt" (`.claude/rules/ci.md`, its body).
- task-specific: "a release-check entry that ends in about 2 s still rebuilt the product crate" (one reading, one entry; the artifact's mtime answered it).

The deferred list the handoff carries is unchanged; no recurrence of a recorded defect was met in this session.
