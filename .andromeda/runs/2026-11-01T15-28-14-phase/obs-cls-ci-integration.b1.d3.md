# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S9 · new integration-test viola homes live under target/e2e-home/, which the diag-<os> upload covers
ci-integration#1.2 · no · the harness logs and status capture on failure is CI-wide; the scope adds no CI job or upload, and artifact names are not moved
ci-integration#1.3 · no · the junit upload is CI-wide; the scope adds no test job or upload, and artifact names are not moved
ci-integration#1.4 · no · the per-hook perf sample upload is CI-wide; the chunk adds no perf row and states no hook budget value it reads
ci-integration#1.5 · no · no OTel exporter in v1; the chunk adds no exporter
ci-integration#2.1 · no · standing S1 · raw tracing macros are banned by clippy disallowed-macros, and the same stage enforces S5, S7 and S11, each owed by every change
ci-integration#2.2 · yes · the chunk's new home-level lines (process-exit at WARN with during:"connect", the new refusal detail codes, hook-decision fail-open) need in-memory unit tests that parse them against schemas/diag-line.v1.json, which the Needs name
ci-integration#2.3 · no · standing S9 · new integration-test viola homes live under target/e2e-home/ so G2, G4, the secret scan and the diag-<os> upload cover them
ci-integration#2.4 · yes · the chunk's E2E run (run start refusals, wait/last connect refusals, cleanup measurement) reads agent-run logs and status, and its home-level lines must read zero panics under G2 and conform under G4
ci-integration#2.5 · no · the cargo-mutants gate on obs code is a check every obs-code change owes equally (test oracles), and the scope adds no mutant job or score
ci-integration#3.1 · no · standing S5 · a new span takes a static name with skip_all first, checked by gate G1
ci-integration#3.2 · yes · the chunk's new refusal paths and its E2E runs must emit no panic line outside the seam exemption, which the chunk does not touch; G2 scores every home-level role file
ci-integration#3.3 · yes · the Needs name schemas/diag-line.v1.json: every new detail value or field on a home-level line must be admitted there, or the G4 conformance gate fails
ci-integration#4.1 · no · standing S9 · a new test's viola home is kept until the gate steps have read it, a rule each new integration home owes equally; the chunk's cleanup measurement re-measures endpoint_gone, not home retention
ci-integration#5.1 · no · the secret-scan timing after each job's E2E run is CI-wide; the chunk's new line content is governed by S3, not by the scan's timing
