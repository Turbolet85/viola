# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diagnostics upload's condition, name and retention are unchanged because no workflow file was edited
ci-integration#1.2 · no · the harness upload's name, path and retention are unchanged because no workflow file was edited
ci-integration#1.3 · no · the new Windows cases ride the existing junit-windows-2025 upload, which no CI step, upload, gate or retention change touched
ci-integration#1.4 · no · no hook timing, perf row or heartbeat code changed, the run and statusline edits being test capture calls only
ci-integration#1.5 · no · no obs dependency was added, and the Cargo.toml change is only a windows-sys dev-dependency table, so no exporter exists
ci-integration#2.1 · no · clippy on the host and Windows targets, lint-probes and the cargo deny bans and Tokio-free checks are green on the changed tree
ci-integration#2.2 · yes · stale · the root-bin unit tests now capture process-log lines through with_scoped_default, not with_default, so this row's reading of in-memory writer tests has moved
ci-integration#2.3 · no · the new Windows cases run on test-owned pipes and assert only the pipe listener's descriptor, so the home, diagnostics and G2 and G4 coverage stand
ci-integration#2.4 · no · G2 reads zero panics and G4 reads conformant on the changed tree, and no E2E step or gate command changed
ci-integration#2.5 · no · the mutation counts cite a 2026-10-10 dispatch on src/panic_frames.rs, which the session did not touch, and its one new obs site is test-only
ci-integration#3.1 · no · G1 stays green on the changed tree and no gate command changed
ci-integration#3.2 · no · G2 reads zero panics on the changed tree and no gate command changed, so the seam exemption stands
ci-integration#3.3 · no · G4 reads conformant and G1 and G2 stay green, and the Cargo.toml change adds only a windows-sys dev-dependency table, so the G3 abort scan still passes
ci-integration#4.1 · no · no workflow file was edited, so the test, G2 and G4 order and the kept homes stand
