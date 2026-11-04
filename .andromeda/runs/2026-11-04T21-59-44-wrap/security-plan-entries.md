@amendment marker=2026-11-04-linux-and-macos-parity
title: the macOS carve-out's tests named in the census
section: Authentication & Authorization (authentication-authorization.md: the macOS carve-out's Stack reference)
change:
  - The carve-out's Stack reference names `tests/contract_os_gated_tests.rs` beside `server_verdict_rows` and `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`): its `GATES` table holds a row for each test this carve-out keeps off macOS, all under the one reason `NO_PEER_PID` (was `server_verdict_rows` and `tests/security_negatives_server_identity.rs` alone).
why: the report's Symbols / APIs bullet: the new census `tests/contract_os_gated_tests.rs` gives every OS-gated test function and file a `GATES` row with its reason, `NO_PEER_PID` the reason for the tests this carve-out keeps off macOS. The carve-out itself is unchanged; the only change is the added stack reference.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
