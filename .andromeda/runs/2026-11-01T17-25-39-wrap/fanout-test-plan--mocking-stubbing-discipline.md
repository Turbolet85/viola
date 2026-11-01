proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 22
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add after the Forced-window test parameters row the row | Injected peer decisions | none (a parameter of the private function under test; plain arguments of the pure decisions) | A foreign peer is never driven and never doubled, one account making none; `Server::serve` calls the private `serve_admitting(dispatch, peer_admitted)` and `accept_loop(listener, dispatch, admitted)` filters by the decision it is handed (`type Admission = fn(Option<u32>) -> bool`), so `serve_drops_a_refused_peer_before_any_frame` injects a refusing decision, `channel_endpoint_answers_protocol_faults` the control; `peer_euid_admitted(observed, own, reported)` takes its euids as arguments; no env, flag or feature seam and no mockall double; the three client seams' `peer_admitted` reads take no injected decision and no seam is added for them (§1 untestable zones) |
    sidecar: §8 gains the row Injected peer decisions, the wrapper's drop and the euid decision held by injected values with no seam, chunk 2026-11-01-unix-endpoint-and-home-hardening.
    rationale: The report brings a new test-double form to this category, dependency injection at the accept loop with no env, flag or feature seam (report.md:13), the decision over injected euids (report.md:12), and the wiring no committed test repeats because no seam may be added, owed to §1's untestable zone by this wrap's ruling (report.md:54).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 21
    section: "## 8. Mocking & Stubbing Discipline"
    change: In the Filesystem faults row's Pattern cell add, before the clause on a test that widens a home tree above `bin/<key>/conpty/`, the clause; a loosened socket directory is made on the real directory under a test-owned `SocketRoot` (`SOCKET_ROOT_VAR`, `TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), at 0755 or as a symlink to a 0700 directory (`tests/security_negatives_socket_dir.rs` `Form`, each read back as found), and refused by the product's own check, never a stubbed one; a foreign owner, which one account cannot make, is an injected uid to the pure `socket_dir_verdict` at the unit tier (`socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`);
    sidecar: §8 Filesystem faults gains the loosened socket directory on the real file and the foreign owner as an injected uid at the unit tier, chunk 2026-11-01-unix-endpoint-and-home-hardening.
    rationale: The row names home trees only, while the chunk's new fault is a socket directory made on the real path (report.md:21, report.md:107) with the foreign owner held by an injected uid (report.md:11, report.md:107, report.md:130).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 20
    section: "## 8. Mocking & Stubbing Discipline"
    change: In the Fake servers row's Pattern cell add, before the closing clause on HTTP fake servers, the clause; a test server's endpoint directory is made through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`, which reads 0755 on a default umask (tempfile 3.27) and which a client's socket-directory check refuses before it reaches the server check (`crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs`, `refused_inside` in `src/cmd/client.rs`);
    sidecar: §8 Fake servers gains the rule that a test server's endpoint directory is made private, a bare tempdir meeting the new socket-directory check, chunk 2026-11-01-unix-endpoint-and-home-hardening.
    rationale: The plan's reading of a 0700 tempdir was measured false, the new client check refused nine handlers cases on the first run, and four sites narrowed (report.md:56, report.md:96); the check runs ahead of the euid read and verify_server (report.md:15), so an impostor in a bare tempdir would not exercise the server check this row describes.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56
note: no retired item or moved count stands in this file; line 20's (Linux and Windows) stays true, the server-identity negatives compiled out on macOS (report.md:110).
