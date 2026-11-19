@task_prose part=session
viola verify now prints one fixed line on stdout when its run met the CLI's browser-tools prompt: the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again.
The line comes after the stamps.json update succeeds and before the summary; the summary stays the last line, and the prompt-absent stdout is unchanged. It is plain ASCII with no SGR, and verify adds no log line, span or event for it.
The agent ledger gained a public reading of whether a run met the browser-tools prompt, which verify uses for that line.
The test-only live rig gained a real-home arm (vh) and a /context arm (cx), with the labels L1, L1-2 and L2 and the rehearse flags --target-home and --claude-dir. No IPC method, socket, env var, config key, stamp shape or exit code was added.
Measured live on claude 2.1.328 under the founder's real home: verify read 24 of 24 relied rows, trust-record pass, browser-tools-prompt absent, so the home's 2.1.328 stamp is now a 24-row stamp. A /context send under the rig's private config came back not-delivered:no-prompt-submitted, with no transcript to read.
The release build was placed beside the pinned product, not switched in. Nothing was retired.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the agent crate's verification ledger, which reads the relied rows of a verify run, the browser-tools prompt among them, and writes the stamps.json stamp
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's unit cases for stamps and for the browser-tools prompt reading
@task_file n=3 path=src/cmd/verify.rs
what: the viola verify verb, which runs the relied-row checks, updates stamps.json and prints the step lines and the summary on stdout
@task_file n=4 path=tests/cli_verify.rs
what: the integration tests of viola verify's output, its step lines, summary and refusals
@task_file n=5 path=tests/support/fake.rs
what: the shared fake-agent test support, the screens the fake CLI shows, the browser-tools prompt screen among them
@task_file n=6 path=tests/cli_send.rs
what: the integration tests of viola send and its readback mirror
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig that starts the real claude CLI under viola, its arms and labels and its printed summary words
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: the test-only rig's isolation of the child's home and config, its child environment and its names-only reads of a Claude config
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig's command-line entry, its live and rehearse commands and their flags
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rehearsal tests of the test-only live rig against the fake agent
@task_item n=1 part=touches
The viola verify output contracts: the summary and summary-line rows (the summary stays the last line, now with one possible line before it) and the step-line row (the new line is no step line).
@task_item n=2 part=touches
The viola verify output for a stopped or refused run and for a refused stamps.json update: the new line prints only after the stamps update succeeds, so a refused update shows no line.
@task_item n=3 part=touches
CLI streams and the data-on-stdout, messages-on-stderr decision: the new line is written to stdout as part of verify's human result, not to stderr.
@task_item n=4 part=touches
Hints in human mode only, the hint line and no hint under --json: the new line carries an instruction (answer it once in claude, then run viola verify again) yet is a stdout result line, not a hint: line.
@task_item n=5 part=touches
Fixed phraseology vocabulary and confirmations carry context: the new line is a fixed literal in the founder's words.
@task_item n=6 part=touches
ASCII-only human output and the CLI colour limits: the new line is plain ASCII and carries no SGR span.
@task_item n=7 part=touches
The step counter as the one progress pattern and plain output modes: verify gains a line that is neither a step nor progress.
@task_item n=8 part=touches
CLI verification state verified and unverified-cli: the live home's 2.1.328 stamp is now a 24-row stamp with every relied row passing; the 2.1.325 entry stays at 21 rows and reads unverified to a binary of this tree.
@task_item n=9 part=touches
The viola verify trust-record row: trust-record read pass live on 2.1.328 for the first time.
@task_item n=10 part=touches
The not-delivered no-prompt-submitted refusal hint and the send readback rows: the rig's /context send came back not-delivered:no-prompt-submitted, a reading only, no change to the refusal.
@task_item n=11 part=touches
The send readback /mcp row: /mcp disable on 2.1.328 wrote no server name into the user's Claude config, a reading of one version only.
@task_item n=12 part=touches
CLI stderr context lines: shares the word context with the /context readings and is not changed by them.
@task_item n=13 part=touches
The verify command name: the verb keeps its name; its output gains the browser-tools line.
@task_fact n=1 slug=surface-cli
fact: When a viola verify run met the CLI's browser-tools prompt, verify prints on stdout, after the stamps.json update succeeds and before the summary, the one fixed line: the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again. The summary stays the last line, the line is plain ASCII with no SGR, and the prompt-absent output is unchanged.
@task_fact n=2 slug=surface-cli
fact: viola never answers the CLI's browser-tools prompt; verify names it and leaves the human to answer it once in claude.
@task_item n=1 part=untouched
Brand identity: the strip board, the readback box, the cocked strip and the strip fields are not reached; no strip or readback changed.
@task_item n=2 part=untouched
Colour palette and colour tokens: no colour, accent or SGR token moved.
@task_item n=3 part=untouched
viola list and its board, columns, row order, NAME truncation and skip counts: not reached.
@task_item n=4 part=untouched
viola run and revive start refusals, revive --json codes and revive output: not reached.
@task_item n=5 part=untouched
viola link and unlink, pause, release, answer, wait and last output: not reached.
@task_item n=6 part=untouched
CLI exit codes, exit 21 causes and their --json causes, wrapper fault details and peer reply screening: no exit code or cause moved.
@task_item n=7 part=untouched
The hook and mcp verbs, the --help verb groups and the CLI colour decision order: not reached.
@task_item n=8 part=untouched
The rendered-page and web-surface absences, typography, spacing, iconography and the self-validation protocol: not reached.
