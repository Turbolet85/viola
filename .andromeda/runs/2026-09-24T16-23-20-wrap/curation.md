CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Three pattern probes that match what they were not aimed at"
    Proof (owed): `grep -c owed` over security-plan.md gave 11 raw hits; a word-bounded read gave 3 real sites (:134, :315, :648). Wrap P1, this run.
    Proof (https): `grep -E '[A-Za-z]:/'` over target/supply-chain/deny.json gave 1 hit, the `s:/` of a crates.io `https://` URL (wrap P1, this run).
    Proof (pathspec): phase P5 control. `git ls-files --cached --others --exclude-standard -- 'crates/viola-ui/**/tsconfig*.json'` printed nothing for a planted `crates/viola-ui/tsconfig.json`; the fnmatch form `'crates/viola-ui/*tsconfig*.json'` listed it (phase run 2026-09-24T15-54-06-phase, baseline/controls.log and the re-run).
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
    - rejected at Filter 4: "a directory-leaf presence grep (`cmd/`) false-matches an unrelated line (`src/cmd/`)". It was a design-time avoidance, not a measured failure: 0.2.
    - not candidates, because this wrap amended each into a master:
      - cargo-modules 0.27.0 `--acyclic` is unusable as a gate (architecture Build system, test-plan §9 Lint row);
      - a shared `target/release/` keeps stale exes (architecture target job 6);
      - cargo-deny resolves the root deny.toml from `--manifest-path fuzz/Cargo.toml` (architecture and security-plan).
  No-other-home: "Three pattern probes that match what they were not aimed at" (each part scored 0.4 measured + 0.2 technical detail = 0.6 exactly, and +0.2 no-other-home = 0.8)
  CLAUDE.md size: 121/200 · T1 1.3 KB, 0 over 600 B
