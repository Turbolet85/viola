@apply_row n=1 ref_file=fanout-security-plan--cascade.md ref_n=1 v=applied edit=E1
@fact id=fct-ghctc7rk expect_rev=1
old: `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)
new: `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`); `tests/contract_os_gated_tests.rs` (`GATES`, each test this carve-out keeps off macOS a row under its one reason `NO_PEER_PID`)
