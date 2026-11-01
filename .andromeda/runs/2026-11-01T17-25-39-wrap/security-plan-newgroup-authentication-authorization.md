# New groups — security-plan · authentication-authorization

## Verdicts
- fanout-security-plan--cascade-g1.md 9 · distinct · line 18 holds only the lstat requirement; no group states absent passes, the `Refused` variants of the directory check or `check_socket_dir`
- fanout-security-plan--cascade-g1.md 10 · distinct · the as-found statements in lines 35–36 are of the home and instance checks; none is of the socket-directory check
- fanout-security-plan--cascade-g1.md 11 · distinct · no group names `check_endpoint_dir` or a helper shared by the bind and the client seams
- fanout-security-plan--cascade-g1.md 12 · contradiction · lines 18–18 · «a failure at `bind_endpoint`'s `create_private_dir` has no refusal code» · the line gives a failed create no refusal code, its code still owed, while the proposal has a failed create and a refused check read one code
- fanout-security-plan--cascade-g1.md 13 · distinct · no group records a leftover socket file or a killed wrapper; line 18's `try_overwrite` stays off names a listener option, not this fact, though the orchestrator may weigh it against the takeover by a next bind
- fanout-security-plan--cascade-g1.md 14 · distinct · no group records the provenance of `socket-dir-refused`, `server-verify-failed` on Linux and macOS, the board's read rule or the P5 word
- fanout-security-plan--cascade-g1.md 16 · distinct · line 21 holds only the drop before a frame; the `accept_loop` filter on the accept thread, the peer's end-of-file and the silence stand nowhere
- fanout-security-plan--cascade-g1.md 19 · contradiction · lines 24–24 · «the client-side euid check is not built» · the line says the client euid check is not built, while the proposal's order has a peer euid decision on every client seam
- fanout-security-plan--cascade-g1.md 22 · contradiction · lines 35–36 · «`viola_state::board::scan` runs `check_path` on the home and on `instances/`» · the lines hold the board's home to `check_path`, the write rule, and carry that as an owed inconsistency, while the proposal has `scan` hold the home to `check_read` on every OS
- fanout-security-plan--cascade-g1.md 23 · contradiction · lines 24–24 · «the client-side euid check is not built» · the line (and line 85's CARRY) says the client euid check is not built and owed, while the proposal states it built as `peer_euid_admitted` over the read euid
- fanout-security-plan--cascade-g1.md 30 · distinct · lines 24 and 85 hold the macOS pid gap and the compile-out, but no group says it is owned by no route entry or that `server_verdict_rows` keeps its row
- fanout-security-plan--cascade-g1.md 31 · contradiction · lines 85–85 · «the per-user socket directory loosened to 0755 giving exit 21» · the line counts the macOS 0755 case as a CARRY still owed, while the proposal has the 0755 negatives held on `macos-latest`
- fanout-security-plan--cascade-g1.md 32 · distinct · no group states that a foreign euid and a foreign owner are held by injected values and never driven
- fanout-security-plan--cascade-g1.md 34 · distinct · line 88 holds the test going past every CLI check, but no group says no check sits inside `Client::connect` or why
