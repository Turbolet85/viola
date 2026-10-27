# Task — test-plan · 2026-10-26-home-and-code-bearing-file-integrity

## The session
Built the Epoch 6 entry "Home and code-bearing file integrity": a strict-modes check now runs before the first read or open at every entry point (six CLI frame verbs, the hook event and dialog paths, the MCP frame tools, run, revive, verify, update_stamps, the diagnostics opens, the Windows conpty/ companions).
Added a read rule (Unix: no group or other bit on the home, the instance dir and diagnostics/; Windows: no read grant to another SID, READ_RIGHTS 0x9000_0001), the tree checks check_home_tree / check_diagnostics / check_present, and the refusal variants Readable and Symlink.
Moved four snapshot readers (hook connect, statusline serve, run's collision_check, live_endpoint) onto read_snapshot_or_replay; read_snapshot product callers under src/ went from 3 to 0, which builds E5 steps 1–3.
Retired the eight founder-ratified dated strict-modes gaps that waited on this entry; a refused check writes no line anywhere (the founder's word), and the six frame verbs print link's instance pair (exit 21).
Added two new root test binaries (security_negatives_strict_modes, chaos_snapshot_replay) and new test-home helpers; measured on windows-2025 the seeded-home hard-link hazard, the LA alias in canonical SDDL, and that a plain create_dir_all home is refused Writable (an inherited Users write ACE).
CI ci#40403389629 was green 15/15 on three OSes (Windows 3175 of 3175), after two red runs caused by the chunk's own test code.

## Reached files
- crates/viola-state/src/strict.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/liveness.rs
- crates/viola-state/src/pin.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/client.rs
- src/cmd/link.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- src/obs.rs
- schemas/diag-line.v1.json
- tests/security_negatives_strict_modes.rs
- tests/chaos_snapshot_replay.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/hook_statusline.rs
- tests/run_cli.rs
- tests/cli_verify.rs
- tests/conpty_sideload.rs
- tests/cli_controls_not_disableable.rs
- tests/support/home.rs
- tests/cli_revive.rs
- tests/cli_wait_last.rs
- tests/contract_diag_schema.rs
- tests/cli_machine_contract.rs
- tests/tui_env_strip.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/scope-record.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/remove-the-guard.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/operator-pass.md

## Touches
- §6 E5 "As landed: in part": steps 1–3 are now built (tests/chaos_snapshot_replay.rs). The readers no longer stay on read_snapshot, so the CARRY owed to "Home and code-bearing file integrity" is closed. The replay's product readers went from 2 (revive, list) to 6 (plus hook connect, statusline serve, run's collision_check, live_endpoint).
- §6 E5 verification signals and the set-against note: they are now restated as built. Every start appends wheel{holder:"driver", cause:"start"}, so wheel:"human" does not hold after step 3, and the replay writes no file.
- Every test-plan row naming the route entry "Home and code-bearing file integrity" as the owner of a gap, CARRY or unbuilt case (pattern owner-title, 6 test-plan rows): that entry has now landed, so each such row is stale.
- Any test-plan row saying a reader stays on read_snapshot, or naming its 3 product callers (pattern read-snapshot, 5 test-plan rows, among them unit-test strategy rows on the snapshot read): read_snapshot product callers under src/ are now 0.
- Rows listing the emitters of state-recovered or where state.snapshot_recover sits: it now sits under run.start before run.collision_check, and under hook.handle on the hook path. Two span-order oracles list it.
- §4 viola-state unit coverage: the instance check's set widened. Under the read rule: the home, the instance dir and the new diagnostics/. Under the write rule: instances/ and snapshot.json. events.ndjson is under the write rule on Unix and the read rule on Windows.
- §4 unit-test strategy row "Not covered: a widened DACL on an instance tree on Windows": it is now covered by cfg(windows) cases, green on windows-2025.
- §4 unit-test strategy rows stating the read rule as a target (pattern read-rule, 2 test-plan rows): the rule is now built. Re-read them for any "not yet in code" wording.
- §5 On-disk strict-modes exit line: it now holds verify exit 1 (home pair) and the run/revive start tree check exit 1 (home or instance pair), both before any log. The six frame verbs give exit 21 with the instance pair. The MCP frame tools give instance-unreachable with detail null. The hook fails open with no line.
- §5 setup/teardown seed carve-out, §7 on-disk product state, and the matching integration/test-data rows (pattern seed): viola now creates the seeded home and the seed dir through create_private_dir before bin/<key>/conpty/. The wording "a seeded home skips it, so that entry owns making the seed let viola create the home first" is stale.
- Test-data / integration rows saying "the refused home's DACL was not read": the measurement disproves this. A plain create_dir_all home is refused Writable, the refused entry an inherited S-1-5-32-545 (Users) write ACE. The plan's canonical SDDL of that home was not recorded.
- §6 Security control negatives: single writers of stamps.json and snapshot.json are witnessed with the wrapper stopped (Wrapper::stop), not SIGSTOPped (test-plan.md:606 and the e2e security-negatives row). The suite gains the binary security_negatives_strict_modes.
- §6 Security control negatives, Windows --home outside the profile: the protected DACL is read back through icacls /save and reads D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY). A pre-existing Users-read dir is refused with exit 1.
- §6 Exit-cause matrix: new exit-21 cause, the six frame verbs' strict-modes refusal (instance pair, or the instance-unreachable --json document). New exit-1 cause, the run / verify home-tree refusal (home pair).
- §6 Security sweep: the hook fail-open matrix now holds a refused or absent instance (exit 0, empty streams, no frame, no line, no SessionEnd append). It also holds a no_instance_dir row: a VIOLA_DIR naming a directory that does not exist writes nothing.
- §1 coverage triggers: hook stdin, CLI and env (--home / VIOLA_DIR failing strict-modes → exit 1 / 21 / hook 0), and filesystem (group/world-writable home refused; Windows DACL cases where the runner allows). Group or world readable is now refused too, a symlinked diagnostics path is refused, and Users write/read grants are provisioned on windows-2025.
- §1 entity rows on viola-state and on config / code-bearing state ("Reason if not fully testable", the Windows DACL limitation): widened instance and home trees are now provisioned on the hosted runner through icacls. Only the foreign-owner SID and non-persistent-ACL cases stay stubbed.
- §6 Path 1 run start sequence: a home-tree strict-modes check (the home, the instance set, ledger/, bin/, plugin/, bin/<key>/ and the pinned copy, plugin/<key>/) now runs before the wrapper log opens. A collision check over a replayed snapshot reads the beat alone.
- §6 Schema conformance and contract_diag_schema: in schemas/diag-line.v1.json, process-start.sideload_fallback gains strict-modes-failed (4 → 5 words).
- §6 Path 6 / the hook-statusline trigger "statusline_command never runs when strict-modes fails": still true, and a statusline over a replayed snapshot now also runs nothing.
- §5 Hooks and §6 E1 "no VIOLA_NAME means an immediate exit 0": the event and dialog paths now also exit 0 at once on a refused instance or one that is not present, before any config.json read or obs init. The capture arms are unchanged.
- §5 MCP handler tests (crates/viola-mcp/tests/handlers.rs): they gain a widened-instance refusal before any frame.
- Rows that state what check_instance reads (pattern check-instance, 4 test-plan rows): re-read them against the widened set and the moved call sites (statusline.rs → hook.rs, link.rs → client.rs).
- Rows listing no-line refusal sites (pattern no-line, test-plan rows such as the e2e exit-cause and integration rows): the six verbs, the hook event/dialog paths, the MCP frame tools, verify, the run/revive tree check and the diagnostics opens join them.
- §10 perf: run --perf now times the hook rows with the instance check in the timed path. It stays green under SPINE_DEADLINE (max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms).
- Test homes made with create_dir_all (0755) are now refused by the read rule. Tests make their home through viola_state::fs::create_private_dir, and the cli_wait_last catch-site cases use a viola-made home.

