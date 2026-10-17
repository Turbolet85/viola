proposals:
  - detector: D-arch-resources
    severity: warning
    section: Occupied Resources → Environment variables (Test-harness only, `AGENT_RUN_KEEP_FAILED`)
    change: "Name viola-e2e's `harness::session` as a third reader of `AGENT_RUN_KEEP_FAILED`, beside the root test chain (`tests/support/home.rs` `keep_decision`) and the `harness_lifecycle.rs` `Booted` guard: `HarnessSession`'s `Drop` (after its `cleanup`) removes the boot-created home parent unless `keep_home` holds, i.e. `AGENT_RUN_KEEP_HOMES=1`, or `AGENT_RUN_KEEP_FAILED=1` while the thread is panicking (chunk 2026-10-17-end-to-end-path-binary-and-harness-legs)."
    sidecar: "Occupied Resources / Environment variables: AGENT_RUN_KEEP_FAILED gains its third reader, viola-e2e harness::session's HarnessSession Drop guard (keep_home), chunk 2026-10-17-end-to-end-path-binary-and-harness-legs."
    rationale: "The report's Symbols bullet for the new `harness::session` says `Drop` runs `cleanup` and then removes the home parent unless `keep_home(KEEP_HOMES, KEEP_FAILED, panicking)` = `KEEP_HOMES=1` or (`KEEP_FAILED=1` and panicking). The registry entry lists its readers one by one, and it now leaves out a test-side reader of an existing harness env var. No variable is new, which matches the report's (arch) outcome, \"no new env var read\". But the reader set this entry records has grown."
    basis: "crates/viola-e2e/src/harness/session.rs:107-122"
  - detector: D-arch-resources
    severity: warning
    section: Occupied Resources → Environment variables (Test-harness only, `AGENT_RUN_KEEP_HOMES`)
    change: "Name viola-e2e's `harness::session` (the `HarnessSession` guard's `Drop`, through `keep_home`) as a reader of `AGENT_RUN_KEEP_HOMES` beside `viola-harness` and the root test chain (`tests/support/home.rs`). Under `1` the guard keeps the session's home."
    sidecar: "Occupied Resources / Environment variables: AGENT_RUN_KEEP_HOMES also read by viola-e2e harness::session's HarnessSession Drop guard, chunk 2026-10-17-end-to-end-path-binary-and-harness-legs."
    rationale: "This is the same retired claim, the closed reader set of the AGENT_RUN_KEEP_* variables, stated in the sibling entry. The report's `harness::session` keep rule reads `KEEP_HOMES=1` as well. `harness_session` is a library guard that test binaries call (`path6_budget_harness`, `harness_lifecycle.rs`), not the `viola-harness` bin, so the entry's \"read by viola-harness\" no longer covers it."
    basis: "crates/viola-e2e/src/harness/session.rs:30-34"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: Occupied Resources → Filesystem (repository, `target/agent-run/<session>/`)
    change: "List `bin/agents-mode` among the contents of `target/agent-run/<session>/`, beside the session's `bin/claude[.exe]` copy. `boot --agents-mode recorded|oversize|malformed` writes it (default `recorded`; `replace_private`, 0600; a stale one removed before the stamp, the new one written after the stamp and the statusline-source plant, before `supervise.json`). The session record (`session.json`) and `supervise.json` carry `agents_mode` (serde default)."
    sidecar: "Occupied Resources / Filesystem: target/agent-run/<session>/ adds bin/agents-mode (written by boot --agents-mode after the stamp) and agents_mode in session.json / supervise.json, chunk 2026-10-17-end-to-end-path-binary-and-harness-legs."
    rationale: "The report's `harness::boot` Symbols bullet says `start` removes a stale `<session>/bin/agents-mode` before the stamp and writes it after the stamp and before `supervise.json`, and that `SessionRecord` and `SuperviseSpec` gain `#[serde(default)] agents_mode` (Schema / config bullet). The registry entry lists what the harness writes under the session directory, and it now leaves out a file that the harness writes and that the fake agent's `agents` mode reads beside `bin/claude[.exe]`. The report's Decisions bullet rules on the order: a write made before the stamp failed `agents-join`."
    basis: "crates/viola-e2e/src/harness/boot.rs:47-56"
note: "D-arch-decisions found no drift. There are no dependencies (Dependencies: none) and no new runtime or crate (Crates: none added). The `status` read of `viola list` and `harness_session` spawn directly and never shell out (Symbols). The Path 6 leg compares instants as text, so viola-e2e takes on no chrono. The architecture.md site that the report's `boot-flags` search names (it cites architecture :299, the fake agent's `agents-mode` file) still holds, because the agent reads the file the same way. No IPC method, endpoint, event, socket, port, env var or workspace crate was added."
