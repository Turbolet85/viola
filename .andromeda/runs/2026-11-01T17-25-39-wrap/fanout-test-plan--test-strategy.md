proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 10
    section: "## 2. Test Strategy"
    change: "Append to the Unit row's tier depth, after the `server_pid` clause: the socket-directory check's captured-span cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` run on macOS, reading the span recorder `Seen` from the test module `refused_seen` in `src/cmd/client.rs`, outside the macOS-gated `refused_inside` (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "test-strategy Unit row: the socket-directory captured-span cases run on macOS through `refused_seen`, the server check's cases staying compiled out there."
    rationale: "The report's Deviations (item 4) moves `Seen` out of the gated `refused_inside` so two new cases run on macOS, while this row names `refused_inside` as the macOS-gated helper and no captured-span case running there; the report's New text lists these two as the chunk's captured-span cases, which is the reading that names them."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:82
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 29
    section: "## 2. Test Strategy"
    change: "Append to the Mutation row's tier depth: a known survivor stands against the `missed` rule, a mutant replacing a client seam's `peer_admitted` read (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) with an always-admit survives every committed test by construction, because one account can make no foreign peer and no seam may be added; the wiring was read only once, by forced refusals in that chunk's remove-the-guard runs, and the mutation audit reads the survivor at the route entry \"Version done-check\" (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "test-strategy Mutation row: the client seams' euid wiring named as a survivor the boundary audit will read, with no committed test at any tier."
    rationale: "The client euid wiring is a new path with no committed test at the mandated tier; the report's Insufficient fixes rules the remainder to `matrix#v1-44` and test-plan section 1's untestable zone and says the audit reads the same survivor, while this row states any unexcluded `missed` outcome fails the run without naming it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 43
    section: "## 2. Test Strategy"
    change: "New bullet following this line, at its indent: **Socket-root helpers:** `tests/support/home.rs` holds the test-owned socket root: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`; `first_life_on` stands there, not beside `first_life` in `tests/support/start.rs` (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "test-strategy naming: the socket-root helpers in `tests/support/home.rs` recorded, `first_life_on` placed apart from `first_life`."
    rationale: "The report's Symbols / APIs adds these test helpers and its Deviations (item 3) places `first_life_on` in `home.rs`; this file describes `home.rs` only as the home-widening helpers and `start.rs` as the shared start helpers."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 59
    section: "## 2. Test Strategy"
    change: "In the IPC vector (V1) row: after the MCP impostor case add `handlers_frame_tools_refuse_a_loosened_socket_directory` as a second case of `crates/viola-mcp/tests/handlers.rs`; after the `security_negatives_server_identity.rs` clause add root `security_negatives_socket_dir.rs`, Unix (`ubuntu-latest` and `macos-latest`): a socket directory at 0755 or a symlink to 0700 refusing a start with exit 1, no bind and no child, a frame verb with exit 21 and no frame, and a hook with exit 0, empty streams and no frame, each leaving it as found."
    sidecar: "test-strategy trigger map V1: the new binary `security_negatives_socket_dir` and the MCP loosened-directory case traced."
    rationale: "The report adds the binary and its Outcome holds these acceptance readings; the V1 row names each sibling `security_negatives_*` binary and the MCP case, and traces neither new witness. Its off-macOS clauses stay true, the server-identity negatives staying compiled out there."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 64
    section: "## 2. Test Strategy"
    change: "At the end of the Filesystem (V7) row's Security control negatives parenthesis add: root `cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`, `viola list` holding the home to the read rule, met on all three OSes with the unit `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance` (the rule PROVISIONAL, chunk 2026-11-01-unix-endpoint-and-home-hardening `inputs#I2` Q3)."
    sidecar: "test-strategy trigger map V7: `viola list`'s read rule on the home traced to its witnesses, PROVISIONAL on the delegate's answer."
    rationale: "The report moves `viola list`'s home from two rules to the read rule and its Outcome names these witnesses; the V7 row traces the read rule at `run` alone."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:112
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 74
    section: "## 2. Test Strategy"
    change: "In the error sanitization row, after root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`, add: beside it on the same surface `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`; the row's three surface witnesses stay three surfaces."
    sidecar: "test-strategy trigger map error sanitization: the CLI's loosened-directory witness named beside the surface witness."
    rationale: "The report's New text adds this case to `tests/cli_error_bodies.rs`, and this row names that file's witness by function."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:235
note: "No drift for D-tests-framework (nextest and the harness match) or D-tests-obs-harness (no harness shape in this file); no moved count literal, retired item or `Unix endpoint and home hardening` clause stands in this file."
