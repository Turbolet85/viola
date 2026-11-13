CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A remove-the-guard control can go red without running the guard's own case" (confidence 0.8)
  Filters: 1 dup · 5 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  No-other-home: "a control's removed firing under nextest fail-fast can leave the guard's own new case unrun"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

Applied
- Tier 3 · session-learnings.md · 2026-11-13 entry. Signals: verified by measurement +0.4, specific technical detail +0.2, no other home +0.2 (not in a master — the test-plan add `decision/control-isolates-its-own-case` was rejected at Validate check 3; not on the route; no hook) = 0.8.
  Proof: `.andromeda/cache/chunks/2026-11-13-rows-proposal-rule-work/evidence/control-4.txt` — record-email's removed firing `261/610 tests run: 258 passed, 3 failed` with `unclean_refuses_an_email_in_a_string_or_a_key_after_a_path` absent from the log; `control-5.txt` — hygiene-email `62/93`, the status-line and script cases not reached.
  First written as an extension of testing.md's 2026-09-25 entry (Filter 1 additive facet, tx 1013); health check 4 then read `testing.md … 1 over 1.5 KB`, so the facet was moved to Tier 3 whole (testing.md re-landed at its committed bytes, tx 1014; session-learnings.md tx 1015).

Decisions & corrections — each entry's disposition
1. The operator's implement directive (no live start, no cargo-mutants, controls under a kill deadline, journalling) — rejected, task-specific (a directive for one run).
2. `gate.py control` names its clone on its head line — rejected, below threshold (0.2: a tool's printed form, specific technical detail alone).
3. A control fires its entry whole; fail-fast can leave the guard's own case unrun — applied (Tier 3, above).
4. veil 0.3.0 refuses `#[derive(Redact)]` with no redacted field — rejected, below threshold (0.6 exactly: measurement +0.4, detail +0.2; its home is the obs-plan rows amended this wrap, so no no-other-home signal).
5. A ~1 s removed firing read as a test red only after its `panicked at` line was read — rejected, task-specific (one event's check, the practice testing.md's 2026-09-29 entry already states).
6. Guard refusals (`cd` out of the project, a `cat` heredoc to a file) — rejected, duplicate (host-linux.md §Paths and its 2026-09-28 Session Addition); each cost one re-issue, no recurrence.
7. A stray `cat > /dev/null` blocked on stdin — rejected, task-specific (one self-made slip).
8. The phase directive counted `fn digit_runs` twice, HEAD held three — rejected, task-specific (corrected at phase).
9. Standing learning met (testing.md 2026-11-13, a control clone holds no build output) — not a learning: the entry held; no recurrence.
10. The operator's wrap directive (inputs#I2) — rejected, task-specific; its item 1 landed as an obs-plan row (P2).
