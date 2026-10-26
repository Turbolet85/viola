# Task — test-plan · 2026-10-26-server-verification-before-any-frame

## The session
Every frame-writing client now checks the wrapper's identity before its first frame: the CLI verbs send, wait, last, answer, pause and release, the hook (event and dialog), and the four MCP frame tools. The check compares the connected peer's pid and the OS start time with the one snapshot read.
On a mismatch, or when the check cannot be made, the CLI exits 21 with the unchanged instance-unreachable document, adds a new server-verify hint, and writes a process-exit line with detail server-verify-failed during connect. The hook fails open with hook-decision server-verify-failed. MCP returns isError instance-unreachable and writes no line.
This retires the server-identity half of the dated gaps on Linux and Windows. The strict-modes half stands until "Home and code-bearing file integrity". macOS stays on the recorded gap, because it reports no peer pid.
New tests: the root binary security_negatives_server_identity (compiled out on macOS), impostor rows in the hook fail-open matrices, an MCP impostor handler case, a server_verdict decision table, server_pid tests per OS, and captured-span cases for each verb.
Measured: the snapshot readers are now three product callers, not the five the masters state. The E5 reader move is re-pinned to "Home and code-bearing file integrity" and held for the founder's word, with fork 1 PROVISIONAL.
Also folded in: a test-only ETXTBSY fix in the fake-agent drift contract test, and a macOS-target check-only clippy host recipe that reproduced CI's lint (macos-latest) red, which this chunk caused and then fixed.

## Reached files
- tests/security_negatives_server_identity.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/lib.rs
- crates/viola-state/src/liveness.rs
- src/cmd/client.rs
- src/human/hints.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/contract_fake_agent_drift.rs

## Touches
- §6 E5 As landed (test-plan.md:590, e2e-test-strategy E5 :235): it says steps 1–3 and the move of the other snapshot readers onto the replay are owed to "Server verification before any frame". That entry closes without them. They are re-pinned to "Home and code-bearing file integrity", held for the founder's word (fork 1 PROVISIONAL).
- The snapshot-readers count (e2e-test-strategy :235, unit-test-strategy :18): both name five product callers, src/cmd/client.rs among them and src/cmd/hook.rs twice. The measured count is three: src/cmd/hook.rs:204, src/cmd/hook/statusline.rs:142 and src/cmd/run.rs:379. src/cmd/client.rs holds no read_snapshot call, and hook.rs's event and dialog paths now share one read.
- §6 Security sweep, the hook fail-open matrix (test-plan.md:598, e2e-test-strategy suite-security-sweep :242, :244): its list of causes lacks the server-verification mismatch. The text at :242 also states an 11-case matrix, a count the masters must not state: name the functions instead.
- §6 Exit-cause matrix (test-plan.md:604, e2e-test-strategy suite-exit-cause-matrix :261, :264): it places the server-verification case in cross_exit_causes.rs, as an rstest table row. That file does not exist. The case landed in tests/security_negatives_server_identity.rs, and the macOS 0755-directory case is not built.
- §6 Security control negatives (test-plan.md:606, e2e-test-strategy suite-security-negatives): the security_negatives_*.rs control list gains the server-identity negative and its control.
- §5 integration-test-strategy :47: it says the MCP impostor case is in cross_exit_causes.rs with the rmcp client. It landed in crates/viola-mcp/tests/handlers.rs and drives the transport-free Tools handle.
- §5 MCP handler tests (test-plan.md:449): still true, and handlers.rs gains the impostor case and a held-child helper.
- §5 Wrapper channel (test-plan.md:435) and §1 viola-channel entity (test-plan.md:46, test-scope-summary :11): server verification, listed there as testable, is now built and tested on Linux and Windows. macOS has no peer pid.
- §1 OS-branch trigger (test-plan.md:250) and §9 OS matrix (test-plan.md:767): both name macOS euid + dir-only server verification. That check is still not built and is owned by "Unix endpoint and home hardening". Re-read: the claim stands.
- §4 viola-channel unit coverage (test-plan.md:376): it gains the connected stream's peer pid (Client::server_pid, TokioClient::server_pid, SERVER_PID_REPORTED).
- §4 viola-state unit coverage (test-plan.md:378): the liveness classifier gains the pure server_verdict decision table and verify_server. live_endpoint now returns a LiveEndpoint struct.
- §4 root bin unit coverage (test-plan.md:386): it gains the closed Unreachable cause (instance-dead or server-verify-failed), the verified connect, and the captured-span cases for each verb.
- §6 Paths 2–5 (the killed-wrapper exit 21 / MCP instance-unreachable signals, Path 3 at test-plan.md:504): a server that is not the recorded wrapper now yields the same exit 21 document and MCP result.
- §6 Error sanitization and secret scan (test-plan.md:600): the new negatives assert that no pid, start time, path or endpoint reaches any output or line (digit_runs).
- §6 Schema conformance (test-plan.md:602): process-exit and hook-decision lines with detail server-verify-failed are written for the first time. The schema enums already held the value.
- §6 Contract suite (test-plan.md:617): the fake-agent agents-replay contract test plants every set's binary sequentially before its threads start, which closes an ETXTBSY flake.
- §10 Perf (test-plan.md:835, quality-gates-coverage-targets gate-perf): the hook rows now time the server check in path. The fail-open matrix's < 1.0 s Instant bound covers the impostor rows too. The perf rows and the gate are unchanged.
- §9 Lint (test-plan.md:741, ci-integration): the dev-host Windows-target clippy is named as not a gate. A macOS-target check-only clippy joins it as a host recipe, also not a gate.
- §2 Unit pyramid (test-plan.md:285): new test modules compile out on macOS whole (last.rs and pause.rs, refused_inside, security_negatives_server_identity, the impostor rows).
- §7 fake-agent data (test-data-fixtures data-fake-agent): the fake agent's holder mode --hold-stdout-internal now serves as the held recorded-pid process in the new negatives and handler tests.
- The Windows mutation-scope contract (contract_windows_mutation_scope): its is_windows_gate reads cfg!(windows) as a Windows gate, and a second occurrence of that is now measured. The new test-only helper was restacked to avoid it.

