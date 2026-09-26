# Local windows mutation leg — 2026-09-26 (implement)

`bash scripts/agent-run.sh run --mutants` (plan entry 7), NO `AGENT_RUN_CHUNK_BASE`: the harness derived the base.
Counts are read from `mutants.out/outcomes.json`. The file itself is not kept here: it carries absolute argv paths and
test output (security-plan §Bootstrap phases `secret-scanning-ci-gate`).

| run | tree | base | tested | caught | missed | timeout | unviable | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 (full block) | P1 tree | — | 0 | — | — | — | — | red: `move mutants.out → mutants.out.old: Access is denied (os error 5)` — rust-analyzer (the session LSP) held `mutants.out`; stopped by exact ExecutablePath, re-run |
| 2 (`--only 7,8,9`) | P1 tree | fcca1ce | 81 | 73 | 1 | 0 | 7 | red: `mutants.rs:30:5: replace commit_exists -> bool with true` MISSED — `commit_exists` had lost its last production caller to `commit_sha`; removed with its re-export |
| 3 (`--only 7,8,9`) | final | fcca1ce | 79 | 73 | 0 | 0 | 6 | green |
| 4 (full block) | final | `fcca1cebef44778f7475b61535bded1484218289` | 79 | 71 | 0 | 0 | 8 | green; block 14/14 green |

The base line is the run document's own `"base"` field (plan step 2). It equals the last master flip, as step 1b predicted:
HEAD's tree holds no pending record yet, so the bound is HEAD.

rust-analyzer restarted between runs (the LSP restarts after edits) and was stopped again before runs 3 and 4.
The ubuntu leg and the union are CI's (the operator entries 16–17).
