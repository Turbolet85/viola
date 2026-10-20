CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A test that ends a served duplex session (`tokio::io::split` over one stream) calls `shutdown()` on its write half" (confidence 0.8)
                                              + verification-harness.md: "`target/debug/viola` is a hard link cargo re-points per feature set … read the build's hash right before the start, and restore a ledgered build by `ln -f`" (confidence 0.8)
  Corrected (exempt from the cap):            testing.md 2026-09-25 "Keep every `#[cfg(test)]` module inline" — in place, `[corrected 2026-10-20: …]`
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 1 deferred (→ handoff)
  No-other-home: "a live rig gives the prompt on stdin past a variadic `--mcp-config` and files the CLI's stderr privately" · "a served duplex test calls `shutdown()` on its write half" · "`target/debug/viola` is re-pointed per feature set"
  Extended: T2/verification-harness.md: "2026-10-10: On the Linux dev host the bare name `claude` can resolve to a newer CLI" + "`--mcp-config` is variadic: the prompt on stdin, the CLI's stderr filed privately"
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B

## Applied
- testing.md (new): the `tokio::io::split` EOF hazard — +0.4 verified by measurement (two hung runs at implement, 600 s and
  300 s, terminated; the report's Deviations, step 13) · +0.2 specific detail · +0.2 no other home (no master, route
  annotation or contract states it) = 0.8.
  Proof: report.md *Decisions & corrections* "A test-writing hazard" and Deviations "Step 13 — the served-duplex case".
- verification-harness.md (new): the `target/debug/viola` hard-link swap — +0.4 verified (rig setup read
  `e8d82882501399d5`, the start `5a63d1c72218cf00`; the two names relinked by `ln -f` and re-hashed equal to the ledger)
  · +0.2 detail · +0.2 no other home = 0.8.
  Proof: report.md Deviations "Step 18 — two starts" and *Decisions & corrections* "A build-identity hazard".
- verification-harness.md (extended, 2026-10-10 entry): `--mcp-config` variadic → the prompt on stdin; the CLI's stderr
  filed privately — +0.4 verified (`claude --help` 2.1.302 reads `--mcp-config <configs...>`; start 1 in the plan's argv
  exited 1 in 252 ms with no server and its capped failure text was lost; start 2 with the prompt on stdin recorded the
  exchange) · +0.2 detail · +0.2 no other home = 0.8.
  Proof: report.md Cross-project bullet (`--help`), Spec claims disproved (plan step 18), *Decisions & corrections* "A
  measured CLI fact … variadic" and "A rig rule".

## Corrected
- testing.md 2026-09-25 (in place, `## Session Additions`): "so moving tests out of a file never shrinks it" was false —
  cargo-modules 0.27.0 passes an out-of-line `#[cfg(test)]` file whose stem begins `test_`, and this chunk shrank
  `ledger.rs` into `ledger/test_cases.rs` under `#[path]`.
  Proof: report.md Cross-project bullet (the probe crates: `test_support` / `test_helpers` rc 0; `tests`, `testing`,
  `support`, `foo`, `unit` rc 1; `#[path = "a/test_cases.rs"] mod tests;` rc 0) and gate entry 7 (red on
  `ledger/tests.rs`, green on the `test_cases.rs` form).

## Dispositions of *Decisions & corrections*
- The founder's own word on the item-(7) widening — not a learning: a ratification, recorded in the owning sidecars (P2).
- The overseer's technical words (Q2, Q3) — not a learning: a decision for this chunk (scope record, gate entry 20).
- The operator's words at implement and at this wrap — not a learning: directions for this chunk.
- The measured 2.1.302 MCP opening (`server/discover`, no `initialize`) — not curated: its home is the master
  (architecture §Standard Contracts, amended at P2) and its leaves.
- `--mcp-config` is variadic — extended (verification-harness.md 2026-10-10 entry), merged with the rig rule below.
- The rig rule (file the CLI's stderr) — extended, with the line above.
- cargo-modules skips only a `test_` stem — corrected in place (testing.md 2026-09-25).
- `is_windows_gate` reads `cfg!(windows)` too — → handoff (Filter 5 cap; 0.8, the fourth slot).
- `tokio::io::split` EOF — applied (testing.md).
- The guard's `--allow` regex matches a path whole — not a project learning: the pipeline's (gate.py guard's grammar and
  phase P5's controls); the evolve stream's.
- `target/debug/viola` re-pointed per feature set — applied (verification-harness.md).
- A hosted macOS job cancelled with 0 steps — rejected, Filter 1 duplicate of ci.md 2026-10-13 ("A CI job that reads
  `cancelled` with no steps and no runner name never ran").
- The phase run's P5 records carried `/tmp/<segment>` paths — not a project learning: the phase skill's authoring form.
- Bash guard refusals — not a learning: telemetry.
- The recurrence of testing.md 2026-09-25 — → handoff as `recurrence-despite-learning` (the plan took the stale premise
  scope.md read on one `test_support` file); the entry itself is corrected above.
