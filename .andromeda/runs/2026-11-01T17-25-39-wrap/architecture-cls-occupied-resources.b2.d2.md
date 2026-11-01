# Classified — architecture · occupied-resources.b2

## Answers
occupied-resources#26.1 · yes · stale · the session adds a socket-directory refusal at MCP's frame tools (`UnreachableCause::SocketDir`, F7), not only `strict-modes-failed`, and runs `socket_dir_ready` before run's and revive's start (F1).
occupied-resources#27.1 · no · the home's strict-modes reading under a regular file (NotFound on Windows, refusal on Unix) is unchanged; the new socket check is Unix-only and separate from the home check.
occupied-resources#28.1 · yes · contradict · the session moves `board::scan`'s home check to the read rule on every OS (F10), so the two home checks no longer disagree and the owed reconciliation is done.
occupied-resources#29.1 · no · the budget read still runs `check_path` on the home (the write rule), which F10 keeps for `read_budget_stamped`; the file and its order are unchanged.
occupied-resources#30.1 · no · the stamps path (`stamps`, `read_stamps_strict`) is untouched; the reached strict.rs gains only the socket-directory check, not the stamps' check.
occupied-resources#31.1 · no · the `ledger/probes/` directories are verify-only, and no reached file writes or removes them.
occupied-resources#32.1 · no · verify's interactive probe dirs sit outside the home and no reached file touches them.
occupied-resources#33.1 · yes · add-to · F13 and F14: a force-killed wrapper's socket file stays until a next bind takes it over (no remover, no owner), and harness `cleanup`'s `endpoint_gone` reads a connect refused or NotFound beside `holder_gone`'s record.
occupied-resources#34.1 · no · the error-body classes in `tests/support/events.rs` are not reached, and the new socket-dir refusal bodies carry no path, so the four classes hold.
occupied-resources#34.2 · no · the four checks over `tests/cross_exit_causes.rs` are generic over the table and hold for the sixth exit-21 cause; the row states no count.
occupied-resources#35.1 · yes · add-to · F15: test-only socket roots (`SocketRoot`, `SOCKET_ROOT_VAR`) in `tests/support/home.rs` sit under TMPDIR or XDG_RUNTIME_DIR, a second test-only /tmp use beside this one, which viola never names.
occupied-resources#36.1 · no · no fixture is reached (the fixtures row is untouched) and the session names no fixture.
occupied-resources#37.1 · no · the `target/agent-run/` session record, handshake files and artifacts keep their shape, and no reached file changes what is written there.
occupied-resources#38.1 · no · the e2e-home keepers, owner sweep and `remove_owned` in `tests/support/home.rs` and `cleanup` are not changed; the session's home.rs edits are additive socket-root helpers.
occupied-resources#39.1 · no · `.github/workflows/ci.yml` is not reached and the session changes no CI step.
occupied-resources#40.1 · no · `seed_conpty`'s `create_private_dir` use is unchanged; the session only adds test callers of that function.
occupied-resources#41.1 · no · the profraw census script is not reached and runs no path the session changes.
occupied-resources#42.1 · no · the mutants scratch uses `create_private_dir`, which the session does not edit (it gains test callers only).
occupied-resources#43.1 · no · the protected home DACL is a Windows measurement, and the session's socket check returns Ok(()) off Unix.
occupied-resources#43.2 · no · the Windows inherited-grant test in strict.rs is untouched, since `check_socket_dir` returns Ok(()) off Unix and the plain-home case reads as before.
