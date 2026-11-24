@task_prose part=session
Added a new CLI verb, viola switch <name> [--model] [--effort] [--json]. It restarts a running wrapped session's claude child on a given model and effort pair and resumes the same session id. The pair is recorded so that a later revive replays it, and nothing is written to the user's own settings.
Added a new wrapper channel method, switch {v, sender, model?, effort?}. It answers {ok:{model,effort}} or a refusal, checked in this order: human-typing, not-delivered/turn-running, not-delivered/dialog-pending, unverified-cli, not-delivered/no-session, then not-delivered/child-not-ended and not-delivered/resume-failed. A call with neither half, or a bad half, gets -32602 with data null. The closed channel method-label list goes from 10 to 11.
The PTY pump now owns a respawned child. It kills only the child its own PTY handle holds, reaps it within a new compiled bound RESPAWN_REAP_BOUND of 10 s, appends one launch line, then spawns the child on a new pseudo-terminal pair under the same host terminal. A child not ended within the bound is not replaced.
While a switch is under way, a send is refused turn-running until the new child's session-start. After that, the old child's late session-end for the switched id is screened once.
Everything was proven over the fake agent alone and no capability is claimed. The forks F1, F3, F5 and F6 are PROVISIONAL on the delegate overseer's word. The live readings R6a and R6b on a real claude are carried to the founder's host. CI on the pre-CI commit was green 7/7.
@task_file n=1 path=src/cmd/switch.rs
what: new; the viola switch CLI verb, a frame verb that checks strict modes and the instance first, sends a switch call to the recorded wrapper and maps each refusal to its exit code and human or --json output
@task_file n=2 path=src/run/switch.rs
what: new; the wrapper's switch channel method handler, which holds the refusal order, the -32602 param checks on the model and effort halves, the launch append and the respawn outcomes, with its unit cases
@task_file n=3 path=tests/cli_switch.rs
what: new; the root integration binary for viola switch over the fake agent covering idle switch, user settings unchanged, revive replay, the refusals mid-turn, with a dialog pending and while the human types, usage errors, a gone instance and the --json document
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: edited; the PTY pump, which gains the respawn handle, the RESPAWN_REAP_BOUND reap bound, the old-child kill and the new-PTY swap under the same host stdin and stdout, plus its mock-PTY unit cases
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: edited; the viola-pty crate's public surface, which now re-exports Respawn, RespawnHandle and RESPAWN_REAP_BOUND
@task_file n=6 path=crates/viola-channel/src/server.rs
what: edited; the wrapper channel server's closed method-label list METHODS, which gains switch (10 to 11)
@task_file n=7 path=src/cmd/run.rs
what: edited; the viola run wrapper start and pump_child, which threads the respawn handle and routes the switch channel method in dispatch_call
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: edited; viola verify's typed run spawn, which passes a respawn handle that nothing requests
@task_file n=9 path=src/cmd/mod.rs
what: edited; the CLI verb table and dispatch, where switch is added to the setup row of viola --help after up
@task_file n=10 path=src/run/mod.rs
what: edited; the wrapper run module list, which gains run::switch
@task_file n=11 path=src/run/send.rs
what: edited; the send method's refusal path, where a send during a switch is refused turn-running
@task_file n=12 path=src/run/send/slot.rs
what: edited; the send slot and flight, which gain the switch mark and the one-shot screen of the old child's late session-end
@task_file n=13 path=src/human.rs
what: edited; the human-mode output module list
@task_file n=14 path=src/human/verbs.rs
what: edited; the human-mode verb lines, which gain the switched line <name>  switched  <model> <effort>
@task_file n=15 path=src/human/hints.rs
what: edited; the exit-cause hint table, with four new send_hint causes: dialog-pending, no-session, child-not-ended and resume-failed
@task_file n=16 path=tests/cli_output_plain.rs
what: edited; the plain-output literal check of viola --help, where the served verb list goes from 17 to 18 with switch
@task_file n=17 path=tests/contract_error_bodies.rs
what: edited; the error-body surface witness census: CHANNEL_METHODS goes from 10 to 11, and CLI_DOCUMENTS gains 3 switch rows (human-typing 10, unverified-cli 12, not-delivered 13)
@task_file n=18 path=tests/cross_exit_causes.rs
what: edited; the cross-surface exit-cause table REFUSALS, which goes from 1 to 5 rows with dialog-pending, no-session, child-not-ended and resume-failed, each at exit 13
@task_file n=19 path=tests/support/home.rs
what: edited; the shared test home and wrapper helpers, which gain Wrapper::adopt_as_user; adopt now calls it with a test-owned user home
@task_item n=1 part=touches
contract/viola-help/verb-table: the setup row of the grouped verb table now reads open, run, revive, adopt, up, switch, verify, plugin install, and the served verbs go from 17 to 18.
@task_item n=2 part=touches
decision/send-refusal-order and contract/viola-send/turn-running: a send is now also refused turn-running while a switch stands, from the switch's request until the new child's session-start. No new word is added.
@task_item n=3 part=touches
decision/hint-names-its-cause and decision/each-exit-cause-own-hint: four new exit causes, each with its own hint (dialog-pending, no-session, child-not-ended, resume-failed). decision/no-release-in-driver-hints and decision/hints-never-suggest-release apply to them.
@task_item n=4 part=touches
decision/error-writers-each-named and decision/error-bodies-held-by-surface-witnesses: a new CLI error writer (viola switch) has three named bodies, and the channel method census grows from 10 to 11.
@task_item n=5 part=touches
contract/cli-exit-codes/10-14, contract/cli-exit-codes/12, contract/cli-exit-codes/13 and contract/cli-exit-codes/14: a new verb uses them. human-typing is 10, unverified-cli is 12, the not-delivered refusals are 13, plus 14. 20 and 21 also apply.
@task_item n=6 part=touches
contract/wrapper-channel/-32602, contract/json-rpc-errors/-32602, contract/channel-errors/32602 and contract/channel-response-detail/invalid-params: the switch method answers -32602 with data null for neither half, a half of another type, or a half that fails its check.
@task_item n=7 part=touches
contract/channel-log/unserved-method and decision/closed-enums-need-decisions-log: the closed channel method-label list gains switch, so a switch call's channel-request and channel-response lines now name their method.
@task_item n=8 part=touches
decision/unverified-cli-gates-answer-only, decision/unverified-cli-send-only and decision/unverified-cli-gate: an unverified CLI now gates switch as well as answer (refused unverified-cli, exit 12). Rows saying the gate covers answer only are contradicted.
@task_item n=9 part=touches
decision/version-gate-needs-all-relied-rows: switch's unverified-cli check reads the start's cli_verified. Because every ledger row is relied, a verified CLI counts resume-session by construction.
@task_item n=10 part=touches
decision/child-exit-read-on-the-process-handle, decision/pty-exit-from-wait-not-eof, decision/exit-detected-on-handle and decision/child-exit-by-wait: the respawn reaps the old child on its handle within RESPAWN_REAP_BOUND. A child that does not end is reported NotEnded and not replaced.
@task_item n=11 part=touches
decision/pty-tests-real-openpty, decision/real-pty-in-integration and decision/mocks-only-at-product-seams: the respawn seam is tested in units over the mock PTY through a spawner seam, and in integration over the real fake-agent child.
@task_item n=12 part=touches
contract/instance-snapshot/child-pid and contract/snapshot/run-fields: a switch rewrites the snapshot's child_pid and launch while the wrapper pid stays the same.
@task_item n=13 part=touches
contract/process-start/claude-child, contract/process-start-claude-child/env-kept, contract/process-start-claude-child/env-stripped-count and contract/process-start-claude-child/env-stripped-known: each switch writes a new claude-child process-start for the respawned child, and the env strip is applied again.
@task_item n=14 part=touches
decision/snapshot-replay-fields and decision/revive-preflight-order: one launch line is appended per switch, after the reap and before the respawn. revive with no pair replays the switched pair.
@task_item n=15 part=touches
decision/fake-agent-not-proof-of-real-cli, decision/adopt-proven-live, decision/window-reopen-proven-live and decision/live-failing-reading-not-verification: the switch is proven over the fake agent alone. The real-CLI readings R6a and R6b are carried, so no capability is claimed.
@task_item n=16 part=touches
decision/fake-agent-exit-ignores-sighup and decision/fake-agent-own-process-group: the respawn's SIGHUP leaves the old fake child to the kill fallback (exit 1 kill-fallback), as measured.
@task_item n=17 part=touches
decision/session-end-direct-append, contract/perf-budgets/session-end and decision/defensive-log-reader: after a respawn, a late session-end of the switched id is screened once while the new child's pid is alive.
@task_item n=18 part=touches
decision/human-never-blocked, decision/human-input-takes-the-wheel and decision/human-takes-wheel-on-edit: a switch is refused human-typing, exit 10, while the human holds the wheel, and a manual pause is refused with its detail.
@task_item n=19 part=touches
decision/frame-verbs-check-instance-first, contract/strict-modes-refusal/other-verbs and decision/server-checked-before-any-frame: the new frame verb runs the untrusted-home refusal, then the live endpoint, then the verified connect, before any frame.
@task_item n=20 part=touches
decision/mcp-no-control-tools, contract/mcp-tools-list/tool-set and contract/mcp-server/tools-list: switch is CLI-only (F5, PROVISIONAL), so the MCP tool set is unchanged.
@task_item n=21 part=touches
decision/integration-binary-surface-prefix and decision/test-function-naming: a new root integration binary with the cli_ prefix, and new tests named switch_ in units and integration.
@task_item n=22 part=touches
decision/cli-json-one-document, contract/cli-json-output/stdout-document and contract/cli-human-output/no-sgr: viola switch --json prints one pair document. The human switched line is plain ASCII, one line, with no SGR.
@task_item n=23 part=touches
decision/run-under-test-owned-user-home and decision/harness-wrappers-under-session-user-home: a new shared helper adopts a wrapper under a test-owned user home, and the existing adopt helper now goes through it.
@task_item n=24 part=touches
decision/ci-stamped-home-kill-line, decision/within-7s-wait-bound and decision/root-wait-bound: a new compiled 10 s reap bound is waited on inside switch tests, so it must sit below the per-test kill lines.
@task_item n=25 part=touches
decision/start-lines-read-from-start-offset and decision/waits-key-on-events: switch tests read the launch and the resumed session-start from the switch's own log offset, and their waits key on logged events.
@task_item n=26 part=touches
decision/child-argv-read-from-proc: the idle-switch test reads --resume <id> --model M --effort E from the live child's cmdline.
@task_item n=27 part=touches
decision/claude-child-holds-nothing-of-viola and decision/single-writers: the switch passes no --settings and leaves the user's config and settings byte-equal and mtime-equal. The pump stays the single owner of the child.
@task_fact n=1 slug=unit-test-strategy
fact: The switch refusal order is human-typing, turn-running, dialog-pending, unverified-cli, then no-session. The first rung wins, and no refusal appends a line or ends the child (PROVISIONAL, F3).
@task_fact n=2 slug=unit-test-strategy
fact: A send is refused turn-running from a switch's request until the new child's session-start.
@task_fact n=3 slug=unit-test-strategy
fact: After a respawn, the first late session-end of the switched id is screened once while the new child's pid is alive. Known limit: the screen cannot tell the old child's late session-end from the new child's own, which is owed to the live reading R6a.
@task_fact n=4 slug=unit-test-strategy
fact: RESPAWN_REAP_BOUND is a compiled time bound of 10 s. A child not exited within it is not replaced, and the switch is refused not-delivered child-not-ended (PROVISIONAL, F6).
@task_fact n=5 slug=e2e-test-strategy
fact: viola switch exits 0, 10 (human-typing), 12 (unverified-cli), 13 (not-delivered turn-running, dialog-pending, no-session, child-not-ended, resume-failed), 14, 20 and 21. Passing neither --model nor --effort, or a bad value, is a usage error with exit 2.
@task_fact n=6 slug=integration-test-strategy
fact: The switch channel method takes at least one of model and effort, re-checks each half in the wrapper and reads no from. Neither half, a half of another type, or a failing half is -32602 with data null.
@task_fact n=7 slug=e2e-test-strategy
fact: A switch appends one launch line with the new pair after the reap and before the respawn. A later revive with no pair replays the switched pair.
@task_fact n=8 slug=e2e-test-strategy
fact: A switch leaves the user's config and settings byte-equal and mtime-equal and passes no --settings. This is tested under a test-owned user home.
@task_fact n=9 slug=e2e-test-strategy
fact: A switch ends only the child its own PTY handle holds, with no pid lookup. It resumes the same session id on a new pseudo-terminal pair under the same host terminal, and the wrapper pid does not change (PROVISIONAL, F1).
@task_fact n=10 slug=e2e-test-strategy
fact: A real switch is proven only by the gated live readings R6a (an idle switch read on the next turn, settings byte-equal, whether SessionEnd fires on SIGHUP) and R6b (a switch refused mid-turn). Over the fake agent, no capability is claimed.
@task_fact n=11 slug=integration-test-strategy
fact: The human output of viola switch is the single plain line <name>  switched  <model> <effort>. A refusal prints the unable line with its word.
@task_fact n=12 slug=test-scope-summary
fact: switch is a CLI-only control verb with no MCP tool (PROVISIONAL, F5).
@task_item n=1 part=untouched
CI integration rows (workflows, SHA pinning, artifacts, secret scan, rust-cache, least permissions, msrv): no workflow or CI job changed.
@task_item n=2 part=untouched
Coverage floors, mutation gates and their thresholds, perf budgets and hyperfine rows: no threshold, floor or budget moved.
@task_item n=3 part=untouched
Harness contract rows (agent-run, boot, status, cleanup, gate, pre-push, supervise, run verdicts and reasons): the harness and its documents are unchanged.
@task_item n=4 part=untouched
Fake-agent command line, receipts, scripts and recorded fixtures, and the fixture hygiene and scrub rows: the fake agent and its fixtures were not modified.
@task_item n=5 part=untouched
Log format, the diag-line schema and log-event corr rows: no schema change, and no new detail word is emitted on any line.
@task_item n=6 part=untouched
Hook decision bodies, dialog answer mapping, dialog hold and the viola hook exec contract: no dialog or hook code changed. The switch only reads pending_dialog from the snapshot.
@task_item n=7 part=untouched
Link and unlink methods, the list board, wait and last, and release rows: their behaviour is unchanged.
@task_item n=8 part=untouched
Supply-chain rows (cargo-deny bans, tokio bans, rmcp features, serde_json depth): no dependency was added or removed.
@task_item n=9 part=untouched
decision/no-switch-disables-a-control and similar rows that use switch as an English word: they do not concern the new verb.
@task_item n=10 part=untouched
Absences (browser suite, Windows and macOS, database, outbound HTTP, deploy stage): none is reached.
