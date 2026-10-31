proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "Add one table row after the Time-sensitive data row, reading: | The channel error-body witness's planted value | `tests/channel_error_bodies.rs` `booted_with_the_value` starts from `stamped_home` and boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env` (`Wrapper` takes no environment); the value reaches that one child through the PTY spawn's `env_set`, as `tests/tui_env_strip.rs` does. The check `tests/support/events.rs` `assert_error_body_clean` takes the planted values and reads a hit as class `planted`, reporting place and class, never bytes (chunk 2026-10-31-sanitised-error-surfaces). | per-test (rstest `stamped_home`) |"
    sidecar: "test-data-fixtures: the channel error-body witness's planted value recorded as a data row: planted through the PTY spawn's env_set, read by assert_error_body_clean as class planted (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Deviations item 1 names how step 7's planted value reaches the child, and its Symbols / APIs names the check and its class `planted`; the §7 table states a strategy per data type and holds no row for this value."
    basis: tests/channel_error_bodies.rs:56
note: "test-plan body §7 Test data lifecycle says env is set per child with `Command::env`; the report's deviation 1 shows the PTY spawn's `env_set` as another per-child form, which is outside this file and is left for the body's owner to weigh."