## New facts
- F1 · e2e-test-strategy · tests/security_negatives_strict_modes.rs (new) runs on three OSes. send, wait, last, answer, pause and release each refuse a widened instance with exit 21, the instance pair or {"v":1,"error":"instance-unreachable","detail":null}, no frame and no line, leaving mode or ACL as found. It also holds the umask-independent modes, single-writer and Windows protected-DACL cases.
- F2 · e2e-test-strategy · tests/chaos_snapshot_replay.rs (new) is the built E5 steps 1–3 and the replayed readers.
- F3 · unit-test-strategy · New read-rule unit tables: unix_read_verdict (the write rule plus mode & 0o077 refused) and windows_read_verdict (any allow ACE granting READ_RIGHTS = FILE_READ_DATA | GENERIC_READ | GENERIC_ALL to an untrusted SID, inherit-only included), plus read_rights_are_the_named_flags.
- F4 · unit-test-strategy · New strict units: check_home_tree_*, check_diagnostics_* (a symlink refused; a folder under the read rule; a file under the read rule on Unix and the write rule on Windows; an absent path passes) and strict_instance_holds_the_content_folders_to_the_read_rule.
- F5 · unit-test-strategy · cfg(windows) strict cases run green on windows-2025. A tree granted Users write is refused Writable beside a create_private_dir tree that passes, a diagnostics/ granted Users read is refused, and strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry holds.
- F6 · unit-test-strategy · New replay units: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable, statusline_over_a_replayed_snapshot_runs_nothing and collision_check_over_a_replayed_snapshot_reads_the_beat_alone.
- F7 · unit-test-strategy · New stamps and pin units: update_stamps_refuses_a_widened_ledger_and_leaves_it (StampsError, checked before create_private_dir), stamps_error_messages_are_fixed, pin_key_is_the_version_and_the_content_key and pin_companions_refuses_a_path_another_user_may_write.
- F8 · unit-test-strategy · obs.rs units: a diagnostics folder or role/detail file that fails check_diagnostics is a failed open. The writer ends at std::io::sink and the detail line is dropped, with nothing narrowed first.
- F9 · integration-test-strategy · run refuses a 0770 home, a widened instances/<name>, a widened bin/ or a widened pinned copy with exit 1 and its pair before any log, and with no child. revive's start does the same, its --json keeping strict-modes-failed. verify refuses a widened ledger/ with exit 1, the home pair, no step line and stamps.json unchanged.
- F10 · integration-test-strategy · A read-granted pre-existing home refuses viola run on every OS (Unix 0750, Windows a Users read grant).
- F11 · integration-test-strategy · verify_under_a_regular_file_refuses_the_home has a per-OS oracle. On Unix it prints the home pair. On Windows the path stats NotFound, so the catch-site line stands. A separate case forces the catch-site line with a regular file where ledger/ goes in a viola-made home.
- F12 · integration-test-strategy · On windows-2025, Get-Acl printed nothing for a protected DACL, so it is read back with icacls <home> /save. Canonical SDDL spells the runner's user (RID 500) LA, so the oracle takes LA for a user SID ending -500.
- F13 · test-data-fixtures · tests/support/home.rs gains the helpers widen, grant_read, unwiden and still_widened (Unix modes; Windows icacls with its exit asserted).
- F14 · test-data-fixtures · Measured on windows-2025: an inheritable icacls grant on a seeded home's bin/ (or on the home) propagates through seed_conpty's hard links into the shared target/conpty-seed/<key>/ companion and widens every seeded home. A test that widens a home tree above bin/<key>/conpty/ starts that home unseeded (run_refuses_a_widened_home_tree_before_its_log).
- F15 · e2e-test-strategy · On windows-2025, run_on_a_companion_another_user_may_write_falls_back_to_conhost leaves the child on the inbox ConPTY with sideload_fallback strict-modes-failed and the exit unchanged. In the kept homes, exactly one strict-modes-failed appears, from that test.
- F16 · integration-test-strategy · handlers_frame_tools_refuse_a_widened_instance_before_any_frame drives the MCP tools over stdio with an initialize handshake. Each refusal is instance-unreachable with detail null, no frame and no line.
- F17 · integration-test-strategy · In hook_fail_open, the cases that expect lines run in a viola-made instance dir (Env::Made). The widened-instance rows are one function over the events, not an rstest matrix row each.
- F18 · test-strategy · Remove-the-guard rule: a control whose neutralised guard lets a product process start needs a bounded witness. The G4 / G10 witnesses spawn through support::cli::Running, bounded by WITHIN, after a measured 27-minute hang under nextest's default profile, which has no kill line.

