@task_prose part=session
The session made a local slash command that the CLI runs count as delivered, confirmed by the command's own echo, which is a new local-command-stdout tag in the session transcript after the send.
The send now recognises a local command by its name, with or without arguments. The known list grew from three names to six: /clear, /remote-control, /compact, /mcp, /cost and /usage. /mcp is confirmed by its echo, while /cost, /usage and /remote-control have no post-condition and read unconfirmable.
A local command whose echo cannot be watched reads unconfirmable. That happens when the session's start declared no valid transcript or when the CLI is unverified. An echo command that writes no tag within the window reads not-delivered with no-prompt-submitted.
viola verify gains a 22nd relied capability row, local-command-echo, measured by a seventh Run B paste (/mcp disable viola-probe-absent). The step counter goes from [NN/21] to [NN/22] and the summary from 21 pass to 22 pass. A stamp with 21 rows now reads unverified.
The send output added and changed no words: an echo-confirmed send still prints the read-back mirror line, and an unconfirmable one prints the unconfirmable line. The transcript path never reaches any output line, log, reply or recorded stamp.
Nothing was retired, and no verb, flag, config key, colour or glyph was added.
@task_file n=1 path=src/run/send/echo.rs
what: added; the send's echo reader, which counts the local command's stdout tags written to the session transcript after the send, the count alone, bounded and owner-checked
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: edited; the capability ledger, which holds the relied rows verify measures (now 22, with local-command-echo), the local-command list by name with each command's post-condition, and the transcript path shape check
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: edited; the hook payload normaliser, which keeps the SessionStart transcript path out of the event data and holds it only when its shape checks
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited; the capability ledger's unit tests
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited; the ledger row checks' tests, now with a 22-row verdict vector
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited; the local-command name table and transcript path shape tests
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: edited; the ledger run-measurement tests, now covering the local-command-echo row's check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited; the stamp verdict tests (a 21-row stamp reads unverified)
@task_file n=9 path=src/cmd/hook.rs
what: edited; the hook verb, which delivers the hook event to the wrapper and now sends the transcript beside a SessionStart event
@task_file n=10 path=src/cmd/run.rs
what: edited; viola run's wrapper channel method for hook events, which re-validates the transcript of a session-start line and drops any other
@task_file n=11 path=src/cmd/verify.rs
what: edited; viola verify, its relied-row steps, step counter and summary
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: edited; verify's typed probe runs, which pass the echo measurement on
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: edited; verify's Run B framing pastes, now with the seventh paste (the /mcp echo probe) and its transcript read
@task_file n=14 path=src/run/send.rs
what: edited; the send, which classifies the text, issues it and confirms it, and now records the transcript's end before the paste
@task_file n=15 path=src/run/send/classify.rs
what: edited; the send's classification of a text as a prompt or a local command, now by name, with the echo confirmation cases
@task_file n=16 path=src/run/send/slot.rs
what: edited; the send slot, the in-flight send and its confirmation window, now confirmed also by a new echo tag
@task_file n=17 path=src/run/send/relabel.rs
what: edited; the send's relabel of a confirmation, its tests updated for the transcript
@task_file n=18 path=src/run/send/test_support.rs
what: edited; the send's test support
@task_file n=19 path=src/run/version_gate.rs
what: edited; the CLI version gate, verified against the stamp's relied rows
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: edited; the fake agent, now with an argv-only transcript option that writes the echo tag for the echo command
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited; the fake agent's option tests
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the e2e harness boot, whose verify passes the fake agent the transcript option
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: edited; the e2e harness run, which holds the expected verify step lines at [NN/22]
@task_file n=24 path=tests/cli_send.rs
what: edited; the send verb's integration tests, with the echo read-back on a verified CLI and unconfirmable on an unstamped home
@task_file n=25 path=tests/cli_verify.rs
what: edited; viola verify's integration tests (step lines and summary)
@task_file n=26 path=tests/cli_verify/runs.rs
what: edited; verify's run-level integration tests
@task_file n=27 path=tests/cli_verify/stamps.rs
what: edited; verify's stamp integration tests, with the summary now counting 22 rows
@task_file n=28 path=tests/contract_ledger_probes.rs
what: edited; the capability ledger probes contract test
@task_file n=29 path=tests/support/verify.rs
what: edited; the stamped-home verify test support, which passes the fake agent the transcript option
@task_item n=1 part=touches
Send readback for a local command gains a further confirmer: /mcp reads back once its stdout echo appears in the transcript after the send. This sits beside the /clear and /compact readback contracts. The words are unchanged.
@task_item n=2 part=touches
The send readback unconfirmable contract and the readback box unconfirmable contract now also cover /cost, /usage and /remote-control. They also cover an echo command whose session declared no transcript, or a session on an unverified CLI.
@task_item n=3 part=touches
The viola send readback mirror's read-back and unconfirmable lines now print for a local command confirmed by its echo, and for one that cannot be confirmed. The words are unchanged.
@task_item n=4 part=touches
The not-delivered no-prompt-submitted refusal and its hint now also cover an echo command that wrote no new tag within the confirmation window.
@task_item n=5 part=touches
A local command is now recognised by its name, optionally followed by one space and arguments, where before only the exact text was. Slash text that names no known command is still not-delivered. Any row saying a local command must match its exact text is stale.
@task_item n=6 part=touches
No success is announced before confirmation and no optimistic send confirmation, both indirectly: a measured echo tag counted after the send now counts as confirmation. The tag is not trusted before the send.
@task_item n=7 part=touches
viola verify's step line counter goes from [NN/21] to [NN/22]. Any row whose cell holds the total of 21 now holds 22.
@task_item n=8 part=touches
viola verify's summary and summary line go from 21 pass to 22 pass. Any row holding the relied-row count of 21 now holds 22.
@task_item n=9 part=touches
The CLI verification state's verified and unverified-cli items: verified now needs the 22nd relied row, local-command-echo, so a stamp with 21 rows reads unverified. This reaches the unverified-cli refusal hint indirectly.
@task_item n=10 part=touches
The step counter remains the only progress pattern. verify's added seventh paste prints no new line beyond its row's step.
@task_item n=11 part=touches
No internals in errors or hints, and no paths in output, both indirectly: the transcript path and the tag count reach no output line, hint, reply, log or recorded stamp.
@task_item n=12 part=touches
Turn-ended never fills the readback box, indirectly: the readback for a local command is filled by the echo tag, never by a turn end.
@task_fact n=1 slug=brand-identity
fact: A local slash command whose post-condition is its stdout echo (/mcp) reads back on a verified CLI once a new local-command-stdout tag appears in the session transcript after the send; it prints no new word.
@task_fact n=2 slug=brand-identity
fact: A local command with no post-condition (/cost, /usage, /remote-control) reads unconfirmable. So does an echo command when the session declared no transcript or the CLI is unverified.
@task_fact n=3 slug=surface-cli
fact: A local command is recognised by its name, alone or followed by one space and its arguments. Six names are known: /clear, /remote-control, /compact, /mcp, /cost and /usage.
@task_fact n=4 slug=surface-cli
fact: viola verify measures 22 relied capability rows, the 22nd being local-command-echo, so its step counter reads [NN/22] and a clean summary reads 22 pass 0 fail.
@task_item n=1 part=untouched
Colour palette, typography, spacing, border radius, depth and iconography: no colour, glyph, separator or column changed.
@task_item n=2 part=untouched
viola list board and table, the flight-progress strip, stable slots, the stale and dialog-pending strips, and the fleet console: nothing on the board changed.
@task_item n=3 part=untouched
The viola mcp verb and its stdout: the /mcp slash command sent to the CLI is a different thing from viola's own mcp verb, and that verb is unchanged.
@task_item n=4 part=untouched
viola link and unlink output, viola wait output, viola last output, viola pause, answer and release output, and the handoff transfer marker are all unchanged.
@task_item n=5 part=untouched
The exit codes, the exit 21 causes and their --json causes, wrapper fault detail, peer reply screening and the strict-modes refusals are unchanged.
@task_item n=6 part=untouched
viola revive refusals and its --json codes, and viola plugin install output are unchanged.
@task_item n=7 part=untouched
Command names and the help verb groups: no verb, flag or name was taken or freed.
@task_item n=8 part=untouched
The web-surface and rendered-page absences, the CLI tape absence, and the self-validation protocol tests are unchanged.
