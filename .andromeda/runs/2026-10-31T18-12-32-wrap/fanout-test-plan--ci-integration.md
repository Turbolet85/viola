proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: Add after line 11, in its one-space continuation form, At chunk 2026-10-31-exit-cause-code-catalogue the recipe did not run before the first push, and `lint (macos-latest)` and `test (macos-latest)` went red on `115cdbb9` (`ci#41225642322`) on one compile error, an every-OS case in `crates/viola-mcp/tests/handlers.rs` calling a helper gated off macOS; on the fixed tree the recipe reads exit 0 (2026-10-31T17:42:00Z), and CI reads green on `8a4c1a2c` (`ci#41226917636`).
    sidecar: test-plan CI Integration gains the macOS check-only clippy recipe's reading at chunk 2026-10-31-exit-cause-code-catalogue, unrun before the first push, CI's first read red on the compile error, exit 0 on the fixed tree.
    rationale: The report's Deviations item 8, Decisions (the cfg-gate sweep hazard) and Cross-project claims bring a new reading of the recipe this file's line 11 describes; the recipe's standing stays as line 11 says, and the report records no skipped gate.
note: No CI step, harness verb, status shape, tool pin or count this file states moved (report Harness / gate surface, Dev-tool versions, Counts), no Retired item stands here, and making the recipe a plan entry is not proposed.
