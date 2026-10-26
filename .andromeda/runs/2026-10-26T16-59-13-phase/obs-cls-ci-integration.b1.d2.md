# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S4 · every new integration-test home lives under target/e2e-home/ so G2, G4, the secret scan and the diag-<os> upload cover them, a convention the task lists
ci-integration#1.2 · no · the harness artifact's capture order and its own upload are CI mechanics the chunk does not change (Not needed: CI artifact retention and upload step order)
ci-integration#1.3 · no · the JUnit XML upload is CI mechanics the chunk does not change (Not needed: CI artifact retention and upload step order)
ci-integration#1.4 · yes · the per-hook hyperfine exports and their .results[0].max read are the budget witness the strict-modes check on every hook path must stay inside (Needs: hook perf budgets)
ci-integration#1.5 · no · there is no OTel exporter in v1 and the chunk changes no export (Not needed: OTel SDK, OTLP exporter)
ci-integration#2.1 · no · standing S1 · every new log line goes through obs_event! under #[instrument(skip_all, fields(..))] with no raw tracing macros, a convention the task lists
ci-integration#2.2 · no · standing S2 · every new field or detail code validates against schemas/diag-line.v1.json (G4, default-deny), a convention the task lists
ci-integration#2.3 · yes · its perf row, hyperfine JSON with max assertions and budget exit codes, is the hook-budget witness the strict-modes check on every hook path must stay inside (Needs: hook perf budgets); its home rule is standing S4
ci-integration#2.4 · no · standing S5 · zero unlogged panics over home-level role files (G2), the E2E gate this row names, a convention the task lists
ci-integration#2.5 · no · the mutation gate runs only at the epoch boundary and on demand, with no push or pull-request job, so this chunk owes no mutant kill
ci-integration#3.1 · no · standing S1 · every new log line goes through obs_event! under #[instrument(skip_all, fields(..))] with no raw tracing macros, a convention the task lists
ci-integration#3.2 · no · standing S5 · zero unlogged panics over home-level role files (G2), no retry-once policy, a convention the task lists
ci-integration#3.3 · no · standing S2 · every new field or detail code validates against schemas/diag-line.v1.json (G4, default-deny), a convention the task lists
ci-integration#4.1 · yes · the perf job's last step, gate --require perf, is the hook-budget witness the strict-modes check on every hook path must stay inside (Needs: hook perf budgets)
