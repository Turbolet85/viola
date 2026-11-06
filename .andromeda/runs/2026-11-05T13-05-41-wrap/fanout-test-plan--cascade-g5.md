@proposal n=1 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-53es7vlo basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy › #### Scenario: E3 — `link` / `unlink`
change: In the As-landed line, «green on the three CI runners (run 38098238731)» reads «green at run 38098238731 (a dated reading of the retired three-OS matrix; CI now runs `ubuntu-latest` alone)».
sidecar: E3 as-landed: the three-runner green reading re-worded as a dated reading; CI is ubuntu-latest alone (Linux-only chunk).
rationale: Report Counts (`win-ci`, `os-target`): the windows-2025 and macos-latest legs are retired, and a dated measurement on them is re-worded, never left as current.
@proposal n=2 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-7jgwgzx2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-jwxlf7mu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The impostor rows (`case::impostor`, `question_impostor`, `permission_impostor`) carry no `cfg_attr(not(target_os = "macos"), …)` gate and run unconditionally, and the fail-open matrix's last case reads «a server that is not the recorded wrapper (`server-verify-failed`)» with no «on Linux and Windows» qualifier.
sidecar: Security sweep: the hook_fail_open impostor rows ungated; the server-verify-failed case no longer limited to Linux and Windows.
rationale: Report deviations 9 and 11: the multi-line cfg_attr pair in tests/hook_fail_open.rs left and both cases run unconditionally; Retired: the macOS no-peer-pid carve-out.
@proposal n=3 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7jgwgzx2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the paragraph «Not a refusal case: a hook reaching an exiting wrapper's own Windows pipe (ci#36532038635) … one CARRY on the route entry "Windows-only live measurements"»: the Windows claim it owes is retired, and the CARRY with it.
sidecar: Security sweep: the exiting-wrapper Windows-pipe paragraph and its Windows-only-measurements CARRY retired with the named-pipe endpoint.
rationale: Report Retired: the Windows named-pipe endpoint is retired, and the route entry «Windows-only live measurements» was retired at the 2026-11-04T23-11-43-wrap; a wolm site whose Windows claim leaves is retired with it.
@proposal n=4 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7jgwgzx2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Re-word the OS readings as dated history: «0.414 s on the Windows debug build, as measured at chunk 2026-09-28-hook-perf-gate» reads as that chunk's dated reading on the retired Windows host, and the MAX_FRAME case's «green on the three CI OSes, as measured at ci#40458923095» reads «green at ci#40458923095 (then three OS legs; CI now `ubuntu-latest`)».
sidecar: Security sweep: the Windows forced-panic timing and the three-OS MAX_FRAME green re-worded as dated readings.
rationale: Report Counts (`win-ci`, `os-target`): a dated measurement on windows-2025 / macos-latest that records history is re-worded by its owner, never left as current.
@proposal n=5 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-iplgfuqy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The rule «`diagnostics/` files are 0600 on Unix» reads «`diagnostics/` files are 0600»: on the Linux-only product the qualifier names no other arm.
sidecar: Error sanitization: the diagnostics 0600 rule loses its Unix-only qualifier (Linux-only product).
rationale: Report Counts (`os-target`): an OS qualifier on a Linux-only product is re-worded to the Linux statement, the claim kept; the Windows home owner/DACL arm is retired.
@proposal n=6 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-eoindiy7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The causes the top comment names outside the table drop «the `.cmd` human pair»: server verification, stale heartbeat, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, and `viola mcp`'s two silent exits.
sidecar: Exit-cause matrix: the .cmd human pair leaves the top comment's list (batch-script-child retired).
rationale: Report Retired: the `.cmd`/`.bat` refusal, `batch-script-child` and its human pair are retired; tests/cross_exit_causes.rs changed (lines 9, 11).
@proposal n=7 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-eoindiy7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: «the home fails strict-modes (Unix a widened mode; Windows a Users grant through `icacls`, on windows-2025)» reads «the home fails strict-modes (a widened mode)»; `tests/security_negatives_strict_modes.rs` is named without «the three CI OSes»; and the `run` home-tree cause's «a read-granted pre-existing home included on every OS» reads «a read-granted pre-existing home included».
sidecar: Exit-cause matrix: the Windows icacls strict-modes arm and the three-OS / every-OS qualifiers leave.
rationale: Report Symbols: strict.rs's Windows owner/DACL arm and test support's icacls arms removed; Retired: the Windows home owner/DACL strict-modes arm and the windows-2025 / macos-latest legs.
@proposal n=8 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-eoindiy7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The server-verification cause reads on Linux alone: a same-user impostor listener on the recorded endpoint (`snapshot.json` pid or start time not matching the listening process) and a server of another euid through the client euid check each read `server-verify-failed`, the euid decision held by injected values and each seam's wiring by no committed test; the `#[cfg(not(target_os = "macos"))]` qualifier, the macOS carve-out, and the macOS socket-directory and same-user-impostor sentences leave, and «its Linux and Windows case» reads «its case».
sidecar: Exit-cause matrix: the server-verification cause re-worded Linux-only; the macOS no-peer-pid carve-out and its sentences retired.
rationale: Report Symbols: SERVER_PID_REPORTED / PEER_EUID_REPORTED removed, server_verdict loses the macOS Matches arm; Retired: the macOS no-peer-pid carve-out; deviation 11: security_negatives_server_identity.rs ungated.
@proposal n=9 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-eoindiy7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the exit-1 `run` cause «the PTY child is `.cmd` / `.bat` (`#[cfg(windows)]`)».
sidecar: Exit-cause matrix: the .cmd/.bat start refusal leaves exit 1's causes.
rationale: Report Retired: the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild`); Symbols: src/cmd/run.rs's BatchScriptChild start refusal removed.
@proposal n=10 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-eoindiy7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: revive's post-preflight refusals drop `batch-script-child` and the row `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, leaving `pinned-hash-mismatch`, `squatted-name` and a `claude` not on `PATH` as `internal-error`; `viola revive --json` prints one exit-1 document for the ten codes `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` and `internal-error`; the sentence naming that row `verification-matrix.json#v1-39`'s Windows witness leaves.
sidecar: Exit-cause matrix: revive's closed --json refusal codes 11 → 10 (batch-script-child and its row retired).
rationale: Report Counts: revive's closed `--json` refusal codes 11 → 10, `batch-script-child` gone; Outcome: revive's codes ten, pinned in tests/cross_exit_causes.rs.
@proposal n=11 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The socket-directory refusal's closing clause «off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls» leaves; the rest of the sentence stands.
sidecar: Security negatives: check_socket_dir's off-Unix Ok(()) clause retired.
rationale: Report Symbols: the off-Unix `Ok(())` arm of `check_socket_dir` removed, its Unix body unconditional; Retired: the Windows pipe DACL controls.
@proposal n=12 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the **Windows client SQOS** bullet (the `ImpersonateNamedPipeClient` reading, `tests/channel_sqos_open.rs` and its two cases).
sidecar: Security negatives: the Windows client SQOS negative retired with the SQOS open; tests/channel_sqos_open.rs deleted.
rationale: Report Retired: the SQOS controls and the test-only SQOS pipe; Files: tests/channel_sqos_open.rs deleted whole.
@proposal n=13 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the **Windows `--home` outside `%USERPROFILE%`** bullet (the protected DACL of `create_private_dir`, `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl`, the canonical-SDDL read-back), keeping its last sentence as Linux's: a pre-existing home another user may read is refused with exit 1 (`a_home_another_user_may_read_refuses_run`, a 0750 home).
sidecar: Security negatives: the Windows outside-profile protected-DACL negative retired; the read-granted home refusal stays as the Linux case.
rationale: Report Symbols: `outside_profile` and the DACL arm of `create_private_dir` removed; Retired: the Windows home owner/DACL strict-modes arm.
@proposal n=14 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The **Single writers** and **Strict-modes before the first read or open** cases lose «on the three CI OSes» / «the three CI OSes»; the strict-modes case reads «the mode read back as found» (no «or ACL»), and «The same binary holds the umask-independent modes, single-writer and Windows `--home` outside the profile cases above» drops the Windows case.
sidecar: Security negatives: the three-OS and ACL qualifiers of the single-writer and strict-modes cases leave.
rationale: Report Retired: the windows-2025 / macos-latest legs and the Windows owner/DACL arm; Counts (`os-target`): «three OSes» is stale on a Linux-only product.
@proposal n=15 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the **Windows listener remote-client rejection** and **Windows listener DACL denial** bullets, their cases and controls in `tests/security_negatives_channel.rs`.
sidecar: Security negatives: the Windows listener remote-reject and anonymous-open negatives retired; tests/security_negatives_channel.rs deleted.
rationale: Report Retired: the Windows named-pipe endpoint, its SDDL and `accept_remote(false)` controls; Files: tests/security_negatives_channel.rs deleted whole.
@proposal n=16 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The **Server identity** bullet loses its gate `#![cfg(not(target_os = "macos"))]`: `tests/security_negatives_server_identity.rs` runs unconditionally.
sidecar: Security negatives: security_negatives_server_identity.rs ungated (its macOS sibling left).
rationale: Report deviation 11: `security_negatives_server_identity.rs` made unconditional, its only sibling gone; tests/security_negatives_server_identity.rs line 1 changed.
@proposal n=17 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-7lzg7pgb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the **ConPTY companion owner/DACL** bullet (`tests/conpty_sideload.rs` `run_on_a_companion_another_user_may_write_falls_back_to_conhost`, `sideload_fallback` `strict-modes-failed`).
sidecar: Security negatives: the ConPTY companion negative retired with the sideload; tests/conpty_sideload.rs deleted.
rationale: Report Retired: the ConPTY sideload and `sideload_fallback`; Files: tests/conpty_sideload.rs deleted whole.
@proposal n=18 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-hdfefjry basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The As-landed sentence drops «on Windows the terminal case reads each escaped name as text under a 200-column terminal (ConPTY re-renders the stream with escapes of its own) and the no-ESC assertion is Unix-only»: the terminal case is the openpty one, its no-ESC assertion standing.
sidecar: list row integrity: the Windows ConPTY 200-column reading retired; the no-ESC assertion stands on the Linux terminal case.
rationale: Report Retired: ConPTY and the windows-2025 leg; Files: every listed file lost its Windows arms (tests/cross_list_rows.rs among the changed files).
@proposal n=19 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-3fk5wzsy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: «(`cfg(unix)`, on `booted_wrapper`)» reads «(on `booted_wrapper`, ungated)», and «green on `ubuntu-latest` and `macos-latest` (as measured at ci#41918694243)» reads «green on `ubuntu-latest` (as measured at ci#41918694243, then also on the retired `macos-latest` leg)».
sidecar: Stale-row dim witness: ungated, and its macos-latest green re-worded as a dated reading.
rationale: Report deviation 11: `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` made unconditional; Retired: the macos-latest CI leg.
@proposal n=20 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-6whu4pj5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The no-EOF bullet reads on Linux alone: the wrapper detects exit on the handle (`exit_source:"handle-wait"`), and the output outlives the session leader only while the `--exit-no-eof` holder runs in its own process group (a holder in the leader's foreground group dies by SIGHUP when the leader exits, as measured in CI runs 36165685381 / 36166907442); the held-output half is asserted on Linux, and the ConPTY and macOS readings leave with their legs.
sidecar: Chaos no-EOF: re-worded Linux-only; the ConPTY and macOS readings retired.
rationale: Report Retired: ConPTY, the windows-2025 and macos-latest legs; Symbols: viola-pty is openpty alone, child exit still detected on the handle wait.
@proposal n=21 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-6whu4pj5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The forced vt100 panic bullet reads «the outer terminal shows the child's own panicking bytes (`e4 b8 ad`) and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:`» (no «Unix» / «every OS»), and its Windows sub-bullet (ConPTY re-rendering, the byte oracle owed to the route entry "Windows-only live measurements") leaves.
sidecar: Chaos forced vt100 panic: the OS qualifiers and the Windows sub-bullet owed to Windows-only live measurements retired.
rationale: Report Retired: ConPTY and the route entry «Windows-only live measurements» (retired at the 2026-11-04T23-11-43-wrap); a wolm site whose Windows claim leaves is retired with it.
@proposal n=22 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-6whu4pj5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: Drop the bullet «`.cmd` child (`#[cfg(windows)]`): `run` exits 1 — landed in `tests/cli_program_resolution.rs` …» with its `claude.cmd` → `claude.exe` resolution clause.
sidecar: Chaos: the .cmd child case retired; tests/cli_program_resolution.rs deleted.
rationale: Report Retired: the `.cmd`/`.bat` refusal; Symbols: resolve_program's `Search::windows`, `pathext`, `is_script`, `SHIM_TARGET` removed; Files: tests/cli_program_resolution.rs deleted whole.
@proposal n=23 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-nwgyqyjs basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The clause «it does not run on `macos-latest`, whose `ps` has no keyword … (as measured at ci#41915621814), a gate set by the overseer's technical answer …, so the fake agent's SIGHUP-ignoring exit, shared Unix code, is witnessed on Linux only» reads «it runs on CI's one leg, `ubuntu-latest`»; the `ignores_sighup` read and its failure report stay.
sidecar: Hangup case: the macos-latest exclusion reasoning retired with that leg.
rationale: Report Retired: the macos-latest CI leg; CI is `[ubuntu-latest]` alone.
@proposal n=24 detector=D-retired severity=warning file=e2e-test-strategy anchor=fct-sn6e6lxw basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: `tests/contract_content_hash.rs`'s two literal-oracle cases read without «on all three OSes».
sidecar: Contract suite: the content-hash cases lose their three-OS qualifier.
rationale: Report Counts (`os-target`): «three OSes» is stale on a Linux-only product, re-worded to the Linux statement, the claim kept.
@proposal n=25 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-liulfbun basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: «`faults` reads both ways, an unnamed gate and a row that no longer stands» reads «`faults` reads three ways: an unnamed gate, a row that no longer stands, and a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`) even when a row names it»; its tests add `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named`, and the planted self-check is re-planted (its Windows cases build the word from a `WINDOWS` const).
sidecar: OS-gate census: the another-OS fault class and its self-check join faults and the test list.
rationale: Report Harness / gate surface: GATES names exactly the remaining gates and a windows / macos predicate reads as a fault even when named; New text: tests/contract_os_gated_tests.rs 626-647 adds the self-check.
@proposal n=26 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-liulfbun fact=F1 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-jwxlf7mu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: - **The another-OS fault class:** the census reads any gate whose predicate names `windows` or `target_os = "macos"` as a fault (`another OS: …`), even when a `GATES` row names it; its planted self-check `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` holds this, and its parse catches a multi-line `cfg_attr` OS predicate that a line grep cannot see.
sidecar: OS-gate census: new detail, the another-OS fault class and its multi-line cfg_attr parse (F1).
rationale: Report deviation 9 and Decisions sweep hazard: the census's new fault class caught the multi-line cfg_attr pair in tests/hook_fail_open.rs that the line grep read clean.
@proposal n=27 detector=D-tests-count-literals severity=warning file=e2e-test-strategy anchor=fct-vgds3skp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The literals «52 OS-gated test functions and 7 gated files, 59 `GATES` rows» and the clause «its rule taking the `cfg_attr` case rows (`tests/hook_fail_open.rs`'s two functions)» are stale: re-read the figure by its own rule (the rows of `GATES` in `tests/contract_os_gated_tests.rs`) and drop the hook_fail_open cfg_attr clause.
sidecar: OS-gate census count: the 52/7/59 literals re-read by their rule; the hook_fail_open cfg_attr rows gone.
rationale: Report Harness / gate surface: GATES now names exactly the remaining gates; deviations 9 and 11: the hook_fail_open cfg_attr pair and other gates made unconditional; the report states no new GATES figure, so the count is re-measured by its rule.
@proposal n=28 detector=D-cascade severity=warning file=body anchor=fct-4aawmv2h dependent_of=D-cascade basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-jwxlf7mu
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The fail-open list's «and, on Linux and Windows, a server that is not the recorded wrapper» reads «a server that is not the recorded wrapper»; the socket directory and another-euid clauses stand.
sidecar: §6 Security sweep broad line: the server-verify-failed case no longer limited to Linux and Windows.
rationale: Report deviations 9 and 11: the impostor cases run unconditionally; Retired: the macOS carve-out and the Windows pipe.
@proposal n=29 detector=D-retired severity=warning file=body anchor=fct-ndx7lm2p dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The top comment's list drops «the `.cmd` human pair»; the server-verification cause is a case of `tests/security_negatives_server_identity.rs` with no «on Linux and Windows» and no macOS-carve-out parenthesis (a loosened socket directory staying a case of `tests/security_negatives_socket_dir.rs`); the strict-modes cause's «on three OSes» leaves.
sidecar: §6 Exit-cause matrix broad line: the .cmd pair, the macOS carve-out and the three-OS qualifier retired.
rationale: Report Retired: the `.cmd`/`.bat` refusal, the macOS no-peer-pid carve-out and the windows-2025 / macos-latest legs; deviation 11: server_identity ungated.
@proposal n=30 detector=D-retired severity=warning file=body anchor=fct-4uc5ehox dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The list drops Windows client SQOS, the Windows listener's remote-client and anonymous-open rows, the Windows `--home` outside the profile and the Windows x64 ConPTY companion (`tests/conpty_sideload.rs`); its «Linux and macOS», «three OSes» and «on Linux and Windows» qualifiers leave, the socket-directory and server-identity negatives reading as plain cases.
sidecar: §6 Security control negatives broad line: the Windows and ConPTY negatives and the per-OS qualifiers retired.
rationale: Report Retired: the named pipe and its SDDL / SQOS / accept_remote controls, the Windows owner/DACL arm and the ConPTY sideload; Files: the three test binaries deleted.
@proposal n=31 detector=D-retired severity=warning file=body anchor=fct-47fi4ugg dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
change: The chaos list drops «and a `.cmd` child refused exit 1».
sidecar: §6 Chaos broad line: the .cmd child case retired.
rationale: Report Retired: the `.cmd`/`.bat` refusal; Files: tests/cli_program_resolution.rs deleted whole.
@remark
Other §6 body lines outside this group carry the same retirements: the tui driver fct-ktlsbc3h («child's own bytes on Unix, no viola literal on every OS»), the skipped list fct-lkgqv65p (same-user SDDL read-back §5), fct-dcxdz6xw (foreign-SID SDDL stubs) and fct-x7oaiapn (macOS local runs); the v1-39 Windows witness (proposal 10) binds verification-matrix.json, another owner.
