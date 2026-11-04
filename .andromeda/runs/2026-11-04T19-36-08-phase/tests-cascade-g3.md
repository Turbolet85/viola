@carried n=1 label=test-plan/unit-test-strategy fact=fct-iqktwdfa
why: The one diagnostics schema must carry the same closed unreachable-cause words on every OS, `socket-dir-refused` among them even though only Unix runners reach it, never an OS-only word or a missing key.
quote: `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost`, `ServerVerify` and `SocketDir`, the last with the word `socket-dir-refused`, and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words (`crates/viola-core/src/lib.rs`).
@carried n=1 run=2 label=test-plan/unit-test-strategy fact=fct-id225cf7
quote: `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `endpoint-lost` `during` `call`, the other three `during` `connect`); `Unreachable::SocketDir` logs `socket-dir-refused`, `during` `connect`.
@carried n=2 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: The multi-os-compat trigger requires the named pipe and Unix socket branches tested on their own runners, so the Linux and macOS endpoint forms must hold on `ubuntu-latest` and `macos-latest` beside the Windows pipe form.
quote: FNV-1a endpoint golden vectors: `viola-<12 hex>` of `ViolaName + "\0" + abs home`, with the Windows `\\.\pipe\` form and the Unix per-user 0700 directory forms `$XDG_RUNTIME_DIR/viola/viola-<h12>.sock` (Linux) and `$TMPDIR/viola/viola-<h12>.sock` (macOS), plus the `/tmp/viola-<uid>/viola-<h12>.sock` fallback (security IPC access control, Unix).
@carried n=3 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: Any Unix test the chunk adds must hold the peer-euid decision only over injected euids and the real euid read on Linux and macOS, and no env, flag or feature seam may force a refusing decision.
quote: The peer euid decision and read: a peer `peer_euid_admitted` cannot read is refused where the euid is reported (`PEER_EUID_REPORTED`, `cfg!(unix)`), held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`; the real read by `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`, with their siblings `client_server_euid_is_none_where_the_peer_reports_no_euid` and `tokio_client_server_euid_is_none_where_the_peer_reports_no_euid` (`crates/viola-channel/src/client.rs`, `tokio_client.rs`).
@carried n=3 run=2 label=test-plan fact=fct-2hayha6u
quote: the peer-credential decision with injected euids (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`), the real euid read on the sync and Tokio clients (`client_server_euid_is_this_user`, `tokio_client_server_euid_is_this_user`)
@carried n=4 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The macOS carve-out the scope keeps must stay recorded where the decision table holds it: on macOS the server check compares no pid or start time, and no test may read the macOS check as a comparison.
quote: The server check: `ServerVerdict { Matches, Mismatch, Unverifiable }` from the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`, pinned by the decision table `server_verdict_rows`; where no peer pid is reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap.
@carried n=4 run=2 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
quote: The connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), asserted on each OS by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature); `SERVER_PID_REPORTED` is false on macOS.
@carried n=5 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: A Windows-only unit case must stay `cfg(windows)` with its reason named where it stands, and Windows stays the live-supported target: its evidence does not move to make a Unix case pass.
quote: `#[cfg(windows)]` handle non-inheritance, which backs E2 and the §12 open question. After the `viola-channel` server creates a pipe instance and a client connects, `GetHandleInformation` (windows-sys) on both handles (via `AsRawHandle`) reports `HANDLE_FLAG_INHERIT` clear. This is the only Windows evidence; there is no child-side probe.
@carried n=5 run=2 label=test-plan fact=fct-2hayha6u
quote: the connected stream's server pid on each OS, and Windows handle non-inheritance.
@carried n=5 run=3 label=test-plan fact=fct-sbml5vi6
quote: **viola-pty:** exit is reported from `wait()` on the handle, never from reader EOF. Consumers are tested with a `mockall` double of the six-method seam whose reader never returns EOF while `wait()` returns an exit status. The `TerminateProcess` fallback path is `#[cfg(windows)]`.
@carried n=6 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The v1-40 integration witness rests on the 5 s stale threshold, which must stay clock-free at the unit tier, and the Unix child-ends reading identifies the recorded child by pid and start time.
quote: The liveness classifier (live / stale / gone) is the pure `classify(beat_age, same_process)`, fed ages directly (no clock): a pid that is dead or carries another start time is `gone` at any age; for the same process 4.9 s and 5.0 s are live, 5.1 s and an absent beat `stale`; `same_process` compares the snapshot's `started_at` with the pid's OS start time.
@carried n=6 run=2 label=test-plan fact=fct-45jodiii
quote: E2E may block on a deadline inside the product that it cannot inject: the send confirmation window behind Path 2's `no-prompt-submitted`, `wait --timeout-ms` in Path 3, and heartbeat staleness in the chaos SIGSTOP case. Two conditions apply: the verdict is the product's own exit code or status field, and the same deadline is clock-driven in a §4 unit case.
@carried n=7 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The per-OS gating reading must keep each `cfg(unix)` and `cfg(windows)` strict-modes case on its own runner with its OS reason named, the Unix arms green on `ubuntu-latest` and `macos-latest` and the Windows arms unmoved.
quote: Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, and a path that cannot be statted is unreadable. Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk). On Windows the instance tree is covered by `cfg(windows)` cases green on `windows-2025` (as measured at ci#40403389629)
@carried n=7 run=2 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=8 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The Unix socket directory 0700 and refusal behaviours justify their Linux-and-macOS tests, so the verdict and checks must read green on both Unix runners while the off-Unix case keeps Windows reading nothing.
quote: The socket directory check (`crates/viola-state/src/strict.rs`): the plain `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)` reads a symlink `Refused::Symlink`, a non-directory `Refused::NotADirectory` and else `unix_read_verdict`, held by `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, a foreign owner by injected uid; `strict::check_socket_dir`, which passes an absent path and never chmods, by `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found` and, off Unix, `check_socket_dir_off_unix_reads_nothing`; `check_endpoint_dir` by `check_endpoint_dir_reads_the_endpoints_directory`.
@carried n=8 run=2 label=test-plan fact=fct-7vfmbe3m
quote: the socket directory verdict and checks (`socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`, `check_endpoint_dir_reads_the_endpoints_directory`, `check_socket_dir_off_unix_reads_nothing`)
@carried n=9 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: These are one-OS test arms whose split the per-OS gating reading must keep, each named for the OS behaviour it reads (a read-only directory on Unix, a read-only file on Windows).
quote: the refusal of a longer target under a read-only directory runs on Unix only, Windows using a read-only file.
@carried n=10 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The R8 strip's Unix arm is one of the OS-specific behaviours that justify a one-OS test, `claude_env_keep` surviving on Unix, and the Windows floor stays the eleven literal names.
quote: plus the prefix rule: every inherited `CLAUDE*` name is removed (ASCII case-insensitive on Windows, exact on Unix) unless it is in the persistent set, and a floor name is removed whatever the persistent set says. A Linux dev-host reading on `claude` 2.1.287 stands beside the Windows one and differs (12 names in a hook's environment, four outside the eleven, three of the eleven absent; architecture §Occupied Resources → Environment variables); the floor and this literal stay the eleven.
@carried n=10 run=2 label=test-plan fact=fct-nr357doe
quote: the R8 strip against a literal copy of the identity floor (never imported)
@carried n=11 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The verified verdict reads a per-OS relied set, so a fake-agent case moved onto Unix runners over a stamped home must read verified by the Unix rows, and a verdict that holds on Windows only keeps its OS reason where it stands.
quote: the verdict `verified` reads `LedgerRow::relied()`: 24 rows on Unix, 21 on Windows, `agents-join` then `local-command-compact` appended last, both relied on every OS)
@carried n=11 run=2 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
quote: the version gate's older-stamp oracle (`src/run/version_gate.rs` `stamps_verdict_needs_the_dialog_rows`) holds that an all-`pass` stamp of the twenty-three older ids reads unverified on every OS, one of the eighteen, `resume-session` and `agents-join` too, and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` verifies on Windows only.
@carried n=12 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The Unix forced vt100 panic chaos case reads the outer terminal only by byte search, so the `input-not-ready` verdict must stay held at the unit tier and no screen content beyond the compiled literals is asserted.
quote: The vt100 feed runs under `catch_unwind`, and a forced panic gives `input-not-ready`.
@carried n=13 label=test-plan/unit-test-strategy fact=fct-id225cf7
why: The typed exit set must stay closed on every OS, with no new code, and every Unix-run case of the exit-cause matrix must read through this one mapping.
quote: The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21. It includes the multi-version-compat case: a wrapper `{"result":{"refusal":"<literal unknown string>","detail":…}}` payload deserialises to the `#[serde(other)] unknown` variant and maps to exit 14 with a `--json` refusal document. It never panics and never maps to 20.
@carried n=14 label=test-plan/unit-test-strategy fact=fct-id225cf7
why: The scope names the server check's captured-span cases as compiled out on macOS, so that gate must carry its named reason while the two socket-directory span cases keep running on macOS.
quote: The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit)
@carried n=14 run=2 label=test-plan/unit-test-strategy fact=fct-id225cf7
quote: the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, so these two run on macOS.
@carried n=14 run=3 label=test-plan fact=fct-fn3ybwy5
quote: (`connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, both running on macOS too)
@carried n=15 label=test-plan/unit-test-strategy fact=fct-id225cf7
why: The test job must read green on `ubuntu-latest` and `macos-latest`, and this witness of the scoped capture runs on every OS, while the three bare crate-level captures stay an unowned exposure that a Unix red would name.
quote: Its witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first` (`src/main.rs`) runs on every OS under nextest: red on a bare `with_default` (the capture stays empty), green on the helper (as measured at `viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/evidence/capture-witness.md`).
@carried n=15 run=2 label=test-plan/unit-test-strategy fact=fct-id225cf7
quote: The three crate-level scoped captures that share the mechanism, in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`, run on a bare `with_default`; never observed red, they have no owner.
@binding n=1
**unit-test-strategy liveness classifier ↔ integration v1-40 witness and §11 Universal**: the integration-tier witness of a stale `viola list` row under the outer PTY may block on the product's 5 s staleness only because the same boundary is clock-driven at the unit tier; the witness's verdict must be the product's own status field, and `classify`'s 5.0 s live / 5.1 s stale table must stay the unit-tier holder (test-plan/unit-test-strategy fct-vbcc7bad) (test-plan fct-45jodiii).
@binding n=2
**unit-test-strategy server verdict and server pid ↔ e2e security negatives (server identity)**: the macOS carve-out is held at the unit tier by `server_verdict_rows` and `SERVER_PID_REPORTED` false on macOS, and at the root tier by `tests/security_negatives_server_identity.rs` compiling out on macOS; the parity reading must keep both gates with the one named reason, and no Unix test may read the macOS check as a comparison (test-plan/unit-test-strategy fct-vbcc7bad) (test-plan/unit-test-strategy fct-o57nkgzg) (test-plan fct-ndx7lm2p).
@binding n=3
**unit-test-strategy viola-pty seam ↔ integration Module ↔ PTY**: the openpty wrapper must read exit on the process handle, never on reader EOF, at the unit tier through the mockall seam double and at the integration tier over real openpty spawns of the fake agent (a resize propagated, a key written only once the child reports the new size) (test-plan fct-sbml5vi6) (test-plan fct-nvtf2boy).
@binding n=4
**unit-test-strategy per-OS gated cases ↔ quality-gates per-OS coverage**: every `cfg(unix)`, `cfg(windows)` or macOS-gated unit case counts only in its own runner's coverage job (lines 85, functions 95, regions 80), so a case moved onto or off the Unix runners changes what each Unix job counts and must still clear the gate there (test-plan/unit-test-strategy fct-id225cf7) (test-plan fct-ryyndnld).
@binding n=5
**unit-test-strategy peer-euid decision ↔ test-scope untestable zones**: the cross-user boundary stays held only by the decision over injected euids, the real euid read on Linux and macOS and the wrapper's drop over an injected decision; no Unix test the chunk adds may force a refusing decision through a seam (test-plan/unit-test-strategy fct-o57nkgzg) (test-plan fct-syqfxw5d).
@binding n=6
**unit-test-strategy R8 strip ↔ e2e Scenario E2**: the Unix arm of the strip (exact match, `claude_env_keep` kept) is read end to end in E2 on Unix only, where a `claude_env_keep` name survives in the child, and both tiers must read green on `ubuntu-latest` and `macos-latest` (test-plan/unit-test-strategy fct-hpqcf7r7) (test-plan fct-tenptmlu).
@binding n=7
**unit-test-strategy exit-code mapping and unreachable causes ↔ e2e exit-cause matrix**: the closed set 0, 1, 2, 10–14, 20, 21 and the six closed unreachable causes, `socket-dir-refused` included, are what the exit-cause matrix asserts on each OS; it must gain no code and no OS-only cause word (test-plan/unit-test-strategy fct-id225cf7) (test-plan/unit-test-strategy fct-iqktwdfa) (test-plan fct-ndx7lm2p).
