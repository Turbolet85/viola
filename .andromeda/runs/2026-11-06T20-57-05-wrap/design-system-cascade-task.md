@task_prose part=session
The version done-check of 0.1.0 on Linux: every gate green (ci#42288148289, seven jobs, on 849aad18), the open caps ruled, no product surface added (no verb, flag, env var, channel method, log field or event kind; the HELP literal and help tests byte-identical).
Dependency move: portable-pty =0.8.1 to =0.9.0 (nix 0.28.0, serial family and pin-utils gone), deny.toml advisories ignore list emptied, RUSTSEC-2026-0441 closed; pty_backend stays openpty.
Retired the Windows remnants: the Git Bash rewritten-path warning (send no longer writes its stderr line; emitter, unit table and CLI witness removed, the founder's T2), plain_drive and its verbatim-drive handling, every EXE_SUFFIX join and .exe trim; binaries are the plain names viola, claude, decoy-hook.
Added one contract test, dialog_drift_names_a_changed_annotation; measured a live round: viola verify read stamped 2.1.287 23 pass 1 fail, the statusline-rate-limits row red because the account status-line payload carries five_hour unknown and only seven_day, so the row reads the account plan, not the CLI.
Founder rulings: v1-27 and v1-11, v1-21, v1-34, v1-36 deferred to 0.2.0; Epoch 7's four PROVISIONAL items confirmed (socket-directory pair wording, the board read rule, a P5 word and leans, an answer-card confirm) and their marks lifted; the near-ceiling hold stays PROVISIONAL, owned by 0.2.0's first entry; why viola leans on the user status line goes to 0.2.0 intent (F-06i).
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
@task_file n=20 path=crates/viola-e2e/tests/cli.rs
@task_file n=21 path=crates/viola-e2e/tests/mcp_tools.rs
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
Platform-Specific Notes: the Git Bash on Windows line (leading-slash rewrite, the plain stderr warning for a rewritten-path argument) is retired; send writes no such warning and the line leaves the doc.
@task_item n=2 part=touches
The rule that prompt text comes only from stdin or --file stands, but its Git Bash reason is gone with Windows; any text giving that reason must be re-worded or dropped.
@task_item n=3 part=touches
Streams and stderr listings of send: the rewritten-path warning was the only stderr line send wrote on a rewritten path; any enumeration of send stderr output naming it is stale.
@task_item n=4 part=touches
Epoch 7 PROVISIONAL marks in design-system (about five rows, including the Surface: cli OPEN on fct-bmfbeece) are discharged on the founder's own word: the socket-directory pair wording, the board read rule, the P5 word and leans, the answer-card confirm; any PROVISIONAL or OPEN mark on them is lifted.
@task_item n=5 part=touches
The socket-dir-refused cause and the strict-modes refusal pairs (list board read rule, instance-state and home-state unable/hint pairs, exit 21 detail words) rest on the confirmed Epoch 7 wording; their marks change, the words do not.
@task_item n=6 part=touches
viola verify step counter and summary form stamped ver n pass m fail: measured live on 2.1.287 as 23 pass 1 fail with statusline-rate-limits red; the ledger stays 24 rows, so the relied count is unchanged.
@task_item n=7 part=touches
Fuel state / budget governor (5H and 7D figures, budget-paused, the BAY line): the statusline-rate-limits row it rests on reads the account plan state, not the CLI; the account payload measured five_hour unknown and only seven_day; whether the budget gate stays and what it stands on is owned by 0.2.0 (F-06i).
@task_item n=8 part=touches
Verified aircraft type anchor (verified vs unverified-cli): a verify can go red on an unchanged CLI, so an unverified-cli start can follow a red verify; the live round ran one unverified wrapped start with cli_verified false.
@task_item n=9 part=touches
viola list carries no link column in v1 (iconography, list columns): re-read and unmoved; v1-36, the cap about showing the link set, is deferred to 0.2.0.
@task_item n=10 part=touches
Surface: cli platform line (Linux terminals, CI on ubuntu-latest): re-read and holds; CI seven jobs green on ubuntu-latest, nextest timing comments moved to the ubuntu reading.
@task_item n=11 part=touches
Toolkit claim that no dependency reads a colour or terminal variable: the dependency graph moved (portable-pty 0.9.0, nix 0.28.0, serial2 and cfg_aliases added, serial family, pin-utils, termios removed); the claim is re-read on the new graph, no dependency added to the viola bin styling.
@task_item n=12 part=touches
viola run prints nothing once the child starts: witnessed live on the unverified start (pty_backend openpty, the child holding fds 0-2 on the pty); unmoved.
@task_item n=13 part=touches
The binary name in examples and samples is the plain name viola (EXE_SUFFIX joins and the .exe trim removed); any text allowing a viola.exe or OS-suffixed name is stale.
@task_item n=14 part=touches
The near-ceiling hold and its PROVISIONAL ceiling: kept PROVISIONAL, its owner re-pointed from Version done-check to 0.2.0's first entry; any design-system text naming the done-check as owner moves.
@task_fact n=1 slug=surface
fact: viola verify can read red on an unchanged CLI: the statusline-rate-limits ledger row reads the account's status-line payload, which on a plan carrying only the seven-day window (five_hour unknown) fails the row, as measured on 2.1.287 (23 pass 1 fail).
@task_item n=1 part=untouched
Color Palette, contrast rules, surface scale, text hierarchy, border progression and domain status colours: no hex, ratio or role changed.
@task_item n=2 part=untouched
Typography, including the Bahnschrift and Cascadia Mono Windows-only font notes and the Linux fallback stacks: a font provenance fact for a 0.2.0 frontend, not Windows code; the remnant retirement did not reach it.
@task_item n=3 part=untouched
Spacing, Depth Strategy, Border Radius, Motion and the expression level: no token or duration moved.
@task_item n=4 part=untouched
The readback box and its CLI mirror [RB] / [  ] / [/ ], the send sample lines, hints and unconfirmable: no send result form changed.
@task_item n=5 part=untouched
viola --help verb grouping and the flat verb list: the HELP literal and both help tests are byte-identical; no verb added or removed.
@task_item n=6 part=untouched
Exit-code phraseology: no code, cause word or document shape changed.
@task_item n=7 part=untouched
Colour decision order (--json, run, hook and mcp, NO_COLOR, TERM=dumb, non-TTY) and depth selection: unchanged.
@task_item n=8 part=untouched
wait / last output forms and the message-mode escaper: unchanged; the new dialog_drift contract test covers the fake agent's synthetic dialog set, not a design surface.
@task_item n=9 part=untouched
Surface: web-spa and every 0.2.0 frontend design input: still deferred; nothing in the session reaches them.
@task_item n=10 part=untouched
Anti-Patterns and Self-Validation Protocol: no ban or test changed.
@task_item n=11 part=untouched
The terminal-reply hypothesis and the tab-close leg carried to 0.2.0 have no design-system text; nothing to flag for them.
