# Apply — test-plan · part 2 of 5

## Verdicts
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:12#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:12+#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:34#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:43#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:69#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/test-scope-summary.md:101#1
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan-detail/test-scope-summary.md:128#1, .andromeda/test-plan-detail/test-scope-summary.md:128#2
- fanout-test-plan--cascade-g1.md 9 · applied · .andromeda/test-plan-detail/test-scope-summary.md:134+#1
- cascade-compare-test-plan.md 7 · applied · .andromeda/test-plan-detail/test-scope-summary.md:122#1
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/test-strategy.md:59#1, .andromeda/test-plan-detail/test-strategy.md:59#2
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/test-strategy.md:64#1
- fanout-test-plan--cascade-g1.md 13 · applied · .andromeda/test-plan-detail/test-strategy.md:74#1, .andromeda/test-plan-detail/test-strategy.md:74#2
- cascade-compare-test-plan.md 8 · applied · .andromeda/test-plan-detail/test-strategy.md:10#1
- cascade-compare-test-plan.md 9 · applied · .andromeda/test-plan-detail/test-strategy.md:29#1
- cascade-compare-test-plan.md 10 · applied · .andromeda/test-plan-detail/test-strategy.md:43+#1

## Edits
- line: .andromeda/test-plan-detail/test-scope-summary.md:12
  old: «where the client reads no peer pid; the macOS cases are owed to the route entry "Unix endpoint and home hardening".»
  new: «where the client reads no peer pid, a named carve-out owned by no route entry; there `server-verify-failed` means an euid mismatch alone, its decision and its real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`.»
- after: .andromeda/test-plan-detail/test-scope-summary.md:12
  anchor: «  - **As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, `detail` `server-verify-failed` under `--json`, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable` with `detail` `server-verify-failed`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed to the route entry "Unix endpoint and home hardening".»
  add: «  - **As landed (the euid wiring, untestable):** each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed witness, since one account can make no foreign peer and no seam may be added; it is read only through forced refusing decisions, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening (`evidence/remove-the-guard.md` rows 7, 8 and 9). The decisions over injected euids and the wrapper's drop over an injected decision are committed tests (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `serve_drops_a_refused_peer_before_any_frame`).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:34
  old: «`pinned-hash-mismatch`, `squatted-name`, `internal-error`»
  new: «`pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`»
- line: .andromeda/test-plan-detail/test-scope-summary.md:43
  old: «(its ten codes)»
  new: «(its eleven codes)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:69
  old: «`endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`»
  new: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, the CLI exit-21 document less `v`»
- line: .andromeda/test-plan-detail/test-scope-summary.md:101
  old: «and the Unix socket file or pipe no longer exists.»
  new: «and on Unix a connect to the socket path is refused or the path is absent, on Windows the pipe no longer exists; `cleanup` removes no leftover socket file, so a force-killed wrapper's file stays until a next bind of that name takes it over.»
- line: .andromeda/test-plan-detail/test-scope-summary.md:122
  old: «whichever SID holds it).»
  new: «whichever SID holds it). A test placing an endpoint in a directory of its own makes that directory the same way: a bare `tempfile::tempdir()` takes its mode from the umask (`tempfile` 3.27, 0755 on a default umask, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening), and the socket-directory check refuses it.»
- line: .andromeda/test-plan-detail/test-scope-summary.md:128
  old: «version gate, exclusive endpoint bind,»
  new: «version gate, on Unix the socket directory check (`socket_dir_ready`: the endpoint's parent checked, created only when absent, then checked again), exclusive endpoint bind,»
- line: .andromeda/test-plan-detail/test-scope-summary.md:128
  old: «A second `run` of the same name, or a squatted endpoint, exits 1.»
  new: «A second `run` of the same name, a squatted endpoint or a refused socket directory exits 1.»
- after: .andromeda/test-plan-detail/test-scope-summary.md:134
  anchor: «    - A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1. A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).»
  add: «    - A refused socket directory (at 0755, or a symlink to 0700) exits 1 on `viola run` and on `revive`'s start, with no bind and no child, left as found, and one `process-exit` line of `detail` `socket-dir-refused` (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`; `tests/cross_exit_causes.rs` `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`); a `Server::bind` failure is no socket-directory refusal.»
- line: .andromeda/test-plan-detail/test-strategy.md:10
  old: «the macOS `server_pid` case still run there.»
  new: «the macOS `server_pid` case still run there, and so do the socket-directory check's captured-span cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, which read the span recorder `Seen` from the test module `refused_seen` in `src/cmd/client.rs`, outside the macOS-gated `refused_inside`.»
- line: .andromeda/test-plan-detail/test-strategy.md:29
  old: «as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).»
  new: «as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`). A known survivor stands against the `missed` rule: a mutant replacing a client seam's `peer_admitted` read (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) with an always-admit survives every committed test by construction (§1 `viola-channel`, **As landed (the euid wiring, untestable)**), and the mutation audit reads that survivor at the route entry "Version done-check".»
- after: .andromeda/test-plan-detail/test-strategy.md:43
  anchor: «  - **Shared start helpers:** `tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`.»
  add: «  - **Socket-root helpers:** `tests/support/home.rs` holds the test-owned socket root: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`; `first_life_on` stands there, not beside `first_life` in `tests/support/start.rs`.»
- line: .andromeda/test-plan-detail/test-strategy.md:59
  old: «(the MCP impostor case, `crates/viola-mcp/tests/handlers.rs`)»
  new: «(the MCP impostor case and the socket-directory refusal `handlers_frame_tools_refuse_a_loosened_socket_directory`, `crates/viola-mcp/tests/handlers.rs`)»
- line: .andromeda/test-plan-detail/test-strategy.md:59
  old: «a CLI frame verb against a server that is not the recorded wrapper; root `channel_links.rs`»
  new: «a CLI frame verb against a server that is not the recorded wrapper; root `security_negatives_socket_dir.rs`, Unix: a socket directory at 0755 or a symlink to 0700 refusing a start, the eight CLI frame verbs and the hook, each left as found; root `channel_links.rs`»
- line: .andromeda/test-plan-detail/test-strategy.md:64
  old: «each byte-identical after every other writer-capable entry point)»
  new: «each byte-identical after every other writer-capable entry point; root `cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`: a home another user may read refusing `viola list`, with and without an instance)»
- line: .andromeda/test-plan-detail/test-strategy.md:74
  old: «`cli_error_documents_hold_no_path_or_upstream_text`, root `channel_error_bodies.rs`»
  new: «`cli_error_documents_hold_no_path_or_upstream_text` beside its sibling `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`, root `channel_error_bodies.rs`»
- line: .andromeda/test-plan-detail/test-strategy.md:74
  old: «root `contract_error_bodies.rs`, the enumeration and the census of writers»
  new: «root `contract_error_bodies.rs`, the enumeration, the socket-directory document a row of it, and the census of writers»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = 'the macOS cases are owed to the route entry'

[[pattern]]
id = "tp2-02"
fixed = '`squatted-name`, `internal-error`'

[[pattern]]
id = "tp2-03"
fixed = 'its ten codes'

[[pattern]]
id = "tp2-04"
fixed = '`server-verify-failed`, the CLI exit-21 document less'

[[pattern]]
id = "tp2-05"
fixed = 'the Unix socket file or pipe no longer exists'

[[pattern]]
id = "tp2-06"
fixed = 'or a squatted endpoint, exits 1'
