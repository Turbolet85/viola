# Cascade dispositions — 2026-09-29-h2-conpty-resize-probe

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `90aba7ce`, the pre-CI parent), ten
patterns derived from all four amendments of this pass (P-A1 arch :396, P-A2 arch :47 [PTY], P-T1 test-plan :926,
O-T2 test-plan :1506) — the report-file registry (`watchdir`, `childreport`), the resize/key mechanism (`resize` —
the broad `resiz` stem, after the narrower `resizekey` regex's control never fired on the pre-pass masters and the tool
refused it; `resizeprop`, `ptydefect`, `h2`, `modulepty`), and the coverage-propagation claim (`profile`, `envclear`,
`covchild`). Every control fired. 71 rows; each dispositioned below. Sections read whole beyond the rows: arch §[PTY]
(:47), §Occupied Resources → Filesystem (:396); test-plan §5 row :926, §10 Stack adjustments :1502-1507, §Anti-Patterns
:1572-1577.

## Masters — amended lines (this pass's own text)
- architecture.md:47 (`resize` ×6, `ptydefect`, `h2` ×2) — amended (P-A2). Re-read for an intra-line duplicate of a
  retired claim: none retired; the `no PTY defect` clause (@c500) is the 0.8.1 pin's spike rationale — true of
  portable-pty 0.8.1; H2 is measured below the seam in ConPTY and is stated as such in the appended sentence. No change.
- architecture.md:396 (`watchdir`, `childreport` ×2, `resize`) — amended (P-A1). No duplicate.
- test-plan.md:926 (`resize` ×12, `resizeprop` ×2, `h2`, `modulepty`) — amended (P-T1). The row's standing claim "a
  resize is propagated" stays true (the key-free witness proves it); no duplicate.
- test-plan.md:1506 (`profile` ×2, `envclear`, `covchild` ×2, `resize`) — amended (O-T2). The standing rule for child
  `viola` product processes stays; the exception names the throwaway crate only.

## Masters — standing lines, no change (true claims sharing a token)
- architecture.md:70 — wheel: focus/mouse/resize sequences do not count as editing. Unrelated.
- architecture.md:382 — the forced-window pump-start resize test. True, unrelated to H2.
- security-plan.md:232, :691 — the `FAKE_AGENT_PUMP_DELAY_MS` seam and its forced-window test. Unrelated.
- test-plan.md:40 — the seam's verb list. True.
- test-plan.md:125 — the tui driver writes keys and resizes. True.
- test-plan.md:1369 — fake-agent receipt `size`: "at start and again whenever the terminal size changed (the resize
  oracle)". NOT changed by this chunk (the fake agent is untouched); research.md read it at HEAD as sampled only around
  a byte read (`src/bin/viola-fake-agent.rs:463-473`), so a size change with no later byte yields no receipt — the
  spec's wording overstates the fake agent. Playbook "Not this chunk's drift" (its caution): routed to the owning
  entry as a CARRY at P5 — working-route "Fake-agent drift contract" (the fake agent's receipts), not amended here.
- test-plan.md:1893, :1897, :1901, :1903 — §12 Decisions Log history. The baseline; never rewritten.
- test-plan.md:1025, :1034 — `H2` is a home name in a Path scenario. Unrelated token.
- test-plan.md:789, :991, :1516, :1577 — `env_clear()` must re-add `LLVM_PROFILE_FILE`; perf never runs with it set.
  True; the new exception removes names by `env_remove` in a throwaway's nested cargo, never `env_clear`.
- obs-plan.md:61 — the seam's verb list. True.
- a11y-plan.md:100, :321, :624, :793, :1210 — focus/mouse/resize sequences never move the wheel. Unrelated.
- a11y-plan.md:533, :958 — SC 1.4.4 Resize text. Unrelated.

## Curation home
- .claude/rules/testing.md:57 (Session Additions, `childreport`) — "have a spawned child stream its report to a known
  file outside the test's tempdir". True as written; the test-side sibling is a realization of it. No curation
  extension required (routed nowhere).

## Leaves
- .claude/docs/gotchas.md:69 (`profile`, `envclear`, `covchild`) — the `env_clear` re-add rule. True; no change.
- .claude/docs/gotchas.md:102-105 — the pump-start resize entry. True; no change.
- .claude/docs/gotchas.md:107-112 (`resize`, `h2`, `watchdir`, `modulepty`) — the H2 entry written at the operator
  pass. Recomputed against the amended arch [PTY] and test-plan §5: the same count (13/200), class, `dsr-cpr 0`,
  builds and run ids; consistent. No change.
- .claude/docs/services/viola-pty.md:6, :30, :31, :36 — the seam's verbs, the H2 finding, the test-rig report files,
  the tests entry point. Recomputed against arch :47 and :396: consistent. No change.
- .claude/docs/services/viola.md:21 — wheel: resize never takes it. Unrelated.
- .claude/rules/testing.md:30 — the `env_clear` re-add rule. True; no change.
- CLAUDE.md:22 — viola-pty module line. True; no change.

## Leaf set recomputed (amendment-flow §Cascade step 3)
- architecture changed (§Established Decisions [PTY], §Occupied Resources → Filesystem) → CLAUDE.md `GENERATED:setup:*`
  (overview · modules · warnings · pointer-table · architecture) recomputed from their arch sections: no line derives
  from the [PTY] as-built detail or the test-only filesystem row — unchanged. Docs leaves of the amended sections:
  `docs/gotchas.md` and `docs/services/viola-pty.md` — current (above).
- test-plan changed (§5, §10) → `.claude/docs/tests-summary.md` (grep `LLVM_PROFILE_FILE|PTY|resize|env_clear|
  throwaway|propagat` → 0 hits: it carries neither the §5 row nor the §10 stack adjustments) — unchanged;
  `.claude/rules/testing.md` / `verification-harness.md` (the rules test-plan scopes) — rows above, unchanged;
  CLAUDE.md `GENERATED:setup:warnings` — its test-plan-derived lines untouched by §5/§10 — unchanged.
- Lateral binds: test-plan §3 ↔ obs-plan §3 — §3 not amended; a11y ↔ obs schema — not amended.
