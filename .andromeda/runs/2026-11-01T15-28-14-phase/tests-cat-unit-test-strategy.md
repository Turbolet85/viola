# tests · Unit Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:18) The peer euid check on both sides must be held at this pure decision function with injected euids, the only layer that can reach the cross-user rejection.
  > The peer-credential decision function with injected `(peer_euid, own_euid)`, covering the cross-user rejection boundary from test-scope Sec 1.
- (.andromeda/test-plan-detail/unit-test-strategy.md:17) The directory verification must read the same three per-user directory forms these golden vectors pin; the vectors must stay green unchanged.
  > the Unix per-user 0700 directory forms `$XDG_RUNTIME_DIR/viola/viola-<h12>.sock` (Linux) and `$TMPDIR/viola/viola-<h12>.sock` (macOS), plus the `/tmp/viola-<uid>/viola-<h12>.sock` fallback (security IPC access control, Unix).
- (.andromeda/test-plan-detail/unit-test-strategy.md:20) The macOS pid gap must be recorded consistently with these per-OS unit cases: the peer pid stays `None` on macOS and `SERVER_PID_REPORTED` stays false, with no substitute value.
  > The connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), asserted on each OS by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature); `SERVER_PID_REPORTED` is false on macOS.
- (.andromeda/test-plan-detail/unit-test-strategy.md:29) The macOS carve-out must stand as this decision table's row, a `Matches` with no comparison named as the recorded gap, and no test may read it as a comparison.
  > where no peer pid is reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap.
- (.andromeda/test-plan-detail/unit-test-strategy.md:31) Making the board's home check one rule must choose between these two rules as defined, the read rule being the write rule plus the refusal of any group or other bit.
  > `unix_read_verdict(mode, uid, euid)` is the write rule plus a refusal of `mode & 0o077` (`unix_read_verdict_needs_the_write_rule_and_no_foreign_bit`)
- (.andromeda/test-plan-detail/unit-test-strategy.md:33) The one-rule board must agree with the instance check's holding the home to the read rule, and the unit cases `strict_instance_*` and this call-site list must be true at the chunk's end, the board's reach included.
  > `strict::check_instance(home, instance_dir)` reads, each that exists and a folder before what it holds, the home (read rule), `instances/` (write rule), the instance directory (read rule), its `diagnostics/` (read rule), its `snapshot.json` (write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows); its call sites (`check_instance(` under `src/`) are three, `src/cmd/hook.rs` `trusted`, `src/cmd/client.rs` `refuse_untrusted` (shared by the eight frame verbs) and `src/cmd/revive.rs`. Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, and a path that cannot be statted is unreadable.
- (.andromeda/test-plan-detail/unit-test-strategy.md:35) Strict-modes work on the paths this entry reaches must keep these home-tree cases green, or move them with a recorded reason.
  > The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`,
- (.andromeda/test-plan-detail/unit-test-strategy.md:9) Each closed code this chunk adds for a socket-directory refusal or a peer euid mismatch must be a word this rule accepts.
  > `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]` with a lower-case letter first, held by `code_word_literal_table` (`crates/viola-core/src/lib.rs`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:10) A refusal cause added to this closed set must keep no serde path and must be held by this unit case under its own code word.
  > `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify` and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words (`crates/viola-core/src/lib.rs`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:69) Every refusal of this chunk must map into this exit set with no new code, its integer asserted in the unit mapping.
  > The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21.
- (.andromeda/test-plan-detail/unit-test-strategy.md:90) The client's peer euid check runs before its first frame, so it must be held beside this server check at the unit layer: refused inside the span with no frame written, with a new closed cause logged by `unreachable_logs_its_cause`; the captured-span helper's macOS compile-out must not leave the euid check unheld on macOS.
  > The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `endpoint-lost` `during` `call`, the other three `during` `connect`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:91) A hint added for a refusal of this chunk must be built at this one home and held by a unit case in the same way.
  > The `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`;
- (.andromeda/test-plan-detail/unit-test-strategy.md:92) The hook is a Unix client of the peer euid check and a client of the directory verification, so its refusal must write no frame and emit no decision, and SessionEnd must keep its direct append, each held by a unit case.
  > The hook's server check (`src/cmd/hook.rs`): `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the dialog path writes no frame and emits no decision), and a SessionEnd whose check fails takes the direct append (`append_session_end`), held by a mismatch row of the hook's unit cases.
- (.andromeda/test-plan.md:46) The chunk must not claim a driven cross-user peer on a hosted runner; the other-euid case must be asserted at the check boundary.
  > only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:88) The directory check's owner leg and the euid mismatch cannot rest on a second account in CI and must take an injected boundary.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:186) The chunk's tests must assert both modes as this trigger requires.
  > Unix socket dir 0700 and file 0600 asserted
