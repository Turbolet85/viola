# Curation — 2026-11-22-epoch-4-cleanup wrap (2026-11-22T19-49-19Z-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A rename of a product constant or a move of a nextest filter is swept over the test tree's string literals too, not only its code …"
  Tier 3 (.claude/docs/session-learnings.md): none
  Corrected: testing.md 2026-09-24 entry's 2026-10-06 extension — «a root test with no `send_window_` / `verify_window_` prefix runs under the `mutants` profile's 15 s kill» now names the two fake-agent replay tests the override takes (cap-exempt)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
  No-other-home: "A rename of a product constant or a move of a nextest filter is swept over the test tree's string literals too"
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

## Applied
- testing.md `## Session Additions`, 2026-11-22 sweep hazard — confidence 0.8 (verified by a real gate failure +0.4 · specific technical detail +0.2 · no other durable home this wrap +0.2).
  Proof: `bash scripts/agent-run.sh run --e2e` red at implement firing 1 on the two pins (report, Deviations from intent: `tests/contract_bounded_readers.rs` needle `VERSION_DEADLINE,` → `AGENTS_READ_DEADLINE,`; `tests/contract_lints.rs` `VERIFY_WINDOW_FILTER`), green at firings 2 and 3 after re-pointing; no master, route annotation or rule holds it (dedup: testing.md 2026-10-04's 2026-11-14 extension covers counted literals of the events log only, an other failure mode).

## Corrected
- testing.md 2026-09-24 entry, its 2026-10-06 extension, in place with `[corrected 2026-11-22: …]`.
  Proof: `.config/nextest.toml` (new-text 21-22 · 24 · 56-57 · 59): both `verify_window_` overrides read `test(/verify_window_/) | test(/fake_agent_(dialog|framing)_replay_matches_every_recorded_/)`; the cascade sweep's `vwin2` curation row `.claude/rules/testing.md:57` routed it here; `binary(contract_fake_agent_drift)` under `--profile mutants` green (6.65 s, 6.66 s).

## Dispositions — the report's Decisions & corrections (implement's, then After implement's)
- Host reads before the layers (QUIET, then CONTENDED with no neighbour) — not a learning: the standing verification-harness.md 2026-11-19 rule applied as written.
- Guard refusals (a heredoc to a scratch script, a `cd` outside the root ×2) — not a learning: each re-issued once in the guard's passing form; host-linux.md's 2026-09-28/29 entries hold the form.
- Sweep hazard (census needle, nextest filter line) — applied (above).
- `rec.py check --no-heal` reads red over the whole store until evidence is landed — rejected, Filter 4: 0.3 (gate failure +0.4 · detail +0.2 · one-off -0.3); a fact of the pipeline tool's plan entry, not of this project's code.
- Exit-drain test-only hold — not a learning: testing.md 2026-09-27 applied as written.
- After implement: the operator pass gave no correction or ruling — nothing to weigh.
- After implement: the directive's deviation closes (block in parts, tenth control `env-ne`, the two companion pins, the `rec.py check` red, the sweep hazard) — the sweep hazard applied; the rest are recorded deviations, not learnings.
- This wrap: two leaf comparer findings carried code figures into leaves (written by hand by constant name) — recurrence of the handoff's deferred «A leaf names a time bound by its constant, never its figure»: `recurrence-despite-learning`, carried on the handoff's Deferred learnings; the remedy is the comparer prompt's (the pipeline's), not a third entry.
