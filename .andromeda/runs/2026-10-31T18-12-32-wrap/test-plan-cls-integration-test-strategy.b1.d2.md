# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · yes · stale · Cargo.toml and .config/nextest.toml are reached whole and the report does not show the rstest 0.27.0 pin or the cargo-nextest tool held
integration-test-strategy#1.2 · yes · stale · Cargo.toml is reached whole and the report does not show the tempfile 3.27.0 and serde_json pins held
integration-test-strategy#1.3 · no · viola-fake-agent is untouched per the report (no mode, argument, receipt or script format moved), so the recorded, oversize and malformed modes and the test-scoped PATH copy hold
integration-test-strategy#1.4 · yes · stale · Cargo.toml is reached whole and the report does not show the interprocess 2.4.4, jsonschema 0.57.0 and windows-sys 0.61.2 pins held
integration-test-strategy#1.5 · yes · stale · Cargo.toml is reached whole and the report does not show the portable-pty =0.8.1 pin held
integration-test-strategy#1.6 · yes · stale · Cargo.toml is reached whole and the report does not show the assert_cmd 2.2.2 pin held; the hook fail-open claim itself holds, since hook.rs is untouched and hooks stay silent at the catch site
integration-test-strategy#2.1 · no · crates/viola-state/src/fs.rs is not in the reached files and the placed-copy witness names no changed code path
integration-test-strategy#3.0 · yes · add-to · F15 a Windows Serving has no Drop so its unserved handlers row's pipe lives to process exit, F16 the empty_home is viola-made where a bare 0755 temp dir was refused, F17 Frozen is a SIGSTOP Drop guard on Unix as well as Windows
integration-test-strategy#4.1 · yes · contradict · the MCP impostor and widened-instance cases no longer read detail null: they now read server-verify-failed and strict-modes-failed (F14)
integration-test-strategy#4.2 · yes · contradict · the MCP frame tools' detail null and link's and unlink's --json detail null are now strict-modes-failed on a widened instance, the six frame verbs and link reading that cause
integration-test-strategy#5.1 · yes · add-to · F11 verify writes its refusal code on process-exit beside interrupted, F12 a refused home tree under verify stays lineless and verify takes no --json, F13 verify_a_tampered_pinned_copy_refuses_with_its_code, verify_window_a_widened_stamps_file_refuses_with_its_code and the program-not-found row are new in tests/cli_verify.rs
