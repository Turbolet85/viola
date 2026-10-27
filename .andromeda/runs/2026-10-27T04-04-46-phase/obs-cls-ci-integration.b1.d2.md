# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S4 · every new integration-test home lives under target/e2e-home/, so the diag-<os> upload covers it
ci-integration#1.2 · no · the chunk adds no CI job or upload, and the harness logs and status artifact is CI's own record
ci-integration#1.3 · no · the chunk adds no CI job or upload, and the nextest JUnit artifact is CI's own record
ci-integration#1.4 · no · the chunk adds no CI job or upload and re-measures no perf; the hyperfine samples and their upload are CI's own record
ci-integration#1.5 · no · no OTel exporter in v1, and the chunk adds no exporter or reporter
ci-integration#2.1 · no · standing S1 · every new log line goes through obs_event! under #[instrument(skip_all, fields(..))], and raw tracing and print macros are banned
ci-integration#2.2 · no · standing S5 · each new test meets the zero-schema-failure budget, which its in-memory writer parse checks
ci-integration#2.3 · yes · the tests/chaos_feed_panic.rs outside_scan() carve-out binds: the forced vt100 panic witness runs in that home, asserting its panic and parse-rejected lines; the target/e2e-home rule is S4
ci-integration#2.4 · no · standing S5 · each new test meets the zero-unlogged-panics and zero-schema-failure budgets, and the chunk adds no E2E test
ci-integration#2.5 · no · the mutation gate runs at the epoch boundary and on demand, not per change, and the chunk names no mutant or score
ci-integration#3.1 · no · standing S1 · every new #[instrument] is skip_all-first, the form G1 checks
ci-integration#3.2 · yes · a forced vt100 panic still fires the panic hook's event:"panic" line, so G2's scope and seam-only exemption decide the witness, which must run in the chaos home outside target/e2e-home/
ci-integration#3.3 · yes · G4 checks the real output of the G2 scope and the detail files, so the chunk's new home-level and detail lines must conform, and any new value joins the schema's default-deny (S2)
ci-integration#4.1 · yes · the viola-chaos-* carve-out binds: the forced vt100 panic witness runs in tests/chaos_feed_panic.rs's home, outside G2 and G4 by the founder's ruling; the job order and kept homes need no chunk change
