@task_prose part=session
A local slash command the CLI ran now reads as delivered on a verified CLI when its own echo is measured: /mcp is confirmed by a new <local-command-stdout> tag in the session transcript past the end read before the send, a count alone, no content crossing.
Local commands are classified by name, no longer by exact text: the compiled list grows from 3 to 6 names (/clear, /remote-control, /compact, /mcp, /cost, /usage), a name alone or a name, one space and a non-empty rest.
The hook reads SessionStart's transcript_path, shape-checks it, and sends it beside the event on the hook.event channel method; the wrapper re-checks it and remembers it per flight; an echo command with no remembered transcript is unconfirmable.
A 22nd relied capability-ledger row, local-command-echo, is added; verify's Run B pastes /mcp disable viola-probe-absent seventh as its probe, and the stamp's measured typed probe gains echo_latency_ms; verify counts move from 21 to 22.
The fake agent gains an argv-only --transcript option (launch SessionStart carries a transcript path, echo commands append a stdout tag line); every stamped-home verify and the harness boot's verify pass it beside --framing.
Nothing retired; no fixture, config key, env var, dependency or CI gate changed.
@task_file n=1 path=src/run/send/echo.rs
what: new bounded transcript reader of the send path — reads the transcript's end and counts stdout tags past an offset (no-follow open, regular file, owner check, MAX_FRAME cap)
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger — the compiled local-command list with its post-conditions, by-name classification, transcript path shape check, the relied ledger rows and their checks, the stamp merge
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: hook payload normalisation — SessionStart's transcript_path read, redacted, kept out of event data
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the capability ledger
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger row checks (typed rows fail without a measurement)
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit cases of the local-command name table and the transcript path shape
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit cases of the ledger row checks over verify runs (the local-command-echo check)
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the stamps and the relied-row verdict (a 21-row stamp reads unverified)
@task_file n=9 path=src/cmd/hook.rs
what: the viola hook verb — delivers the hook event to the wrapper channel, now with the transcript beside the event
@task_file n=10 path=src/cmd/run.rs
what: the wrapper's channel methods — hook.event handling, re-validating the transcript for a session-start line
@task_file n=11 path=src/cmd/verify.rs
what: viola verify — the relied ledger rows walked, step lines and summary line
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's typed probe runs (Run A, Run B)
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: verify Run B's framing pastes — the seventh echo paste and the bounded transcript read of the recorded SessionStart
@task_file n=14 path=src/run/send.rs
what: the driver send — local-command classification and the echo start recorded before send-issued and the paste
@task_file n=15 path=src/run/send/classify.rs
what: send classification and its unit cases (echo-confirmed, unconfirmable and not-delivered local commands)
@task_file n=16 path=src/run/send/slot.rs
what: the send slot — the hook-event append, the remembered transcript per flight, the confirmation window poll
@task_file n=17 path=src/run/send/relabel.rs
what: send relabelling and its unit cases (hook-event append call sites)
@task_file n=18 path=src/run/send/test_support.rs
what: shared send test support
@task_file n=19 path=src/run/version_gate.rs
what: the unverified CLI version gate — relied rows a stamp must hold
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the fake agent — argv options, fired SessionStart, input handling of local commands and framing pastes
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's command-line options
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot — its verify of the made home against the fake agent, stamping it
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command — the local-live suite's expected verify output
@task_file n=24 path=tests/cli_send.rs
what: viola send integration cases over a booted wrapper (echo read back on a verified CLI, unconfirmable on an unstamped home)
@task_file n=25 path=tests/cli_verify.rs
what: viola verify integration cases — step and summary lines
@task_file n=26 path=tests/cli_verify/runs.rs
what: viola verify integration cases of the typed probe runs
@task_file n=27 path=tests/cli_verify/stamps.rs
what: viola verify integration cases of the stamps and the fixture-free summaries
@task_file n=28 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite — every ledger row has a verify probe
@task_file n=29 path=tests/support/verify.rs
what: shared verify helpers making stamped test homes against the fake agent
@task_item n=1 part=touches
decision/local-command-exact-match is stale: local commands are now matched by name (a name alone, or a name, one space and a non-empty rest), not by exact equality.
@task_item n=2 part=touches
contract/driver-send/local-command and contract/viola-send/off-list-slash-text: the ledger-listed local command list is six names, classified by name; slash text off the list still reads not-delivered.
@task_item n=3 part=touches
decision/local-command-post-condition: /mcp gains a post-condition (a new stdout tag in the transcript), beside /clear (new session) and /compact (same session); /cost, /usage and /remote-control have none.
@task_item n=4 part=touches
decision/local-command-unconfirmable and contract/viola-send/unconfirmable-local-command: an echo command with no remembered transcript, an unreadable transcript end or an unverified CLI is unconfirmable; /cost and /usage join the unconfirmable list.
@task_item n=5 part=touches
contract/send-result/unconfirmable, contract/viola-send/unconfirmable and contract/viola-send-human-mode/unconfirmable gain the echo-without-transcript case; no output literal changed.
@task_item n=6 part=touches
contract/viola-send/confirmed, contract/driver-send/confirmation, contract/send-confirmation/prompt-submitted-driver and contract/cli-human-lines/read-back: a further confirmer, the measured echo tag, reads a local command back with no prompt-submitted line.
@task_item n=7 part=touches
contract/viola-send/no-prompt-submitted and contract/cli-send-json/no-prompt-submitted: an echo command with no new tag within the window ends not-delivered no-prompt-submitted; a tag already there before the send is not its echo.
@task_item n=8 part=touches
decision/send-confirmation-window-expiry and decision/compact-confirm-window: the confirmation window now polls the echo count each step before its deadline check; the echo row's check reuses the compact row's fallback window.
@task_item n=9 part=touches
decision/version-gate-needs-all-relied-rows, decision/stamps-refuse-widened-ledger and decision/unverified-cli-gate: relied rows move from 21 to 22; a 21-row stamp now reads unverified.
@task_item n=10 part=touches
decision/ledger-row-verify-probe and decision/ledger-probe-contract-suite: the new local-command-echo row has its verify probe, Run B's seventh paste.
@task_item n=11 part=touches
contract/verify-output/step-line, contract/verify-output/summary-line and contract/verify-output/last-line: the step counter reads [NN/22] and the clean summary 22 pass 0 fail.
@task_item n=12 part=touches
contract/local-live-suite/pass: the local-live suite's expected verify output now counts 22 rows.
@task_item n=13 part=touches
contract/fake-agent-cli/argv-only-options and decision/fake-agent-argv-only: the fake agent gains the argv-only --transcript option.
@task_item n=14 part=touches
contract/fake-agent-events/session-start: under --transcript the launch SessionStart (variant default or resume-1) carries transcript_path to a seeded owner-only file; other SessionStarts go out as recorded.
@task_item n=15 part=touches
contract/fake-agent-input/framing-paste and contract/fake-agent-input/compact: under --framing the exact echo probe command appends a stdout tag line and fires no hook; under --local-command-mode an echo-confirmed local command does the same.
@task_item n=16 part=touches
decision/fake-agent-matches-recordings, decision/fixture-drift-byte-compare and decision/fake-transcripts-compared-to-recordings: the drift contract now runs with --transcript beside --framing; replayed variants stay byte for byte because only the launch SessionStart is rewritten.
@task_item n=17 part=touches
decision/stamps-from-fake-agent-verify, decision/fake-agent-replay-for-stamps and decision/harness-boot-creates-stamps-and-starts: the stamping verify (harness boot and stamped-home helpers) passes --transcript beside --framing.
@task_item n=18 part=touches
decision/bounded-readers and decision/bounded-readers-keep-the-cap: two new readers of the agent's transcript, each capped at MAX_FRAME; the bounded-reader contract check reads take only ahead of the read call.
@task_item n=19 part=touches
decision/tests-never-read-host-sessions and decision/tailing-ignores-symlinks: the product now reads the agent session transcript (no-follow, regular file, owning euid); tests read only fake-agent transcripts under the exe dir.
@task_item n=20 part=touches
decision/hook-unknown-fields-tolerated and contract/viola-hook/stdout: the hook now keeps SessionStart's transcript_path (redacted) and sends it as an optional sibling param of hook.event, never in event data.
@task_item n=21 part=touches
decision/logs-carry-no-secrets, decision/no-secrets-in-logs and decision/no-names-in-wrapper-log: the transcript path reaches no event data, events log, process log, span or reply; a booted case byte-searches the home for it.
@task_item n=22 part=touches
decision/private-filesystem-permissions: the fake agent creates its transcripts dir 0700 and seeds each transcript file 0600.
@task_item n=23 part=touches
decision/verify-driven-tests-20s-kill and decision/ci-stamped-home-kill-line: Run B adds a seventh paste waiting up to the probe deadline inside every stamped-home verify.
@task_item n=24 part=touches
decision/send-and-wait-latency-not-timed: the stamp now records echo_latency_ms as a measurement, checked against the fallback window, not a timed budget.
@task_fact n=1 slug=test-scope-summary
fact: A local command is classified by its name: the text is exactly a listed name, or a listed name, one space and a non-empty rest not starting with a space; the list holds /clear, /remote-control, /compact, /mcp, /cost and /usage.
@task_fact n=2 slug=test-scope-summary
fact: /mcp is confirmed only by at least one new <local-command-stdout> tag in the session transcript past the end read before the paste, a count alone; it is the one exception to transcripts and CLI internals being no contract.
@task_fact n=3 slug=unit-test-strategy
fact: An echo-confirmed local command is unconfirmable when the flight remembers no transcript, the transcript end is unreadable, or the CLI is unverified; /cost, /usage and /remote-control carry no post-condition and are unconfirmable.
@task_fact n=4 slug=unit-test-strategy
fact: A transcript path is accepted only when absolute, with no .. component, its file name exactly {session_id}.jsonl for a valid session id of its own SessionStart; it is checked hook-side and again wrapper-side, and any other transcript param is dropped while the line is still taken.
@task_fact n=5 slug=unit-test-strategy
fact: hook.event carries an optional sibling param transcript beside event, sent only for a SessionStart that holds a valid path, and the wrapper re-validates it for a session-start line alone against that line's own session id.
@task_fact n=6 slug=6-e2e-test-strategy
fact: The transcript is read with a no-follow open, only as a regular file owned by the effective uid, through MAX_FRAME, and only a tag count is kept; a tag split across the offset is not counted.
@task_fact n=7 slug=unit-test-strategy
fact: local-command-echo is the 22nd relied ledger row; its check passes when no UserPromptSubmit carries the probe command and echo_latency_ms is within the compact row's fallback window.
@task_fact n=8 slug=integration-test-strategy
fact: Verify Run B pastes /mcp disable viola-probe-absent seventh, after the newline paste and before /compact, waiting up to the probe deadline for a tag or a prompt; with no readable shape-checked transcript the paste is skipped and the echo row fails.
@task_fact n=9 slug=unit-test-strategy
fact: The ledger stamp's measured typed probe carries echo_latency_ms, set only for a tag with no prompt.
@task_fact n=10 slug=7-test-data-fixtures
fact: The fake agent's argv-only --transcript creates <exe dir>/transcripts/ 0700 at start and gives the launch SessionStart a transcript_path to a seeded 0600 file named by its session id; replayed framing variants are not rewritten.
@task_fact n=11 slug=7-test-data-fixtures
fact: The fake agent appends one stdout tag line to the last started session's transcript and fires no hook for an echo-confirmed local command under --local-command-mode and for the exact probe command under --framing.
@task_fact n=12 slug=test-data-bootstrap
fact: Every stamping verify against the fake agent (harness boot and the stamped-home helpers) passes --transcript beside --framing; the framing-free verify helper does not.
@task_fact n=13 slug=6-e2e-test-strategy
fact: Residuals of the echo probe: whether /mcp disable viola-probe-absent writes the user's real Claude config is unmeasured (under a private config it wrote no disabled key); the paste adds a line to the input history like every Run B paste; /context's echo shape on 2.1.325 is unmeasured.
@task_item n=1 part=untouched
Dialog hook decision bodies, answer, permission, question and plan rows: no dialog path changed.
@task_item n=2 part=untouched
MCP server rows (tools/list, tool results, invalid-params fields, stdin line cap): no tool, field or MCP reply changed.
@task_item n=3 part=untouched
CI integration, coverage, performance budgets, mutation and supply-chain gates: no gate, threshold, workflow or artifact changed.
@task_item n=4 part=untouched
Revive, list, release, wait, last and link rows: no verb outside send and verify changed.
@task_item n=5 part=untouched
Recorded Claude fixtures per CLI version, fixture scrub and hygiene walk: no fixture was added or changed.
@task_item n=6 part=untouched
Screen-parsing and screen-signature rows: the echo is read from the transcript file, never from the rendered screen.
@task_item n=7 part=untouched
Log format, log-event corr and event-line rows: no new log line, span field, event kind or event data key.
@task_item n=8 part=untouched
CLI exit codes, refusal codes and send refusal order: no exit code or refusal word added or moved.
@task_item n=9 part=untouched
Harness exit codes, session record, status shape, cleanup and pre-push contracts: unchanged.
