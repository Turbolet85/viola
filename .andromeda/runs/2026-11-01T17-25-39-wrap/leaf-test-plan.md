group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 107 · «- 2026-10-31: A case that runs on every OS must not call a h»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»
- leaf: .claude/docs/tests-summary.md:25   class: omission
  leaf-text: «duplicate / squatted / tampered-exe starts and a refused home tree exit 1»
  source: .andromeda/test-plan.md:135 «a second `run` of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree exits 1.»
  replace-with: «duplicate / squatted / tampered-exe starts, a refused socket directory (Unix, checked before the bind) and a refused home tree exit 1»
- leaf: .claude/docs/tests-summary.md:33   class: omission
  leaf-text: «on Linux and Windows), security-control negatives, `list` row integrity»
  source: .andromeda/test-plan.md:613 «the table holds the sixth closed cause `socket-dir-refused`, at exit 21 on a frame verb»
  replace-with: «on Linux and Windows, and the table holds the sixth closed cause `socket-dir-refused`, at exit 21 on a frame verb and at exit 1 on a start), security-control negatives (the Unix socket directory refusal in `tests/security_negatives_socket_dir.rs`, Linux and macOS), `list` row integrity»
- leaf: .claude/docs/tests-summary.md:42   class: omission
  leaf-text: «0 missed beyond the host-excluded mutants the harness leaves out, 0 timeout, unviable ≤ caught at a `run --mutants` (the epoch-boundary audit, or on demand)»
  source: .andromeda/test-plan-detail/test-strategy.md:29 «A known survivor stands against the `missed` rule: a mutant replacing a client seam's `peer_admitted` read»
  replace-with: «0 missed beyond the host-excluded mutants the harness leaves out, 0 timeout, unviable ≤ caught at a `run --mutants` (the epoch-boundary audit, or on demand); one known survivor stands against the `missed` rule: a mutant replacing a client seam's `peer_admitted` read (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) with an always-admit survives every committed test by construction, one account making no foreign peer and no seam allowed, and the mutation audit reads it at the route entry "Version done-check"»
- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «the one complete-lines reader in `ndjson.rs`»
  source: .andromeda/test-plan-detail/test-strategy.md:44 «`tests/support/home.rs` holds the test-owned socket root: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`»
  replace-with: «the test-owned socket root in `home.rs` (`SOCKET_ROOT_VAR`, `TMPDIR` on macOS, else `XDG_RUNTIME_DIR`; `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not beside `first_life` in `start.rs`), the one complete-lines reader in `ndjson.rs`»
- leaf: .claude/rules/testing.md:33   class: omission
  leaf-text: «and CI's G2/G4/secret scan cover it.»
  source: .andromeda/test-plan-detail/test-scope-summary.md:123 «A test placing an endpoint in a directory of its own makes that directory the same way: a bare `tempfile::tempdir()` takes its mode from the umask»
  replace-with: «and CI's G2/G4/secret scan cover it. A test placing an endpoint in a directory of its own makes that directory through `viola_state::fs::create_private_dir` too: a bare `tempfile::tempdir()` takes its mode from the umask (`tempfile` 3.27, 0755 on a default umask), and the socket-directory check refuses it.»
- leaf: .claude/rules/testing.md:46   class: omission
  leaf-text: «zero missed beyond the host-excluded mutants the harness leaves out and names in `host_excluded`, zero timeout, unviable ≤ caught;»
  source: .andromeda/test-plan-detail/test-strategy.md:29 «A known survivor stands against the `missed` rule: a mutant replacing a client seam's `peer_admitted` read»
  replace-with: «zero missed beyond the host-excluded mutants the harness leaves out and names in `host_excluded`, zero timeout, unviable ≤ caught, one known survivor standing against the `missed` rule: a mutant replacing a client seam's `peer_admitted` read with an always-admit survives every committed test by construction, no seam may be added for it, and the mutation audit reads it at the route entry "Version done-check";»
- leaf: .claude/rules/verification-harness.md:34   class: stale
  leaf-text: «verify endpoint gone (CLI exit 21, socket path absent).»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:73 «on Unix a connect to the `<per-user 0700 dir>/viola-<h12>.sock` path is refused or the path is absent»
  replace-with: «verify endpoint gone (CLI exit 21; on Unix a connect to the socket path refused or the path absent, on Windows a client connect reading NotFound). Cleanup removes no leftover socket file: after a force-killed wrapper on Unix the file stays until a next bind of that name takes it over, and the document reads `endpoint_gone` true and `ok:true`.»
