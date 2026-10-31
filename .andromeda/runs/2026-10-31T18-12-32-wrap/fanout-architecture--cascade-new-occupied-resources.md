proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 217
    section: "## Occupied Resources"
    change: "- Test-only: `tests/cross_exit_causes.rs` is the exit-cause table, a root `cross_` binary of 39 tests on Linux (as measured at chunk 2026-10-31-exit-cause-code-catalogue, `agent-run.sh run --e2e --filter 'binary(cross_exit_causes)'`), with four checks over the table itself: `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`."
    sidecar: "Occupied Resources gains the test-only row of the exit-cause table `tests/cross_exit_causes.rs`, a root `cross_` binary of 39 tests on Linux, and its four checks."
    rationale: "The report's Changes name the new file and its root `cross_` binary, its Outcome reads `39 tests run: 39 passed` on Linux and names `no_two_codes_share_a_hint` and `no_hint_names_release`, its Deviations add `every_code_is_a_code_word`, and its New text lists `shared_hints_reads_a_planted_duplicate`."
    fact: F16
note: "F16 calls all four tests controls; the report calls only one thing the control of `no_two_codes_share_a_hint`, so the change says four checks over the table."
