@task_prose part=session
The version done-check of viola 0.1.0 on Linux: every gate green, ci#42288148289 on 849aad18 seven jobs green on ubuntu-latest, pre-push coverage suite 3367 passed 0 failed.
The dependency move: portable-pty =0.8.1 to =0.9.0 (nix 0.28.0, the serial crate family and pin-utils gone), deny.toml ignore list emptied (RUSTSEC-2017-0008 gone, RUSTSEC-2026-0441 closed); crates/viola-pty compiles unchanged, pty_backend stays openpty.
Remnant sweep in 47 files: the Git Bash rewritten-path warning of viola send retired (its emitter, unit table and CLI witness), plain_drive and its Windows verbatim-drive handling removed, every EXE_SUFFIX join and .exe trim gone; no verb, flag, env var, log field or event kind added.
One new contract test, dialog_drift_names_a_changed_annotation (a11y n/a); the HELP literal and both help tests byte-identical.
The live round: viola verify read red 23 pass 1 fail on the statusline-rate-limits row (the row reads the account plan, not the CLI); the dialog hold unread; v1-27 deferred to 0.2.0 on the founder word, the hold ceiling stays PROVISIONAL owned by 0.2.0 first entry; Epoch 7 PROVISIONAL items confirmed by the founder.
@task_file n=1 path=Cargo.toml
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_file n=4 path=tests/contract_fake_agent_drift.rs
@task_file n=5 path=src/cmd/mod.rs
@task_file n=6 path=src/cmd/send.rs
@task_file n=7 path=src/cmd/client.rs
@task_file n=8 path=src/cmd/hook.rs
@task_file n=9 path=src/cmd/list.rs
@task_file n=10 path=src/bin/viola-fake-agent.rs
@task_file n=11 path=crates/viola-pty/src/lib.rs
@task_file n=12 path=crates/viola-pty/src/pump.rs
@task_file n=13 path=crates/viola-e2e/src/harness/mod.rs
@task_file n=14 path=crates/viola-e2e/src/harness/run.rs
@task_file n=15 path=crates/viola-e2e/src/harness/run/coverage.rs
@task_file n=16 path=crates/viola-e2e/src/harness/run/mutants.rs
@task_file n=17 path=crates/viola-e2e/tests/cli.rs
@task_file n=18 path=crates/viola-e2e/tests/harness_lifecycle.rs
@task_file n=19 path=crates/viola-e2e/tests/mcp_recorded_exchange.rs
@task_file n=20 path=crates/viola-e2e/tests/mcp_tools.rs
@task_file n=21 path=crates/viola-e2e/tests/path6_budget_harness.rs
@task_file n=22 path=tests/channel_endpoint.rs
@task_file n=23 path=tests/chaos_feed_panic.rs
@task_file n=24 path=tests/cli_fake_agent.rs
@task_file n=25 path=tests/cli_list.rs
@task_file n=26 path=tests/cli_output_plain.rs
@task_file n=27 path=tests/cli_send.rs
@task_file n=28 path=tests/cli_verify.rs
@task_file n=29 path=tests/cli_wait_last.rs
@task_file n=30 path=tests/contract_ledger_probes.rs
@task_file n=31 path=tests/cross_list_rows.rs
@task_file n=32 path=tests/hook_fail_open.rs
@task_file n=33 path=tests/run_cli.rs
@task_file n=34 path=tests/security_negatives_socket_dir.rs
@task_file n=35 path=tests/tui_passthrough.rs
@task_file n=36 path=tests/tui_wheel.rs
@task_file n=37 path=tests/support/fake.rs
@task_file n=38 path=tests/support/home.rs
@task_file n=39 path=tests/support/start.rs
@task_file n=40 path=.config/nextest.toml
@task_item n=1 part=touches
The portable-pty pin moved =0.8.1 to =0.9.0: every a11y-plan line naming the inherited =0.8.1 outer-PTY boundary driver (the tui reach row fct-4ktfiloi in §4, fct-twqx5x46 in the detail) now reads =0.9.0; pty_backend stays openpty and the outer-PTY e2e binaries are green on 0.9.0.
@task_item n=2 part=touches
Any a11y-plan rationale or caveat for the old pin (Windows garbage-read, ConPTY EOF, the portable-pty swap trigger, pin-utils or the serial crates) is retired with the pin; the deny.toml ignore list is now empty.
@task_item n=3 part=touches
The trycmd / assert_cmd wording of the harness contracts (keys a11y-testing-tool-pick, keyboard-test-harness, bootstrap-phases in §3) and the §8 verification lines is the plan entry 12 amendment: re-worded to the literal pins (CARRY 1), the strings pinned by literal rather than by a trycmd file.
@task_item n=4 part=touches
Indirect through CARRY 1: the trycmd mentions in §2 pyramid rows, §4 P2 / P6 scenarios, the cli tool-reach row and the §9 stage row (assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1) follow the same literal-pin re-wording.
@task_item n=5 part=touches
The Git Bash rewritten-path warning, the only stderr line viola send wrote on a rewritten path, is retired: any a11y-plan inventory of send stderr lines, fixed CLI strings or warnings that names it, or a Git Bash / Windows-path reason for the stdin-or-file-only input rule, is stale; the stdin / --file input rule itself stands.
@task_item n=6 part=touches
plain_drive and its Windows verbatim-drive handling, the EXE_SUFFIX joins and the .exe trim are removed; any a11y-plan mention of Windows, macOS, ConPTY or .exe binary names is a remnant now (the matrix is ubuntu-latest only).
@task_item n=7 part=touches
The dialog hold ceiling stays PROVISIONAL (the hold was not read live); its owner is re-pointed from the Version done-check to 0.2.0 first entry: the one hold-ceiling row in a11y-plan (the dialog hold among the time bounds of §8 Timeout extensions) follows.
@task_item n=8 part=touches
The Epoch 7 PROVISIONAL items are confirmed on the founder word (socket-directory pair wording, the board read rule, chunk 2026-11-01-unix-endpoint-and-home-hardening P5 word and leans, the answer-card confirm): the one epoch7-prov row in a11y-plan loses its PROVISIONAL or OPEN mark.
@task_item n=9 part=touches
v1-27 is deferred to 0.2.0: its live clauses (cli_verified true and no viola byte on screen) were not read; the tui zero-viola-bytes boundary stays proven by the fake-agent and outer-PTY tests and the unverified live run, any a11y-plan line citing a live verified proof of that boundary for 0.1.0 is stale.
@task_item n=10 part=touches
The statusline-rate-limits ledger row is disproved as a CLI behaviour: it reads the account plan state, so viola verify can read red (23 pass 1 fail) on an unchanged CLI; the budget pause and status-line deadline in §8 Timeout extensions rest on that row, its owner is 0.2.0 (finding F-06i).
@task_item n=11 part=touches
viola verify step-line output re-read live on 2.1.287 as [21/24] statusline-rate-limits fail and stamped 2.1.287 23 pass 1 fail: the §4 / §1 P6 verify step-line format [NN/NN] pass or fail is unmoved; the capability ledger stays 24 rows.
@task_item n=12 part=touches
CI re-read: ci#42288148289 seven jobs green on ubuntu-latest (test job gate tools, G2, G4, secret scan, gate verdict); the test job gate --require coverage,doctest and the single instrumented coverage suite are unmoved; nightly.yml last run success 2026-11-02 and carries no a11y step.
@task_item n=13 part=touches
.config/nextest.toml timing comments re-worded from three runners to the ubuntu reading, and coverage.rs / mutants.rs comments only: the a11y cases still run in the coverage suite and the epoch-boundary mutation audit, nothing moved.
@task_item n=14 part=touches
The tui passthrough, wheel and chaos feed-panic test files were edited only for remnant lines: the boundary assertions (zero viola bytes, keystrokes unblocked past the atomic paste, focus / mouse / resize not moving the wheel, the contained feed panic) are unchanged.
@task_item n=15 part=touches
The live child fd table and socket modes were read (fds 0 to 2 on the pty, socket directory 0700, socket 0600), and the killed-wrapper leg read the child gone 0.401 s after SIGKILL; the founder-attended tab-close leg and the terminal-reply hypothesis (CSI ? 6 n, kitty graphics) are carried to 0.2.0 unmeasured: an a11y-plan line on terminal replies passing through stays as a rule, unmeasured live.
@task_item n=1 part=untouched
The A11y tier Standard (1), the triggered and not-triggered trigger lists (visual-discrimination, keyboard-only, screen-reader-priority; wcag-mapping, regulated-compliance, cognitive, motion, multi-language, target-size): no trigger input moved.
@task_item n=2 part=untouched
The viola list caption order NAME LIVE STATUS WHEEL DIALOG CLI and its CAPTION literal, the UNWRAPPED READ-ONLY separator and the --help group labels: the HELP literal and help tests are byte-identical, no list column moved.
@task_item n=3 part=untouched
The ARIA landmarks N/A inventory, the critical paths P2 / P3 / P4 / P6 and their WCAG SC stand-ins (2.1.1, 1.3.2, 1.4.1, 3.2.4, 3.3.1, 3.3.3): no surface, verb or refusal added or removed apart from the send warning.
@task_item n=4 part=untouched
The refusal layout unable reason detail then hint:, the unknown exit 14 no-hint exception, the hint-free --json and the answer / send exit-2 paths: no refusal, reason code or exit code changed.
@task_item n=5 part=untouched
The manual Orca screen-reader passes (GNOME Terminal for cli, foot on Hyprland for tui) and their founder ownership: not exercised or changed by this session.
@task_item n=6 part=untouched
The frontend deferral to 0.2.0 and its unchosen technology, the no browser a11y leg, and the ban on a new suite value such as a11y: unchanged.
@task_item n=7 part=untouched
The §11 anti-pattern bans (no WCAG conformance claim, no spinners, no retry-once, no soft SLO, no screen-content parsing of the claude TUI): no rule changed.
@task_item n=8 part=untouched
The §12 Decisions Log pointers to a11y-plan-amendments.md and its archive: no a11y decision taken this session.
