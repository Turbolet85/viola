# Classified — tests · test-strategy.b2

## Answers
test-strategy#3.17 · no · the mutation gate is outside the chunk (Not needed: the plan holds no chunk gate for mutation, and a mutation run is the boundary audit's), so no test of this kind is added or changed
test-strategy#3.18 · yes · the no-flakes half binds: the chunk tallies the verify_stopped_during_the_resume_run watch's 0/3 green CI reads, and a flake there keeps the chunk red until its root cause is fixed
