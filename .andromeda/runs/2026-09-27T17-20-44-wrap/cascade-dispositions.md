# Cascade step-2 dispositions — 2026-09-27-epoch-2-cleanup wrap

Search: `cascade.py sweep` over `cascade-patterns.toml` (12 patterns; the listing is `cascade-sweep.txt`, baseline `a0e6506`, the
parent of the one pre-CI commit `f0e6dbc`). Every pattern's known-positive control fired on the pre-pass masters. Sections read:
architecture §Occupied Resources (Workspace crates, Env vars, Filesystem, Repository), §Project directory structure, §CI/CD
approach (job 6, the pre-push bullet); test-plan §2, §3 (Exit codes, `run`, `gate` Inputs, `pre-push`, Closed enums, Test data
bootstrap), §4 Test grouping, §9 Release rows, §10 Mutation gate, §12; security-plan §Secret Management :426, Decisions Log
2026-09-26/27; obs-plan §8 item 6.

Patterns and the claims they retire or qualify: `3/3 refused` (probe count) · `Read only by the root test chain` (KEEP_FAILED reader)
· stop rust-analyzer (operator pass step) · `mutants.out/outcomes.json` (unconditional repo-root location) · release-check judging
executables only · raw `cargo mutants --file` (the one-file selector) · `AGENT_RUN_KEEP_FAILED` (readers) · scratch/temp copies
(where cargo-mutants copies) · rstest `#[case]` (the table form) · `test-only-rust-delta` (the closed verdict list) · every WSL call
(the launcher set) · `unknown-suite` (the usage detail list).

## Master rows
| row | disposition |
|---|---|
| test-plan :441, :556, :557, :565, :650, :1509 (`mutout-json`, edited) | amended this pass (the location qualified) |
| test-plan :1795, :1821 (`mutout-json`) | no change — §12 Decisions Log history (past entries, true when written) |
| architecture :496, :550 (`rc-exec-only`, edited) | amended |
| security-plan :689 (`rc-exec-only`) | no change — 2026-09-26 Decisions Log Condition (history); the new 2026-09-27 entry records it now executable |
| architecture :369, :552; test-plan :555, :748 (`keep-failed`, edited) | amended / still true (`run --mutants` forces `0`) |
| test-plan :1921 (`keep-failed`, new) | this pass's §12 entry |
| architecture :383, :408, :552; security-plan :698; test-plan :1923 (`temp-copies`, new/edited) | this pass's text |
| test-plan :214, :1744 (`temp-copies`) | no change — a test's `--home` temp dir / the runner temp dir, not cargo-mutants |
| test-plan :664 (`temp-copies` ×2) | no change — the Linux leg's distro temp dir claim, still true |
| test-plan :555 (`temp-copies`, edited ×3) | amended; "the scratch copy carries those bins" still true |
| test-plan :832 (`rstest-case`, edited) | amended (the viola-e2e exception) |
| test-plan :961, :964, :1275 (`rstest-case`) | no change — root-package rstest tables, true |
| test-plan :556, :566, :669, :1509 (`verdict-list`, edited) | amended (`scoped` added where the list is closed: :669; :566 gains the scoped object) |
| test-plan :1857, :1859 (`verdict-list`) | no change — §12 history of the test-only verdict's own entry |
| security-plan :426 (`every-wsl`, edited) | amended (the second launcher + its invariant) |
| security-plan :686 (`every-wsl`) | no change — the 2026-09-26 entry about the pre-push gate, still true for the gate |
| test-plan :509, :670 (`usage-detail`, edited) | amended (`scoped-leg`) |
| test-plan :646, :1815 (`usage-detail`) | no change — `gate` usage / §12 history |
| `keepfail-only`, `stop-ra`, `probe-3of3`, `raw-mut-file`: 0 master rows after the pass | the retired wording is gone from all seven masters |

## Leaf rows (→ step 3 re-derivation)
- `.claude/docs/commands.md` :33 (every WSL call, the pre-push context — true; the recompute adds wsl-exec.sh), :43 (stale-outcomes
  location), :44 (raw `cargo mutants --file`), :73 (`3/3`).
- `.claude/docs/workflow.md` :11 (stop rust-analyzer), :47 (`mutants.out/outcomes.json` locally).
- `.claude/docs/conventions.md` :13 (KEEP_FAILED readers).
- `.claude/docs/tests-summary.md` :20 (KEEP_FAILED), :42 (the verdict list).
- `.claude/docs/security-summary.md` :66 (every WSL call).
- `.claude/docs/services/viola-core.md` :38 (raw `cargo mutants --file`).
- `.claude/rules/testing.md` :48 (outcomes.json, the verdict list).
- `.claude/rules/verification-harness.md` :24 (pre-push scratch), :43, :44 (KEEP_FAILED; `run --mutants` body).
- `.claude/rules/security.md` :32 (every WSL call).
- `.claude/rules/host-win32.md` :10 — no change (Bash `/tmp` vs the Windows temp dir, unrelated).
- Curation homes: 0 rows. Judgment bases: 0 rows.
