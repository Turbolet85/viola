# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · the session adds no event kind, module edge or tool version; viola-core gains only the sixth exit cause, which this boundary procedure does not name
integration-test-strategy#1.2 · no · the concurrent-append check is unchanged; the new hook refusal line is a short hook-decision with no path, pid or uid, so the panic line stays the only hook detail line over 4 KiB
integration-test-strategy#1.3 · no · child processes remain the only external boundary and the fake agent is untouched
integration-test-strategy#1.4 · no · the server's bind, squatter and chmod-after-bind read-back are unchanged; the new socket-dir check and its narrowed test sites sit on the CLI, MCP and hook side
integration-test-strategy#1.5 · no · the run start's socket-dir check is Unix-only and returns Ok on Windows, so the sideload and ConPTY claims hold
integration-test-strategy#1.6 · no · the instance check still runs first and a no-instance VIOLA_DIR still writes nothing; the socket-dir refusal sits after it and the hook rows live in the new binary
integration-test-strategy#2.1 · no · the session does not reach crates/viola-state/src/fs.rs, where the placed-copy witness lives
integration-test-strategy#3.0 · yes · add-to · F11: a bare tempfile::tempdir() reads 0755 and now meets the socket directory check as a plain home meets the read rule, so the bare temp-dir claim extends to a socket directory and endpoint dirs go through create_private_dir
integration-test-strategy#4.1 · yes · stale · the macOS euid and directory check is no longer owed to the retired route entry, and server-verify-failed now reaches macOS as an euid mismatch; also carries F8 (MCP frame tools refuse a loosened socket directory) and F12 (the raw client has no directory check)
integration-test-strategy#4.2 · no · the strict-modes refusal rules and their no-line claims hold; the socket-dir refusal is a separate cause the entity does not name
integration-test-strategy#5.1 · yes · add-to · F9 setting_does_not_disable_the_socket_dir_check joins tests/cli_controls_not_disableable.rs; F10 list_refuses_a_home_another_user_may_read_with_and_without_an_instance joins tests/cli_list.rs
