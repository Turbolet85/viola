proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 29
    section: "## Occupied Resources"
    change: In the `mcp` subcommand sentence the parenthetical now reads (exit 0 once initialized, 1 before `initialize`, both streams empty; 11 is only the unit-test child's own code), replacing (exit 0 once initialized, 11 before `initialize`); the rest of the line stands.
    sidecar: occupied-resources `mcp` exit before `initialize` corrected from 11 to 1, as measured at chunk 2026-10-31-sanitised-error-surfaces.
    rationale: The report's Spec claims disproved measures exit 1 with both streams empty and one oversize `parse-rejected` line (`mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`), `Role::Mcp` + `Failed` giving exit 1 and 11 being the unit child's own code; its Expected amendments carry this correction for §Occupied Resources (`mcp`) and name this detector's sweep the control, since pattern `exit-eleven` reads 0 hits here.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:44
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: "## Occupied Resources"
    change: In the `mcp.ndjson` clause, the words and `process-exit` on stdin EOF or at that cut — none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk now read and no self `process-exit` at stdin EOF, at that cut or when Claude Code ends it by a signal (the signal case as measured live on claude 2.1.287 at that chunk), the `list` tool's `agents-probe` pair standing as written above; the rest of the line stands.
    sidecar: occupied-resources `mcp.ndjson` no longer lists a self `process-exit`, which chunk 2026-10-31-sanitised-error-surfaces measured absent on the `mcp` role.
    rationale: The report's Spec claims disproved finds no self `process-exit` written on the `mcp` role (`ObsEvent::ProcessExit` grep, the `src/run/agents.rs:64` hit being the `agents-probe` pair) and `internal_error_exit_line` writing for `Run` and `Cli` only; pattern `mcp-exit-line` lists no site in this file, so this standing claim is outside its 3 counted sites.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:47
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 205
    section: "## Occupied Resources"
    change: Before the sentence beginning One of the waits is, add the sentence `tests/support/events.rs` also holds the tests' error-body check since chunk 2026-10-31-sanitised-error-surfaces (`error_body_hit`, `assert_error_body_clean`; classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes), read by the surface witnesses, with no after-every-test helper built.
    sidecar: occupied-resources test-support entry extended with the error-body check in `tests/support/events.rs`, the after-every-test helper replaced and not built.
    rationale: The report's pattern `after-every-test` counts `occupied-resources.md:205` as 1 extended; its Symbols / APIs name the check and its classes, and its Spec claims disproved state the helper is not built, the three surface witnesses, the enumeration and the census of writers replacing it.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:48
note: Lines 29 and 124 are sites the report's `exit-eleven` and `mcp-exit-line` patterns did not count, so their standing totals read one site short; the Retired items and D-arch-decisions read no drift in this file.
