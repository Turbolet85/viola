@task_prose part=session
The version done-check for 0.1.0 on Linux: portable-pty moved from =0.8.1 to =0.9.0 (nix 0.25.1 to 0.28.0; serial2 and cfg_aliases added; pin-utils, the serial family, ioctl-rs, memoffset 0.6.5 and termios removed), so RUSTSEC-2026-0441 is closed and the deny.toml ignore list is now empty (RUSTSEC-2017-0008 removed).
The Windows remnants left the tree (47 files): plain_drive and its verbatim-drive strip, the Git Bash rewritten-path warning in send (its emitter, its unit table and its CLI witness), every EXE_SUFFIX join and the .exe trim. No verb, flag, env var, channel method, log field or event kind was added; HELP is byte-identical.
One new contract test, dialog_drift_names_a_changed_annotation: a planted change to a forwarded annotations byte reads dialog_drift red.
The live round on claude 2.1.287 read viola verify red, 23 pass 1 fail, on statusline-rate-limits. The row reads the account's plan (five_hour unknown, only seven_day), not the CLI. The founder deferred v1-27 to 0.2.0, and the dialog-hold ceiling stays PROVISIONAL, now owned by 0.2.0's first entry.
The founder ratified the five one-enum divergences as named exceptions (T1). Measurement B is recorded as known, with no action in 0.1.0. Epoch 7's four PROVISIONAL items are confirmed. The fd-inheritance mechanism reads that portable-pty closes every inherited descriptor in the child.
CI ci#42288148289 on 849aad18 was green on all seven jobs. The GATES census is unchanged at 41 rows, 39 functions and 2 files.
@task_file n=1 path=Cargo.toml
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_file n=4 path=tests/contract_fake_agent_drift.rs
@task_file n=5 path=src/cmd/mod.rs
@task_file n=6 path=src/cmd/send.rs
@task_file n=7 path=src/cmd/client.rs
@task_file n=8 path=src/cmd/hook.rs
@task_file n=9 path=src/cmd/list.rs
@task_file n=10 path=src/run/version_gate.rs
@task_file n=11 path=src/bin/viola-fake-agent.rs
@task_file n=12 path=crates/viola-pty/src/lib.rs
@task_file n=13 path=crates/viola-pty/src/pump.rs
@task_file n=14 path=crates/viola-agent-claude/src/lib.rs
@task_file n=15 path=crates/viola-state/src/events.rs
@task_file n=16 path=crates/viola-state/src/pin.rs
@task_file n=17 path=crates/viola-state/src/stamps.rs
@task_file n=18 path=crates/viola-state/src/strict.rs
@task_file n=19 path=crates/viola-e2e/src/harness/mod.rs
@task_file n=20 path=crates/viola-e2e/src/harness/boot.rs
@task_file n=21 path=crates/viola-e2e/src/harness/run.rs
@task_file n=22 path=crates/viola-e2e/src/harness/run/coverage.rs
@task_file n=23 path=crates/viola-e2e/src/harness/run/mutants.rs
@task_file n=24 path=crates/viola-e2e/src/harness/run/perf.rs
@task_file n=25 path=crates/viola-e2e/src/harness/secret_scan.rs
@task_file n=26 path=crates/viola-e2e/src/harness/session.rs
@task_file n=27 path=crates/viola-e2e/src/harness/supervise.rs
@task_file n=28 path=crates/viola-e2e/tests/mcp_tools.rs
@task_file n=29 path=tests/channel_endpoint.rs
@task_file n=30 path=tests/chaos_feed_panic.rs
@task_file n=31 path=tests/cli_output_plain.rs
@task_file n=32 path=tests/cli_send.rs
@task_file n=33 path=tests/cli_verify.rs
@task_file n=34 path=tests/contract_ledger_probes.rs
@task_file n=35 path=tests/hook_fail_open.rs
@task_file n=36 path=tests/security_negatives_socket_dir.rs
@task_file n=37 path=tests/support/fake.rs
@task_file n=38 path=tests/support/home.rs
@task_file n=39 path=tests/support/start.rs
@task_file n=40 path=.config/nextest.toml
@task_item n=1 part=touches
[PTY] decision (fct-omwrdnlc) and the stack line (fct-zegjs5rw), along with fct-ad44nn2e and fct-kxiazeyf: portable-pty =0.8.1 becomes =0.9.0. The rationale changes from 0.8.1 being spike-proven, with a move owed to the Version done-check, to an exact pin like every other third-party version. The pin-utils delta is done, and pty_backend stays openpty.
@task_item n=2 part=touches
fct-co2wsjkw is retired: the PTY swap trigger (the portable-pty-psmux path) is spent, because 0.9.0 drops pin-utils upstream. fct-6oxorhgk is re-read on 0.9.0, and viola-pty compiles unchanged with comment edits only.
@task_item n=3 part=touches
The stack-and-technologies detail and any dependency-graph citation: nix is 0.28.0; serial2 0.2.39 and cfg_aliases 0.1.1 are added; pin-utils, serial, serial-core, serial-unix, serial-windows, ioctl-rs, memoffset 0.6.5 and termios are removed.
@task_item n=4 part=touches
Supply-chain claims, including the ci-cd-approach key and the 4 architecture rows naming pin-utils: RUSTSEC-2026-0441 is closed (cargo tree -i pin-utils finds no package). The deny.toml advisories ignore list goes from 1 entry to 0, and the RUSTSEC-2017-0008 entry for serial is removed.
@task_item n=5 part=touches
[Error Handling] fct-sha7o35z, the Rust error types convention fct-nhsg4k7g and the Inherited Defaults errors line fct-vf75c7sp (also fct-xqkfyxga, fct-ekks4dfz and fct-kekl3p46): the open divergences (ServeError, StampsError, and the PinError, StampsReadError and Refused the census added) become ratified named exceptions (T1), alongside PtyError.
@task_item n=6 part=touches
The thiserror census (fct-gctodeyz, and the count in fct-nhsg4k7g): the grep reads 10 rows. The 10th is crates/viola-e2e/src/harness/mod.rs:25, which is test-only. The rule is reworded to cover the product sources only, and the count stays 9.
@task_item n=7 part=touches
[CLI Conventions] fct-nvft5tqc and fct-onw4pywk: the sentence saying viola warns on a Git Bash rewritten-path prefix is retired, along with its Git Bash rationale, because send no longer writes that stderr line. The stdin or --file rule and the ban on leading-slash arguments stand.
@task_item n=8 part=touches
[Hook Transport] fct-zhr3wx2i: part of its rationale is that exec form skips Git Bash path rewriting. That Windows reason is gone; shell startup remains the live reason.
@task_item n=9 part=touches
The cross-cutting config detail (crosscut-config fct-k6htnrfj): the Windows verbatim-drive clause leaves with plain_drive. canonical_home keeps its signature, and tests/support/home.rs resolved loses its verbatim-prefix strip.
@task_item n=10 part=touches
The dialog-hold ceiling in [Hook Contract] fct-4nt6qtyp, config.json fct-i4wacyrp, config.v1.json fct-6e4hf2ai, fct-vxml3b56 and fct-wpbp4hte: the ceiling stays PROVISIONAL because the hold was not read. Its owner changes from Version done-check to 0.2.0's first entry.
@task_item n=11 part=touches
[Deployment / Distribution] fct-c3wssf3l and fct-wtlwpexy: Measurement B (two files whose loaded sections are equal, keying two copies) is recorded as known, with no action in 0.1.0. The pin re-hash stays the guard, and the owner wording naming the Version done-check route entry goes.
@task_item n=12 part=touches
The capability ledger rows (fct-qwpklckh) and [CLI Version Compatibility]: on an unchanged CLI 2.1.287, the statusline-rate-limits row measured red (23 pass 1 fail). It reads the account's plan state, so it no longer stands as a pure CLI behaviour. The ledger stays at 24 rows, and the dev home's stamps are not moved.
@task_item n=13 part=touches
[Budget Governor] fct-3hy3yfb4: the budget-paused refusal rests on the statusline-rate-limits row, which reads the account's plan. The reach is indirect, through that row. The founder's question of why viola leans on the user's status line goes to 0.2.0's intent (F-06i).
@task_item n=14 part=touches
The project-directory-structure contract key (fct-irhgihut): its seven pointer lines ending in removed, recoverable at the tag multi-os-final are removed, along with its cmd/*.toml trycmd line (F434, a whole-fact edit).
@task_item n=15 part=touches
The Epoch 7 PROVISIONAL marks (11 architecture rows of the epoch7-prov pattern) are discharged on the founder's own word, dialog 6/7. They cover the socket-directory pair's wording, the board's read rule, the P5 word and leans of chunk 2026-11-01-unix-endpoint-and-home-hardening, and that wrap's answer-card confirm.
@task_item n=16 part=touches
Test-binary names in the test inventory and the root test support: every EXE_SUFFIX join becomes the plain names viola, claude and decoy-hook. The registered paths bin/claude and target/perf/release/viola are unchanged. The harness test is renamed exe_is_the_plain_name_in_the_dir.
@task_item n=17 part=touches
The OS-gated test census (crosscut-os-gated-tests and fct-fa2klbw5): re-read and unchanged at 41 rows, 39 gated functions and 2 gated files. A re-read, not a move.
@task_item n=18 part=touches
v1-44's always-admit survivor stands as the version's known limit. Three master sites name this entry as owner (fct-upolcsa3, fct-cz5jn5fs, fct-ocdr3xdn) and take the epoch-boundary audit instead.
@task_item n=19 part=touches
The live round on the Linux dev host re-witnessed several standing claims, all unmoved. process-start pty_backend reads openpty. The socket directory is 0700 and the socket 0600. The pinned copy lives under bin/ and plugin/<key>/ is written at start. The child holds no viola endpoint fd. After the wrapper's SIGKILL the child was gone in 0.401 s. An unverified CLI's hook.dialog is replied null at once.
@task_item n=20 part=touches
Harness and test comments only, with no rule moved: run/coverage.rs (the COVERAGE_IGNORE regex trees are byte-identical), run/mutants.rs, secret_scan.rs, session.rs, supervise.rs, and the three-runner timing comments in .config/nextest.toml, now the ubuntu reading. The other remnant tests (crates/viola-e2e/tests cli.rs, harness_lifecycle.rs, mcp_recorded_exchange.rs, path6_budget_harness.rs; tests cli_fake_agent.rs, cli_list.rs, cli_wait_last.rs, cross_list_rows.rs, run_cli.rs, tui_passthrough.rs, tui_wheel.rs) lost Windows wording or EXE_SUFFIX only.
@task_fact n=1 slug=established-decisions
fact: The capability ledger row statusline-rate-limits reads the account's plan state (on this account the status-line payload carries five_hour unknown and only seven_day), so viola verify can read it red on an unchanged CLI (2.1.287 read 23 pass 1 fail); owner 0.2.0 by the founder's finding F-06i.
@task_fact n=2 slug=established-decisions
fact: OPEN — whether the budget gate stays and what it stands on, given that it rests on the user's status-line rate_limits payload (the founder's question, finding F-06i) · owner: 0.2.0's intent.
@task_fact n=3 slug=cross-cutting-patterns
fact: The contract test tests/contract_fake_agent_drift.rs::dialog_drift_names_a_changed_annotation plants a change to a forwarded annotations byte of synthetic_dialog_set and requires dialog_drift to read red.
@task_item n=1 part=untouched
Conventions RefusalReason, CLI exit codes, viola hook exit codes and MCP results: no refusal reason, detail, exit code or result shape was added or changed.
@task_item n=2 part=untouched
Standard Contracts for the channel methods, frames, event data per kind, list payload, snapshot envelope and instance snapshot: no field, method or event kind was added. The live round only re-witnessed them.
@task_item n=3 part=untouched
[Module Boundaries] and the workspace crates list: no crate was added or removed, and only viola-mcp enables tokio.
@task_item n=4 part=untouched
[MCP] rmcp 3.4.1 and its tool set, plus the Claude Code integration names (plugin, marketplace, registered hook events, child flags): unchanged.
@task_item n=5 part=untouched
[Message Broker / IPC] endpoint naming (the FNV-1a hash, viola-<h12>) and interprocess 2.4.4: unchanged.
@task_item n=6 part=untouched
[Session Liveness] (heartbeat and pid plus start time) and [Links]: unchanged. T3's link count of 0 is a fact of the driver's setup, not a rule change.
@task_item n=7 part=untouched
Environment variables and the [Naming] exceptions: no env var was added or removed.
@task_item n=8 part=untouched
The fuzz workspace (targets and corpus counts), the fixture and schema files, and the frontend or GUI deferral: not reached.
@task_item n=9 part=untouched
The matrix deferrals of v1-11, v1-21, v1-34, v1-36 and v1-27, and the carries of the terminal-reply hypothesis and the tab-close leg to 0.2.0, are ledger writes with no architecture site.
