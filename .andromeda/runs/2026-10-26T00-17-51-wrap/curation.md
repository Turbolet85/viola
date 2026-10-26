# Curation — 2026-10-25-windows-endpoint-admission · wrap

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md (extended 2026-09-27): "the same cache also loses the capturing thread's own line …; a root-bin scoped capture goes through test_support::with_scoped_default"
  Tier 3 (.claude/docs/session-learnings.md): + "A windows-sys item needs every feature its signature's types need"
  Filters: 0 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  Extended: T2/testing.md: "2026-09-27: A scoped tracing subscriber installed as the only dispatcher caches a callsite first hit on another thread as disabled …" + "the capturing thread's own line is lost too; the root bin's with_scoped_default"
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B
```

## Applied
- T2 `.claude/rules/testing.md`, the 2026-09-27 entry extended in place (an additive facet: the on-thread loss and the
  remedy the matched entry lacks). Confidence 1.1: verified by measurement +0.4 (the witness), explicit curation request
  +0.5 (the founder-approved plan's «Curation candidate (wrap's, not a master amendment)» line), specific technical
  detail +0.2.
  Proof: `chunks/2026-10-25-windows-endpoint-admission/evidence/capture-witness.md` — the witness red on a bare scoped
  default (`left: []`, exit 100) and green on the helper (`1 test run: 1 passed`), nextest, base `b6740d59`.
- T3 `.claude/docs/session-learnings.md`, «A windows-sys item needs every feature its signature's types need».
  Confidence 0.8: verified by measurement +0.4 (it falsified the plan's step 5 and research's «no new feature»),
  specific technical detail +0.2, no other durable home +0.2 (no master states the gotcha: the key file holds this
  dependency's two features, not the rule; a sidecar's Why is no home). Tier 3, not testing.md: its imperative half is
  a manifest's, which no rule file's `paths:` scopes.
  Proof: windows-sys 0.61.2 `src/Windows/Wdk/Storage/FileSystem/mod.rs` (`#[cfg(feature = "Win32_System_IO")]` above
  `NtQueryInformationFile`'s `link!`); `cargo tree -e features -i windows-sys@0.61.2 --target x86_64-pc-windows-msvc`
  (`Wdk_Storage_FileSystem` from mio 1.2.4, `Win32_System_IO` from interprocess, mio, socket2) — report Dependencies.

## Every Decisions & corrections entry, dispositioned
- The operator's word at implement's invocation (operator pass, CI read, journaling, transcripts) — not a learning: a
  directive for that run, carried out (inputs and journal).
- The operator pass made by the session on that word — not a learning: a decision of the run.
- The wrap directive (inputs#I7) and invocation (inputs#I8) — not a learning: directives for this wrap.
- The founder's P4 forks (inputs#I5) and P5 yes (inputs#I6) — not a learning: rulings, landed in the masters at P2
  (test-plan §2 Runner, §4 the three crate captures) and in P5's route insertion.
- Learning candidate, testing.md 2026-09-27 extension — applied (T2, extended).
- Learning candidate, windows-sys feature gates — applied (T3).
- Learning candidate, the anonymous-token test binds before it impersonates — rejected, Filter 4 (confidence 0.2:
  specific technical detail only; a design choice, never a measured failure) and Filter 2 (it names a test helper,
  `anonymous_read`).
- Hook refusal, the stamp-ahead hook (three times this session: implement's journal, this wrap's journal, `relay-3.md`)
  — rejected, Filter 4: the hook is its home (it already refuses the act), so no-other-home cannot fire; repeated
  pattern +0.3 alone.
- Guard refusal, the Bash guard's `cd` (twice this session, each re-issued once with absolute paths) — not a
  learning: the guard held the standing rule (`.claude/rules/host-linux.md`, «Paths & the working directory»), so it is
  no recurrence; a `guard-refused` fact of the steps it happened in.
- Sweep hazard, the 0-site new-fact patterns — not a learning: the report states it is no absence of a stale site;
  the masters carry the new facts now.

## Also met this wrap, outside the report (this window)
- `cascade.py leaves --apply --only` takes several ids after one switch; a second `--only` replaces the first — a
  pipeline tool's behaviour, recorded as evolve telemetry at P7 (`gates`), never curated.
- Two hard P2 tool stops (F350 the newdetail re-spawn, F351 a writer's label) — pipeline defects, recorded as evolve
  friction at P2, never curated.
