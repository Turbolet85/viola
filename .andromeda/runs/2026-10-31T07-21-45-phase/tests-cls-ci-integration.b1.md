# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the OS-only witnesses (the status-line Windows arm, the Unix-only claude_env_keep list) are linted only by the per-OS lint job on their own runners, and a Linux-host Windows or macOS clippy is no CI step
ci-integration#1.2 · no · a re-check every change owes: the deny, tokio-free and zizmor gates read the dependency graph and the workflows, and the scope names no new crate, tokio edge or workflow
ci-integration#1.3 · yes · the unit stage runs the JSON-RPC error-mapping and -32602 cause unit cases, the AgentError and Refusal cases, and the fixed-message Display cases
ci-integration#1.4 · yes · the integration stage runs the root contract cases (the five BOUNDARIES rows, the viola mcp stdin line before initialize), the root CLI --json per-exit-code cases, the viola-mcp handlers and the channel suites
ci-integration#1.5 · yes · the E2E stage on all three OSes runs scenario E2's planted stripped-value witness, the Path 1 watch test path1_run_start_one_home_in_order and the secret scan after every E2E test (v1-47 leg)
ci-integration#1.6 · yes · in CI the per-OS test job is the one nextest run carrying the chunk's unit, integration and E2E tests, so it is where the Path 1 CI timeout is read and the per-OS coverage gate judges the new code
ci-integration#1.7 · no · the mutation gate and windows-mutants.yml run only at the epoch boundary, and the task lists the mutation gate as not needed
ci-integration#1.8 · no · the perf stage (hyperfine hook timings, gate --require perf, its schema-check and secret-scan after run --perf) is not reached, and the changed channel-response line is read by the chunk's own schema-validated witness
ci-integration#1.9 · no · a re-check every change owes: the 1.96 compile and unit run of every change, which the scope does not single out
ci-integration#1.10 · no · fuzz-replay and the nightly fuzz job replay existing corpora, the task lists the nightly fuzz job as not needed, and the scope names no fuzzed parser
ci-integration#1.11 · yes · the test-only seam for the -32603 handler fault must stay behind fake-agent and out of the release build, which release-check judges from the build's compiler-artifact records
ci-integration#1.12 · yes · the junit XML and run-summary.json reads are where the Path 1 timeout and its three-green-run count are read, and the §10 breach exit judges coverage and the zero-flakiness budget on the chunk's runs
