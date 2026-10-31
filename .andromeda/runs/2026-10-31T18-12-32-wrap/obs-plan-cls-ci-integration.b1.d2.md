# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the artifact path glob target/e2e-home/**/diagnostics/*.ndjson, its retention and storage are unchanged, and the new binary's homes fall under that glob; the rule lives in .github, which is not reached
ci-integration#1.2 · no · the agent-run logs and status artifact is a harness surface, and no harness verb or status shape moved
ci-integration#1.3 · yes · stale · the junit path line it cites sits in reached file .config/nextest.toml, whose ci profile took the new kill operand, and nothing shown holds that line unmoved
ci-integration#1.4 · no · the hyperfine perf rows, run --perf and the perf job are unmoved, and src/cmd/hook.rs is not in the diff
ci-integration#1.5 · no · OTel stays N/A and no exporter was added
ci-integration#2.1 · no · clippy and cargo deny ran green on the final HEAD and no dependency was added or bumped, so the lint and ban rows still hold
ci-integration#2.2 · no · the unit tests parse against diag-line.v1.json, which still exists and accepts the grown process-exit list, and CI green on the final HEAD ran them
ci-integration#2.3 · no · the new root binary tests/cross_exit_causes.rs and the setups in tests/support/start.rs fall under target/e2e-home/, and the carve-out file tests/chaos_feed_panic.rs is not reached
ci-integration#2.4 · no · the E2E rows name agent-run output and gate exit codes, which are unchanged on the final HEAD
ci-integration#2.5 · no · the mutation row names no count, and its job and rule sit outside the reached files
ci-integration#3.1 · no · G1 ran green on the final HEAD, so no bare #[instrument] reached the workspace, and the rg procedure is unchanged
ci-integration#3.2 · no · the G2 scope and seam exemption are unchanged, src/cmd/hook.rs is not in the diff, and CI green on the final HEAD ran G2
ci-integration#3.3 · no · G4 validated against the grown diag-line.v1.json and G3 scanned Cargo.toml green on the final HEAD, so the gate claims hold on the changed files
ci-integration#4.1 · no · the homes-kept rule holds, since G2 found homes at the gate steps on the final HEAD, and the viola-chaos carve-out is not reached
ci-integration#5.1 · no · the secret-scan reading (AGENT_RUN_KEEP_HOMES, if always, viola-chaos out) is unchanged, and CI green on the final HEAD ran it
