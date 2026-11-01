proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 36
    section: "## Cross-cutting Patterns"
    change: "After the sentence naming the three surface witnesses, add one sentence, keeping the three as they stand. Beside the CLI's, `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`, also in `tests/cli_error_bodies.rs`, reads the error documents of a loosened socket directory (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "Cross-cutting Patterns, Sanitised error surfaces: the CLI surface's second witness, for the documents of a loosened socket directory, recorded beside the three surface witnesses."
    rationale: "The chunk adds this witness to `tests/cli_error_bodies.rs` (.andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:8, :235), and the entry at line 36 names the witnesses that read the error bodies by function and file, so the new one is unregistered there."
    basis: tests/cli_error_bodies.rs:278
note: "No proposal, for the orchestrator to weigh: report.md:21 names the test helper `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), and cross-cutting-patterns.md:13's closed list of variables a viola build reads names neither; the report does not say whether the product reads them, and this pass did not check the code."
