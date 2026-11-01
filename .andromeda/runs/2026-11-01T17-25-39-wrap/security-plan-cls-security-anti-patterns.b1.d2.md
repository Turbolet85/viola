# Classified — security-plan · security-anti-patterns.b1

## Answers
security-anti-patterns#1.1 · yes · contradict · the session built every client's euid check and retired the route entry "Unix endpoint and home hardening", so the exception's "owed to" clause for the euid check no longer holds; only macOS's pid and start-time gap stays named
security-anti-patterns#2.1 · no · the report leaves the `run --mutants` scratch path unreached (Untouched), and the session builds nothing on the Windows-only scratch wipe
security-anti-patterns#3.1 · yes · stale · the change reaches `tests/support/home.rs` (`prepare_home_base`) and the harness `cleanup.rs`, the two keepers and the test-side removal, and the report does not show the link and removal rule holds on those files
security-anti-patterns#4.1 · no · the hook's new directory refusal sends no frame and leaves the decision null (`NotSent::SocketDir`, `decision_emitted` false), so a non-null decision still needs a stamp
security-anti-patterns#4.2 · no · `strict.rs` gains only `check_socket_dir` and the `NotADirectory` variant; `check_stamps` and verify's check order are unchanged, and the report leaves `viola verify` unreached
security-anti-patterns#5.1 · no · the ban already names the socket-directory check and peer verification; the session adds no config, env, flag or seam, and the seam carve-outs do not grow