## New facts
- F1 · e2e-test-strategy · tests/security_negatives_server_identity.rs is a root test binary, compiled out on macOS. security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper runs send, wait, last, answer, pause and release in --json and human modes, against a server that is not the recorded wrapper. Expected: exit 21, the exact instance-unreachable document (stderr empty under --json), the server-verify hint last on stderr, one process-exit line with server-verify-failed during connect, and no frame.
- F2 · e2e-test-strategy · The control security_negative_cli_verb_reaches_the_recorded_wrapper: each of the six verbs, in both modes, reaches the recorded wrapper with one frame.
- F3 · e2e-test-strategy · The hook fail-open matrices gain impostor rows, gated off macOS: case::impostor (stop) in hook_fails_open_silently_within_the_spine_bound, and question_impostor and permission_impostor in hook_dialog_tier_fails_open_silently. Each: exit 0, empty streams, under 1.0 s, no frame, hook-decision server-verify-failed.
- F4 · e2e-test-strategy · Exit 21's server-verification cause has its own hint, `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` (hint cause server-verify), and its own log detail, server-verify-failed. Its --json document is the unchanged instance-unreachable one.
- F5 · e2e-test-strategy · The macOS server-verification cases are owed to "Unix endpoint and home hardening": the 0755 socket-directory exit-21 case and the client euid check. This chunk's negatives compile out on macOS.
- F6 · e2e-test-strategy · A hook reaching an exiting wrapper's own Windows pipe passes the check by design. Measuring its SessionEnd outcome in that window is owed to "Windows-only live measurements".
- F7 · integration-test-strategy · handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper is in crates/viola-mcp/tests/handlers.rs. It runs MCP send, wait, last and answer against a server that is not the recorded wrapper. Expected: isError, the instance-unreachable code with detail null, text refused: instance-unreachable, no frame, no log line.
- F8 · unit-test-strategy · viola-state liveness has ServerVerdict (Matches, Mismatch, Unverifiable) and the pure server_verdict decision table, server_verdict_rows. Where no peer pid is reported (macOS) the verdict is Matches with no comparison. verify_server_reads_the_observed_start_time_from_the_os covers the OS start-time read.
- F9 · unit-test-strategy · viola-channel: client_server_pid_is_the_serving_process and client_server_pid_is_none_where_peer_credentials_hold_no_pid, with Tokio twins, assert the peer pid on each OS. SERVER_PID_REPORTED is false on macOS.
- F10 · unit-test-strategy · Root bin: request_refuses_an_unverified_server_inside_its_dispatch_span exists in each of send, wait, last, answer, pause and release. hook_refuses_an_unverified_server_inside_hook_handle covers the hook. All go through the shared test-only cmd::client::refused_inside helper (assert_refused_inside). unreachable_logs_its_cause logs each closed cause.
- F11 · unit-test-strategy · Hook unit: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper, and a SessionEnd whose server check fails takes the direct append. The unit impostor varies the recorded start time rather than recording another live pid.
- F12 · quality-gates-coverage-targets · With the server check in the timed path, the run --perf hook rows read max 2.2–2.4 ms over 30 runs on the Linux dev host.
- F13 · ci-integration · Host recipe, not a gate: the macOS-target check-only clippy (rustup target aarch64-apple-darwin, --target aarch64-apple-darwin, CARGO_TARGET_DIR=target/maccheck). It reproduces CI's lint (macos-latest) line for line.

## Untouched
- §3 Test Harness Contract and its registry keys: no script, harness command, session record or exit code changed.
- §9 CI workflow jobs, uploads and secret scans: no .github file changed, and the check set stays 15.
- §8 Mocking & Stubbing Discipline: no new mockall seam. The impostor is a real endpoint, not a double.
- §6 Path 1 run start sequence: run's collision check (src/cmd/run.rs) is unchanged and makes no server check.
- §6 Path 6 budget governor and the statusline arm: src/cmd/hook/statusline.rs is not reached.
- §6 Path 7 version gate, E1 unwrapped hooks, E2 R8 strip, E3 link / unlink and E4: none changed.
- §6 list row integrity, chaos and property suites: no case added or moved.
- §7 recorded fixtures under fixtures/claude/<cli-version>/, including the Mcp.<variant>.json exchanges: none added or re-recorded.
- §10 coverage thresholds and the mutation gate: unchanged.
- Dependency and tool pins (interprocess 2.4.4, tokio, sysinfo, cargo-nextest, cargo-deny, the toolchain): unchanged, and viola-channel's normal graph stays tokio-free.
- viola revive and viola list tests (cli_revive.rs, cli_list.rs): not reached.
