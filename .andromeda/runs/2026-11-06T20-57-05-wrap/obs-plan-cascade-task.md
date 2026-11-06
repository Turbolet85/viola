@task_prose part=session
The 0.1.0 version done-check: every gate green on Linux (ci#42288148289 on 849aad18, seven jobs), 0.1.0 closed.
portable-pty moved =0.8.1 to =0.9.0 (nix 0.25.1 to 0.28.0, the serial family and pin-utils gone); deny.toml advisories ignore emptied, RUSTSEC-2026-0441 closed; pty_backend stays openpty.
47 Windows/macOS remnant files cleaned: plain_drive, the Git Bash rewritten-path warning in send, EXE_SUFFIX joins and the .exe trim retired; one new contract test dialog_drift_names_a_changed_annotation.
Live round on claude 2.1.287: viola verify read 23 pass 1 fail (statusline-rate-limits red, the row reads the account plan, not the CLI); the near-ceiling hold unread, so the ceiling stays PROVISIONAL owned by 0.2.0's first entry; v1-27 deferred to 0.2.0 on the founder's word.
The five one-enum error divergences ratified as named exceptions (T1); the thiserror census rule re-worded to the product sources, the count stays 9; Epoch 7's four PROVISIONAL items confirmed and their marks lifted.
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
@task_file n=33 path=tests/cli_send.rs
@task_file n=34 path=tests/cli_output_plain.rs
@task_file n=35 path=tests/support/home.rs
@task_file n=36 path=tests/support/fake.rs
@task_file n=37 path=tests/support/start.rs
@task_file n=38 path=tests/chaos_feed_panic.rs
@task_file n=39 path=tests/hook_fail_open.rs
@task_file n=40 path=.config/nextest.toml
@task_item n=1 part=touches
The viola-pty scope entity in section 1 (fct-t2iihkln) names portable-pty =0.8.1; the pin is now =0.9.0, the openpty backend unchanged.
@task_item n=2 part=touches
Span-trace coverage names portable-pty =0.8.1 for the PTY seam (fct-xrkfnxjv); the pin is now =0.9.0. Any other =0.8.1 pin or its Windows garbage-read / ConPTY-EOF rationale in obs-plan moves to =0.9.0 with no Windows reason.
@task_item n=3 part=touches
Log coverage fct-yb5jpaot: the process-start claude-child field cells cut to the header's 2 cells, pty_backend carries openpty alone and sideload_fallback is gone (F434 whole-fact edit).
@task_item n=4 part=touches
The PTY seam boundary wrapper and process-start claude-child rows (fct-f3gi2vhc, the process-start rows) re-read on 0.9.0: pty_backend openpty, exit from the handle wait, unchanged; a re-read, no value moved.
@task_item n=5 part=touches
Error capture platform pick fct-nwrbbhk2 and fct-j5jj3gux: the thiserror type census rule is re-worded to the product sources (a grep now reads 10 rows, the 10th crates/viola-e2e/src/harness/mod.rs test-only); the count stays 9.
@task_item n=6 part=touches
The five one-enum error divergences are ratified as named exceptions (T1); any obs-plan error-capture or detail-code text naming those divergences as open or unratified now reads them as named exceptions.
@task_item n=7 part=touches
The near-ceiling hold ceiling stays PROVISIONAL (the hold was not read); its owner moves from Version done-check to 0.2.0's first entry; the obs-plan rows naming that ceiling and its owner (2 rows) re-point.
@task_item n=8 part=touches
Epoch 7's four PROVISIONAL items are confirmed on the founder's word (the socket-directory pair's wording, the board's read rule, the unix-endpoint chunk's P5 word and leans, the 151-proposal answer-card confirm); obs-plan text carrying those PROVISIONAL marks (about 5 rows, e.g. socket-dir-refused and strict-modes wording) loses the mark, the rule itself unchanged.
@task_item n=9 part=touches
Budget governor scenario and the capability ledger edge flow: the statusline-rate-limits ledger row read red on an unchanged CLI 2.1.287 because the account's payload carries five_hour unknown and only seven_day; the row reads the account's plan, the budget-paused refusal rests on it, owner 0.2.0 (finding F-06i).
@task_item n=10 part=touches
Supply-chain trigger and cargo deny rule (fct-r2s52jjf, section 10 cargo deny failure condition): deny.toml advisories ignore is now empty (RUSTSEC-2017-0008 gone with serial), cargo deny reads advisories, bans, licenses, sources ok; any obs-plan mention of an ignored advisory moves to none.
@task_item n=11 part=touches
The send verb no longer writes the Git Bash rewritten-path warning to stderr (send writes no stderr line on a rewritten path); any obs-plan stderr or human-output text naming that warning or plain_drive / verbatim-drive handling is retired.
@task_item n=12 part=touches
The fixed binary names: EXE_SUFFIX joins are gone, the harness and tests use the plain names viola, claude, decoy-hook; any obs-plan path or artifact naming a .exe suffix is retired.
@task_item n=13 part=touches
CI section 9 and multi-platform exporter trigger: matrix ubuntu-latest only, nextest timing comments now the ubuntu reading; artifacts junit-ubuntu-latest, diag-ubuntu-latest, diag-perf-ubuntu-latest, perf-ubuntu-latest, supply-chain on ci#42288148289 confirm the diag-os naming on Linux; a re-read, no rule moved.
@task_item n=14 part=touches
v1-27 (cli_verified true and no viola byte on screen) deferred to 0.2.0; obs-plan text citing v1-27 or the unverified-CLI dialog branch as proved in 0.1.0 reads the verified clauses as unread, the unverified live clauses and the fake-agent half green.
@task_fact n=1 slug=span-trace-coverage
fact: The statusline-rate-limits capability-ledger row reads the account's plan state, not the CLI: on claude 2.1.287 a payload with five_hour unknown and only seven_day reads it red (verify 23 pass 1 fail); whether the budget gate stays and what it stands on is 0.2.0's (finding F-06i).
@task_item n=1 part=untouched
The obs tier, the Minimal exporter carve-outs, the OTel SDK and sentry rejections and the no-egress posture: no product surface, verb, flag, env var, log field or event kind was added.
@task_item n=2 part=untouched
Section 11 anti-patterns and the PII scrubbing rules (veil toggle ban, default-deny catalog, never-log floor): unchanged by the session.
@task_item n=3 part=untouched
Section 5 metric coverage and the derived-metric naming and cardinality rules: no metric moved.
@task_item n=4 part=untouched
Section 10 SLO invariants and perf budgets: run --perf and gate --require perf green, budgets and the zero-unlogged-panics invariant unchanged.
@task_item n=5 part=untouched
The closed event enum, the detail code catalog and the channel trace propagation (corr plus conn): no event kind or detail code added or removed.
@task_item n=6 part=untouched
Section 12 Decisions Log pointer and the section 3 contracts table: no obs decision taken in this session.
@task_item n=7 part=untouched
The CLI-1 send, wait / last and wheel / release scenarios: their logging rules are unchanged; only the Git Bash stderr warning left send.
