# Apply — test-plan · part 1 of 5

## Verdicts
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan.md:88#1
- fanout-test-plan.md 1 · folded into fanout-test-plan--cascade-g1.md 3
- fanout-test-plan--cascade-g1.md 10 · applied · .andromeda/test-plan.md:135#1, .andromeda/test-plan.md:135#2
- fanout-test-plan--cascade-g1.md 17 · applied · .andromeda/test-plan.md:380#1
- fanout-test-plan.md 7 · folded into fanout-test-plan--cascade-g1.md 17
- fanout-test-plan--cascade-g1.md 20 · applied · .andromeda/test-plan.md:382#1
- fanout-test-plan.md 8 · folded into fanout-test-plan--cascade-g1.md 20
- fanout-test-plan--cascade-g1.md 24 · applied · .andromeda/test-plan.md:393#1
- fanout-test-plan.md 9 · folded into fanout-test-plan--cascade-g1.md 24
- fanout-test-plan--cascade-g3.md 3 · applied · .andromeda/test-plan.md:604#1
- fanout-test-plan--cascade-g3.md 13 · applied · .andromeda/test-plan.md:610#1, .andromeda/test-plan.md:610#2
- fanout-test-plan.md 12 · folded into fanout-test-plan--cascade-g3.md 13
- fanout-test-plan--cascade-g3.md 18 · applied · .andromeda/test-plan.md:612#1, .andromeda/test-plan.md:612#2
- fanout-test-plan.md 13 · folded into fanout-test-plan--cascade-g3.md 18
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:484#1
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:186+#1
- fanout-test-plan.md 10 · applied · .andromeda/test-plan.md:457#1
- fanout-test-plan.md 11 · applied · .andromeda/test-plan.md:453#1
- fanout-test-plan.md 14 · applied · .andromeda/test-plan.md:618#1

## Edits
- line: .andromeda/test-plan.md:88
  old: «Hosted runners provide one account (security excerpt, Vectors 1 and 7).»
  new: «Hosted runners provide one account (security excerpt, Vectors 1 and 7), and no seam may be added, so each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed test, read only through a forced refusing decision outside the suite, and a mutant replacing a seam's `peer_admitted` read with an always-admit survives every committed test; the committed tests hold the decision over injected euids, the real euid read on Linux and macOS and the wrapper's drop over an injected decision.»
- line: .andromeda/test-plan.md:135
  old: «version gate, exclusive endpoint bind»
  new: «version gate, the socket directory check, exclusive endpoint bind»
- line: .andromeda/test-plan.md:135
  old: «a squatted endpoint, a tampered pinned exe or a refused home tree exits 1»
  new: «a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree exits 1»
- line: .andromeda/test-plan.md:380
  old: «the peer-credential decision with injected euids,»
  new: «the peer-credential decision with injected euids (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`), the real euid read on the sync and Tokio clients (`client_server_euid_is_this_user`, `tokio_client_server_euid_is_this_user`), the wrapper's drop of a refused peer before any frame (`serve_drops_a_refused_peer_before_any_frame`, its control `channel_endpoint_answers_protocol_faults`),»
- line: .andromeda/test-plan.md:382
  old: «the home-tree and diagnostics checks,»
  new: «the home-tree and diagnostics checks, the socket directory verdict and checks (`socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`, `check_endpoint_dir_reads_the_endpoints_directory`, `check_socket_dir_off_unix_reads_nothing`), the board scan holding the home to the read rule (`board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`, on `windows-2025` `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`),»
- line: .andromeda/test-plan.md:393
  old: «with each closed unreachable cause logged,»
  new: «with each closed unreachable cause logged, the socket directory check refused inside `connect`'s dispatch span and the hook's (`connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, both running on macOS too),»
- line: .andromeda/test-plan.md:604
  old: «a server that is not the recorded wrapper, each exit 0 with no body»
  new: «a server that is not the recorded wrapper and, on Unix, a socket directory the check refuses or a server of another euid (its decision held by injected euids, §1 Untestable zones), each exit 0 with no body»
- line: .andromeda/test-plan.md:610
  old: «(its macOS case owed to the route entry "Unix endpoint and home hardening")»
  new: «(macOS compares no peer pid, the named carve-out `server_verdict_rows` holds, so these negatives compile out there; a loosened socket directory on Linux and macOS is a case of `tests/security_negatives_socket_dir.rs`)»
- line: .andromeda/test-plan.md:610
  old: «its control `shared_hints_reads_a_planted_duplicate`); the causes outside it»
  new: «its control `shared_hints_reads_a_planted_duplicate`); the table holds the sixth closed cause `socket-dir-refused`, at exit 21 on a frame verb (`a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused`) and at exit 1 on a start (`a_loosened_socket_directory_on_a_start_is_socket_dir_refused`); the causes outside it»
- line: .andromeda/test-plan.md:612
  old: «the Unix socket directory refusal, Windows client SQOS»
  new: «the Unix socket directory refusal (`tests/security_negatives_socket_dir.rs`, Linux and macOS: a directory at 0755 or a symlink to a 0700 directory refuses `viola run` exit 1 before any bind, the eight frame verbs exit 21 with no frame and the hook exit 0 with empty streams and no frame, each leaving it as found and naming no path, pid or uid, and a clean start under an open umask keeps the socket private), Windows client SQOS»
- line: .andromeda/test-plan.md:612
  old: «a pre-existing home another user may read refusing `viola run` exit 1,»
  new: «a pre-existing home another user may read refusing `viola run` exit 1 and `viola list` exit 21 (`list_refuses_a_home_another_user_may_read_with_and_without_an_instance`),»
- line: .andromeda/test-plan.md:484
  old: «and same-user SDDL read-back (§5).»
  new: «and same-user SDDL read-back (§5); the euid wiring at the three client seams is held by no committed test (§1 Untestable zones).»
- after: .andromeda/test-plan.md:186
  anchor: «    - Unix socket dir 0700 and file 0600 asserted»
  add: «    - a socket directory at 0755 or a symlink to a 0700 directory → refused and left as found: `run` exit 1 with no bind and no child, a frame verb exit 21 with no frame, the hook exit 0 with empty streams and no frame, MCP `isError` with no frame»
  add: «    - a foreign owner at the socket directory verdict, by injected uid»
  add: «    - a peer of another euid, decided by injected euids → a client reads `server-verify-failed`, the wrapper drops the peer before any frame»
- line: .andromeda/test-plan.md:457
  old: «The full stdio path is E2E (§6).»
  new: «They hold the frame tools refusing a loosened socket directory (`handlers_frame_tools_refuse_a_loosened_socket_directory`: `isError` with `socket-dir-refused`, no frame and no line). The full stdio path is E2E (§6).»
- line: .andromeda/test-plan.md:453
  old: «no setting that disables a control, and the home resolution order»
  new: «no setting that disables a control (the socket directory check by `tests/cli_controls_not_disableable.rs` `setting_does_not_disable_the_socket_dir_check`), the board on the read rule on all three OSes (`tests/cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`), and the home resolution order»
- line: .andromeda/test-plan.md:618
  old: «`handlers_frame_tools_name_the_cause_of_an_unreachable_instance`, `mcp_tools_a_wrapper_killed»
  new: «`handlers_frame_tools_name_the_cause_of_an_unreachable_instance`, `handlers_frame_tools_refuse_a_loosened_socket_directory`, `mcp_tools_a_wrapper_killed»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = 'its macOS case owed to the route entry'
