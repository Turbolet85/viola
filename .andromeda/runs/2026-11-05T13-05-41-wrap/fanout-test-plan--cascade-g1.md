@proposal n=1 detector=D-retired severity=warning file=test-scope-summary anchor=fct-jnkzde5v basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: In **Reason if not fully testable:** drop the sentence «The Windows listener's admission is asserted on CI `test (windows-2025)` (`tests/security_negatives_channel.rs`): … beside the same open admitted on a default-descriptor pipe.» and the closing sentence «The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs".»; in **As landed (server verification):** «is asserted on Linux and Windows» → «is asserted on Linux», and drop the sentence «These negatives compile out on macOS, where the client reads no peer pid, … `tokio_client_server_euid_is_this_user`.»; the cross-user part, the euid wiring and the fault screen stay as written.
sidecar: viola-channel scope entity: the Windows pipe admission cases (deleted tests/security_negatives_channel.rs) and the macOS no-peer-pid carve-out retired with the Linux-only ruling; server verification asserted on Linux.
rationale: The report's Retired bullet retires the Windows named-pipe endpoint, its SDDL / SQOS / accept_remote(false) controls and the macOS no-peer-pid carve-out; Files lists tests/security_negatives_channel.rs deleted, and Symbols removes SERVER_PID_REPORTED with server_verdict's macOS Matches arm.
@proposal n=2 detector=D-retired severity=warning file=body anchor=fct-qg6j3l2y dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: Drop «, and the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`» so the line reads: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable; only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
sidecar: viola-channel broad line: the Windows pipe admission clause retired (Linux only).
rationale: Same retirement as the detail entity: the Windows named pipe and its controls are retired in the report's Retired bullet, with their test file deleted.
@proposal n=3 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-zgqp2syf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Test Scope Summary
change: Replace the item «npm-shim → `claude.exe` resolution» with «program resolution (`resolve_program`: a given path as given, a bare name on `PATH`)».
sidecar: viola-agent-claude scope entity: the Windows npm-shim to claude.exe resolution replaced by the Linux program resolution.
rationale: Symbols / APIs removes resolve_program's Search::windows, pathext, is_script and SHIM_TARGET: a given path is taken as given and a bare name is looked up on PATH; ProgramNotFound is unchanged.
@proposal n=4 detector=D-cascade severity=warning file=body anchor=fct-wcpph3y4 dependent_of=D-cascade basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Test Scope Summary
change: «npm-shim resolution» → «program resolution (a given path as given, a bare name on `PATH`)»; the rest of the line stays.
sidecar: viola-agent-claude broad line: npm-shim resolution re-worded to the Linux program resolution.
rationale: Same change as the detail entity: Search::windows, pathext and SHIM_TARGET are gone per Symbols / APIs.
@proposal n=5 detector=D-retired severity=warning file=test-scope-summary anchor=fct-xttglbra basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: In revive's `<code>` list drop «`batch-script-child`, » so it reads `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error` (ten codes); the rest stays.
sidecar: CLI verbs scope entity: batch-script-child left revive's closed --json refusal codes (11 to 10) with the .cmd/.bat refusal.
rationale: Retired names the .cmd/.bat refusal (batch-script-child, AgentError::BatchScriptChild); Counts moved records revive's closed --json refusal codes 11 → 10.
@proposal n=6 detector=D-tests-count-literals severity=warning file=test-scope-summary anchor=fct-fnukifof basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 1. Test Scope Summary
change: «(its eleven codes)» → «(its ten codes)»; in **Notes:** «is witnessed under the outer PTY on Unix by `tests/cross_list_rows.rs`» → «is witnessed under the outer PTY by `tests/cross_list_rows.rs`» (the witness now runs unconditionally).
sidecar: cli surface: revive's code count 11 to 10; the dim stale-row witness is no longer limited to Unix.
rationale: Counts moved: revive's closed --json refusal codes 11 → 10 (batch-script-child gone). Deviation 11 makes cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal unconditional.
@proposal n=7 detector=D-retired severity=warning file=test-scope-summary anchor=fct-b2j3wr36 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: In **Signal:** «the child's own bytes (`e4 b8 ad`) arrive on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` appears on it on every OS (`tests/chaos_feed_panic.rs` `chaos_feed_panic_refuses_send_and_keeps_passthrough`; the Windows byte reading owed to the route entry "Windows-only live measurements")» → «the child's own bytes (`e4 b8 ad`) arrive and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` appears on it (`tests/chaos_feed_panic.rs` `chaos_feed_panic_refuses_send_and_keeps_passthrough`)».
sidecar: tui surface: the vt100-panic witness stated for Linux; the Windows byte reading owed to the retired route entry Windows-only live measurements removed with its Windows claim.
rationale: Retired: the OS target is Linux only, and the route entry «Windows-only live measurements» was retired (wolm sites are re-pointed or retired with the Windows claim they owe, per Expected amendments).
@proposal n=8 detector=D-retired severity=warning file=test-scope-summary anchor=fct-5qkdel63 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: «JSON-RPC 2.0 over ndjson on a named pipe / Unix socket» → «JSON-RPC 2.0 over ndjson on a Unix socket»; the rest stays.
sidecar: ipc-internal surface: the Windows named-pipe transport retired; the channel is a Unix socket.
rationale: Retired: the Windows named-pipe endpoint (\\.\pipe\viola-<h12>); Symbols removes the named-pipe arms of client.rs, endpoint.rs, tokio_client.rs and server.rs.
@proposal n=9 detector=D-retired severity=warning file=test-scope-summary anchor=fct-67zmkx76 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: In `boot`: «(exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child)» → «(exit 1: a `live` or `stale` name, or a tampered pinned copy)»; in `cleanup`: «and on Unix a connect to the socket path is refused or the path is absent, on Windows the pipe no longer exists;» → «and a connect to the socket path is refused or the path is absent;».
sidecar: 5-command requirements: the .cmd/.bat start refusal and the Windows pipe cleanup check retired (Linux only).
rationale: Retired names the .cmd/.bat refusal and the Windows named-pipe endpoint; Symbols removes run.rs's BatchScriptChild start refusal and cleanup.rs's Windows pipe arm of endpoint_gone.
@proposal n=10 detector=D-retired severity=warning file=test-scope-summary anchor=fct-6upjfbpd basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. Test Scope Summary
change: «the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: an inherited write grant to a SID strict-modes does not trust, whichever SID holds it)» → «the instance and home-tree checks refuse a `create_dir_all` home (its 0755 mode fails the read rule)»; the rest stays.
sidecar: test data strategy: the Windows inherited-grant reason for refusing a create_dir_all home retired with the owner/DACL strict-modes arm.
rationale: Retired: the Windows home owner/DACL strict-modes arm; Symbols removes strict.rs's Windows owner/DACL arm and windows_read_verdict.
@proposal n=11 detector=D-retired severity=warning file=test-scope-summary anchor=fct-n2arkmru basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Test Scope Summary
change: In **Path:** drop «the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), » and «on Unix the socket directory check» → «the socket directory check»; in the snapshot signal «and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.» → «and the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters).».
sidecar: Path 1 run start sequence: the ConPTY sideload step and the Windows no-settings.json clause retired; the socket-directory check and settings.json stated unconditionally.
rationale: Symbols / APIs removes src/cmd/run.rs's run.conpty_sideload step, pin_companions call and sideload_fallback; Retired lists the ConPTY sideload; on a Linux-only product the Unix arms are the one path.
@proposal n=12 detector=D-retired severity=warning file=body anchor=fct-2ls6kelo dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Test Scope Summary
change: Drop «the fail-open ConPTY sideload, » from the ordered start steps; the rest of the line stays.
sidecar: Path 1 broad line: the ConPTY sideload step removed.
rationale: Same removal as the detail entity: run.conpty_sideload is removed from src/cmd/run.rs per Symbols / APIs.
@proposal n=13 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-qobe5v26 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Test Scope Summary
change: In **Path:** drop «(on Unix only: on Windows `run` writes no `settings.json` override and the arm runs no command)»; in the statusline signal drop the sentence «The equality is Unix only; on Windows the arm runs no command and prints nothing.»; the rest stays.
sidecar: Path 6 budget governor: the statusline pass-through runs on every build (statusline::shell_argv returns [&str; 3]); the Windows no-command arm is retired.
rationale: Symbols / APIs: WRAPPED_HERE folded to true and statusline::shell_argv returns [&str; 3] (was Option); deviation 3: with WRAPPED_HERE folded it was always Some.
@proposal n=14 detector=D-cascade severity=warning file=body anchor=fct-tcg4lsy4 dependent_of=D-cascade basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. Test Scope Summary
change: «passes the user's statusline through unchanged on Unix;» → «passes the user's statusline through unchanged;».
sidecar: Path 6 broad line: the on-Unix qualifier dropped from the statusline pass-through.
rationale: Same change as the detail entity: the shell-out runs on every build now that shell_argv returns [&str; 3].
@proposal n=15 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-yxyv2kzp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 1. Test Scope Summary
change: «child exits without EOF (ConPTY): exit detected on the handle» → «child exits without EOF: exit detected on the handle»; the rest stays.
sidecar: chaos trigger: the ConPTY qualifier dropped from the no-EOF child case (pty_backend is always openpty).
rationale: Counts moved: pty_backend words 3 → 1 (openpty), stale where a per-OS word is named; Symbols / APIs: child exit is still detected on the handle wait.
@proposal n=16 detector=D-retired severity=warning file=test-strategy anchor=fct-7osf2h3d basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-baujzndh
section: ## 2. Test Strategy
change: Replace «Runs on all 3 OSes, except … outside the macOS-gated `refused_inside`.» with «Runs on Linux (`ubuntu-latest`), except the two real-cargo-mutants harness tests (§10 Mutation gate); the server check's captured-span cases (each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the shared test-only `cmd::client::refused_inside` helper and the `last.rs`, `pause.rs` and `unlink.rs` test modules) run unconditionally, beside the decision table `server_verdict_rows` and the socket-directory check's captured-span cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`.»; **Runner:** stays.
sidecar: Unit level: Linux only; the macOS gate on the server-check captured-span cases retired with the no-peer-pid carve-out.
rationale: Retired: the macOS no-peer-pid carve-out and the macos-latest leg; Outcome: the carry pattern (cfg naming windows or target_os = "macos") reads no .rs line (entry 8 green), so no macOS gate is left.
@proposal n=17 detector=D-retired severity=warning file=body anchor=fct-njccdctv dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-baujzndh
section: ## 2. Test Strategy
change: «on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.» → «on Linux (`ubuntu-latest`), except the two real-cargo-mutants harness tests.».
sidecar: Unit pyramid line: three OSes become Linux only; the macOS compile-out clause retired.
rationale: Same retirement as the detail entity: no cfg naming target_os = "macos" is left in any .rs file (Outcome, entry 8).
@proposal n=18 detector=D-cascade severity=warning file=test-strategy anchor=fct-zx23mmpw basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 2. Test Strategy
change: «and the per-OS `perf` CI job are built» → «and the `perf` CI job (its one `ubuntu-latest` matrix leg) are built»; the rest stays.
sidecar: Performance level: the perf CI job runs on the one ubuntu-latest leg (no longer per OS).
rationale: Harness / gate surface: ci.yml's test, perf, lint and release keep strategy.matrix.os: [ubuntu-latest]; Counts: the CI matrix goes from 3 OS legs to [ubuntu-latest].
@proposal n=19 detector=D-retired severity=warning file=test-strategy anchor=fct-hckuzazj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 2. Test Strategy
change: «`widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted)» → «… (Unix modes)»; «`attach(pid)` and `Drop`): on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop; on Windows a debugger attach, detached on drop.» → «`attach(pid)` and `Drop`): `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop.»; «`SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`)» → «`SOCKET_ROOT_VAR` (`XDG_RUNTIME_DIR`)»; «**Harness shims:** `scripts/agent-run.{sh,ps1}`.» → «**Harness shim:** `scripts/agent-run.sh`.».
sidecar: Directory and naming conventions: the icacls, debugger-attach and macOS TMPDIR arms of tests/support removed and scripts/agent-run.ps1 deleted.
rationale: Symbols / APIs (Test support): home.rs loses the icacls arms of widen, grant_read, unwiden and still_widened, and SOCKET_ROOT_VAR reads XDG_RUNTIME_DIR; frozen.rs keeps kill -STOP ungated. Retired and Files: scripts/agent-run.ps1 deleted.
@proposal n=20 detector=D-retired severity=warning file=test-strategy anchor=fct-34amt7nu basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-yq5vc7zj
section: ## 2. Test Strategy
change: Drop «root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; »; «root `security_negatives_server_identity.rs`, off macOS:» → «root `security_negatives_server_identity.rs`:»; «(the hook fail-open impostor rows, off macOS)» → «(the hook fail-open impostor rows)»; the rest of the row stays.
sidecar: IPC vector row: the deleted Windows channel negatives removed; the server-identity and hook impostor cases run unconditionally.
rationale: Files: tests/security_negatives_channel.rs is deleted whole; deviation 11 makes security_negatives_server_identity.rs and the hook-fail-open impostor cases unconditional.
@proposal n=21 detector=D-retired severity=warning file=test-strategy anchor=fct-mqdaf5bf basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 2. Test Strategy
change: Drop «a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`; »; the rest of the row stays.
sidecar: Filesystem vector row: the Windows outside-profile protected-DACL case retired.
rationale: Symbols / APIs removes strict.rs's outside_profile, the Windows owner/DACL arm and the DACL arm of create_private_dir; Retired lists the Windows home owner/DACL strict-modes arm.
@proposal n=22 detector=D-cascade severity=warning file=test-strategy anchor=fct-gldxhydz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 2. Test Strategy
change: | multi-os-compat | CI matrix + contract | §9 Matrix builds (one leg, `ubuntu-latest`); §6 Contract suite (root `contract_os_gated_tests.rs`: every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run, and a gate whose predicate names `windows` or `target_os = "macos"` a fault even when a row names it) |
sidecar: multi-os-compat trace row: the matrix is the one ubuntu-latest leg; the census also reads a gate naming another OS as a fault.
rationale: Harness / gate surface: strategy.matrix.os is [ubuntu-latest], and the census's GATES reads a gate whose predicate names windows or target_os = "macos" as a fault (`another OS: …`) even when a row names it.
@remark
F2 (slug test-scope-summary) fits no g1 entity and is left to the new-detail pass; stale body lines that no g1 entity details: §1 fct-cyzmyvps fct-yjkq3rgm fct-njspb5nv fct-i65r3xxr fct-ykziik74 fct-u3dkdc2w fct-paewrdc5 fct-syqfxw5d fct-oftlgaxh fct-lgbstcvc fct-phy2lvsc fct-5akis2u7, §2 fct-elca5edh (the Windows mutation scratch); the agent-run.ps1 deletion also touches §3 fct-yqn2bvnu (D-tests-obs-harness, not g1)
