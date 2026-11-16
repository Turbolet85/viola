@task_prose part=session
The session made a local slash command that the CLI runs read as delivered when its own echo is measured: after the send, a new <local-command-stdout> tag in the session transcript confirms it.
Local commands are now classified by name (exact name, or name plus one space plus arguments), and the known list grew from 3 to 6: /clear, /remote-control, /compact, /mcp, /cost, /usage.
/mcp is confirmed by its stdout echo. /cost, /usage and /remote-control have no post-condition and read unconfirmable, as does an echo command sent when no transcript was declared or none can be read.
The hook event gains an optional transcript param beside the event, checked on the hook side and again on the wrapper side; the path never reaches data, logs, spans or replies.
viola verify gains a 22nd relied capability row, local-command-echo, probed as Run B's seventh paste; the step counter goes from [NN/21] to [NN/22] and the clean summary from 21 pass to 22 pass. The stamp's typed probe gains echo_latency_ms.
No output literal was added or changed: the send still renders [RB] read back and [  ] unconfirmable as before. Nothing was retired.
@task_file n=1 path=src/run/send/echo.rs
what: the send's transcript echo reader (new): the transcript's end offset, and a count of stdout tags past an offset, read with no-follow, owner and size bounds
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the local command list by name with each one's post-condition, the transcript path shape check, and the relied rows (now 22, local-command-echo added)
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: hook payload normalisation: a SessionStart's transcript path, kept redacted and out of event data
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit cases
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger row checks' unit cases (the per-row verdict vector, now 22 wide)
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the local command name table and transcript path shape unit cases
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger run checks' unit cases, including the local-command-echo check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp verdict unit cases (a 21-row stamp reads unverified)
@task_file n=9 path=src/cmd/hook.rs
what: the hook verb's delivery to the wrapper: the hook.event call, now carrying the transcript param beside the event
@task_file n=10 path=src/cmd/run.rs
what: the wrapper started by viola run: the channel's hook.event method, which re-validates the transcript param
@task_file n=11 path=src/cmd/verify.rs
what: the viola verify verb: its step lines, the [NN/22] counter and the summary
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's typed probe runs (Run B's framing run and its echo measurement)
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: verify's Run B framing pastes, the seventh being the /mcp echo probe
@task_file n=14 path=src/run/send.rs
what: the send path: classification of the text, the confirm window, unconfirmable when no echo source exists
@task_file n=15 path=src/run/send/classify.rs
what: send classification of a text as prompt or local command, and its unit cases for the echo confirmation
@task_file n=16 path=src/run/send/slot.rs
what: the send slot: the in-flight send, the hook event feed, and confirmation polling (now also by echo tag)
@task_file n=17 path=src/run/send/relabel.rs
what: send outcome relabelling cases
@task_file n=18 path=src/run/send/test_support.rs
what: the send unit test support
@task_file n=19 path=src/run/version_gate.rs
what: the CLI version gate that reads the stamp's relied rows
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the fake agent: its new transcript option and its echo of local commands into the transcript
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option parsing unit cases
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot, whose verify passes the fake agent its transcript option
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run, holding the expected verify output at [NN/22]
@task_file n=24 path=tests/cli_send.rs
what: the viola send integration tests, now with an echo-confirmed /mcp read back and an unconfirmable case
@task_file n=25 path=tests/cli_verify.rs
what: the viola verify integration tests and their summary oracles
@task_file n=26 path=tests/cli_verify/runs.rs
what: viola verify's integration cases over Run B's pastes
@task_file n=27 path=tests/cli_verify/stamps.rs
what: viola verify's stamp integration cases and their summaries
@task_file n=28 path=tests/contract_ledger_probes.rs
what: the contract test over the ledger's probes
@task_file n=29 path=tests/support/verify.rs
what: the stamped-home verify support used by the integration tests
@task_item n=1 part=touches
The viola send read-back contract: a local command confirmed by its stdout echo (/mcp, with arguments too) now reads back, beside the /clear and /compact read-backs. Same words.
@task_item n=2 part=touches
The viola send unconfirmable contract: /cost, /usage and /remote-control, and an echo command sent with no readable transcript, read unconfirmable.
@task_item n=3 part=touches
The read-back for /clear and /compact rows: local commands now match by name with arguments allowed, not by exact text.
@task_item n=4 part=touches
The viola verify step line: the counter is now [NN/22], with local-command-echo as the 22nd relied row.
@task_item n=5 part=touches
The viola verify summary and the exit 0 and exit 1 contracts: a clean run now reads 22 pass 0 fail. Any row cell that holds 21 relied rows is stale.
@task_item n=6 part=touches
The viola verify failing row: local-command-echo can fail, with its words «/mcp writes its stdout tag to the transcript and submits no prompt». A missing or unreadable transcript makes it fail.
@task_item n=7 part=touches
The viola verify --record and help paragraph: the stamp's typed probe gains echo_latency_ms. No path or count reaches a step line, a log or --record.
@task_item n=8 part=touches
The viola send --json document and the json ok document: an echo-confirmed local command returns submitted_at and cursor as any confirmed send does.
@task_item n=9 part=touches
The readback vocabulary and the readback mirror signature: the echo is one more confirmer under the existing words. No new word was added.
@task_item n=10 part=touches
The send refusals the wrapper repeats: slash text that is not on the list (like /not-a-local-command) still reads not-delivered.
@task_item n=11 part=touches
The viola mcp send tool's results: they share the send path, so a local command sent through MCP gets the same echo confirmation or unconfirmable result.
@task_item n=12 part=touches
The viola send verb row: the local commands it recognises grew from 3 to 6 names.
@task_fact n=1 slug=surface-cli
fact: viola send reads back /mcp, with or without arguments, as [RB] read back when a new stdout tag lands in the session transcript after the send. The transcript is read from its end as it stood before the send.
@task_fact n=2 slug=surface-cli
fact: viola send renders /cost, /usage and /remote-control as [  ] unconfirmable, and renders the same for an echo-confirmed command when the session declared no readable transcript.
@task_fact n=3 slug=surface-cli
fact: viola send treats text as a local command by its name: either the name alone, or the name, one space and a non-empty rest. Other slash text still reads not-delivered.
@task_fact n=4 slug=surface-cli
fact: viola verify walks 22 relied rows ([NN/22], 22 pass 0 fail when clean). The 22nd is local-command-echo, probed by Run B's seventh paste /mcp disable viola-probe-absent.
@task_item n=1 part=untouched
The web surface absence: viola still renders no page.
@task_item n=2 part=untouched
The viola list board, the BAY line and viola list --json: no column, width, colour or row changed.
@task_item n=3 part=untouched
The viola run and viola revive start outputs, their refusals and their --json error codes.
@task_item n=4 part=untouched
viola link and unlink output, viola plugin install output, and viola last output.
@task_item n=5 part=untouched
CLI token rendering, the hand-written SGR styling, TTY detection and plain output modes: no styling changed.
@task_item n=6 part=untouched
The CLI exit codes, the exit-21 hints and the --json detail for an unreachable instance, the internal-error forms, and the global flags.
@task_item n=7 part=untouched
The viola wait and viola answer outputs and the wheel verb (pause, release) output.
@task_item n=8 part=untouched
viola --help and per-verb usage, and the no-banner, no-spinner, no-pager and no-progress-glyph decisions.
