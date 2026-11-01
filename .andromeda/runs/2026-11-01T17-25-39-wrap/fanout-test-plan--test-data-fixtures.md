proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 15
    section: "## 7. Test Data & Fixtures"
    change: "Add a table row after `Widened test homes`: | A test-owned socket root | `tests/support/home.rs` `SocketRoot` (`new` · `path` · `dir` · `made`), under `/tmp`; `SOCKET_ROOT_VAR` names the variable that carries it (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`); `Wrapper::boot_on_socket_root` boots on it, and `first_life_on(stamped, root, open_umask)` stands in `home.rs`, not beside `first_life` in `tests/support/start.rs`, which the guard entry's pattern does not allow. `tests/security_negatives_socket_dir.rs` boots through them. | per-test |"
    sidecar: "§7 gains the test-owned socket root row (`SocketRoot`, `SOCKET_ROOT_VAR`, `Wrapper::boot_on_socket_root`, `first_life_on`), chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The report's Symbols / APIs names these new test helpers and Expected amendments names the test-owned socket roots under /tmp; the category's table has no row for this test data (report.md:21, report.md:63, report.md:81)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 15
    section: "## 7. Test Data & Fixtures"
    change: "Add a table row after `Widened test homes`: | A loosened socket directory (the socket-directory negatives) | Set by the test on the real directory, never by viola, whose check never chmods it: `tests/security_negatives_socket_dir.rs` `Form` (`apply` · `assert_as_found` · `restore`), a directory at 0755 or a symlink to 0700, the refused directory asserted as found. A foreign owner is not driven: `socket_dir_verdict` takes it by an injected uid. | per-test |"
    sidecar: "§7 gains the loosened socket directory row (`Form` in `tests/security_negatives_socket_dir.rs`), chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The chunk's new negatives alter a real directory as the strict-modes negatives alter a real home, and the Widened test homes row names only the home helpers (report.md:11, report.md:107, report.md:130)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 19
    section: "## 7. Test Data & Fixtures"
    change: "In the `Human-mode expected output` row, before `are literals in their test files`, add the clause: and the new refusal pair's last `hint:` line (`tests/cross_exit_causes.rs` `SOCKET_DIR`, `tests/security_negatives_socket_dir.rs` `HINT`), a planning form, its wording owed to the founder at Epoch 7's boundary batch,"
    sidecar: "§7 Human-mode expected output names the socket-directory hint literals and their planning-form status, chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The chunk adds a closed refusal pair pinned as test literals, its wording a planning form with the founder's word owed (report.md:49, report.md:52, report.md:76, report.md:252, report.md:264)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 32
    section: "## 7. Test Data & Fixtures"
    change: "Add a bullet after the home-made-before-viola bullet: - A directory holding a test's endpoint, where the case needs it to pass the socket-directory check, is made through `viola_state::fs::create_private_dir`, never left a bare `tempfile::tempdir()`: `tempfile` 3.27 leaves the mode to the umask (0755 as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening), and such a directory meets `check_endpoint_dir` as a home meets the read rule, as `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs` do. It binds with §5's own line that a bare temp dir reads 0755 under the umask."
    sidecar: "§7 records that a test's endpoint directory is made through `create_private_dir`, the 0700 temp dir reading disproved at chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The report measured the claim that impostor endpoints sit in a 0700 tempdir false, narrowed four sites, and calls a bare tempdir a sweep hazard for any test placing an endpoint in one; the category states the private-dir rule for a home alone (report.md:56, report.md:96)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56
note: "No Retired item, moved count or harness change stands in this file, and the four detectors find no violation in it; body §7's On-disk product state line holds the private-directory rule for a home only, so the new socket-directory bullet binds with it."
