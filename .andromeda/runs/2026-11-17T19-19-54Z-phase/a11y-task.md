@task_prose part=chunk
Adds the 23rd relied capability-ledger row in viola-agent-claude: the claude CLI honours a per-project trust record (hasTrustDialogAccepted) another process wrote into the config file its child reads.
Adds its viola verify probe as a SIXTH interactive verify run: it writes the trust record for its own OS-temp dir, expects the child's first settled frame to show no trust dialog, then removes only its own record on every exit path.
Builds the record writer once (lock when free, atomic replace, read-back, at most 3 attempts), where the next entry's product write can use it; the fake agent now reads the record, not argv --trusted-root alone.
Re-runs the step-count sweep the sixth run moves over every spelling of the count, and stamps the row with one live viola verify (S0) on the 2.1.325 install under the rig's private config.
Changes nothing of viola run or revive; the product's own trust write is not part of this chunk.
@task_item n=1 part=needs
viola verify output gains a sixth interactive run: its step lines (the step-line contract) and the step count every printed form of verify's progress carries move from five interactive runs to six.
@task_item n=2 part=needs
The new probe's row prints a closed word for what it could not do (config file absent, unreadable, a link, not the expected JSON, or oversize), never pass; it prints fail when the CLI does not honour the record. These are fixed verify output strings pinned by literal.
@task_item n=3 part=needs
verify's summary prints what proves verify's own trust record is gone after the run; a fixed line in verify's output.
@task_item n=4 part=needs
An interrupted verify (Ctrl-C or the stop) during the sixth run still removes verify's own record and prints the interrupted pair and exit 1 unchanged; the plan says what a SIGKILLed verify leaves and that the stop removes it.
@task_item n=5 part=needs
verify's output must not carry the OS-temp dir path or the config file path it writes into; paths are not carried, and any byte printed from a path or reason is escaped as a C0/C1 control character.
@task_item n=6 part=needs
The probe's post-condition reads only the child's first settled frame for the trust dialog signature; no screen-content parsing beyond signatures, and the check never inspects claude content otherwise.
@task_item n=7 part=needs
The probe's control (record withheld, the dialog read) and post-condition are proved on the fake agent in the existing test suites; each control fires with its --timeout S, waits are bounded under the 20 s lint, and time bounds fall on the driver, never the human.
@task_item n=8 part=needs
A change to the fake agent's trust reading or screens puts the cli_verify tests in the gate block; they run with no skips, no retries, zero flakiness.
@task_item n=9 part=needs
Live readings never gate: the live S0 verify stamps the ledger row but no CI check depends on it; the tests' shared stamp and CI stay green with the 23rd row relied.
@task_item n=10 part=needs
verify's refusals before any start (home strict-modes failure, socket directory failure) are unchanged by the sixth run and still come before any write, now including the trust-record write.
@task_item n=11 part=needs
No SGR in plain modes and no colour on refusal or failure words: the new probe row's closed word and fail word print uncoloured, with the word carrying the state.
@task_item n=12 part=needs
No spinners and linear static output: the sixth run's progress prints as static step lines like the other five.
@task_standing n=1
convention: each new a11y assertion lives inside the existing test suites, not a separate a11y suite or driver
home: 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new fixed CLI string is pinned by its literal in a test and its copy comes from design
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: each new test runs under the CI gate with no skips, no retries and zero flakiness, its failures nextest failures
home: 10-slo-invariants-a11y-budgets · 9-ci-integration
@task_item n=1 part=not_needed
The viola list board, its captions, colour, stale rows, NAME truncation and the unwrapped separator: the chunk prints nothing on the board.
@task_item n=2 part=not_needed
viola revive refusals, revive --json documents and revive --list: nothing of revive changes here.
@task_item n=3 part=not_needed
viola run TUI passthrough rows (human keystrokes, focus/mouse/resize sequences, own terminal bytes, feed panic): viola run is not changed.
@task_item n=4 part=not_needed
viola link and unlink, plugin install, and viola --help rows: no verb is added or changed besides verify.
@task_item n=5 part=not_needed
MCP frame tools, hook event and dialog calls, frame verb refusals and instance-unreachable details: the chunk touches no frame channel.
@task_item n=6 part=not_needed
viola send and viola wait output and refusals, and the dialog hold: the chunk sends nothing and answers no dialog.
@task_item n=7 part=not_needed
Screen-reader targets, manual Orca and keyboard passes, and visual design rows (amber, bold): no manual pass is owed by this chunk.