## Untouched
- §3 harness contract (agent-run shims, session record, exit codes, usage details): the report states no harness or gate-surface change.
- §9 CI pipeline, matrix, artifact uploads and build failure conditions: no workflow, CI script or tool version changed.
- §10 coverage thresholds, the ignore regex and the mutation gate: no change. Only a perf reading moved.
- §7 fake agent contract behaviour (modes, receipts, scripts, framing replays): the fake agent was not changed.
- §7 fixture hygiene and the verify recorder's scrub codes: no fixture or scrub changed.
- §6 Property suite and Contract suite (recorded fixture sets, pinned-exe hash literal vectors): no parser, fixture set or hash vector changed.
- §6 list row integrity (cross_list_rows.rs) and the viola list board: board.rs is unchanged. Its home-check inconsistency is routed to a route entry, not to tests.
- §8 Mocking & Stubbing discipline: filesystem is still unmocked, and the synthetic SDDL stubs remain for foreign-owner SIDs and non-persistent-ACL volumes.
- §1 untestable zones (real claude CLI, cross-OS-user peers, foreign-owner SIDs, non-persistent-ACL volumes, macOS local runs): unchanged.
- Rows naming the instance detail file detail-<process>.ndjson (pattern detail-file, 6 test-plan rows): the report reads them as true claims.
- The single-writer pattern hits that describe the chaos suite's SIGSTOPped stale-wrapper case (test-plan.md:613, :943, the e2e chaos row, test-strategy.md:26): that is another test, and the report reads them as no-change.
- §11 Test Data ban on hand-writing stamps.json, snapshot.json or budget.json: unchanged. The chaos replay corrupts a file the product wrote.
