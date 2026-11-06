@task_prose part=session
The version done-check for viola 0.1.0: every gate green on Linux (ci#42288148289 on 849aad18, seven jobs), the open caps ruled, the version closed; no product surface added (no verb, flag, env var, channel method, log field or event kind).
Dependency move: portable-pty =0.8.1 to =0.9.0 (nix 0.25.1 to 0.28.0, the serial crate family, pin-utils, memoffset 0.6.5 and termios gone); deny.toml advisories ignore emptied (RUSTSEC-2017-0008 left with serial), RUSTSEC-2026-0441 closed; pty_backend stays openpty, viola-pty 33 of 33 green.
Windows remnants retired across 47 files: plain_drive and its verbatim-drive handling, the Git Bash rewritten-path warning of send (emitter, unit table, CLI witness), every EXE_SUFFIX join and the .exe trim, the home.rs resolved strip; comments re-worded to the Linux/ubuntu reading.
One new contract test, tests/contract_fake_agent_drift.rs dialog_drift_names_a_changed_annotation; one harness test renamed, exe_is_the_plain_name_in_the_dir.
Live round on claude 2.1.287: viola verify read red 23 pass 1 fail (statusline-rate-limits reads the account plan, not the CLI); the unverified wrapped start and killed-wrapper leg green; the dialog hold unread, so v1-27 deferred to 0.2.0 and the hold ceiling stays PROVISIONAL, owner 0.2.0's first entry.
Founder's word discharges the Epoch 7 PROVISIONAL marks; Measurement B known with no action in 0.1.0; the census GATES 41 / 39 / 2 re-read unchanged.
@task_file n=1 path=Cargo.toml
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_file n=4 path=tests/contract_fake_agent_drift.rs
@task_file n=5 path=src/cmd/mod.rs
@task_file n=6 path=src/cmd/send.rs
@task_file n=7 path=src/bin/viola-fake-agent.rs
@task_file n=8 path=crates/viola-pty/src/lib.rs
@task_file n=9 path=crates/viola-pty/src/pump.rs
@task_file n=10 path=crates/viola-e2e/src/harness/mod.rs
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
@task_file n=12 path=crates/viola-e2e/src/harness/run/coverage.rs
@task_file n=13 path=crates/viola-e2e/src/harness/run/mutants.rs
@task_file n=14 path=crates/viola-e2e/src/harness/run/perf.rs
@task_file n=15 path=crates/viola-e2e/src/harness/secret_scan.rs
@task_file n=16 path=crates/viola-e2e/src/harness/session.rs
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
@task_file n=18 path=crates/viola-e2e/tests/mcp_tools.rs
@task_file n=19 path=crates/viola-e2e/tests/mcp_recorded_exchange.rs
@task_file n=20 path=crates/viola-e2e/tests/path6_budget_harness.rs
@task_file n=21 path=tests/channel_endpoint.rs
@task_file n=22 path=tests/chaos_feed_panic.rs
@task_file n=23 path=tests/cli_fake_agent.rs
@task_file n=24 path=tests/cli_list.rs
@task_file n=25 path=tests/cli_output_plain.rs
@task_file n=26 path=tests/cli_send.rs
@task_file n=27 path=tests/cli_verify.rs
@task_file n=28 path=tests/cli_wait_last.rs
@task_file n=29 path=tests/contract_ledger_probes.rs
@task_file n=30 path=tests/cross_list_rows.rs
@task_file n=31 path=tests/hook_fail_open.rs
@task_file n=32 path=tests/run_cli.rs
@task_file n=33 path=tests/security_negatives_socket_dir.rs
@task_file n=34 path=tests/tui_passthrough.rs
@task_file n=35 path=tests/tui_wheel.rs
@task_file n=36 path=tests/support/fake.rs
@task_file n=37 path=tests/support/home.rs
@task_file n=38 path=tests/support/start.rs
@task_file n=39 path=.config/nextest.toml
@task_file n=40 path=crates/viola-e2e/tests/harness_lifecycle.rs
@task_item n=1 part=touches
The viola-pty entity and every pin line naming portable-pty =0.8.1 (fct-njspb5nv, fct-vmzc7an2, fct-tr6c4sml and the other pty-pin rows) now read =0.9.0; the entity's Source citing Windows ConPTY EOF loses its Windows reason; exit on the handle, never on EOF, stands.
@task_item n=2 part=touches
The supply-chain trigger fct-hlx2zkbt says cargo deny check may only ignore RUSTSEC-2017-0008: the ignore list is now empty (ignore = [], entries 1 to 0), RUSTSEC-2026-0441 (pin-utils via nix 0.25.1) closed; any serial-ignore or pin-utils row in the detail files moves with it.
@task_item n=3 part=touches
The Git Bash rewritten-path warning is retired: the trigger line naming the §6 rewritten-path warning (fct-engo5ir7) and any detail row naming the warning, its rstest table looks_rewritten_matches_only_the_git_bash_shape or the CLI witness send_rewritten_path_argument_draws_the_warning are stale; the rule that a leading-slash argument is never taken as a prompt (stdin or --file only) stands.
@task_item n=4 part=touches
plain_drive and its Windows verbatim-drive handling are gone, with the rstest table plain_drive_strips_only_the_verbatim_drive_prefix; canonical_home keeps its signature; any test-plan row naming a plain_drive test is stale.
@task_item n=5 part=touches
Every EXE_SUFFIX join and the .exe trim in tests/cli_output_plain.rs are removed (binaries named plain viola, claude, decoy-hook; registered paths bin/claude and target/perf/release/viola unchanged); the exe-suffix rows of test-plan move; the harness test is now exe_is_the_plain_name_in_the_dir.
@task_item n=6 part=touches
tests/support/home.rs resolved no longer strips the verbatim prefix; the path oracle rule (compare resolved(home), never TestHome::path) stands but any Windows reason given for it is stale.
@task_item n=7 part=touches
Windows wording standing in test-plan is a remnant: the identity floor measured on the Windows host (fct-u3dkdc2w), the perf host reading on the Windows dev host (fct-vxnii5yh), per OS in the perf table and build-failure rows (fct-yoazurel, fct-2ztzeoct), the llvm-cov Windows backslash reason of the coverage regex (fct-3mj52n2x, fct-b4avhaux), fct-hhwc4gim in the detail.
@task_item n=8 part=touches
COVERAGE_IGNORE and COVERAGE_FLOORS in run/coverage.rs got comments only: the regex's four trees are byte-identical, so the rule stands, but its Windows separator rationale is now a remnant explanation.
@task_item n=9 part=touches
.config/nextest.toml three-runner timing comments now carry the ubuntu reading: any test-plan row citing three runners or per-OS slow-timeout readings moves.
@task_item n=10 part=touches
The trycmd and assert_cmd text in §1, §5, §6, §7, §9, §10 and keys 5-command-implementation and bootstrap-phases is to be re-worded to the literal pins (CARRY 1); trycmd 27 standing rows in test-plan.
@task_item n=11 part=touches
The os census re-read: GATES 41 rows, 39 gated functions, 2 gated files, unchanged and contract_os_gated_tests green (fct-vgds3skp, fct-5akis2u7): verify the count, nothing moves.
@task_item n=12 part=touches
The fd-inheritance wording (4 test-plan rows incl. key test-data-bootstrap, E2 fct-tenptmlu): portable-pty closes every inherited descriptor in the child (close_random_fds), no longer recorded, not established; witnessed by E2 and the live round's /proc fd table (31 fds, 0-2 on the pty, no viola socket, no home file).
@task_item n=13 part=touches
The dialog-hold ceiling stays PROVISIONAL (the hold unread at the live round); its owner moves from Version done-check to 0.2.0's first entry: the hold-ceiling rows of test-plan (3) carry the old owner title.
@task_item n=14 part=touches
Measurement B is recorded as known with no action in 0.1.0, the pin re-hash staying the guard: the measurement-b rows of test-plan (2) move.
@task_item n=15 part=touches
The Epoch 7 PROVISIONAL marks (the socket-directory pair wording, the board read rule, the unix-endpoint chunk's P5 word, that wrap's answer-card confirm) are confirmed by the founder's word: the epoch7-prov rows of test-plan (17) lose their PROVISIONAL marks.
@task_item n=16 part=touches
The capability-ledger row statusline-rate-limits is disproved as a CLI behaviour: it reads the account's plan state (five_hour unknown, seven_day only), so verify read 23 pass 1 fail on an unchanged 2.1.287; any test-plan text treating the row or the budget data source as a CLI property it relies on every build is touched (ledger still 24 rows; fixture-stamped verify expectations unchanged).
@task_item n=17 part=touches
The contract suite and contract-test-against-sandbox trigger (fct-q6ltykaz, fct-fvjvvfmg, annotations forwarded) gain a planted-drift witness: a changed forwarded annotations byte reads dialog_drift red.
@task_item n=18 part=touches
matrix#v1-44: the always-admit survivor at the client seams stands as the version's known limit, read at the epoch-boundary audit (CARRY 8); fct-upolcsa3 names this entry as owner.
@task_item n=19 part=touches
Line citations into edited files may have moved: crates/viola-e2e/src/harness/secret_scan.rs:129 (fct-gjahavf7), and any line cited into tests/support/home.rs, src/cmd/mod.rs, src/cmd/send.rs, crates/viola-pty/src/lib.rs or the harness run files.
@task_item n=20 part=touches
Gate readings re-measured with no master count moved: deny-probes 14/14 banned control clean, release-check probes 5/5, viola-pty 33 run, pre-push coverage 3367 passed 0 failed at 85/95/80; rows quoting these lines stand.
@task_fact n=1 slug=e2e-test-strategy
fact: tests/contract_fake_agent_drift.rs dialog_drift_names_a_changed_annotation plants a change to a forwarded annotations byte of synthetic_dialog_set and asserts dialog_drift reads red.
@task_fact n=2 slug=test-scope-summary
fact: The ledger row statusline-rate-limits reads the account's status-line plan state, not the CLI: a live viola verify on an unchanged claude 2.1.287 read 23 pass 1 fail when the payload carried only seven_day; owner 0.2.0 (finding F-06i).
@task_item n=1 part=untouched
The product surface: verbs, flags, exit codes, refusal reasons, channel methods, event kinds (EventKind 16, WAIT_WAKE 6), log fields and the list --json shape are unchanged.
@task_item n=2 part=untouched
The MCP adapter handlers in crates/viola-mcp/tests/handlers.rs and the five-tool list are unchanged.
@task_item n=3 part=untouched
The test tier Comprehensive, its justification and the coverage threshold table are unchanged.
@task_item n=4 part=untouched
E3 link/unlink (tests/cli_links.rs) is unchanged: the live round's link count 0 is a fact of the driver's setup, not of the product.
@task_item n=5 part=untouched
E5 snapshot replay (tests/chaos_snapshot_replay.rs) and the chaos snapshot cases are unchanged.
@task_item n=6 part=untouched
The CI pipeline structure, its ubuntu-latest single leg, the Rust 1.98.1 pin and MSRV 1.96 are unchanged; no workflow was edited.
@task_item n=7 part=untouched
Fixture sets under fixtures/claude and the fixture hygiene walk (tests/contract_fixture_hygiene.rs, tests/support/hygiene.rs) are unchanged.
@task_item n=8 part=untouched
The deferrals v1-11, v1-21, v1-34, v1-36 and v1-27, CARRY 10 (terminal-reply hypothesis) and CARRY 11's tab-close leg are ledger writes with no test-plan site.
@task_item n=9 part=untouched
The plan step 10 PermissionRequest premise (user settings allow Bash(*)) is a plan claim with no test-plan site.
@task_item n=10 part=untouched
SPINE_DEADLINE 1.0 s and the hook CONNECT_DEADLINE 750 ms are unchanged.
