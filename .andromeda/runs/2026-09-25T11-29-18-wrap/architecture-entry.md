
## 2026-09-24-epoch-1-cleanup — the mutation union is no longer the only red path
**Section:** §Infrastructure Patterns → CI/CD approach (jobs wired today, the `mutants-verdict` sentence)
**Change:** after "a mutant is red only when no leg caught it and some leg missed it or timed out", the body adds that before that union a leg is red at its own run when its unviable mutants outnumber its caught ones (test-plan §3 `run` step 4, §10 Mutation gate).
**Why:** a cross-master citation of test-plan's mutation verdict, which this chunk amended (test-plan sidecar, same marker). Flagged out of detector scope by the arch doc-agent and folded by the cascade step-2 sweep; the report's Harness/gate surface carries the rule.
**Sweep:** the same pass pattern (`sweep.txt`) found 1 architecture hit, `:515`, amended. There is no other architecture hit: `grep -c` of the union sentence = 1 before the edit.
