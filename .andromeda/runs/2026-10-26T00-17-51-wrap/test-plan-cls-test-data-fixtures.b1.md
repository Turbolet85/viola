# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the five new pipe cases create no home and the src files changed only test-side captures and a helper, so the product-state rule and the seed_conpty carve-out still hold
test-data-fixtures#1.2 · no · ledger/stamps.json is written only by viola verify, which the session did not touch, and the reached src/cmd/run.rs changed only a unit-test capture
test-data-fixtures#1.3 · no · budget.json and its unit cases sit in crates/viola-state/src/budget.rs, which is not reached, and the reached src/run/budget.rs and src/cmd/hook/statusline.rs changed only unit-test captures
test-data-fixtures#1.4 · no · the statusline source is planted by tests/support/home.rs, which is not reached, and the reached src/cmd/run.rs changed only a unit-test capture, so viola still only reads it
test-data-fixtures#1.5 · no · the agents fake sits in tests/support/fake.rs and the fake agent, neither reached, and no reader changed beyond unit-test captures
test-data-fixtures#1.6 · no · no recorded payload was added so the 2.1.287 set still holds 34 files, and the reached src/run/dialog.rs changed only a unit-test capture
test-data-fixtures#1.7 · no · no MCP exchange was added or re-recorded, and viola-mcp-tap and its replay test are not reached
test-data-fixtures#1.8 · no · the trycmd, HELP and list literals sit in test files the session did not reach, and the reached src/main.rs gained only the helper and its witness test, so no help text moved
test-data-fixtures#1.9 · no · no insta snapshot was regenerated or added, and the reached src/run/dialog.rs changed only a unit-test capture, so the decision bodies did not move
test-data-fixtures#1.10 · no · no hook transcript or recorded payload changed, so the byte-for-byte comparison's inputs stand
test-data-fixtures#1.11 · no · no proptest strategy or committed regression seed changed, and the reached src/run/gate.rs changed only a unit-test capture
test-data-fixtures#1.12 · no · the reached root Cargo.toml gained only the cfg(windows) windows-sys dev-dependency table, so no manifest names mock_instant or tokio test-util
test-data-fixtures#2.1 · no · the fake agent's hook-command reading and fixture replay are not reached, and no fixture, mode or payload changed
test-data-fixtures#3.1 · no · no receipt kind or field changed, and the session added no fake-agent receipt
test-data-fixtures#4.1 · no · no mode or argv option was added or changed, so the option counts stand
test-data-fixtures#5.1 · no · the fixture-hygiene walk in tests/contract_fixture_hygiene.rs and its schemas are not reached, and no fixture or script was added
