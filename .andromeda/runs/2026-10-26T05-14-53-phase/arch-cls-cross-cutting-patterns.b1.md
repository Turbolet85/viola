# Classified — arch · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · the bumped tokio under its enabled features must add no environment read, since a dependency or feature reading one breaks the no-env-config rule like a direct read
cross-cutting-patterns#2.1 · no · the chunk adds no new dependence on an undocumented claude behaviour and touches no ledger row or probe (the task's Not needed names this decision)
