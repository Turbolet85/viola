# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diag-<os> upload path is unchanged; the chunk adds no artifact or upload, so its new home-level lines ride the existing upload
ci-integration#1.2 · no · the logs.ndjson / status.json capture and its pre-scan order are unchanged; the new last_error cause only changes content, not what is captured
ci-integration#1.3 · no · the JUnit XML upload is unchanged; the chunk adds no CI job or upload
ci-integration#1.4 · yes · the new directory check and peer-euid check sit on the hook connect that run --perf times against the per-hook max assertions
ci-integration#1.5 · no · no OTel exporter in v1; the chunk adds none
ci-integration#2.1 · yes · the new sync-client and board.rs code must keep the sync crates Tokio-free under cargo check, the Tokio client staying behind its feature
ci-integration#2.2 · no · the in-memory writer oracle is a test convention every new test of that kind keeps; the new fields' schema check is carried by S2 and G4 (3.3)
ci-integration#2.3 · no · standing S9 · a new integration test's viola home lives under target/e2e-home/
ci-integration#2.4 · yes · the new refusals' e2e legs (wait / last, run start, the OS matrix) run in this suite, read through agent-run logs / status and the G2 / G4 exit codes
ci-integration#2.5 · yes · the chunk adds obs_event! call sites and refusal code that the epoch mutation run scores; a missed or unviable mutant there is red at that run
ci-integration#3.1 · no · standing S5 · a new span has a static name, skip_all first, under gate G1
ci-integration#3.2 · yes · the new refusal lines land in home-level role files that G2 scans; a panic on the new refusal path (a 0755 or symlinked directory, a peer euid mismatch) reads red
ci-integration#3.3 · yes · G4 checks every new home-level line; the new detail value or field must be admitted by schemas/diag-line.v1.json or the conformance gate fails
ci-integration#4.1 · yes · the new refusal tests' homes must stay under target/e2e-home/ and survive to G2 and G4 (AGENT_RUN_KEEP_HOMES=1), the harness cleanup measurement included
ci-integration#5.1 · no · the secret-scan timing is unchanged; the no-path / no-pid rule on the new lines is S3's, and the scan is only a backstop
