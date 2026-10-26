proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 10
    section: "## 2. Test Strategy"
    change: "  - **Runner (root bin):** nextest is the runner; plain `cargo test` (`cargo test --bin viola`) is not a supported runner, by the founder's fork (chunk 2026-10-25-windows-endpoint-admission). Under it `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner."
    sidecar: "test-strategy: the Unit level gains a runner sub-item — plain `cargo test` unsupported, its three known-cause reds recorded with no owner (chunk 2026-10-25-windows-endpoint-admission, the founder's P4 fork)."
    rationale: "The report's inputs list I5 records the founder's P4 fork 'the three plain-run reds recorded with no owner (plain `cargo test` unsupported)', and its acceptance reading of plain `cargo test --bin viola` names the three reds (two red that run, `case_2_sigterm` ok that run) as recorded with no owner (`evidence/plain-run.md`, `inputs#I5`); all three are root-bin unit cases, so the Unit level record is their home."
    fact: F9
