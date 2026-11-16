@task_prose part=session
The session lets a local slash command the CLI ran read as delivered when the CLI writes its own stdout tag to the session transcript: send records the transcript's end before the paste and counts new local-command-stdout tags after it, a count of one or more confirming the send.
The SessionStart hook payload's transcript_path is now read, shape-checked hook-side and again wrapper-side, carried as an optional transcript param beside the event on hook.event, kept out of every event data, log, span and reply.
The local command list grew from 3 to 6 compiled names (/clear, /remote-control, /compact, /mcp, /cost, /usage) and classification moved from exact text to by name (a name, or a name, one space and a non-empty rest); /mcp gains the echo post-condition, /cost and /usage confirm nothing.
verify's Run B pastes a seventh synthetic probe, /mcp disable viola-probe-absent, into a settled input box and measures the echo latency; it is the 22nd relied capability-ledger row local-command-echo (relied rows 21 to 22), stamped as typed_probe.echo_latency_ms.
The fake agent gains an argv-only --transcript test option; no dependency, config key, env var or flag of the product was added, nothing retired, and no live claude start was made.
@task_file n=1 path=src/run/send/echo.rs
what: new bounded transcript reader for send - opens the transcript with no symlink follow, regular file, owner the effective user, read capped at MAX_FRAME, returns only a count of stdout tags past an offset
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the Claude adapter's capability ledger - the compiled local command list and its post-conditions, the by-name classifier, the transcript path shape check, the relied ledger rows and their checks, the probe command literals
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the Claude adapter's hook payload parser and normaliser - tolerant hook payload parsing, redacted fields, now the SessionStart transcript path kept out of event data
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the capability ledger
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of the ledger row checks and their verdict vector
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit tests of the local command name table, the by-name classifier and the transcript path shape check
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit tests of the ledger checks over verify's typed runs, the echo row's check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the verify stamps and the stamp verdict over the relied rows
@task_file n=9 path=src/cmd/hook.rs
what: the viola hook command - reads hook stdin, normalises it and delivers hook.event frames to the wrapper channel; now sends the transcript param beside the event for a SessionStart
@task_file n=10 path=src/cmd/run.rs
what: the wrapper (viola run) - serves the channel methods, re-validates every hook.event frame, now re-validating the transcript param for a session-start line and dropping any other
@task_file n=11 path=src/cmd/verify.rs
what: viola verify - the real-CLI capability probe runs, step counter and summary over the relied ledger rows
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's typed probe runs - Run B setup and the stamped typed_probe measurements
@task_file n=13 path=src/cmd/verify/typed/turns.rs
what: verify's Run B framing turns - the compiled probe pastes into a settled input box; now the seventh echo paste and a bounded, shape-checked read of the captured SessionStart transcript path
@task_file n=14 path=src/run/send.rs
what: the wrapper's send path - classifies typed text, issues one bracketed paste and confirms delivery; now records the transcript end before the paste, an echo command with no transcript being unconfirmable
@task_file n=15 path=src/run/send/classify.rs
what: send's delivery classification - local commands from the compiled list by name, the confirmation outcomes; the echo confirmation cases
@task_file n=16 path=src/run/send/slot.rs
what: the send slot - in-flight send state, the remembered transcript from the last session-start, the confirm window polling now counting transcript tags
@task_file n=17 path=src/run/send/relabel.rs
what: send's relabel of hook events onto the in-flight send
@task_file n=18 path=src/run/send/test_support.rs
what: send's unit test support
@task_file n=19 path=src/run/version_gate.rs
what: viola run's version gate - reads the verify stamps and holds the unverified CLI gate over the relied rows
@task_file n=20 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary - test seams under the fake-agent feature; now an argv-only transcript option writing an owner-only transcript beside its exe and the echo tag
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit tests of the fake agent's options
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot - its verify passes the fake agent the framing and transcript options
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run - expected verify output over the relied rows
@task_file n=24 path=tests/cli_send.rs
what: integration tests of viola send - the booted echo read-back case and the byte search of the home for the transcript path and tag
@task_file n=25 path=tests/cli_verify.rs
what: integration tests of viola verify
@task_file n=26 path=tests/cli_verify/runs.rs
what: integration tests of verify's probe runs and summaries
@task_file n=27 path=tests/cli_verify/stamps.rs
what: integration tests of verify's stamps
@task_file n=28 path=tests/contract_ledger_probes.rs
what: the ledger probe contract test - the relied row ids
@task_file n=29 path=tests/support/verify.rs
what: test support for the stamped-home verify - passes the fake agent the transcript option
@task_item n=1 part=touches
Classification sources are compiled only: the local command list stays compiled but grows from 3 to 6 names and classification moves from exact text to by name (a name, or a name, one space and a non-empty rest); the row stating exact-text matching is stale.
@task_item n=2 part=touches
Upstream text is content, never commands, and transcripts are no contract: the session now reads the CLI's transcript, counting only its local-command-stdout tag past the pre-send end as a delivery confirmer, no content crossing.
@task_item n=3 part=touches
Wrapper channel method hook.event and the channel frame hook.event re-validation: hook.event gains the optional sibling param transcript, re-validated by the wrapper for a session-start line only against its own session id, any other dropped with the line still taken.
@task_item n=4 part=touches
Wrapper re-validates every input and paste validation on both sides: the transcript path is checked hook-side and again wrapper-side by the same shared validator in the adapter crate.
@task_item n=5 part=touches
Every product reader bounded or named, reads capped at MAX_FRAME, one external read cap: two new transcript readers (send's echo count and verify's read of the captured SessionStart) read through MAX_FRAME; the bounded-readers census must name them.
@task_item n=6 part=touches
Hook payload is content and tolerant hook payload parsing: the SessionStart transcript_path field is now read, redacted, and kept out of event data; set only for a path passing the shape check.
@task_item n=7 part=touches
Refusals hold no paths, CLAUDE values never logged, user content only in diagnostics: the transcript path is in no data, events log, log line, span or reply; the echo read keeps a count alone.
@task_item n=8 part=touches
Delivery confirmed with a trailing whitespace tolerance and the send confirmation rules: a further confirmer, the echo tag count, confirms an echo-post-condition local command; an echo command with no remembered transcript or no readable end is unconfirmable.
@task_item n=9 part=touches
Unverified CLI gate and the version gate over the stamps: the relied capability-ledger rows go from 21 to 22 (local-command-echo), so a 21-row stamp reads unverified; any row holding the relied count holds 22 now.
@task_item n=10 part=touches
Verify's real-CLI transcripts accepted and verify trusted runs risk: Run B pastes a seventh probe, /mcp disable viola-probe-absent, with residuals - whether it writes the user's real Claude config is unmeasured, and it adds a line to the CLI's input history as every Run B paste does.
@task_item n=11 part=touches
viola writes nothing under the Claude home and nothing in Claude's config: the seventh probe drives the CLI's own /mcp disable of an absent server; under a private config it wrote no mcp disabled key, under the real config unmeasured.
@task_item n=12 part=touches
Probe prompts are synthetic and verify probe effects stay in their own directory: the new probe command is a compiled synthetic literal; verify skips it when Run B's SessionStart declares no readable, shape-checked transcript.
@task_item n=13 part=touches
Verify reads signatures only and verify screen read against compiled literals: verify now also reads a count from the CLI's transcript file, not the screen, for the echo row; no path or count reaches a step line, log or record.
@task_item n=14 part=touches
Verify never types into a dialog and input box readiness read from compiled literals: the seventh paste goes only into a settled input box.
@task_item n=15 part=touches
Test seams only under the fake-agent feature and test seams compiled out of release: the fake agent gains an argv-only --transcript option that creates an owner-only transcripts directory beside its exe and seeds the file 0600.
@task_item n=16 part=touches
Session id on child argv closed shape: the transcript path check requires the file name to be exactly the session id plus .jsonl with a session id of the closed shape.
@task_item n=17 part=touches
Fixture hygiene and fake transcripts compared to recordings: the fake agent with --transcript rewrites only the launch SessionStart; replayed recorded framing variants go out byte for byte and no fixture changed.
@task_fact n=1 slug=input-validation
fact: A SessionStart transcript path is accepted only when absolute, with no parent component, its file name exactly the payload's own session id plus .jsonl and that id of the closed session-id shape; the hook checks it and the wrapper re-checks it for a session-start line only, dropping it otherwise.
@task_fact n=2 slug=input-validation
fact: The transcript is read without following a symlink, only as a regular file owned by the effective user, from the recorded pre-send offset through MAX_FRAME, yielding a count of local-command-stdout tags alone with no byte kept.
@task_fact n=3 slug=data-protection
fact: The Claude transcript path is a redacted value, carried only as the optional transcript param beside hook.event, and appears in no event data, events log, log line, span, reply or verify record.
@task_fact n=4 slug=threat-model-summary
fact: A local command whose post-condition is the stdout echo (/mcp) is confirmed delivered only on a verified CLI by one or more new stdout tags in the transcript past its pre-send end within the confirm window; with no transcript it is unconfirmable, and /cost and /usage confirm nothing.
@task_fact n=5 slug=data-protection
fact: verify's Run B seventh paste /mcp disable viola-probe-absent is the 22nd relied row's probe; its residuals are accepted - an unmeasured write to the user's real Claude config, one more input-history line, and /context's echo shape never measured.
@task_item n=1 part=untouched
Dependency security: no crate, feature or tool pin was added or removed; the dependency graph, deny gate, MSRV, version floors and workflow pins stand.
@task_item n=2 part=untouched
Secret management: no credential, environment variable, env strip or canary scan class moved.
@task_item n=3 part=untouched
Socket and peer authentication: the socket directory checks, peer euid checks, server verification before any frame and strict-modes refusals are unchanged.
@task_item n=4 part=untouched
MCP server surface: the MCP tools, their inputs, the stdin line cap and the rmcp feature set are unchanged.
@task_item n=5 part=untouched
Error handling refusal and exit codes: no new refusal code, channel error code, exit code or error message was added.
@task_item n=6 part=untouched
Pinned binary, plugin files, release and CI pipeline rows: no pin, placement, re-hash, release check or CI job moved.
