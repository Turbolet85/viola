@task_prose part=session
A local slash command the wrapped claude CLI runs itself now reads as delivered when its own measured echo confirms it: send counts the `<local-command-stdout>` tags the transcript gains past its end read before the paste, a count alone, no byte kept.
The SessionStart hook keeps the session's transcript path only when its shape passes (absolute, no `..`, file name the session id plus `.jsonl`); it travels as an optional `transcript` sibling param of the `hook.event` channel method, re-validated wrapper-side, in no data, events line, log line, span field or reply.
Local commands are classified by name (exactly a name, or a name, one space and a rest) and the list grew 3 to 6: `/clear`, `/remote-control`, `/compact`, `/mcp` (confirmed by the stdout tag), `/cost` and `/usage` (unconfirmable); a local command with no remembered transcript is unconfirmable.
The capability ledger gained its 22nd relied row `local-command-echo`, measured by verify's Run B as a seventh paste (`/mcp disable viola-probe-absent`) into the fake agent or the CLI; the stamp's `measured.typed_probe` gains `echo_latency_ms` (additive); verify's step counter reads `[NN/22]` and its summary `22 pass  0 fail`.
The fake agent gained an argv-only `--transcript` option, passed beside `--framing` in every stamped verify of the tests and the harness boot; no fixture changed, no line, span, detail word, config key, env var or dependency was added, nothing retired.
@task_file n=1 path=src/run/send/echo.rs
what: the wrapper's transcript reader for send confirmation (new) — end offset and stdout-tag count past it; O_NOFOLLOW, regular file, owner euid, capped at MAX_FRAME, a count alone
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger of the claude agent — relied rows (now 22), the local-command list and its post-conditions, the probe commands, the stamp's measured typed probe, the transcript path shape check
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the claude hook payload normaliser — the SessionStart transcript path taken, shape-checked against its own session id, redacted, kept out of the event data
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit cases — row list and count
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger's row-check unit cases — the verdict vector a row count pins
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the capability ledger's local-command table and name-classification and transcript path shape unit cases
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the capability ledger's measured-run check unit cases — the echo row's check (a tag within the confirm window and no prompt)
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger stamp unit cases — the stamp's rows and the measured typed probe
@task_file n=9 path=src/cmd/hook.rs
what: viola hook, the hook role's delivery to the wrapper over the channel — the hook.event request with its optional transcript param
@task_file n=10 path=src/cmd/run.rs
what: viola run, the wrapper's channel methods — the hook.event line taken, its transcript re-validated for a session-start line only, any other dropped
@task_file n=11 path=src/cmd/verify.rs
what: viola verify, the capability ledger probe and stamp writer — step lines and the summary over the relied rows
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed probe runs (Run A and Run B) and their measured stamp fields
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: viola verify's Run B framing pastes — the seventh paste of the echo probe command, the transcript read bounded by MAX_FRAME
@task_file n=14 path=src/run/send.rs
what: the wrapper's send path — classification of the sent text, the confirmer chosen, the transcript end recorded before send-issued and the paste, unconfirmable when none
@task_file n=15 path=src/run/send/classify.rs
what: the send classification of local commands and its unit cases — confirmed, no-prompt-submitted, unconfirmable
@task_file n=16 path=src/run/send/slot.rs
what: the send slot and its confirm window — hook-event lines appended, the remembered transcript per session-start, the echo poll before the deadline
@task_file n=17 path=src/run/send/relabel.rs
what: the send slot's relabel cases — hook-event delivery into the slot
@task_file n=18 path=src/run/send/test_support.rs
what: the send path's shared test support
@task_file n=19 path=src/run/version_gate.rs
what: the run version gate over the capability ledger stamps — the relied row set a verified CLI needs
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the fake agent standing in for claude in tests and CI verify — the `--transcript` option, the launch SessionStart's transcript path, the echo tag on a local command
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option parsing unit cases
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot — its verify against the fake agent, now with `--transcript` beside `--framing`
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run — the expected verify output (`[NN/22]` step lines)
@task_file n=24 path=tests/cli_send.rs
what: the send integration tests — an echo-confirmed local command on a verified CLI, unconfirmable on an unstamped home, a byte search of the home for the transcript tag and path
@task_file n=25 path=tests/cli_verify.rs
what: the verify integration tests — the fixture-free verify summaries
@task_file n=26 path=tests/cli_verify/runs.rs
what: the verify integration tests of the typed probe runs
@task_file n=27 path=tests/cli_verify/stamps.rs
what: the verify integration tests of the written stamps — a summary pinning the relied row count
@task_file n=28 path=tests/contract_ledger_probes.rs
what: the capability ledger probe contract test — the relied rows probed
@task_file n=29 path=tests/support/verify.rs
what: the stamped-home verify test support — verify, verify_without_dialogs and verify_started now pass the fake agent `--transcript`
@task_item n=1 part=touches
The decision that Claude Code transcripts are not a contract is stale: it now carries one exception, ratified by the founder — send relies on the count of `<local-command-stdout>` tags a session's transcript gains past its end read before the paste, as the echo of `/mcp`; no transcript content crosses.
@task_item n=2 part=touches
The decision that wrapped claude internals are not a contract gets the same one exception: the stdout tag in the transcript and the SessionStart payload's `transcript_path` are now relied on.
@task_item n=3 part=touches
The confirmed send log fields' hook-invoked item: an echo-confirmed local command reaches send-confirmed with no hook-invoked line between send-issued and send-confirmed (no prompt is submitted); add-to.
@task_item n=4 part=touches
The send path log events' event order and the corr-by-event send rows: an echo-confirmed send's lines are send-issued then send-confirmed, no send-refused, the same lines as before, no new event.
@task_item n=5 part=touches
The run.confirm_window close rows (/clear, /compact, ordinary send): a fourth close — a local command confirmed by its echo tag, polled each step before the deadline inside the existing span, no new field; the compact confirm window decision's fallback window is also the echo row's check bound.
@task_item n=6 part=touches
The send line's unconfirmable send item: `/cost`, `/usage`, `/remote-control`, a local command on an unverified CLI, and `/mcp` with no remembered or readable transcript are unconfirmable.
@task_item n=7 part=touches
The wrapper send-refused detail no-prompt-submitted: `/mcp` with no new tag inside the window reads no-prompt-submitted at the window; a slash text off the list (`/not-a-local-command`) still reads not-delivered.
@task_item n=8 part=touches
verify stdout's step line and summary line, where a cell holds the relied count: `[NN/21]` became `[NN/22]` and `21 pass  0 fail` became `22 pass  0 fail`.
@task_item n=9 part=touches
The verify-lines-carry-no-content and verify-lines-redacted decisions: Run B's new seventh paste is measured and no transcript path, tag count or latency reaches a step line, a log or `--record`.
@task_item n=10 part=touches
The capability ledger path and stamps never logged decision: the stamp gains `measured.typed_probe.echo_latency_ms`, logged nowhere.
@task_item n=11 part=touches
The unreadable-ledger-means-unverified decisions and the run log's parse-rejected ledger-stamps item: a stamp with 21 relied rows now reads unverified; the stamp's new field is additive.
@task_item n=12 part=touches
The channel lines carry no params or results decisions and the channel params contract rows: the `hook.event` method gains an optional `transcript` sibling param (a path), never in a channel-request or channel-response line.
@task_item n=13 part=touches
The type-level redaction of payload types (veil derives redaction): the hook's known SessionStart field `transcript_path` and its normalised `transcript` are marked `#[redact]`.
@task_item n=14 part=touches
The path-free home-level lines and paths-only-in-instance-detail-files decisions: the transcript path appears in no log line, detail file, events line, data or reply; the booted test byte-searches every home file for the tag, `transcripts/` and `.jsonl`.
@task_item n=15 part=touches
The sent-text-never-logged and no-text-in-telemetry decisions: the echo read keeps a count alone, no transcript byte; the sent local command's text is no more logged than before.
@task_item n=16 part=touches
The CI verify against the fake agent only decision and the real-CLI verify never in CI decision: CI's fake-agent verify now runs with `--transcript` beside `--framing`; still no real claude in CI.
@task_item n=17 part=touches
The fixtures-not-a-telemetry-source decisions: the fake agent seeds its own transcript files (0600 under a 0700 dir beside its exe) and replays every recorded fixture byte for byte; no fixture changed.
@task_item n=18 part=touches
The hook latency and spine hook budget decisions, indirectly: the SessionStart hook now does a path shape check on its payload (no I/O added) and sends one more param.
@task_item n=19 part=touches
The uninstrumented components decision: the new transcript reader adds no line and no span of its own, observed only inside the existing run.confirm_window span.
@task_fact n=1 slug=span-trace-coverage
fact: The send's transcript echo read (end offset before the paste, tag count after) is uninstrumented: no line, no span and no span field of its own; its result shows only as the existing send-confirmed line inside run.confirm_window.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: A session's transcript path and transcript bytes are never logged or stored: the path is carried only as the hook.event `transcript` param, redacted in the hook's types, kept out of event data, and the transcript read keeps a count alone.
@task_item n=1 part=untouched
Dialog raise, answer and expiry rows, the dialog hook and its decision lines: no dialog path changed.
@task_item n=2 part=untouched
The viola-mcp stdio server tools, its stdout and its refusal codes: no MCP server row moved (the `/mcp` here is the claude CLI's own local command, not viola's MCP server).
@task_item n=3 part=untouched
Panic capture, the panic line and panic outcome by role, error capture and the error reporter decisions.
@task_item n=4 part=untouched
Exit codes, process-exit detail words, refusal reasons and socket-directory or strict-modes refusals: no detail word was added or changed.
@task_item n=5 part=untouched
Wait and last, wheel pause and release, link and unlink, revive, heartbeat and liveness rows.
@task_item n=6 part=untouched
CI gates, the secret scan, artifact uploads, perf budgets and the OTel, exporter, metrics and network-egress absences.
@task_item n=7 part=untouched
Log sinks, rotation, levels, the diagnostics line schema, field names and service identity: no field or event was added.
