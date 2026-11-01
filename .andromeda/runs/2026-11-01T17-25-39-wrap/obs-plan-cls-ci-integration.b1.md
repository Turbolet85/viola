# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diagnostics upload and its home root stay as they were; ci.yml is untouched and the homes rule holds on the reached tests/support/home.rs
ci-integration#1.2 · no · the harness logs and status artifacts sit outside every reached file, and the session changed no log or store location
ci-integration#1.3 · yes · stale · the change reached .config/nextest.toml, which holds this junit path, and the report shows only the ci profile kill line moved, not that the junit path still holds
ci-integration#1.4 · no · the perf upload and its gate belong to ci.yml, which is untouched, and the row states no value the session moved
ci-integration#1.5 · no · no OTel exporter was added, so the N/A stays true
ci-integration#2.1 · no · the lint gates still hold: channel-tokio-free.sh and cargo deny check are green, viola-channel stays sync, and the new hint stays in the owned human output
ci-integration#2.2 · no · the row names the schema file as its parse target and the session only added enum members to that file, which the parse mechanism does not depend on
ci-integration#2.3 · no · the report moves no rule on test homes, so the target/e2e-home root and the chaos carve-out hold on the reached tests/support/home.rs
ci-integration#2.4 · no · the row names agent-run logs and status and the G2 and G4 gates, none of which the session changed
ci-integration#2.5 · no · the mutation row scores obs code, while the session's always-admit mutant sits in viola-channel's peer seam, and the panic_frames.rs counts are unmoved
ci-integration#3.1 · no · the G1 command is untouched and the report shows the G1 and G2 gate commands run green
ci-integration#3.2 · no · the report shows scripts/g2-zero-panics.sh runs green and is not edited, and the seam exemption path belongs to seam.rs, which the session did not reach
ci-integration#3.3 · no · the G4 row names the schema as its check target and states no enum value, so the session's added enum members leave its claim true
ci-integration#4.1 · yes · stale · cleanup.rs is reached and carries the keep-homes behaviour this row states, and the report shows only its endpoint_gone reading moved, not that homes are still kept before the gates
ci-integration#5.1 · no · the secret-scan reading is in ci.yml, which is untouched