- (.andromeda/test-plan.md:254) The Unix branches must be tested on their own runners, macOS's server verification named as this euid and directory form.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:289) New unit cases must run on all three OSes unless they fall under these named exceptions; a new macOS compile-out is a widening to name.
  > **Unit:** inline `#[cfg(test)] mod tests` in all six crates and the root bin, with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices, on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.
- (.andromeda/test-plan.md:360) New unit tests must take this naming form.
  > Test function naming: `<subject>_<condition>_<expected>` in snake_case.
- (.andromeda/test-plan.md:361) Table-driven cases must carry readable labels, in the form each crate's manifest allows.
  > Table-driven cases use rstest `#[case]`, and each case name must be a readable label (for example `#[case::esc_refused]`). The one exception is `viola-e2e`, which carries no `[dev-dependencies]`: its table-driven cases are a labelled case table (an array of `(label, …)` rows in one `#[test]`, each assertion naming its label), so no manifest edge is added for rstest alone. `viola-core` carries no rstest dev-dependency either, and its table-driven cases are literal tables in one `#[test]` (for example `dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`).
- (.andromeda/test-plan.md:380) The peer euid work must extend this channel unit surface, the injected-euid decision and the per-OS server pid.
  > the peer-credential decision with injected euids, the connected stream's server pid on each OS,
- (.andromeda/test-plan.md:393) The root bin's unit surface must hold the chunk's refusals inside the same span and cause discipline, and the exit mapping must never panic.
  > the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged,
- (.andromeda/test-plan.md:396) The euid and directory decisions must be tested through injected values or real files, adding no mock seam the product does not define.
  > Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan.md:484) The cross-user leg of this chunk must stay covered by the §4 boundary unit test.
  > Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:610) The owed macOS server-verification case is named to this entry, and it must be delivered or dispositioned here; its unit floor is the `Matches` row of the verdict table.
  > beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening"),
- (.andromeda/test-plan.md:715) Directory verification cases for mode, owner and symlink must run over real paths in a tempfile home, never a mocked filesystem.
  > Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
- (.andromeda/test-plan.md:876) Directory, euid and verdict decisions must be held at the unit layer, leaving §6 only the cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:881) A green Linux dev-host run must not be reported as proof of the macOS branch; a leg this chunk cannot prove must stay named.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:889) The expected codes, exits and verdicts of the new cases must be literals in the test.
  > The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:890) New refusal and exit assertions must read the wire value and the integer.
  > NEVER compare `RefusalReason` or exit codes by `Debug` string. Assert on the serde kebab-case value and the integer exit code.

## Bindings
- **Peer-credential decision ↔ body §1 and §6 untestable zones**: the unit case is the sole holder of the cross-user rejection, because hosted runners provide one account (.andromeda/test-plan-detail/unit-test-strategy.md:18), (.andromeda/test-plan.md:46), (.andromeda/test-plan.md:88), (.andromeda/test-plan.md:484).
- **macOS `Matches` row ↔ body §6 Exit-cause matrix**: the verdict row records the gap, and the matrix owes the macOS case to this route entry; the case must not read the row as a comparison (.andromeda/test-plan-detail/unit-test-strategy.md:29), (.andromeda/test-plan.md:610).
- **Code word and closed cause ↔ body §6 Exit-cause matrix**: a new refusal's word passes `is_code_word`, its cause joins the closed set, and its hint meets the matrix's one-hint-per-cause rule (.andromeda/test-plan-detail/unit-test-strategy.md:9), (.andromeda/test-plan-detail/unit-test-strategy.md:10), (.andromeda/test-plan-detail/unit-test-strategy.md:91), (.andromeda/test-plan.md:610).
- **Strict-modes rules ↔ body §5 On-disk**: the board's one rule at the unit layer must agree with the exit and no-line outcomes §5 states for each surface (.andromeda/test-plan-detail/unit-test-strategy.md:31), (.andromeda/test-plan-detail/unit-test-strategy.md:33), (.andromeda/test-plan.md:444).
- **Hook unit cases ↔ body §6 Security sweep**: the hook's no-frame unit refusal stands under the fail-open matrix's exit 0 with no body (.andromeda/test-plan-detail/unit-test-strategy.md:92), (.andromeda/test-plan.md:604).
- **Exit mapping ↔ body §1 CLI verbs entity**: the typed exit set the mapping holds is the set the CLI contract names (.andromeda/test-plan-detail/unit-test-strategy.md:69), (.andromeda/test-plan.md:62).
