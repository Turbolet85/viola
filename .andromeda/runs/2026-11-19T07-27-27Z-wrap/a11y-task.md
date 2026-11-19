@task_prose part=session
The session added one fixed stdout line to viola verify, printed only when the run met the CLI's browser-tools prompt, placed just before the summary, which stays the last line; the prompt-absent output is unchanged.
The line is plain ASCII with no SGR, a fixed literal in the founder's words, telling the human that viola never answers that prompt and to answer it once in claude, then run viola verify again.
The agent crate gained a public reading of whether a run met the browser-tools prompt, and a new integration case pins the new line by literal inside the existing verify suite.
The test-only live rig gained two arms (vh under a given real home that reads names and one needle only, cx that sends /context and reads its post-condition) with labels L1, L1-2 and L2, and rehearsal flags --target-home and --claude-dir.
Live readings on claude 2.1.328 were recorded, never as gates: the founder's home verified 24 of 24 relied rows; the /context send came back not-delivered:no-prompt-submitted with no readable transcript, so that half stays unmeasured.
CI on the pushed head was green 7 of 7; the product rebuild was placed beside the product, not switched; no IPC method, config key, event or env var was added.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the claude agent crate's relied-row ledger, which reads the vendor CLI's screens by signature, including the browser-tools prompt; gains a public yes/no on whether a run met that prompt
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the agent crate's unit cases for the ledger and its stamps; gains a case for the browser-tools reading
@task_file n=3 path=src/cmd/verify.rs
what: the viola verify command, its fixed stdout lines and summary; gains the browser-tools line written before the summary
@task_file n=4 path=tests/cli_verify.rs
what: the viola verify integration suite, which pins verify's stdout by literal; gains the case for the line before the summary
@task_file n=5 path=tests/support/fake.rs
what: the fake-agent test support shared by the CLI suites; now holds the browser-tools prompt screen fixture
@task_file n=6 path=tests/cli_send.rs
what: the viola send integration suite; its browser-tools screen fixture moved out to the shared test support
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig driving the real claude CLI, its arms, labels and summary words; gains the vh real-home arm and the cx /context arm
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation helpers, the child env and the bounded name-only readers of a Claude config
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's binary and its flags; gains the vh and cx arms and the rehearse flags --target-home and --claude-dir, with vh refused as usage exit 2 without an explicit home
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal suite run against the fake agent; gains the vh and cx rehearsals
@task_item n=1 part=touches
contract/viola-verify/step-line: verify's output gains one non-step line before the summary on a met browser-tools prompt; the summary stays the last line and the prompt-absent output is unchanged.
@task_item n=2 part=touches
decision/critical-path-cli-board-output: verify output is on the critical path, and it now holds a conditional line before the summary.
@task_item n=3 part=touches
decision/fixed-cli-strings-pinned and decision/fixed-cli-strings-pinned-by-literal: the new verify line is a fixed CLI string pinned by literal in the verify integration suite.
@task_item n=4 part=touches
decision/fixed-copy-from-design: the line's copy is in the founder's words and is carried as a design-system surface-cli amendment; a11y's rule that fixed CLI copy comes from design reaches it.
@task_item n=5 part=touches
decision/no-sgr-in-plain-modes, decision/no-sgr-without-tty, decision/no-sgr-off-terminal and decision/colour-second-cue-only: the new line is plain ASCII with no SGR in every mode.
@task_item n=6 part=touches
decision/static-linear-output, decision/linear-static-output and decision/linear-static-cli-output: the line is one static line written once before the summary, no spinner or redraw.
@task_item n=7 part=touches
decision/a11y-inside-existing-suites and decision/a11y-runs-in-existing-test-suites: the new output-discipline case runs inside the existing verify integration suite, no separate a11y suite.
@task_item n=8 part=touches
decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver: viola never answers the browser-tools prompt; the human answers it in claude at their own pace and reruns verify, and the rig's /context wait falls on SUBMIT_BOUND, the driver's bound.
@task_item n=9 part=touches
decision/no-screen-content-parsing and decision/boundary-check-never-inspects-claude-content: the browser-tools prompt is read by its screen signature; the rig's cx arm counts tags in a session transcript read through MAX_FRAME with no byte kept, and its vh arm reads Claude config key names and one needle only.
@task_item n=10 part=touches
decision/live-readings-never-gate: the live L1 and L2 readings on claude 2.1.328 are recorded as readings, never as gates or verification.
@task_item n=11 part=touches
contract/send-refusal/no-prompt-submitted: the cx arm's /context send through viola send came back not-delivered:no-prompt-submitted live on claude 2.1.328.
@task_item n=12 part=touches
decision/zero-flakiness: the intermittent red of the rig rehearsal reading the box empty after x and del stays open with its cause unread; no retry rule and no loosened entry were added.
@task_item n=13 part=touches
decision/a11y-ci-github-actions and decision/cli-invariants-pass-in-ci: CI on the pushed head ran green, 7 of 7 checks, with the new verify case inside it.
@task_item n=14 part=touches
contract/cli-usage-error/exit-2 and contract/cli-exit-codes/2: the test-only rig refuses vh without an explicit home as usage, exit 2; this is the rig binary, not viola's CLI.
@task_fact n=1 slug=1-a11y-scope-summary
fact: viola verify, on a run that met the browser-tools prompt, prints one fixed plain line before the summary saying the prompt stood at start, viola never answers it, answer it once in claude and run viola verify again; the summary stays last, and the prompt-absent output is unchanged.
@task_item n=1 part=untouched
The tui passthrough contracts and decisions: no byte viola writes to the terminal, no human keystroke rule, and no focus, mouse or resize handling changed.
@task_item n=2 part=untouched
viola list board, its caption, colour, stale-row dim, NAME bold and truncation rows: the board was not changed.
@task_item n=3 part=untouched
Refusal and fault output: the unable and hint lines, reason codes, --json failure documents, exit codes 1, 10, 11, 13, 14, 20 and 21, and the instance-unreachable details are unchanged; the new verify line is no refusal.
@task_item n=4 part=untouched
viola link, unlink, revive, wait, plugin install, --help, the hook calls and MCP frame tools: none of their output or contracts moved.
@task_item n=5 part=untouched
Control-character escaping, log identity tags, config.json keys and the event line: no new interpolated text, config key, log line, span or event was added.
@task_item n=6 part=untouched
The absences (no web or graphical frontend, no touch surface, no i18n, no rendered page, no user time limit) and the screen-reader targets and manual Orca and keyboard passes stand unchanged.
@task_item n=7 part=untouched
The a11y tier, compliance and WCAG non-claims, the structured violation JSON, and the mutation audit and coverage gate rows: no gate, threshold or tool changed.
