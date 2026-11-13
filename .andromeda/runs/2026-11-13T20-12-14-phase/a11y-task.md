@task_prose part=chunk
Closes in code or tests the rule lines the rows landing owed the route: every refusal reason prints its hint line (unverified-cli and answer's not-delivered details included), the fixture email check on every fixture kind and the record write, run parsing its name through the shared name check, trusted-state and diagnostics opens refusing a symlink, payload types deriving Redact.
Trims the PTY resize test of its needless key and the fd test to the live child table with no fd count assertion; folds three copies of the digit-runs test helper into one shared copy.
Re-asserts the snapshot-replay guard for the replay row, with its one-off remove-the-guard control through the gate tool's control step; no mutation run and no live claude start.
Adds no new refusal reason, exit code, event kind or wire field: a hint is a human-output line.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: viola send's refusal path and the shared CLI name check — edits: the hint_cause mapping (unverified-cli and unknown give no hint today), parse_name made the one shared copy
@task_file n=2 path=src/cmd/answer.rs mark=edits
what: viola answer's refusal output — edits: the not-delivered details that print no hint today
@task_file n=3 path=src/cmd/run.rs mark=edits
what: viola run's entry, the TUI passthrough start — edits: its own copy of parse_name, replaced by the shared name check
@task_file n=4 path=src/human/mirror.rs mark=edits
what: the human refusal output, the unable line and its hint line — edits: write_unable, the hint optional today
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude fixture and record ledger, its fixture scrub and the hook and MCP payload types — edits: text_fault, screen_text_fault, has_email, the payload types deriving Redact
@task_file n=6 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes check on trusted state — edits: check_instance, check_stamps, check_path, check_read, unix::reading
@task_file n=7 path=crates/viola-state/src/fs.rs mark=edits
what: the private-file opener for diagnostics files — edits: open_private_append
@task_file n=8 path=src/obs.rs mark=edits
what: the run diagnostics, kept off the terminal — edits: check_diagnostics, the diagnostics caller of the private open
@task_file n=9 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene walk over committed fixtures — edits: the walk filters, claude_fixtures, check_mcp and the screen walk's email check
@task_file n=10 path=tests/support/hygiene.rs mark=edits
what: the shared fixture hygiene support — edits: the Violation list, which gains an email class
@task_file n=11 path=tests/support/mod.rs mark=edits
what: the shared test helpers — edits: the one shared digit_runs and its digit_runs_reads_whole_numbers_only test
@task_file n=12 path=tests/security_negatives_server_identity.rs mark=edits
what: the server-identity security negatives — edits: its own digit_runs copy and its digit_runs_reads_whole_numbers_only test
@task_file n=13 path=tests/security_negatives_socket_dir.rs mark=edits
what: the socket-directory security negatives — edits: its own digit_runs copy and its digit_runs_reads_whole_numbers_only test
@task_file n=14 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negatives — edits: its own digit_runs copy
@task_file n=15 path=tests/tui_pty_seam.rs mark=edits
what: the TUI passthrough boundary test on a real outer PTY — edits: the resize test's needless key write
@task_file n=16 path=tests/tui_channel_fds.rs mark=edits
what: the TUI child's channel fd test — edits: the agent's fds self-dump, the linux cfg split, the fd count assertion
@task_file n=17 path=tests/cli_links.rs mark=re-reads
what: the snapshot-replay guard, the replay reading the link the wrapper wrote, and its one-off remove-the-guard control
@task_item n=1 part=needs
Every refusal ends with a hint: each refusal reason, unverified-cli and answer's not-delivered details included, prints its hint line after the unable line.
@task_item n=2 part=needs
The CLI refusal output's unable and hint lines and their stderr layout: the new hints keep the unable/hint pair, one write, the layout the literal pins hold.
@task_item n=3 part=needs
One hint per code, and no hint names a release: each new hint string is the one hint for its code word.
@task_item n=4 part=needs
The unknown refusal carries no hint, and faults carry no hint: the carve-outs a hint for every reason must keep.
@task_item n=5 part=needs
One --json document with no hint, and --json failure documents stay clean: the hint is a human-output line only, never a field.
@task_item n=6 part=needs
Fixed CLI strings pinned by literal and fixed CLI copy from design: each new hint string comes from the design copy and is pinned by its literal.
@task_item n=7 part=needs
Reason codes always beside unable, and kebab-case codes beside a plain word: the newly hinted refusals keep their code word on the unable line.
@task_item n=8 part=needs
No new refusal reason, exit code or event kind: the CLI exit codes and refusal exit code contracts stand unchanged.
@task_item n=9 part=needs
Printed names are the parsed arguments: viola run parses its name through the shared name check, so the name it prints is the parsed one.
@task_item n=10 part=needs
Strict-modes refusal before any write, and the strict-modes-failed refusals and details: a symlinked trusted-state path now refuses as a strict-modes failure, through the same refusal word and hint.
@task_item n=11 part=needs
Unstattable home is a strict-modes refusal: the symlink refusal sits beside it in the same strict-modes check.
@task_item n=12 part=needs
Run diagnostics never reach the terminal: the diagnostics open refusing a symlink must still write nothing to the terminal, and the TUI passthrough's run diagnostics contract holds.
@task_item n=13 part=needs
Zero viola bytes on the terminal and the TUI passthrough's own terminal bytes: the diagnostics and fd changes add no byte to the human's terminal.
@task_item n=14 part=needs
Focus, mouse and resize sequences never move the wheel: the resize test drives a resize with no key, the size receipt following the resize alone.
@task_item n=15 part=needs
portable-pty as the outer-PTY boundary driver and the TUI boundary driver: the resize test stays on a real openpty.
@task_item n=16 part=needs
Boundary tests sequenced on records: the resize and fd tests wait on receipts and records, never on time.
@task_item n=17 part=needs
Boundary check never inspects claude content: the fd test reads the live child's fd table, not vendor content.
@task_item n=18 part=needs
Payload types derive Redact: the hook and MCP payload types reach the structured log through Redact, as the channel request does.
@task_item n=19 part=needs
Snapshot replay from the log: the replay row's guard re-asserted as the link replay, with the budget leg's guard left in place.
@task_item n=20 part=needs
Paths not carried: the fixture email check joins the absolute-path and user-word checks on every fixture kind and on the record write.
@task_item n=21 part=needs
Error bodies have a writer and a witness: each new hint has its writer and a test witnessing it.
@task_standing n=1
convention: each new a11y assertion lives inside an existing test suite, no separate a11y suite or driver
home: 1-a11y-scope-summary · a11y-scope-summary
@task_standing n=2
convention: each new test's failure is a plain nextest failure, run in the existing test suites
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: each new test is deterministic, zero flakiness, no retries, and the gate allows no skips
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new test's a11y cases run inside the coverage suite in the push/PR workflow, with no a11y artifact of its own
home: 9-ci-integration
@task_standing n=5
convention: each new fixed string a test asserts is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
Colour and SGR rows (amber, bold, dim, truncation, colour decision inputs): the chunk adds no coloured span and no board column.
@task_item n=2 part=not_needed
The viola list board, caption and unwrapped-separator rows: no board output changes.
@task_item n=3 part=not_needed
Screen-reader target and setup rows and the manual keyboard and Orca passes: no manual pass is run or planned here.
@task_item n=4 part=not_needed
The budget gate and status-line rows: their removal is another entry's; the chunk only adds the replacement replay guard.
@task_item n=5 part=not_needed
viola revive --list, revive --json refusal documents and revive start rows: no revive output changes.
@task_item n=6 part=not_needed
viola link and unlink result lines, exit 20 and exit 21, and plugin install output: untouched.
@task_item n=7 part=not_needed
config.json values, the dialog hold and the MCP wait tool's timeout: no time bound changes.
@task_item n=8 part=not_needed
viola --help rows (usage line, group labels, handoff row, served verbs): no verb or help line is added.
@task_item n=9 part=not_needed
The structured violation JSON rows: no new violation report shape.
@task_item n=10 part=not_needed
Absences of a web or graphical frontend, touch surface, rendered page and internationalization, and the WCAG and browser a11y leg rows: nothing here adds such a surface.
@task_item n=11 part=not_needed
Cognitive escalation and reading-level rows: no copy is graded or escalated.
