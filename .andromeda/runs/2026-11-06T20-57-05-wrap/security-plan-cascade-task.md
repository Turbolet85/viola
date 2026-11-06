@task_prose part=session
The version done-check chunk moved portable-pty from =0.8.1 to =0.9.0 (nix 0.25.1 to 0.28.0, serial2 and cfg_aliases added, pin-utils and the serial crate family removed) and emptied the deny.toml advisories ignore list, the RUSTSEC-2017-0008 entry leaving with the serial crate.
It removed the Windows remnants from 47 files: plain_drive and its verbatim-drive handling, the Git Bash rewritten-path warning in send, every EXE_SUFFIX join and the .exe trim; no product surface (verb, flag, env var, channel method, log field, event kind) was added.
It added one contract test, dialog_drift_names_a_changed_annotation, and measured live on Linux with claude 2.1.287: the child's fd table holds no viola socket, socket directory 0700 and socket 0600, the child gone 0.401 s after the wrapper's SIGKILL.
The ledger row statusline-rate-limits read red on an unchanged CLI (23 pass 1 fail) because it reads the account's plan state, not the CLI; v1-27 is deferred to 0.2.0 and the hold ceiling stays PROVISIONAL, owned by 0.2.0's first entry.
The founder confirmed Epoch 7's four PROVISIONAL items and ratified the five one-enum error divergences as named exceptions; the thiserror census is re-worded to the product sources and stays 9.
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
@task_file n=28 path=crates/viola-e2e/tests/cli.rs
@task_file n=29 path=crates/viola-e2e/tests/harness_lifecycle.rs
@task_file n=30 path=crates/viola-e2e/tests/mcp_recorded_exchange.rs
@task_file n=31 path=crates/viola-e2e/tests/mcp_tools.rs
@task_file n=32 path=crates/viola-e2e/tests/path6_budget_harness.rs
@task_file n=33 path=tests/channel_endpoint.rs
@task_file n=34 path=tests/security_negatives_socket_dir.rs
@task_file n=35 path=tests/hook_fail_open.rs
@task_file n=36 path=tests/cli_send.rs
@task_file n=37 path=tests/cli_output_plain.rs
@task_file n=38 path=tests/support/home.rs
@task_file n=39 path=tests/cli_verify.rs
@task_file n=40 path=.config/nextest.toml
@task_item n=1 part=touches
Dependency Security advisories line: the RUSTSEC-2017-0008 ignore (serial via portable-pty =0.8.1) is gone; deny.toml now reads ignore = [], so the text calling it the only expected ignore, and its re-check-on-pin-move clause, is stale.
@task_item n=2 part=touches
Dependency Security pinning: the portable-pty exact pin moved from =0.8.1 to =0.9.0 in workspace.dependencies; every security-plan citation of =0.8.1 is stale.
@task_item n=3 part=touches
The Critical CVE response SLA and any other text naming the portable-pty to portable-pty-psmux 0.9.7 swap path: the swap trigger is spent and retired, since 0.9.0 drops pin-utils upstream.
@task_item n=4 part=touches
Data Protection handle inheritance: the recorded-not-established wording and its owed measurement owned by Version done-check are discharged; the mechanism now reads portable-pty closes every inherited descriptor in the child (close_random_fds), witnessed live by the child's fd table with no viola or wrapper socket; the swap-candidate clause goes with the retired swap path.
@task_item n=5 part=touches
Error Handling: the five one-enum divergences are ratified as named exceptions on the founder's word, and the thiserror derive census is re-worded to the product sources, the count staying 9 (grep reads 10 rows, the tenth test-only in crates/viola-e2e/src/harness/mod.rs).
@task_item n=6 part=touches
Authentication & Authorization: an OPEN or PROVISIONAL mark resting on Epoch 7's boundary batch (the socket-directory pair wording, the board read rule, chunk 2026-11-01-unix-endpoint-and-home-hardening's word) is closed on the founder's own word; those PROVISIONAL marks are lifted.
@task_item n=7 part=touches
Any security-plan row on the dialog hold ceiling: it stays PROVISIONAL (the hold was not read), its owner re-pointed from the working-route entry Version done-check to 0.2.0's first entry.
@task_item n=8 part=touches
Any security-plan row on Measurement B: recorded as known with no action in 0.1.0; the pinned binary re-hash stays the guard.
@task_item n=9 part=touches
Indirect, through the ledger row statusline-rate-limits: the stamp-gated dialog answering and the unverified-cli gate rest on a verify where every relied row passes; that row reads the account's status-line plan state, so verify can go red on an unchanged CLI; owner 0.2.0 (finding F-06i), with the budget-paused refusal resting on the same row.
@task_item n=10 part=touches
Threat Model supply-chain boundary and the dependency graph: the serial, serial-core, serial-unix, serial-windows, ioctl-rs, termios, pin-utils and memoffset 0.6.5 crates left the graph; nix moved to 0.28.0; cargo deny check reads advisories ok, bans ok, licenses ok, sources ok.
@task_item n=11 part=touches
Any remaining Windows, macOS, ConPTY or Git Bash wording in security-plan (the dep-conpty subject, the 0.9.0 Windows garbage-read and ConPTY-EOF rationale): the Windows code it explained is gone and the pin comment now reads only that the workspace pins every third-party version exactly.
@task_item n=12 part=touches
CLI argument and home resolution: canonical_home keeps its signature but no longer strips a verbatim-drive prefix; the Git Bash rewritten-path warning, the only stderr line send wrote on a rewritten path, is retired.
@task_item n=13 part=touches
Child executable resolution and the pinned binary: the EXE_SUFFIX joins are removed, the names are the plain viola, claude and decoy-hook; registered paths bin/claude and target/perf/release/viola unchanged; a rule citing an executable suffix is stale.
@task_item n=14 part=touches
Live witnesses for standing rules, re-read unmoved: socket directory 0700 and socket 0600, an unverified start recording cli_verified false, the pinned copy under the home's bin, plugin files written at start, the child gone 0.401 s after the wrapper's SIGKILL with no process-exit for self.
@task_item n=15 part=touches
Test inventory: the new contract test dialog_drift_names_a_changed_annotation reads dialog_drift red on a planted change to a forwarded annotations byte of the synthetic dialog set; tests/cli_send.rs lost send_rewritten_path_argument_draws_the_warning.
@task_item n=16 part=touches
Gate readings re-read unmoved: deny-probes 14 of 14 banned with a clean control, release-check probe 5 of 5 refused, viola-pty 33 of 33 on =0.9.0, CI run 42288148289 seven jobs green on 849aad18, the last nightly green 2026-11-02; pty_backend stays openpty.
@task_fact n=1 slug=dependency-security
fact: RUSTSEC-2026-0441 (pin-utils 0.1.0 via nix 0.25.1 via portable-pty 0.8.1) is closed by the move to portable-pty =0.9.0; cargo tree -i pin-utils finds no package.
@task_fact n=2 slug=dependency-security
fact: portable-pty =0.9.0 brings nix 0.28.0, serial2 0.2.39 and cfg_aliases 0.1.1 into the Linux graph, with no advisory ignore needed.
@task_fact n=3 slug=authentication-authorization
fact: The ledger row statusline-rate-limits reads the account's status-line plan state, not the CLI, so a verify can read red (23 pass 1 fail) on an unchanged CLI 2.1.287 and switch no dialog answering on; owner 0.2.0 (F-06i).
@task_item n=1 part=untouched
The security tier, data classification and compliance triggers: no data class, principal or regulated data changed.
@task_item n=2 part=untouched
Secret Management: the R8 CLAUDE* strip, the identity floor and the CI artifact canary classes are unchanged; the secret-scan file changed in comments only.
@task_item n=3 part=untouched
Input Validation paste-text and dialog free-text sets, validate_paste_text and the MAX_FRAME cap and its value: no validator or cap changed.
@task_item n=4 part=untouched
API Security rmcp transport features and the rmcp minor range: no rmcp feature or version moved.
@task_item n=5 part=untouched
The CI workflow rules (SHA-pinned actions, permissions, zizmor, the concurrency decision, the perf job, the release-check script) and the toolchain pins: no workflow, script or toolchain file changed.
@task_item n=6 part=untouched
The statusline shell-out and statusline source reading rules: no shell-out was added or changed; only the rate-limits ledger row's reading moved.
@task_item n=7 part=untouched
The v1.x release prerequisites (dist, cargo-auditable, self_update signatures): no release work in this chunk.
