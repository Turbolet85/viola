@task_prose part=session
The session made /context a ledger-listed local command, confirmed by a new local-command-stdout tag in the transcript and submitting no prompt; before its transcript exists a send of it reads ok with confirmed false and detail unconfirmable.
It added the relied ledger row local-command-context (relied rows 24 to 25), checked by verify's Run B pasting /context as its eighth compiled paste and timing context_latency_ms within the confirm window fallback.
A stamp holding only the 24 earlier ids now reads unverified on every build after this chunk; verify's step lines read [n/25] and its all-pass summary 25 pass 0 fail; every recorded fixture set re-stamps 25/25 over the fake agent with no new recording.
The fake agent under --framing now echoes the exact /context probe literal as it does the echo probe; the standing live rig gained labels CS and CC, a kept stamp copied out of a CS verify and seeded into the CC scratch home, a config-top reading, and an attribute-word reading of every drawn hint cell past the cursor cell.
Measured live on claude 2.1.328 under the rig's private config: CS read local-command-context pass, 25/25; CC read /context after a turn with one stdout tag and no prompt submitted; the hint reading came back unread (no-cell); the live home was not re-stamped.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger of relied rows, its compiled local-command list with each post-condition, the probe literals, each row's check, and the stamps merge — edited
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: inline unit cases of the ledger — edited
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: inline unit cases of the ledger row checks — edited
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: inline unit cases pinning the framing probe literals against the local-command list — edited
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: inline unit cases of the verify runs' row checks, including the local-command-context check — edited
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: inline unit cases of stamps and the version gate's every-relied-row rule, a stamp of the earlier ids reading unverified — edited
@task_file n=7 path=src/cmd/verify.rs
what: viola verify, its step lines and summary line over the relied rows — edited
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: verify's typed probe runs, threading the measured probe latencies into the stamp — edited
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: verify Run B's framing turns, its compiled pastes in order, now with /context as the eighth paste — edited
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake agent standing in for claude, its --framing echo of the probe command literals — edited
@task_file n=11 path=src/run/send/classify.rs
what: the driver send classifier for ledger-listed local commands and their post-condition confirmation, with its inline unit cases — edited
@task_file n=12 path=src/run/version_gate.rs
what: the unverified CLI version gate that needs every relied row passing in a stamp — edited
@task_file n=13 path=tests/cli_send.rs
what: the viola send integration suite run as a subprocess over the fake agent — edited
@task_file n=14 path=tests/cli_verify.rs
what: the viola verify integration suite root, its step-line and summary pins — edited
@task_file n=15 path=tests/cli_verify/runs.rs
what: viola verify integration cases of the probe runs and their step lines — edited
@task_file n=16 path=tests/cli_verify/stamps.rs
what: viola verify integration cases of the stamps file and its summary — edited
@task_file n=17 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite replaying every recorded CLI-version fixture set through verify over the fake agent — edited
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command, its local-live suite and the relied ledger row list with the expected verify pass lines — edited
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: the standing committed live rig, its labels, arms, readings, summary words and floor rows — edited
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model reading box rows and cell attributes as attribute words, never text — edited
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary and its rehearse and live words — edited
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal suite run against the fake agent — edited
@task_item n=1 part=touches
decision/local-command-post-condition and decision/local-command-unconfirmable: /context joins the local commands confirmed by a measured post-condition (a new stdout tag), the fourth such command; on an unverified CLI or before its transcript exists it reads unconfirmable.
@task_item n=2 part=touches
contract/driver-send/local-command, contract/viola-send/unconfirmable-local-command, contract/send-result/unconfirmable and contract/viola-send-human-mode/unconfirmable: a /context send before its transcript exists returns ok with confirmed false and detail unconfirmable, exit 0, send-issued then send-confirmed with confirmed false.
@task_item n=3 part=touches
contract/viola-send/clear-confirmed and contract/viola-send/compact-confirmed: a third listed confirmation shape now stands beside them, /context confirmed by its stdout tag on a verified CLI.
@task_item n=4 part=touches
contract/fake-agent-input/echo-command is stale: under --framing the fake now echoes the exact /context probe literal as well as the verify echo probe command.
@task_item n=5 part=touches
decision/ledger-row-verify-probe: the new relied row local-command-context has its verify probe, Run B's eighth compiled paste /context, checked for no prompt and a tag within the confirm window fallback.
@task_item n=6 part=touches
decision/version-gate-needs-all-relied-rows, decision/unverified-cli-gate and decision/stamps-refuse-widened-ledger: the relied set grew to 25, so a stamp of exactly the 24 earlier ids reads unverified and every stamped version needs a re-stamp.
@task_item n=7 part=touches
contract/verify-output/step-line, contract/verify-output/summary-line, contract/verify-output/last-line, contract/viola-verify/stamped-line and contract/cli-human-lines/stamped: step lines now count to 25 and the all-pass summary reads 25 pass 0 fail; check any cell that holds the figure.
@task_item n=8 part=touches
decision/verify-failing-row-no-refusal: a failing local-command-context row reads fail, never absent, and is no refusal.
@task_item n=9 part=touches
decision/ledger-probe-contract-suite, decision/replay-every-cli-version-fixture, decision/fake-agent-matches-verify-fixtures and decision/stamps-from-fake-agent-verify: every stamped recorded set now stamps 25/25 over the fake agent with no new recording and the stamped set unmoved.
@task_item n=10 part=touches
contract/local-live-suite/pass and contract/local-live-failure/row-missing: the harness's relied row list and its expected live verify pass lines grew to 25 rows.
@task_item n=11 part=touches
decision/standing-live-rig: the rig gained the CS and CC labels (with CS-2 and CC-2 spares), rehearse cs and rehearse cc, a kept stamp, a stamp seed, a config-top reading and hint-cell attribute readings; a CC label with no kept stamp refuses before its ledger line.
@task_item n=12 part=touches
decision/stamps-written-only-by-verify, decision/stamps-only-from-verify, decision/verify-sole-stamp-writer, decision/single-writers and decision/state-files-never-hand-written: the rig copies a stamp a CS verify wrote and places it into the CC scratch home's ledger, a placement by copy rather than by verify.
@task_item n=13 part=touches
decision/live-failing-reading-not-verification and decision/terminal-reply-reading-live-measured: the CC hint words (hint-cells, hint-cells-attr, typed-attr, hint-told-apart) are recorded as readings; live they read unread (no-cell), the carry stays open.
@task_item n=14 part=touches
Rows saying the screen is read as the box row's cell attributes, never text (decision/screen-signatures-only, decision/no-screen-parsing, decision/never-parse-rendered-child-screen, decision/screen-fixture-box-rows-as-literals): the rig now reads the attribute words of every drawn hint cell past the cursor cell after the second send, a wider reading in exactly the founder's width.
@task_item n=15 part=touches
contract/fake-agent-cli/transcript and decision/stamping-verify-passes-transcript: the fake keeps --transcript files beside its own exe, shared by every concurrent test of the shared build; the row is true as worded, the plan's other placement was disproved.
@task_item n=16 part=touches
decision/fake-agent-copies-planted-first and decision/one-shared-test-helper: the first-act send case runs a private copy of the fake in a test-owned tempdir instead of removing a shared transcript file; no helper or fake option was added.
@task_item n=17 part=touches
decision/no-verify-start-outside-test-home: its sweep reads any verify literal in a test source as a verify start; an assertion on a stamp's writer field tripped it and was rewritten without the literal.
@task_item n=18 part=touches
decision/real-claude-local-only, decision/real-claude-never-in-ci, decision/live-verify-never-in-ci and decision/tests-never-read-host-sessions: two live starts ran locally against claude 2.1.328 under the rig's private config and scratch home, the live home's stamps unmoved, nothing in CI.
@task_item n=19 part=touches
decision/send-confirmation-window-expiry and decision/compact-confirm-window: the local-command-context check bounds context_latency_ms by the confirm window fallback; the rig's context waits are bounded by the smaller of the submit bound and the settle.
@task_item n=20 part=touches
decision/integration-binary-surface-prefix and decision/e2e-layer-by-binary-prefix: new cases joined the cli_send, cli_verify, contract_ledger_probes and rig_rehearsal binaries; no binary was added.
@task_fact n=1 slug=6-e2e-test-strategy
fact: The standing live rig's CS label stamps one verify into its scratch home and keeps that stamp; its CC label seeds the kept stamp into its own scratch home before one wrapped start and refuses before its ledger line when no kept stamp exists.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The rig's config-top reading reports only top-level plain names of its private config and their regular file counts, a name outside letters, digits, dot, underscore and dash or over 40 characters reading other; no byte of a file is kept.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The rig's hint-cell reading records the attribute words of every drawn hint cell past the cursor cell after the second send, as a reading and never as verification; no cell drawn reads unread (no-cell).
@task_fact n=4 slug=test-data-fixtures
fact: A test that must have no fake-agent transcript removes none from the shared build; it runs its own private copy of the fake agent in a test-owned directory, since transcripts sit beside the fake's exe.
@task_fact n=5 slug=unit-test-strategy
fact: /context is confirmed by a new local-command-stdout tag in its transcript with no prompt submitted; on an unverified CLI or before its transcript exists it reads unconfirmable.
@task_item n=1 part=untouched
MCP server, tools and their invalid-params, refusal and parity contracts: no MCP tool, schema or reply changed.
@task_item n=2 part=untouched
The wrapper channel, its JSON-RPC errors, link and unlink methods and socket-directory, strict-modes and server identity checks: no IPC method, endpoint or socket changed.
@task_item n=3 part=untouched
Hook exec entry points, dialog decision bodies, dialog holds and answer mapping: no hook or dialog path changed.
@task_item n=4 part=untouched
viola revive, viola list board, viola wait, last and release contracts and their exit codes: none of those verbs changed.
@task_item n=5 part=untouched
CI workflows, artifacts, supply-chain gates, coverage floors, mutation and performance gates: no CI step, threshold or gate changed.
@task_item n=6 part=untouched
Harness session, boot, cleanup, status, pre-push, secret-scan, schema-check and log-format contracts: no shape or code of those changed.
@task_item n=7 part=untouched
Fuzz, property and proptest seed rows and the event log, snapshot and heartbeat rows: nothing in those surfaces moved.
@task_item n=8 part=untouched
Absences (browser suite, database, outbound HTTP, deploy stage, Windows and macOS): no capability was added that ends one.
