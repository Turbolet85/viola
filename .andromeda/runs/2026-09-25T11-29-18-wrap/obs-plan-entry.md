
## 2026-09-24-epoch-1-cleanup — §9 Mutation row names the per-leg unviable rule
**Section:** §9 Pipeline integration → the Mutation row
**Change:** after "a mutant is red only when no leg caught it", the row adds that before that union a leg whose unviable mutants outnumber its caught ones is red at its own run (test-plan §10 Mutation gate).
**Why:** a cross-master citation of test-plan's mutation verdict, which this chunk amended (test-plan sidecar, same marker). Flagged out of detector scope by the obs-plan doc-agent and folded by the cascade step-2 sweep. obs-plan §1 (the verbatim obs-scope copy) was not touched: the new playbook rule "Verbatim scope copy", appended by this wrap, keeps it out of every sweep.
**Sweep:** the same pass pattern (`sweep.txt`) found 1 obs-plan hit, `:1253`, amended. The streaming change adds no obs event, field or sink. Its lines carry repo-relative names and durations only, and `mutants.out/` stays un-uploaded (obs-plan §8), so no other obs section moves.
