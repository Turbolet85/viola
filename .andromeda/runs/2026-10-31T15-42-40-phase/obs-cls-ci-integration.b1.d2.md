# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S1 · every integration-test viola home lives under `target/e2e-home/`, which the diag upload covers
ci-integration#1.2 · no · the failure-path agent-run capture is run evidence, not the test outcome the WATCH tally reads (#1.3), and it states no value the chunk moves
ci-integration#1.3 · yes · the JUnit XML records each named test's CI outcome, the record the WATCH tally on `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` reads
ci-integration#1.4 · no · hyperfine perf budgets and hook latency gates, which the chunk's Not needed excludes; no hook path moves
ci-integration#1.5 · no · OTel trace export is N/A in v1, and the chunk adds no exporter
ci-integration#2.1 · no · standing S2 · a new log line goes through `obs_event!`, never a raw tracing macro
ci-integration#2.2 · yes · the in-memory writer tests parse emitted lines against `schemas/diag-line.v1.json`, which the chunk edits with the new `process-exit` detail values
ci-integration#2.3 · no · standing S1 · every integration-test viola home lives under `target/e2e-home/`
ci-integration#2.4 · no · standing S6 · zero tolerance on `event:"panic"` lines, secret-scan hits and schema-conformance failures across test homes
ci-integration#2.5 · yes · the chunk adds obs_event! call sites for new process-log lines that its exit, `--json` and hint asserts do not read, so a missed obs-code mutant there is red at the epoch run
ci-integration#3.1 · no · standing S9 · a span added on a new path is `#[instrument(skip_all, ...)]`
ci-integration#3.2 · no · standing S6 · zero tolerance on `event:"panic"` lines across test homes
ci-integration#3.3 · yes · G4 checks every home-level line against `schemas/diag-line.v1.json`, which the chunk edits with new `process-exit` detail values that must conform
ci-integration#4.1 · no · the step order, artifact retention and keep-homes setting are CI workflow that the chunk's Not needed excludes (it edits no workflow)
ci-integration#5.1 · no · the v1-47 secret-scan timing is CI workflow order, which the chunk's Not needed excludes; the open decision is not one this chunk settles
