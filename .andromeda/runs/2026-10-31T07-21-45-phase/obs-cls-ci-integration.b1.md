# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the scan-gated diag upload of home diagnostics and detail files under target/e2e-home is a secret-scan input for the detail-file leg of v1-47
ci-integration#1.2 · yes · agent-run logs and status under target/agent-run are secret-scan inputs the scan covers before the harness upload
ci-integration#1.3 · yes · junit.xml is a secret-scan input, and its failing test IDs joined to diag lines are the Path 1 timeout tally the chunk re-reads
ci-integration#1.4 · no · hyperfine perf rows and their budget; the chunk writes no perf JSON and no rule of that job changes
ci-integration#1.5 · no · OTel trace export is N/A in v1; the chunk adds no exporter
ci-integration#2.1 · no · standing S2 · every log line goes through obs_event!, never a raw tracing macro; print and dbg! banned outside output modules, a convention the task lists
ci-integration#2.2 · yes · in-memory writer tests parse the channel-response, process-exit and parse-rejected lines against diag-line.v1.json, the contract read the scope names
ci-integration#2.3 · no · standing S1 · every integration-test home lives under target/e2e-home, so G2, G4, the secret scan and the diag upload cover it, a convention the task lists
ci-integration#2.4 · no · standing S6 · a test run leaves zero event:"panic" lines in home-level role files and uses no retry-once policy, a convention the task lists
ci-integration#2.5 · yes · the chunk's new obs_event! sites (report_internal_error, process-exit, channel-response) are obs code the cargo-mutants run scores; a missed or timed-out mutant reds that run
ci-integration#3.1 · no · standing S4 · every span is #[instrument(skip_all, fields(...))] or info_span! with an allow-list, a convention the task lists
ci-integration#3.2 · no · standing S6 · a test run leaves zero event:"panic" lines in home-level role files and uses no retry-once policy, a convention the task lists
ci-integration#3.3 · yes · G4 checks each detail line against diag-detail.v1.json and each home line against diag-line.v1.json, the schema check the chain and drift_report fields must pass
ci-integration#4.1 · yes · the order of G2, G4 and the secret scan after the E2E tests, with homes kept until they read them, is the scan-after-every-E2E-test leg the chunk re-reads
