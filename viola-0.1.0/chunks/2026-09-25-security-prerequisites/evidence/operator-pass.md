# Operator pass — 2026-09-25-security-prerequisites

Driven by the session on the operator's word (overseer, 2026-09-25): "go for the operator pass. Entry 25: the pre-CI
commit + a plain push on top of 874420d (no force …). Then entries 26 and 27: poll the run in the background and record
them in evidence."

## Entry 25 — pre-CI commit + clean-tree-guarded push
- Commit: `8e25ca70f6232eb3e55bf4710d86814fb2c9e865` — `chore(2026-09-25-security-prerequisites): operator pre-CI commit`,
  parent `874420dc617668ea7c5a7155589d9bf4b954c056` (the wrap sha of 2026-09-24-epoch-1-cleanup).
- Run form: `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → exit 0,
  `874420d..8e25ca7  build/viola-0.1.0 -> build/viola-0.1.0` (fast-forward, no force; CI `event.before` = 874420d).
- CI run on the pushed sha: **36138441784** (workflow `ci`, event `push`).
- Commit-time note: git warned that six captured logs under `.andromeda/runs/` carried CRLF and are stored LF (the repo's
  `.gitattributes` pin); no source file was affected.

## Entries 26 and 27 — CI reads on the pushed sha
Background poll (`.andromeda/runs/2026-09-25T12-41-13-implement/ci/`): 15 check-runs, the last completed at poll 9,
2026-09-25T13:10:25Z. Both entries were then fired once in their exact `run` form (`$(git rev-parse HEAD)` = 8e25ca7).

| entry | reading | expect | verdict |
|---|---|---|---|
| 26 — every conclusion, unique | `success`, exit 0 (`ci/26.log`) | `exit 0`, `last line success` | **green** |
| 27 — the mutants jobs | `success,success,success`, exit 0 (`ci/27.log`) | `exit 0`, `last line success,success,success` | **green** |

Run **36138441784**: all 15 jobs `success`:
- lint ×3, test ×3, release ×3;
- mutants ubuntu-latest, mutants windows-2025, mutants-verdict;
- msrv, supply-chain, fuzz-replay.

Supporting reads from the job logs:
- **`test (windows-2025)`** (job 108082084137, `ci/test-windows.log`) ran the witness, not a skip:
  - PASS `channel_sqos_open sqos_identification_open_adopted_reads_identification`;
  - PASS `sqos_flags_absent_open_reads_impersonation`;
  - PASS on all 5 `contract_content_hash` cases.
- **`mutants (ubuntu-latest)`** (job 108082084196): `9 mutants tested in 78s: 9 caught`, `{"tested":9,"verdict":"counted","leg":"ubuntu-latest"}`.
- **`mutants (windows-2025)`** (job 108082084334): `9 mutants tested in 2m: 9 caught`, `{"tested":9,"verdict":"counted","leg":"windows-2025"}`.
