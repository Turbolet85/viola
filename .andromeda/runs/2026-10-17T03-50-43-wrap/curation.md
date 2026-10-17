# Curation — 2026-10-17-cli-machine-contract

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A capability whose built surface falls short of its claimed acceptance is un-claimed with a dated note naming the gap and the route entry that owns it, never refined down to what was built so that it reads verified." (confidence 0.7)
  Tier 2 (.claude/rules/*):                   + testing.md: "Under a parsed `VIOLA_NAME`, `viola list` (and `viola revive --list`) opens its `cli` log and so creates the home it resolved …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A capture-then-write file is not a capture until its bytes land" (confidence 0.8)
  Filters: 4 dup · 0 task-specific · 0 conflict · 0 deferred · 2 below threshold · 3 recurrence-despite-learning (→ handoff)
  No-other-home: "Under a parsed `VIOLA_NAME`, `viola list` … creates the home it resolved" · "A capture-then-write file is not a capture until its bytes land"
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B

## Applied
- T1 — the un-claim rule. Signals: explicit correction +0.4 (the overseer chose "Un-claim v1-39" over this wrap's recommended "Refine + CARRY", naming the never-weaken rule) · "never" language +0.3 = 0.7.
  Proof: `relay-1.md` Q2, snapped `inputs#I3`; the P7.3 `matrix.py unclaim` of `v1-39` it directs.
- T2 testing.md — a name makes `list` create its home. Signals: measured +0.4 (the first `home_resolution_…` run read red on the oracle "the default home was not made") · specific technical detail +0.2 · no other durable home +0.2 (no master, route annotation or ledger note carries it) = 0.8.
  Proof: report *Decisions & corrections* (the curation candidate, implement's fix loop); `tests/cli_machine_contract.rs` 438-507 `home_resolution_takes_the_flag_then_the_session_then_the_default` (its `logged_as_builder` witness, 482-486).
- T3 — capture-then-write. Signals: measured +0.4 (the red CI `ci#38994749835` on `f6bcce5b`, the unit witness `captured_reads_no_capture_while_the_claim_is_empty` red reverted / green restored) · specific technical detail +0.2 · no other durable home +0.2 (no master site: the report's `capture-wait` pattern found none) = 0.8. Path scope unclear (product `src/cmd/verify/typed.rs`, no rule file covers it) → Tier 3.
  Proof: `viola-0.1.0/chunks/2026-10-17-cli-machine-contract/evidence/ci-red.md` §2, §4-5.

## Every Decisions & corrections entry
- The operator's invocation of implement → not a learning: a per-chunk direction, followed (report).
- The operator's invocation of this wrap → not a learning: its gap was routed to P2 (Q2) and P7.3.
- Capture-then-write → applied, T3.
- A timing window a host cannot open is forced at the decision → rejected, Filter 1 duplicate of testing.md 2026-09-27 (forcing a timing window); the plan's step 2 sampled 20 firings despite it → recurrence-despite-learning (→ handoff).
- A test oracle for a canonical path on Windows takes the plain drive form → rejected, Filter 4: 0.6 (measured +0.4, specific +0.2), no conditional signal — this wrap amended the plain drive form and the `resolved(home)` oracle into security-plan and test-plan.
- A test-started `viola` strips `VIOLA_DIR` and `VIOLA_BIN` too → rejected, Filter 1 duplicate: the cascade wrote it into testing.md's generated body (line 40) from test-plan §5.
- A path comparison against a test home compares the resolved home → rejected, Filter 1 duplicate: the same testing.md:40 sentence ("a path oracle compares `resolved(home)`").
- Under a `VIOLA_NAME`, `viola list` creates the home it resolved → applied, T2 testing.md.
- `viola --help`'s command descriptions hold `--file` → rejected, Filter 1 duplicate: test-plan §5's completeness sub-bullet now names the `Options:` section (this wrap).
- Sweep hazard — the `scanned` token → rejected, Filter 4: 0.2 (specific +0.2; it changed no design, one wrap's reading).
- Sweep hazard — a sed anchored on pre-format text misses after rustfmt reflows → rejected, Filter 1 duplicate of testing.md 2026-09-25's extension → recurrence-despite-learning (→ handoff).
- Guard / hook refusals → not learnings (each re-issued once); the TOML pattern written through `printf` that landed a doubled backslash → recurrence-despite-learning of host-linux.md Transports ("never build … through shell quoting: printf collapses escapes") and its 2026-09-28 addition (→ handoff).
- Process note — implement wrote `v1-39` `implemented` with the gap surfaced → not a learning: the disposition is P7.3's un-claim (Q2).
- This window — "a boundary widening is never settled by a delegate" (Q1's note) → rejected, Filter 1 duplicate of the playbook's "Boundary widening — what ratifies it" rule.
