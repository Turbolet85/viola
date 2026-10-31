# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the upload paths, the secret-scan gate and retention sit in the workflow the report leaves unmoved, and green CI over fa7be1f3 scanned the homes under target/e2e-home on all three test jobs
ci-integration#1.2 · no · the agent-run logs and status capture and its upload sit in the workflow, and the report leaves the harness verbs and status shape unmoved
ci-integration#1.3 · yes · stale · the session reached .config/nextest.toml, whose ci profile moved its kill-line filter from 22 to 24 binary terms, and the report does not show the [profile.ci.junit] path still holds on that file
ci-integration#1.4 · no · the perf exports still come from the five hook rows (run --perf reads five values) and the report keeps hyperfine max through gate --require perf with no bound moved
ci-integration#1.5 · no · no dependency was added or bumped and Cargo.lock is unchanged, so the reached Cargo.toml still carries no OTel exporter
ci-integration#2.1 · no · green CI over fa7be1f3 ran clippy, the lint probes, G1 and cargo deny on the changed tree, and no dependency changed
ci-integration#2.2 · no · the in-memory writer tests still parse emitted lines against schemas/diag-line.v1.json, now closed on detail, and the green unit-test jobs show that holds on the changed files
ci-integration#2.3 · no · integration homes still sit under target/e2e-home where green G2 and the secret scan read them, the chaos carve-out file is not reached, and the new error-body check changes no coverage
ci-integration#2.4 · no · the E2E tests still feed agent-run logs and status to G2 and G4, which read green, and the reached mcp_tools file adds a clean-body witness, not an artifact or gate
ci-integration#2.5 · no · the mutation runs, windows-mutants.yml and src/panic_frames.rs whose counts it cites are untouched, and the report says no CI step moved in nightly.yml or windows-mutants.yml
ci-integration#3.1 · no · the G1 command and its PCRE2 install are unchanged, and green CI over fa7be1f3 ran G1 over the changed Rust workspace
ci-integration#3.2 · no · G2's script, its probe and the seam path src/cmd/hook/seam.rs are not reached, and the report says no counted panic was added
ci-integration#3.3 · no · G4 still checks every G2-scope line against schemas/diag-line.v1.json, which the report says G4 now reads closed on detail, and green CI over fa7be1f3 ran G1 to G4 on the changed tree, Cargo.toml included
ci-integration#4.1 · yes · add-to · F15 (v1-47's secret-scan reading follows each job's end-to-end run, not every test) joins this step order and its kept homes
