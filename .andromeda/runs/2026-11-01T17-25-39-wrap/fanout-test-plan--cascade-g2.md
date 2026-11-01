proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 18
    section: "## 5. Integration Test Strategy"
    change: "Add to the Test approach, after the Unix read-back: the directory is also checked before use: `check_socket_dir`, through `check_endpoint_dir`, refuses a directory with a foreign bit or a foreign owner, a symlink or a non-directory, passes an absent path, and never chmods, a refused directory staying as found; the refusals are asserted in §6 Security control negatives (`tests/security_negatives_socket_dir.rs`), a foreign owner by injected uid at `socket_dir_verdict`."
    sidecar: "§5 Module ↔ IPC: the Unix socket directory's read-back gains the built check, which refuses a loosened directory, a symlink or a non-directory and never chmods."
    rationale: "The row reads the directory's mode back only; the chunk built a refusal on that directory (report.md:11, report.md:14, report.md:107), and the standing 0700 read-back stays true beside it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 18
    section: "## 5. Integration Test Strategy"
    change: "Add to the Test approach: `viola_channel::Client::connect` itself carries no directory check, as `viola-channel` has no `viola-state` edge; `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design."
    sidecar: "§5 Module ↔ IPC: records that the viola-channel client's connect carries no directory check, by design."
    rationale: "The row's named tool is the viola-channel client, and the report's negative API fact says the check is not inside it (report.md:53)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:53
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 41
    section: "## 5. Integration Test Strategy"
    change: "Add to **Bare temp-dir home:** a socket directory meets the same trap: a test placing an endpoint in a temp directory makes it through `viola_state::fs::create_private_dir`, because a bare `tempfile::tempdir()` meets the socket directory check as a plain home meets the read rule; `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs` do so."
    sidecar: "§5 Bare temp-dir home: extended to an endpoint's directory, made through create_private_dir at the four sites."
    rationale: "The report disposes the measured false premise as this extension of the row, whose claim about a home stays true (report.md:56, report.md:96)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56
    fact: F11
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: "## 5. Integration Test Strategy"
    change: "Replace the macOS clause: macOS reports no peer pid, so there the pid comparison is absent, the named carve-out, owned by no route entry, and the server-identity negatives stay compiled out; its peer euid and socket directory checks are built, a refused euid reading `server-verify-failed` and a refused directory `socket-dir-refused`. The rest of the row stands."
    sidecar: "§5 Wrapper channel: macOS's euid and directory check no longer owed to the route entry; the absent pid comparison stays a named carve-out with no owner."
    rationale: "The report's Retired bullet names the owed client euid and socket-directory checks as built (report.md:26), and macOS's pid gap stays named but owned by no route entry (report.md:33, report.md:110)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: "## 5. Integration Test Strategy"
    change: "Add a sibling item after the MCP widened-instance case: the MCP socket directory case `handlers_frame_tools_refuse_a_loosened_socket_directory` (`crates/viola-mcp/tests/handlers.rs`) holds MCP's frame tools at no frame and no line, with `isError` the record, `instance-unreachable` with `detail` `socket-dir-refused`."
    sidecar: "§5 Wrapper channel: adds the MCP socket directory case handlers_frame_tools_refuse_a_loosened_socket_directory."
    rationale: "The new surface's MCP witness (report.md:17, report.md:73, report.md:111) belongs beside the row's other named MCP refusal cases."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:73
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 60
    section: "## 5. Integration Test Strategy"
    change: "In MCP's mapping, replace «a failed `server_verified` is `server-verify-failed` (off macOS)» with: a refused `check_endpoint_dir` after `live_endpoint` is `socket-dir-refused`, a peer euid `peer_admitted` refuses or a failed `server_verified` is `server-verify-failed` (Linux, Windows and macOS, where on macOS it means an euid mismatch alone). The rest of the row stands."
    sidecar: "§5 Wrapper channel: MCP's mapping gains socket-dir-refused and server-verify-failed's reach moves from off macOS to all three OSes."
    rationale: "`frame` now maps both checks (report.md:17), and the word's reach moved from off macOS to Linux, Windows and macOS, each such row changing (report.md:31)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 74
    section: "## 5. Integration Test Strategy"
    change: "Add to the row: the same home refuses `viola list`, held to the read rule on all 3 OSes by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`), with and without an instance; the whole-board refusal's order and no-open are unchanged."
    sidecar: "§5 On-disk: a home another user may read now refuses viola list as well as viola run, with its witness."
    rationale: "`viola list`'s home moved from two rules to the read rule, on every OS (report.md:18, report.md:32, report.md:112)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:32
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 102
    section: "## 5. Integration Test Strategy"
    change: "Add a new sub-item following this line, before «Each case re-runs five control negatives»: `setting_does_not_disable_the_socket_dir_check` (`tests/cli_controls_not_disableable.rs`) runs every `SETTINGS` row against the socket directory check, a case of its own beside the five negatives below."
    sidecar: "§5 CLI: the not-disableable table gains setting_does_not_disable_the_socket_dir_check, its own case over every SETTINGS row."
    rationale: "The report adds this block, looping over `SETTINGS` (report.md:238, report.md:239); it is written as a new line because this line already stands at the 2 000-char limit, and the row count moves in no report bullet."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:238
    fact: F9
note: "F23 (harness witnesses) fits no entity of this group, so the new-detail pass places it; integration-test-strategy#5.1 already stands over the 8 KB entity limit at about 17 KB, and its line 102 at about 2 070 chars, neither moved by this pass."
