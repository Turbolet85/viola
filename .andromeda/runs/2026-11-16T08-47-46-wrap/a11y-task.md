@task_prose part=session
viola send now reads back a local slash command the CLI ran as delivered when the CLI's transcript gains a new local-command stdout tag after the send; the tag is counted from the transcript's end before the paste, and no byte of content is kept.
Local commands are classified by name (a name, or a name with one space and arguments), and the list grew from three to six: /clear, /remote-control, /compact, /mcp, /cost, /usage; /mcp is confirmed by its echo, /cost and /usage stay unconfirmable.
The SessionStart hook's transcript path is validated on both sides and travels as a sibling of the event on the hook channel, never in event data, a log line, a reply or the terminal.
viola verify gains a 22nd relied capability-ledger row, local-command-echo, measured by a seventh Run B paste of /mcp disable viola-probe-absent; the step counter reads [NN/22] and the all-pass summary 22 pass 0 fail.
The fake agent gains a transcript option for tests; no output literal, refusal word, hint or colour changed; nothing was retired.
@task_file n=1 path=src/run/send/echo.rs
what: the bounded transcript reader for a send's confirmation: refuses symlinks, non-regular and foreign-owned files, reads at most a frame, returns a count of stdout tags alone
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows verify checks, the local-command list with each command's confirmer, the transcript path shape check
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload normaliser: what of a claude hook payload is kept, redacted or dropped before it becomes an event
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit tests
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger's row-check unit tests
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the capability ledger's local-command name table and transcript path shape tests
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the capability ledger's Run A and Run B measurement check tests
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's stamp verdict tests
@task_file n=9 path=src/cmd/hook.rs
what: the viola hook event call: delivers a hook event to the wrapper over the channel
@task_file n=10 path=src/cmd/run.rs
what: the viola run wrapper's channel methods, including the hook-event method that takes an event line
@task_file n=11 path=src/cmd/verify.rs
what: viola verify: the step lines, the relied-row count and the pass and fail summary
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed-probe runs and the measurements they stamp
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: viola verify's Run B framing pastes into the child's input box
@task_file n=14 path=src/run/send.rs
what: viola send in the wrapper: classifying the text, issuing the paste and choosing its confirmer
@task_file n=15 path=src/run/send/classify.rs
what: the send classification and its unit cases: delivered, not-delivered, unconfirmable, no-prompt-submitted
@task_file n=16 path=src/run/send/slot.rs
what: the send slot: the in-flight send, the confirm window and the hook events that confirm a send
@task_file n=17 path=src/run/send/relabel.rs
what: the send relabelling of hook events and its tests
@task_file n=18 path=src/run/send/test_support.rs
what: the send unit tests' shared support
@task_file n=19 path=src/run/version_gate.rs
what: the version gate: whether the installed CLI's stamp verifies the relied rows
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the fake agent standing in for claude in tests: its hook payloads, recorded framing variants and local-command behaviour
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing tests
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot that runs verify against the fake agent
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run checks, including verify's expected step-line output
@task_file n=24 path=tests/cli_send.rs
what: the viola send integration tests, including the byte search of the home for leaked paths and content
@task_file n=25 path=tests/cli_verify.rs
what: the viola verify integration tests: step lines and summaries
@task_file n=26 path=tests/cli_verify/runs.rs
what: the viola verify integration tests of the typed-probe runs
@task_file n=27 path=tests/cli_verify/stamps.rs
what: the viola verify integration tests of the stamp verdict and summaries
@task_file n=28 path=tests/contract_ledger_probes.rs
what: the capability-ledger probe contract test over the relied rows
@task_file n=29 path=tests/support/verify.rs
what: the shared stamped-home verify setup for integration tests
@task_item n=1 part=touches
Critical path confirmed send and readback: a local command now reads back as delivered on its echo tag; /cost and /usage read as unconfirmable; an echo command with no readable transcript is unconfirmable.
@task_item n=2 part=touches
viola verify step line: the step counter moves from [NN/21] to [NN/22] and the all-pass summary from 21 pass 0 fail to 22 pass 0 fail, with a new step local-command-echo; any row holding that count or the step list is touched.
@task_item n=3 part=touches
viola send refusal no-prompt-submitted: a local echo command with no new tag within the send's confirm window now ends in no-prompt-submitted.
@task_item n=4 part=touches
Paths not carried: the CLI's transcript path now crosses the hook channel beside the event, kept out of event data, logs, spans, replies and the terminal; a row stating no path is carried gains this bounded exception.
@task_item n=5 part=touches
Single vendor-content boundary and boundary checks that never inspect claude content: the wrapper now reads the CLI's transcript, keeping only a count of the local-command stdout tag, never its content.
@task_item n=6 part=touches
No screen-content parsing beyond signatures: the confirmation reads a transcript tag count, not the screen; a row naming the allowed signatures may need the tag beside them.
@task_item n=7 part=touches
Time bounds fall on the driver only: the echo wait uses the send's existing confirm window, and verify's echo paste waits up to its probe deadline; neither falls on the human.
@task_item n=8 part=touches
Verify's Run B pastes into the child's input box grow from six to seven framing pastes; a row on viola bytes reaching a terminal or on what verify types holds this.
@task_item n=9 part=touches
Fixed CLI strings pinned by literal and fixed copy from design: no output literal changed, but verify's expected step-line literals in the e2e harness and integration tests were updated to the 22-row form.
@task_item n=10 part=touches
Live readings never gate: the 22nd row's live re-stamp on claude 2.1.325 is owed and not done; stamps without the row read unverified.
@task_item n=11 part=touches
Fake transcripts and recorded fixtures: the fake agent's transcript option rewrites only the launch SessionStart, so recorded framing replays still match byte for byte.
@task_fact n=1 slug=1-a11y-scope-summary
fact: A local command sent through viola send reads back by name: /mcp is confirmed by a new local-command stdout tag in the CLI's transcript, /cost and /usage are unconfirmable, and the readback words stay [RB] read back and [  ] unconfirmable.
@task_fact n=2 slug=cognitive-accessibility
fact: The CLI's transcript path is validated by the hook and again by the wrapper, travels only beside the hook event, and the transcript read keeps a tag count alone, so no transcript path or content reaches any output.
@task_item n=1 part=untouched
TUI passthrough contracts: entry, human keystrokes, focus, mouse and resize sequences, vt100 feed panic and refused peers are unchanged.
@task_item n=2 part=untouched
viola list board: caption, order, colour, stale-row dimming, NAME truncation and the unwrapped separator are unchanged.
@task_item n=3 part=untouched
Colour and SGR rules: no word, colour or SGR span was added or changed.
@task_item n=4 part=untouched
Refusal layout, hints and code words: no refusal word, hint, unable line or exit code was added or changed.
@task_item n=5 part=untouched
viola revive, link, unlink, plugin install and --help output contracts are unchanged.
@task_item n=6 part=untouched
Manual Orca and keyboard passes and their screen-reader targets are unchanged.
@task_item n=7 part=untouched
The absences: internationalization, touch surface, web or graphical frontend, rendered page and user time limit stay absent.
