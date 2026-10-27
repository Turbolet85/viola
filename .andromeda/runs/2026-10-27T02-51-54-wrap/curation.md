# Curation — 2026-10-27-pinned-copy-race-and-reproducible-builds

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Comparing two builds by section: three tool traps" (confidence 0.8)
  Filters: 5 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "Comparing two builds by section: three tool traps"
  CLAUDE.md size: 128/200 · T1 3.1 KB, 0 over 600 B
```

## Applied
- Tier 3 · `.claude/docs/session-learnings.md` · «Comparing two builds by section: three tool traps» — confidence 0.8: verified by measurement +0.4 (measurement B's first firing produced false equal readings), specific technical detail +0.2, reached no other durable home this wrap +0.2 (no master, route annotation, rule or contract document carries it; the chunk's evidence file is a record, not a home).
  Proof: `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md`, «The three firings» table — firing 1: the `true` control resolved to the shell builtin and `-O binary --only-section` hashed `.comment` / `.symtab` / `.shstrtab` / `.strtab` as empty; firing 2 stopped at `.symtab` (`objcopy --dump-section` refuses it); firing 3 read every section at its `readelf -S -W` offset and size.

## Every *Decisions & corrections* entry, weighed
- The operator's wrap directive (`inputs#I6`) — not a learning: a ruling for this wrap; its items land at P5 (WATCH, CARRY), P6 (FOR DISCUSSION) and the sidecars.
- Red 2 folded here, PROVISIONAL (`inputs#I4`) — not a learning: a recorded word, carried PROVISIONAL in the sidecars and the handoff.
- F363 (`inputs#I2`) — not a learning: a one-off ruling on the phase's re-spawn limit, recorded on its P5 card and in the friction log.
- Implement's operator directive (kill deadlines, journal before the wait, read listings whole) — rejected, Filter 1 duplicate: `testing.md` 2026-09-25 (a remove-the-guard control's kill line), `host-linux.md` 2026-10-27 (read a process listing whole).
- Guard refusals at implement and at this wrap (stamp-ahead hook ×2, the Bash guard's `cd` ×2) — rejected, Filter 1 duplicate: `host-linux.md` §Paths & the working directory states the `cd` guard; each refusal cost its one re-issue.
- A permission-classifier refusal with no verdict, treated as transient — rejected, Filter 1 duplicate: CLAUDE.md Tier 1 «A tool call refused with no verdict by the permission classifier is a transient outage».
- Recurrence of verification-harness.md 2026-10-09 (a file hash is no proof two builds load the same code) — rejected, Filter 1 duplicate; not a `recurrence-despite-learning`: the entry was read and held (the loaded sections compared), no failure recurred.
- Tool traps in a section comparison — applied (above).
- A JUnit loop that copies the newest archive by name read an old file — rejected, Filter 4: a one-off mention without emphasis (−0.3), 0.3 total.
- A product race read by its errno, not by its symptom — rejected, Filter 4: 0.6 exactly (measured +0.4, detail +0.2), and its fact is amended into test-plan §7 this wrap (the master owns it).
- A closed-handle placement defeats `ETXTBSY` — rejected, Filter 4: 0.6 exactly; amended into architecture [Snapshot writer] and security-plan Data Protection this wrap (the masters own it).
- This wrap's site-sweep reach gaps (the unfired generic owner pattern; `pin-write`'s wording missing `architecture.md:319`) — not a learning: the report template already states that a listing reaches its patterns' wording and no further; recorded as a friction record (`input.report-insufficient`).
- The windows-2025 runners differ in a temp dir's inherited ACL by host; read a runner-inherited grant by its right — rejected, Filter 1 duplicate of a generated body: the cascade wrote it into `.claude/rules/testing.md` (its body) and the test-plan masters this wrap.
