# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diag upload row sits in ci.yml, untouched per the report's untouched list (diag uploads, retention-days 7)
ci-integration#1.2 · no · the harness upload row sits in ci.yml, untouched per the report's untouched list (harness uploads, retention-days 7)
ci-integration#1.3 · no · the JUnit row's cargo-nextest pin and nextest config sit outside the reached Cargo files, and the report's untouched list names the junit uploads
ci-integration#1.4 · no · the hyperfine export row sits in the perf job and `run --perf`, and no workflow change touched them
ci-integration#1.5 · no · "no exporter in v1" still holds on the changed Cargo.lock, since the session added no dependency and its delta moves only the tokio entry
ci-integration#2.1 · no · the Tokio-free sync-crate check and the cargo deny bans still hold on the changed files, since no crate gained or lost tokio and cargo deny exits 0 with deny.toml untouched
ci-integration#2.2 · no · the serde_json unit-test row still holds on the changed Cargo.lock, since no other crate moved and the schema and tests are untouched
ci-integration#2.3 · no · the fake-agent runs and their home scope still hold on the changed Cargo.toml, whose only edit is the tokio pin at line 242
ci-integration#2.4 · no · the E2E row's logs, status and G2 and G4 gates sit in harness and gate scripts, and no gate script changed
ci-integration#2.5 · no · the mutation row's counts are a 2026-10-10 dispatch on `src/panic_frames.rs`, a source file the session did not change
ci-integration#3.1 · no · G1's ripgrep procedure and workspace scope still hold on the changed Cargo.toml, whose only edit is the tokio pin at line 242
ci-integration#3.2 · no · G2's script, scope and seam exemption read sources and home logs the session did not change, and no gate script changed
ci-integration#3.3 · no · G4 and the G1 to G3 code block still hold, since G3's no-abort rule reads clean on the changed Cargo.toml (its panic = "unwind" lines did not move)
ci-integration#4.1 · no · the test-then-gate step order and the AGENT_RUN_KEEP_HOMES home keep sit in ci.yml, which no workflow change touched
