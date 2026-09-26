# Operator pass — 2026-09-26-ci-chunk-base-and-union-verdict

Driven on the operator's (overseer's) word, 2026-09-26, V17 shape: one pre-CI commit, plain fast-forward push, no fix
commit needed.

## Commits
| sha | subject | pushed | CI run |
|---|---|---|---|
| `acd08c718c6b25a6e9ea1d0a292e671bf9e5fe6a` | chore(2026-09-26-ci-chunk-base-and-union-verdict): operator pre-CI commit, for the run this chunk's verdict reads | `e486544..acd08c7` (entry 15, clean-tree guard, exit 0) | **36270173848** |

Before the push, the derived base was re-read by hand at the pre-CI commit: the oldest `chore(2026-09-26-ci-chunk-base-and-union-verdict): operator pre-CI commit` in HEAD's history is `acd08c7`; its parent is `e486544`; `git log -1 -G ' · complete · ' acd08c7^ -- .andromeda/master-route.md` → `fcca1ce`.

## Run 36270173848 (acd08c7) — completed `success`
- **Entry 16** (`…/commits/acd08c7…/check-runs`, unique conclusions): `success` — exit 0; atom `last line success` holds.
  All 15 checks green: test ×3, lint ×3, release ×3, mutants ×2, mutants-verdict, fuzz-replay, supply-chain, msrv.
- **Entry 17** (mutants check-runs, in check-run order): `success,success,success` — exit 0; atom holds.
- **Mutation legs**, the run document each leg's job log prints (job logs read with `gh api …/actions/jobs/{id}/logs`):

  | leg | job | `base` | tested | caught | survived | unviable | verdict |
  |---|---|---|---|---|---|---|---|
  | ubuntu-latest | 108482499487 | `fcca1cebef44778f7475b61535bded1484218289` | 79 | 74 | 0 | 5 | counted, ok:true |
  | windows-2025 | 108482499511 | `fcca1cebef44778f7475b61535bded1484218289` | 79 | 74 | 0 | 5 | counted, ok:true |

  Both legs read the SAME base, the last master flip `fcca1ce`. With no `AGENT_RUN_CHUNK_BASE` in `ci.yml`, the harness
  derived it, and the diff was the whole chunk (79 mutants, the same count as the local windows leg).
  Unviable is the tested count minus caught and survived.
- **Union** (`mutants-verdict`, job 108485375727): `{"v":1,"cmd":"gate","ok":true,"breaches":[]}`.

## Summary
| run | sha | entry 16 | entry 17 | base (both legs) | note |
|---|---|---|---|---|---|
| 36270173848 | acd08c7 | success | success,success,success | fcca1ce | the final HEAD's run; no fix commit |
