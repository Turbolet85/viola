# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the chunk adds no upload or job (Not needed: CI artifact retention and upload step order); its owner-only diagnostics reach this scan-gated upload unchanged
ci-integration#1.2 · no · the chunk adds no upload or job (Not needed: CI artifact retention and upload step order); the agent-run logs and status capture is not reached
ci-integration#1.3 · no · the chunk adds no upload or job (Not needed: CI artifact retention and upload step order); its new tests need no JUnit rule
ci-integration#1.4 · yes · the per-hook hyperfine rows from run --perf (session-start, user-prompt-submit, stop, session-end, pre-tool-use) and their max bind: the strict-modes check on every hook path counts inside the budgets; the upload clause is not reached
ci-integration#1.5 · no · no OTel exporter in v1; the OTel SDK and OTLP pipeline are Not needed (nothing of the pipeline changes)
ci-integration#2.1 · no · standing S1 · every new log line goes through obs_event!, no raw tracing macros
ci-integration#2.2 · no · standing S2 · every new field or detail code validates against diag-line.v1.json; the in-memory writer oracle is what every new test keeps
ci-integration#2.3 · yes · the perf clause binds: the hyperfine max assertions through gate --require perf must hold on every hook path the strict-modes check lengthens; the homes clause is standing S4 and not carried
ci-integration#2.4 · yes · the E2E runs' logs are read by the zero-panic gate G2 and the schema gate G4, which the chunk's new refusal and process-exit lines and check path must satisfy
ci-integration#2.5 · no · the cargo-mutants run is the epoch-boundary or on-demand audit (no push or PR job); the scope names no mutant obligation, and every obs change owes it equally
ci-integration#3.1 · no · standing S1 · a new log line goes through obs_event! under #[instrument(skip_all, fields(..))]
ci-integration#3.2 · yes · G2's zero-panic rule binds the check path: no uncaught unwind before the first role file opens, in the pre-step-4 window where the panic hook writes nothing
ci-integration#3.3 · yes · G4 checks every line the chunk adds in the G2 scope (process-exit at WARN with its closed detail, hook-decision fail-open) against diag-line.v1.json; the detail codes must be in the §6 catalog
ci-integration#4.1 · no · standing S4 · every new integration-test home lives under target/e2e-home/ so G2, G4 and the scan cover it
