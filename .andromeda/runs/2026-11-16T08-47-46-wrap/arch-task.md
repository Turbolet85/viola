@task_prose part=session
A local slash command the CLI runs now reads as delivered when its own echo is measured: send confirms /mcp by counting new `<local-command-stdout>` tags in the session transcript past the end recorded before the paste.
Local commands are classified by name (exactly a name, or a name, one space and a non-empty rest), and the list grew from 3 to 6 names: /clear new session, /compact same session, /mcp stdout echo, /remote-control, /cost and /usage with no post-condition.
The SessionStart transcript path is validated in the hook, carried to the wrapper as an optional `transcript` param beside the event on hook.event, re-validated there, and kept out of event data, logs and Debug.
The capability ledger gains a 22nd relied row, local-command-echo, probed by verify's Run B as a seventh paste (/mcp disable viola-probe-absent); the stamp's measured typed probe gains echo_latency_ms; verify's step counter and summary move from 21 to 22.
The fake agent gains an argv-only --transcript option that writes the launch session's transcript and appends the echo tag; the founder ratified the three widenings, and the live re-stamp on the 2.1.325 recorded fixture set is owed, not done.
Nothing retired; no dependency, crate, config key, env var or output literal added or changed.
@task_file n=1 path=src/run/send/echo.rs
what: new transcript echo reader of the root binary: opens the transcript O_NOFOLLOW, regular, owner-euid, reads from an offset bounded by MAX_FRAME, returns the end or a count of stdout tags alone
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger in the claude agent crate: relied rows, local command list with post-conditions, probe command literals, the transcript path shape check, stamp merge
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the claude hook payload normaliser: known SessionStart fields, the redacted transcript path kept only when its shape matches its own session id
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the capability ledger's row list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger rows' checks and their verdict vector
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit cases of the local command name table, by-name classification and the transcript path shape
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit cases of the typed-probe run checks, now the echo row's check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the ledger stamps envelope, merge and the verified-version verdict over 22 rows
@task_file n=9 path=src/cmd/hook.rs
what: the viola hook role's delivery to the wrapper channel: sends hook.event with the optional transcript param beside the event for a SessionStart
@task_file n=10 path=src/cmd/run.rs
what: the wrapper's run role channel method dispatch: takes a hook.event line and re-validates its transcript against the line's own session id
@task_file n=11 path=src/cmd/verify.rs
what: viola verify, the one stamps writer: step lines, summary line and the relied-row run
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's typed-probe runs that carry measurements into the stamp
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: verify's Run B framing turns: the added pastes into a settled input box, now the seventh echo paste, and the bounded read of Run B's SessionStart capture for the transcript
@task_file n=14 path=src/run/send.rs
what: the send path under the wrapper flight lock: classification, send-issued, paste, confirmation; records the transcript end before the paste or makes the send unconfirmable
@task_file n=15 path=src/run/send/classify.rs
what: the send classification of typed text against the local command list, and its unit cases for echo confirmation
@task_file n=16 path=src/run/send/slot.rs
what: the send slot and wrapper flight state: hook event appends, the remembered transcript per session-start, the confirmation wait that now counts echo tags each step
@task_file n=17 path=src/run/send/relabel.rs
what: the driver relabel of hook events in the send slot, and its unit cases
@task_file n=18 path=src/run/send/test_support.rs
what: test support for the send slot's unit cases
@task_file n=19 path=src/run/version_gate.rs
what: the version gate reading the ledger stamps strictly for verified or unverified CLI
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary: argv options, recorded framing replay, now the --transcript option and the echo tag append
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's argv option parsing
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the end-to-end harness boot that runs a stamped verify against the fake agent with its flags
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the end-to-end harness run and its expected verify step lines
@task_file n=24 path=tests/cli_send.rs
what: integration cases of viola send against a booted wrapper, including the echo read-back and the home byte search
@task_file n=25 path=tests/cli_verify.rs
what: integration cases of viola verify output and exit codes
@task_file n=26 path=tests/cli_verify/runs.rs
what: integration cases of verify's probe runs and their per-row verdicts
@task_file n=27 path=tests/cli_verify/stamps.rs
what: integration cases of verify's stamps and the summary counts
@task_file n=28 path=tests/contract_ledger_probes.rs
what: contract test holding the ledger rows to their probes
@task_file n=29 path=tests/support/verify.rs
what: test support for stamped-home verify runs and the fake agent verify flags
@task_item n=1 part=touches
decision/local-commands-by-ledger: classification moves from exact text to by name, and the list grows from 3 to 6 names; the row is stale.
@task_item n=2 part=touches
decision/local-command-post-conditions: /mcp is confirmed by its stdout echo in the transcript; /cost, /usage and /remote-control carry no post-condition; the row is stale.
@task_item n=3 part=touches
decision/capability-ledger-gate, decision/cli-capability-ledger and decision/version-verified-all-relied-rows: relied rows 21 to 22 with local-command-echo; a 21-row stamp now reads unverified. Any cell holding the relied-row count moves.
@task_item n=4 part=touches
contract/channel-methods/hook-event and the contract/hook-event rows: hook.event gains an optional `transcript` string param beside `event`, sent only for a SessionStart, re-validated by the wrapper, dropped otherwise while the line is still taken.
@task_item n=5 part=touches
contract/send-results/unconfirmable, contract/ok-payload/unconfirmable and contract/send-ok-payload/unconfirmable: an echo-confirmed local command with no remembered transcript, no readable end, or on an unverified CLI is unconfirmable.
@task_item n=6 part=touches
decision/every-send-confirmed, decision/send-confirmed-after-the-fact and decision/unconfirmed-delivery-not-delivered: a measured transcript echo becomes a further confirmer; no tag within the window still reads not-delivered with no-prompt-submitted.
@task_item n=7 part=touches
decision/verify-transcripts-accepted-residual: verify now reads Run B's transcript and its seventh paste adds a line to the input history; residuals named: whether /mcp disable viola-probe-absent writes the user's real Claude config is unmeasured, and /context was never sent.
@task_item n=8 part=touches
contract/viola-verify-output/step-line and contract/viola-verify-output/summary-line: the counter becomes [NN/22] and a clean summary 22 pass 0 fail.
@task_item n=9 part=touches
contract/ledger-stamps/measured: the stamp's measured typed_probe gains echo_latency_ms (additive).
@task_item n=10 part=touches
decision/bounded-readers and contract/wrapper-channel-frames/max-frame: two new readers, the transcript echo reader and verify's SessionStart capture read, both bounded by MAX_FRAME.
@task_item n=11 part=touches
decision/send-flight-lock and decision/send-time-bound: the transcript end is recorded inside the guarded section before send-issued and the paste; the tag count is read outside the flight guard within the existing confirmation window.
@task_item n=12 part=touches
decision/channel-blocks-never-polls and decision/wait-blocks-nothing-polls: the send confirmation wait now re-reads the transcript each step; check whether a rule against polling reaches it.
@task_item n=13 part=touches
decision/confirm-window-compiled and decision/compact-confirm-window: the echo row's check holds echo_latency_ms within the compiled fallback confirmation window, the compact row's pattern.
@task_item n=14 part=touches
decision/frame-params-redacted, decision/content-detail-per-instance and decision/sanitised-error-surfaces: the transcript path is redacted in Debug and reaches no event data, events log, process log, span or reply.
@task_item n=15 part=touches
decision/tolerant-external-payloads and decision/untrusted-upstream-text: the SessionStart transcript_path is accepted only when absolute, free of .., named after its own session id and that id well-shaped.
@task_item n=16 part=touches
decision/agent-claude-pure, decision/claude-shapes-isolated and decision/claude-shapes-in-one-crate: the name table and path shape check stay in the agent crate with no file I/O; the transcript file read lives in the root binary.
@task_item n=17 part=touches
decision/verify-box-wait-for-literal and decision/verify-run-e-resumes-run-b-first-session: Run B's added pastes grow from 6 to 7, the echo paste after the newline paste and before /compact, skipped when no readable transcript, and the echo row then fails.
@task_item n=18 part=touches
decision/no-probe-for-unrelied-shapes and decision/no-typing-into-trust-dialogs: verify types a new probe command into a settled input box for a newly relied shape.
@task_item n=19 part=touches
decision/fake-agent-test-seams and decision/tests-against-fake-agent: the fake agent gains an argv-only --transcript option, passed beside --framing in every stamped verify; without it the recorded sets replay byte for byte.
@task_item n=20 part=touches
decision/version-gate-strict-stamps-read and decision/malformed-stamps-unverified: the strict stamps read now needs all 22 relied rows.
@task_item n=21 part=touches
contract/events-ndjson/send-confirmed-order and contract/wrapper-event-appends/send-records: an echo-confirmed send writes send-issued then send-confirmed with no awaited hook line.
@task_item n=22 part=touches
contract/event-data/session-start and contract/hook-to-kind-map/sessionstart: the SessionStart payload now yields a transcript path that stays out of the session-start data.
@task_fact n=1 slug=established-decisions
fact: send confirms a local command whose post-condition is a stdout echo by counting `<local-command-stdout>` tags in the session transcript past the end recorded before the paste; a count alone crosses, no content, and the read is O_NOFOLLOW, regular file, owner euid, bounded by MAX_FRAME (ratified by the founder).
@task_fact n=2 slug=standard-contracts
fact: hook.event carries an optional `transcript` string param beside `event`, sent only for a SessionStart whose path passes the shape check against its own session id, re-validated by the wrapper for a session-start line, any other dropped; it never enters event data.
@task_fact n=3 slug=established-decisions
fact: local-command-echo is the 22nd relied ledger row, probed by Run B's seventh paste /mcp disable viola-probe-absent; it passes on a tag with no UserPromptSubmit within the confirmation window; with no readable transcript the paste is skipped and the row fails.
@task_fact n=4 slug=established-decisions
fact: residuals of the echo probe: whether /mcp disable viola-probe-absent writes the user's real Claude config is unmeasured, the paste adds a line to the input history, and /context's echo shape is unmeasured; the live re-stamp with the 22nd row on the 2.1.325 recorded fixture set is owed.
@task_item n=1 part=untouched
The wheel rows: byte-source claim, human take, release and wheel events are not reached.
@task_item n=2 part=untouched
Dialog rows: dialog hold, answer, pending dialog and permission or plan handling are unchanged.
@task_item n=3 part=untouched
MCP server rows: rmcp, MCP tools, stdin caps and MCP results are unchanged; /mcp here is a Claude local command, not viola's MCP server.
@task_item n=4 part=untouched
Link, unlink, pause, list, board and revive rows are not reached.
@task_item n=5 part=untouched
Dependency, licence, toolchain, MSRV, workflow and release rows: no dependency, crate or workflow added or changed.
@task_item n=6 part=untouched
Name rows: no binary, crate, command, server, env var or endpoint name taken or freed.
@task_item n=7 part=untouched
Exit-code, refusal-reason and error-document rows: no code, refusal detail or human output literal added or changed.
@task_item n=8 part=untouched
Pinned copy, plugin install, socket directory, endpoint and peer-trust rows are unchanged.
@task_item n=9 part=untouched
Config and environment rows: no config key, env var or flag of viola added.
