# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · fmt, clippy -D warnings (a call to a deprecated safe AsyncFd constructor would turn it red), the tokio-free cargo check of each sync crate and the rmcp-features verdict are re-run on the bumped lock
ci-integration#1.2 · yes · the supply-chain job is the red this chunk closes: cargo deny over the root lock, the sole-root tokio ban per sync crate, the channel tree form and the deny-probes verdict
ci-integration#1.3 · yes · run --unit is one of the workspace tests the bumped lock must pass, ending in the gate
ci-integration#1.4 · yes · run --integration is one of the workspace tests the bumped lock must pass, ending in the gate
ci-integration#1.5 · yes · run --e2e on all three OSes re-runs the E2E MCP stdio path over a viola mcp child on the bumped lock (the scope's multi-OS matrix)
ci-integration#1.6 · yes · the per-OS test job's run --coverage and doctest, ending in gate --require coverage,doctest, are the workspace tests the bump must pass (its ConPTY step is not reached)
ci-integration#1.7 · no · Not needed: the mutation gate is judged at the epoch boundary through /andromeda-code-audit, never by a chunk, and no push job runs it
ci-integration#1.8 · no · Not needed: the perf budgets and hook-deadline rows (the hook path is sync, no hook code changes), and its G2, schema-check and secret-scan steps are not named by the scope
ci-integration#1.9 · yes · the MSRV job runs the unit suite on Rust 1.96 against the bumped lock, so the new tokio must build on 1.96
ci-integration#1.10 · no · Not needed: the fuzz workspace and its corpus replay hold no tokio, so the fuzz-replay job does not move
ci-integration#1.11 · yes · release-check.sh must end release-check: viola only with its probe at 5/5 on the bumped lock (the release-build verdict the scope names)
ci-integration#1.12 · yes · the workspace tests end in viola-harness gate --require suites, which exits non-zero on any §10 breach (named by the scope)
