proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 18
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add this table row after the Process mocking row | Process faults | none (a real wrapper, a real signal or kill) | A stale holder is a real wrapper under `Frozen` (`tests/cli_instance_state.rs:149-181`, Windows-only before chunk 2026-10-31-exit-cause-code-catalogue, on Unix a `SIGSTOP` guard), and `run_refuses_a_stale_name` stands on it; a wrapper lost under an open call is a real kill, `support::start::kill_wrapper` (`tests/support/start.rs:149-173`) for `tests/cross_exit_causes.rs`' lost row and `tests/chaos_wait_vanish.rs`, `kill_by_pid` (`crates/viola-e2e/tests/mcp_tools.rs:931-939`) for `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`; no double stands in for either |
    sidecar: §8 detail gains a Process faults row, the `Frozen` guard on Unix and the shared `kill_wrapper`, real process faults behind the stale and lost causes (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Deviations 1 and 2 add a Unix `SIGSTOP` `Frozen` guard and move the wrapper kill into `tests/support/start.rs`, and the file's Process mocking row names only the held live process as a real-process stand-in.
    basis: tests/cli_instance_state.rs:149-181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add this table row after the Fake servers row | Unserved endpoints | none (a test-built snapshot, no listener) | In `crates/viola-mcp/tests/handlers.rs` the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` binds nothing and records, through `record_live` (`crates/viola-mcp/tests/handlers.rs:64-86`), an endpoint no one ever bound, because a Windows `Serving` has no `Drop` and its pipe lives in the accept thread until the process exits |
    sidecar: §8 detail gains an Unserved endpoints row, the `handlers` row that records an endpoint no one bound through `record_live` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Deviation 4 states this stand-in and its Windows reason, and the file's Fake servers row names only endpoints a listener holds.
    basis: crates/viola-mcp/tests/handlers.rs:64-86
note: No retired item, moved count or framework change stands in this file; both proposals are new detail only.
