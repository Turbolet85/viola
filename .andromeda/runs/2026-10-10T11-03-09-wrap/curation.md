CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md (in place, the 2026-09-25 remove-the-guard entry): "Extended 2026-10-10: a case that asserts an absence cannot read red on a tree without the behaviour, so its remove-the-guard run forces the behaviour on instead; and a red-before-product reading of cases that need a new API is taken on stubs that hold the names and no behaviour."
    Proof: `evidence/red-green.md` of chunk 2026-10-10-self-healing-state. Section 1: on the stub tree 6 of the 39 inline cases of steps 4 to 7 read green, each asserting an absence, where the plan's step 3 said every new case fails. Section 3: the absence control (three guards forced on) read the five remaining ones red; the key-table control read the sixth red. Score: verified by measurement 0.4 + a specific technical detail with context 0.2 = 0.6, then the no-other-home signal 0.2 (no master, route annotation, ledger note or hook carries it) = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred · 1 at the threshold (rejected)
  No-other-home: "a case that asserts an absence takes its red reading from a control that forces the behaviour on"
  Extended: T2/testing.md: "Every new guard test carries its remove-the-guard run" + "an absence case's control forces the behaviour on; a red-before-product reading runs on stubs"

Corrected at the source, not by an entry:
  - `.claude/rules/events.md`, Parsing: "healing lands with the route entry "Self-healing state"" (the operator's curation item, `inputs#I4`). The sentence sat in the rule's generated body, so it was corrected through its master: architecture [Database / State Store] was amended at P2 and the cascade re-derived the bullet (`cascade-dispositions.md`, the `events.md:34` row). The file's `## Session Additions` holds no entry on the subject.

Rejected:
  - "a local G2 or G4 entry reads no line a test's own home held, so a case holds its new line to the schema itself": 0.6 exactly (measurement 0.4 + detail 0.2), and neither conditional signal applies, because this wrap amended the fact into test-plan §6 Chaos suite. The master carries it.
  - "the stamp-ahead hook blocks a fixture's literal time dated later than the clock": task-specific, and the hook itself refuses the act.
  - "clippy's `suspicious_map` refuses `.map(..).count()` over results": task-specific, general tool knowledge.

Recurrence, for the handoff's Deferred learnings:
  - `recurrence-despite-learning: host-linux.md` Paths: a `cd` outside a subshell four times in this session (implement twice, this wrap twice); one moved the working directory into `.andromeda/` for one call.
