# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diagnostics upload glob and its retention sit in ci.yml, which the report's Untouched §9 keeps unchanged
ci-integration#1.2 · no · the agent-run logs and status artifact and its own upload sit in ci.yml and the harness, neither reached, and Untouched §9 keeps the uploads unchanged
ci-integration#1.3 · yes · stale · the session reached `.config/nextest.toml`, which holds this profile's JUnit path, and the report does not show that path still holds on the changed file
ci-integration#1.4 · no · the perf job, its hyperfine rows and the hook paths are unchanged (Untouched §10), and neither ci.yml nor the harness is reached
ci-integration#1.5 · no · no exporter is added and the exporter carve-outs and OTel init are unchanged (Untouched §1)
ci-integration#2.1 · no · the lint and deny gates live in clippy config, deny.toml and the lint probes, none reached, and no CI step changes (Untouched §9)
ci-integration#2.2 · yes · stale · its consumer column rests on the JUnit profile in `.config/nextest.toml`, which the session reached, and the report does not show that profile still holds
ci-integration#2.3 · no · the test-home layout is test-plan owned rather than a rule written in a reached file, and the report names no home change
ci-integration#2.4 · no · the E2E tests run the viola-e2e harness and agent-run logs and status, neither reached, and no harness or gate changes
ci-integration#2.5 · no · no mutation job, cited run or count changes, and the new obs call sites fall under the same epoch mutation pass unchanged
ci-integration#3.1 · no · the new spans open with skip_all (F1), so G1 finds no bare instrument, and the gate command is unchanged (Untouched §9)
ci-integration#3.2 · no · no panic path or hook changed (Untouched §7), so the zero-panic gate and its seam exemption stand
ci-integration#3.3 · no · the new lines reuse existing diag-line fields and no schema or field is added (Untouched §8)
ci-integration#4.1 · no · the step order and kept-homes setting are unchanged (Untouched §9, ci.yml not reached), and the home layout is test-plan owned
