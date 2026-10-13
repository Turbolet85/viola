# Curation — the 2026-10-13T06-38-35 0-pending wrap (the Epoch 4 boundary)

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A planned step or master amendment that departs from a founder ruling is never a decisive lean or a card-confirm item …; his answer covers only what the question shown to him named" (confidence 0.7)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A sweep for a ruling's marks keys on every wording the marks use" (confidence 0.9)
  Filters: 2 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 3 below threshold or not a learning
  No-other-home: "his answer covers only what the question shown to him named" (folded into the Tier 1 entry)
  CLAUDE.md size: 128/200 · T1 2.6 KB, 0 over 600 B
```

## Applied

- **T1** — the founder-rulings rule, drained from the operator's auto-memory `founder-rulings-stand-until-his-word`
  (directive §6 item 1; "drained at this boundary" is the operator's word). The memory file and its `MEMORY.md` line
  are removed. Score 0.7: an explicit correction by the operator (+0.4, his words quoted in the memory: "the founder
  ruling that the mutation workflow is dispatched only at the epoch boundary stands until his own word"), "never"
  language (+0.3).
  Proof: the 2026-10-10 windows-mutation-grade P5 review, where a dispatch outside the boundary was listed as a
  card confirm item and ruled out.
  Its second clause (the batch answer's scope) is a facet scored on its own: two corrections by the operator at
  this wrap's halt (+0.4; `relay-2.md`, answers 1 and 2: "The question shown to him named the statusline arm only",
  "He was shown exactly: an unlistable instances/ gives exit 1, and WHEEL 23"), a specific detail with context
  (+0.2), and no other home (+0.2: the masters state the two scopings as facts and never the rule) → 0.8. It is
  folded into the new entry as one write.
  Proof: `relay-2.md`; the split marks at `architecture.md` `:139`, `:407` and `obs-plan.md` `:394`, `:1072`.
- **T3** — the sweep hazard. Score 0.9: verified by measurement (+0.4), three instances this boundary (+0.3), a
  specific technical detail (+0.2). Tier 3 because no rule file's `paths:` covers `.andromeda/**`.
  Proof: `report.md` (P1, "Sweep hazard, measured this window", twice: `PROVISIONAL`, `confirmed the card`); this
  window: `security-plan-amendments.md:499` missed by the `cardconfirm` pattern, and the `accepted by the operator`
  sites found by reading (`cascade-dispositions.md`, the `acceptedop` row).

## The report's Decisions & corrections, each weighed

| entry | disposition |
|---|---|
| the founder-rulings memory (directive §6 item 1) | applied — T1, above |
| a root case booting a home starts from a stamped one (§6 item 2) | rejected — Filter 1, duplicate: this wrap's cascade wrote it into `testing.md`'s generated body (`leaf-test-plan.md`, `testing.md:34`, from test-plan `:628`). That also settles the handoff's deferred `recurrence-despite-learning: test-plan / tests-summary`: the fact now stands in an auto-loaded rule file |
| a root test's home is removed at its end; schema-check reads only kept homes (§6 item 3) | rejected — Filter 1, duplicate of `verification-harness.md` `## Session Additions`, the 2026-10-11 entry on `schema-check` (G4) |
| the answer-card rule applied to recorded words (§2) | not a learning — the letter holds it (`amendment-flow.md` §Escalate); corrected at its source, the `security-plan` sidecar correction entry |
| a playbook rule written broader than its ruling (§3) | rejected — Filter 4, 0.6 (measured +0.4, specific +0.2); no conditional signal: this wrap's P2 minted the narrowed rule into the playbook |
| sweep hazard: `PROVISIONAL` marks nothing | applied — the T3 entry |
| sweep hazard: `confirmed the card` is in no master; sidecars are outside the sweep | applied — the same T3 entry |
| the directive's `architecture.md:407` is `:408` | rejected — Filter 4, one-off (+0.4 measured, −0.3 one-off) |
| no `inputs.py snap` without a chunk folder | not a learning — a process decision following the Epoch 3 boundary precedent |
| the Bash guard refused a `cd` (P1), and twice in this window (into the skill's base dir, into the run dir) | rejected — Filter 1, duplicate of `host-linux.md` §Paths & the working directory; not a recurrence: each refusal cost its one re-issue, so the guard held the rule (`guard-refused` on the step records) |
| the handoff's carried deferred learnings | carried unchanged, except `test-plan / tests-summary`, settled above |
