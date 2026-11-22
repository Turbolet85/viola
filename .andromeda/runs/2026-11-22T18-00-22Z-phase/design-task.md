@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each item worded by what the next boundary's audit will read.
It measures first, each from a test or a no-start probe, as a number or a closed word: the chrono advisory, the claimed-line wait under a failing snapshot write, verify's agents read under a stalled host, a SIGKILLed verify's trust-record residue, the boot supervisor test.
It then mends: an end for a send slot's claimed line whose settle never comes, a bound of its own for verify's agents read, the boot test's deadline strictly below its kill, each held by a test.
It adds killing tests for the seven agent-adapter and three paste-writer mutation survivors, and re-reads the mutants profile's kill against a cold parallel baseline.
No live start, no mutation or witness run, no timeout raised, no retry; a new closed word, time bound value or user-seen behaviour is the operator's fork.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot that holds a claimed line until it settles and confirms the send — edits: SendSlot::confirm, the Waiting or Claimed wait arm, the settle skipped after a failed snapshot update
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded read of the Claude agents list as JSON inside Run B — edits: the agents read's bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude agent adapter's ledger, its environment-name fault check among the mutation survivors — edits: its tests only
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the Claude agent adapter's screen reading — edits: its tests only
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump's paste writer, bounded so a cut paste ends the send as paste-cut — edits: its tests only
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the end-to-end harness lifecycle tests — edits: the boot supervisor test, its binary copy moved out of the timed body
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the end-to-end harness boot supervisor — edits: ABORT_DEADLINE, kept strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the test runner's profiles — edits: the mutants profile's kill for the agent-drift contract's two replay tests
@task_file n=9 path=deny.toml mark=edits
what: the dependency audit's advisory ignores — edits: the chrono advisory's ignore and its reason line, re-read at each boundary
@task_file n=10 path=.claude/rules/testing.md mark=edits
what: the testing rules for the codebase, at the phase's discretion — edits: its paths: list
@task_item n=1 part=needs
The send confirmation: a claimed line whose settle never comes gets an end, so a send is never left waiting and never announced as read back before its confirmation; the end it reaches is one of the existing send readback outcomes, and a new outcome word is the operator's fork.
@task_item n=2 part=needs
What a send whose confirmation cannot be read shows: the unconfirmable readback and its CLI mirror, never an optimistic success line, when the snapshot write fails under a claimed line.
@task_item n=3 part=needs
Verify's agents read gets a bound of its own, measured under a stalled host; its failure surfaces through the existing verify output (the step line, the fail word, the stopped or refused run line), never a retry and never a loosened bound without its measurement.
@task_item n=4 part=needs
A failed agents read where the board reads the agents list stays shown as the existing failed-agents-read state; the agents read's new bound does not change what that state prints.
@task_item n=5 part=needs
A viola verify ended by SIGKILL, measured under a test-owned home and a test-owned Claude config: whether a project entry pointing under its scratch root is left behind; the trust-record row of the verify output and the interrupted-run refusal are the surfaces that measurement reads; a found residue becomes a named residual or a mend.
@task_item n=6 part=needs
Any line the mends add or change on the CLI is ASCII-only, fixed phraseology with no internals, paths or stack traces, messages on stderr and data on stdout, and no spinner or elapsed-time counter while a bounded wait runs.
@task_item n=7 part=needs
Under --json, a bounded wait or claimed-line end that fails yields one clean failure document on stdout, with no hint line.
@task_item n=8 part=needs
The paste-cut refusal and its not-delivered hint: the paste-writer killing tests pin the existing paste-cut end of a cut paste, the wheel unmoved, without changing what it prints.
@task_item n=9 part=needs
A new time bound is a named member of the bound lists, its carriage said in the founder-forks block, never by dropping another member's words; its value is the operator's.
@task_item n=10 part=needs
Measurements are numbers or closed words from a test or a no-start probe only: no live start, no rig start of a real claude, and nothing reads or edits the operator's real Claude home or config.
@task_item n=11 part=needs
The killing tests for the mutation survivors assert the existing behaviour of the agent adapter's environment-name fault and screen reading, and of the paste writer, exactly; they add no behaviour.
@task_standing n=1
convention: each new test that asserts a CLI line asserts the fixed phraseology word for word, ASCII-only, on its proper stream
home: surface-cli
@task_standing n=2
convention: each new error or refusal path a test exercises shows no internals, no stack trace, and no success before confirmation
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Colour palette, typography and spacing rows: the chunk adds no colour, weight or column spacing to any line.
@task_item n=2 part=not_needed
Iconography rows and the list's six fixed columns: no glyph, separator or column is added or moved.
@task_item n=3 part=not_needed
The web surface, depth, border radius and rendered-page absences: viola renders no page.
@task_item n=4 part=not_needed
Revive and run start refusal rows, their --json refusal codes and the exit 21 causes: no start path is changed or run.
@task_item n=5 part=not_needed
Link and unlink output rows, the wait output rows and the last output rows: none of those verbs is touched.
@task_item n=6 part=not_needed
The help verb groups, the hidden mcp verb, the hook verb and the CLI colour decision order: no verb is added, renamed or regrouped.
@task_item n=7 part=not_needed
The strip-board expression rows (cocked dialog strip, stale dimming, slots, density, handoff marker): the fleet console is not touched.
@task_item n=8 part=not_needed
The self-validation protocol's design tests (squint, swap, signature check): no visual design is changed.
