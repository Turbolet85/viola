# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · no CI step, artifact name or retention changed, so the diagnostics upload and its target/e2e-home glob stand
ci-integration#1.2 · no · no CI step or artifact name changed, so the logs and status capture under its own harness artifact stands
ci-integration#1.3 · no · no CI step or artifact name changed, so the nextest JUnit upload stands
ci-integration#1.4 · no · no hook path or perf row changed, so the five hyperfine exports from run --perf stand
ci-integration#1.5 · no · the row is N/A and the session adds no exporter or dependency
ci-integration#2.1 · no · the veil and obs-crate bans its incl. list names still hold on the changed deny.toml, which only gains a second `[[bans.features]]` table
ci-integration#2.2 · no · the schema change is additive (one parser value, asserted by mcp_tools), so the in-memory unit tests still validate against the same schema
ci-integration#2.3 · no · the chaos carve-out home stays outside G2, G4, the scan and the upload, its home exemption unmoved on the changed test, and the new assertions read the outer terminal
ci-integration#2.4 · no · no CI step, artifact or gate changed, and the new mcp_tools case runs under the existing run --e2e
ci-integration#2.5 · no · the mutation rows score src/panic_frames.rs, which the session did not reach, and obs_event! call sites are already in their class
ci-integration#3.1 · no · the G1 rg gate and its skip_all rule are unchanged, with no CI step or gate command changed
ci-integration#3.2 · no · G2 reads clean after the chunk, and its scope, probes and seam exemption are unchanged
ci-integration#3.3 · no · G4 still checks every G2-scope line, mcp.ndjson included, against the changed schema, and the new mcp-stdin line validates
ci-integration#4.1 · no · the chaos home stays the one carve-out, its home exemption unmoved on the changed test, and no CI step changed
