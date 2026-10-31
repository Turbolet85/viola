# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · yes · its path is the secret-scan input the scope names, detail files included, and its gate on scan success keeps a planted-value hit off the upload
ci-integration#1.2 · yes · it is the target/agent-run/* scan input the scope names, with logs and status captured before the scan and chunk.diff skipped
ci-integration#1.3 · yes · the nextest JUnit at target/nextest/ci/junit.xml is a scan input the scope names, so witness failure text must carry no planted value
ci-integration#1.4 · yes · perf-<hook>.json under target/agent-run/ and the perf job's home diagnostics are scan inputs the scope names, uploaded only on scan success
ci-integration#1.5 · no · the OTel trace export row is N/A, with no exporter in v1 and the task's Not needed list keeping exporter choices out
ci-integration#2.1 · no · standing S2 · its disallowed-macros and print/dbg bans and the G1 gate are the S2, S4 and S5 rules every code change owes
ci-integration#2.2 · yes · the scope reads the channel-response line against schemas/diag-line.v1.json, and this row is the in-memory writer test that parses emitted lines against it
ci-integration#2.3 · no · standing S1 · integration-test homes live under target/e2e-home/, while the chaos carve-out and the perf max assertions sit outside the chunk
ci-integration#2.4 · no · standing S6 · the zero-panic G2 gate it names is the S6 rule every test run owes, and its record and gate exits are each new test's own
ci-integration#2.5 · no · the mutation gate runs at the epoch boundary and on demand, not on this chunk's pushes, and the scope names no mutation run
ci-integration#3.1 · no · standing S4 · the G1 gate rejecting an #[instrument] without skip_all is the S4 span rule every change owes
ci-integration#3.2 · no · standing S6 · the G2 zero-panic gate on home-level role files is the S6 rule every test run owes, and its seam exemption sits in an unreached file
ci-integration#3.3 · yes · the scope validates the detail-file legs against schemas/diag-detail.v1.json, which G4 checks on every detail line without printing its content
ci-integration#4.1 · yes · the scope's secret scan after every end-to-end test reads witness homes only because AGENT_RUN_KEEP_HOMES keeps them until G2, G4 and the scan have run
