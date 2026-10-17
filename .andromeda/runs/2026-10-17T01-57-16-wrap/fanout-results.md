# Fan-out results — 2026-10-17-end-to-end-path-binary-and-harness-legs

Path check: before `tree 25575658d634dc6249a528d68a3b109103c8e41c index 621cc3eaaecf3fb05f5d3f0c123a7c4e994bbc9e head
ae2137622b3a1151d235cd80304c461e3b529084`; after `tree 0510b46a1ce096f2bb3de0701d9eee64d29a9f11`, index and head
unchanged; `git diff-tree` printed exactly the seven `A …/fanout-{doc}.md` lines. No file needed a fix (no twin); every
receipt's count equals its file's. Every coordinate a proposal carries is the report's or a `## New text, by line` row
(`session.rs` 107-122 and 30-34, `boot.rs` 47-56, `.config/nextest.toml` 24): 0 rejected for re-derivation.

architecture: 3 proposals · fanout-architecture.md
  note: D-arch-decisions found no drift (no dependency, runtime or crate added; the architecture :299 `agents-mode` site holds).
  1. apply — check 1 routine (Accurate this-chunk addition): §Occupied Resources → Environment variables, `AGENT_RUN_KEEP_FAILED` gains the `harness::session` guard as a reader.
  2. apply — check 1 routine (dependent of 1): §Occupied Resources → Environment variables, `AGENT_RUN_KEEP_HOMES` read by the `harness::session` guard too.
  3. apply — check 1 routine: §Occupied Resources → Filesystem, `target/agent-run/<session>/bin/agents-mode` and `agents_mode` in `session.json` / `supervise.json`.
security-plan: 0 proposals · fanout-security-plan.md
  note: D-security-input clean (the harness `status` read bounded by `take(MAX_FRAME + 1)` + object check; clap value enum and shell-plain check); auth and deps clean.
design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens holds — every new surface reads `tokens n/a`.
layout-templates: 0 proposals · fanout-layout-templates.md
  note: D-layout-surface holds — no user-facing viola surface added; the `driver - budget-paused` cell is already in the `viola list` wireframe.
test-plan: 17 proposals · fanout-test-plan.md
  note: no obs-plan-side drift (heartbeat ticks keep the closed `state` / `last_error` enum); `oversize` covered by the derivation unit only.
  1. apply — check 1 routine; check 6 disposes the parse-error-qualifier claim: §3 → 5-command implementation, `run` step 1 integration layer `kind(test) & !binary(/^(path[0-9]*|…)_/)`, the until-first-binary clause retires, the parse-error rule re-measured on 0.9.146.
  2. apply — check 1 routine on the operator's recorded direction `inputs#I2` (delegate, PROVISIONAL); check 6 disposes the regex claim: §3 → 5-command implementation, `run` step 2 E2E filterset widened, suite `nextest-e2e` third, `run --e2e` alone no doctest.
  3. apply — check 1 routine: §3 → 5-command implementation, the `--e2e` usage-error clause retires.
  4. apply — check 1 routine: §3 Exit codes, `run --e2e` leaves the not-built example.
  5. apply — check 1 routine (`inputs#I2`): §2 test locations, the numbered `path<N>_` binaries in both homes.
  6. apply, re-derived — check 1 routine (A count in a master that carries no rule does not apply: the master states its rule, "the `binary(…)` names on that filter line"): §3 → Bootstrap phases, the verify-driven list re-derived from `.config/nextest.toml`'s filter line at this wrap — twenty names (`cross_list_rows`, on the line before this chunk and absent from the master's eighteen, and `path1_run_start`), not the proposal's "eighteen → nineteen".
  7. apply — check 1 routine; check 6 disposes the agents-mode write-order claim: §3 → 5-command implementation, `boot`'s two flags built, the file written after the stamp.
  8. apply — check 1 routine: §6 Path 6 step 1, the not-built sentence retires.
  9. apply — check 1 routine: §6 Path 6 As landed, step 7's two-instance leg landed.
  10. apply — check 1 routine (fork 2, `inputs#I1`, PROVISIONAL): §3 → 5-command implementation, `harness_session` a plain function and guard, the spawned `supervise` stays.
  11. apply — check 1 routine: §3 → 5-command implementation, "one of two fixtures" → "one of two boot models".
  12. apply — check 1 routine: §3 → 5-command implementation, `status` runs `viola list` with `VIOLA_*` removed.
  13. apply — check 1 routine: §3 → Status endpoint shape, the interim retires, the derivation stated.
  14. apply — check 1 routine: §3 → Status endpoint shape, the example's `last_error` names `list-exit-none`.
  15. apply — check 1 routine: §3 → 5-command implementation, Closed enums `status.last_error` gains `list-exit-none`.
  16. apply — check 1 routine: §3 Session record, `agents_mode` in `session.json` / `supervise.json`.
  17. apply — check 1 routine (fork 1, `inputs#I1`, PROVISIONAL): §6 Path 1, As landed `tests/path1_run_start.rs`.
obs-plan: 0 proposals · fanout-obs-plan.md
  note: all three detectors hold; `status`'s `viola list` writes no role line (no `VIOLA_NAME`).
a11y-plan: 0 proposals · fanout-a11y-plan.md
  note: neither detector applies; the plan's a11y-plan expected amendment is already the body's text.

## Validate — the other checks
- Check 2 cross-contradiction: none (the test-plan proposals edit disjoint clauses; arch's three are disjoint registry rows).
- Check 3 intent-consistency: the report's deviations are justified (1 by `inputs#I2`; 2–8 in-intent details); scope record none, `gate.py scope` clean.
- Check 4 absence-needs-evidence: the a11y and obs "no edit owed" notes cite their sites (a11y-plan :105, :340 and `a11y-testing-tool-pick.md:4` read here: each says "`nextest-integration` / `nextest-e2e` (locally)", which holds for the complement layers).
- Check 5 expected amendments: every entry of the plan's list is matched — run step 1 (test-plan 1), boot flags (7), `harness_session` (10, 11), `status` (12), Status endpoint shape (13, 14), Closed enums (15), Path 1 (17), Path 6 (8, 9), §2 / run step 2 (2, 5); a11y-plan §3 / §9 — not carried: the body already says it (check 4 read), no edit; the `inputs#I2` regex owed by the operator — test-plan 1, 2, 5, its testing.md copy a leaf (cascade step 3), no a11y-plan site holds the regex. Citation rows dispositioned `claim false`: none.
- Check 6 disproved claims: the regex (test-plan 1, 2, 5) · the 0.9.133 qualifier (test-plan 1) · the agents-mode write order (test-plan 7) — all DISPOSED.
- Escalations: 0.
