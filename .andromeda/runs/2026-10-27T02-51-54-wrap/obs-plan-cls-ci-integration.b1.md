# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diagnostics upload path, condition and retention are unchanged, no CI step changed and the session adds no diagnostics line
ci-integration#1.2 · no · the agent-run logs and status artifact and its upload are unchanged, the logs and status documents are unchanged and no CI step changed
ci-integration#1.3 · no · the nextest JUnit upload is unchanged and no CI step changed
ci-integration#1.4 · no · the perf export and its upload are unchanged, run --perf is green at 7 passed and 0 failed and no hyperfine gate moved
ci-integration#1.5 · no · no OTel SDK, exporter or sink changed, so the N/A row still holds
ci-integration#2.1 · no · the lint stage still holds on the changed fake agent, its print-lint exemption covers the new receipt field and CI ci#40439188053 is green 15/15
ci-integration#2.2 · no · the in-memory line tests still hold on the changed files, since diag-line.v1.json is unchanged and no line or field was added
ci-integration#2.3 · no · no CI step changed, chaos_feed_panic.rs is not reached and the perf assertions are unmoved (run --perf green at 7 passed and 0 failed)
ci-integration#2.4 · no · the CI gate set is unchanged, no CI step changed and the mcp_tools change only changes which pair Path 7 boots
ci-integration#2.5 · no · no obs_event!, MillisUtc or panic hook changed, the session adds no log line, span or obs decision, and panic_frames.rs is not reached
ci-integration#3.1 · no · G1 still holds on the changed files, since place_private_new adds no span and CI ci#40439188053 is green 15/15
ci-integration#3.2 · no · G2 is green (g2-zero-panics.sh), no seam was added or moved and the zero-panic invariant is unchanged
ci-integration#3.3 · no · G4 still holds, since diag-line.v1.json is unchanged, no line or field was added and agent-run schema-check is green
ci-integration#4.1 · no · no CI step changed, so the test-then-G2-and-G4 order and its always() conditions still hold
