@task_prose part=chunk
Closes in code or tests the rows landing's route-owed rule lines: a hint on every refusal reason (unverified-cli and answer's other not-delivered details included), run parsing its name through the shared name check, and trusted-state and diagnostics opens refusing a symlink.
Extends the email check to every committed fixture kind, the hygiene walks and the verify --record write; derives Redact on the hook and MCP payload types.
Reworks the PTY resize test (no needless key) and the fd test (reads the live child table, no fd count assertion); folds three digit_runs test helpers into one shared copy.
Adds the snapshot-replay guard on the link replay test, with its one-off remove-the-guard control through gate.py control; adds no refusal reason, exit code, event kind or wire field.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: viola send verb, its refusal-cause to hint mapping and the shared CLI name check — edits: hint_cause (the UnverifiedCli and Unknown arms), parse_name
@task_file n=2 path=src/human/mirror.rs mark=edits
what: the readback mirror human output, the unable line with its optional hint — edits: write_unable
@task_file n=3 path=src/cmd/answer.rs mark=edits
what: viola answer verb, its not-delivered refusal details and their hints
@task_file n=4 path=src/cmd/run.rs mark=edits
what: viola run start, the name argument check — edits: its own parse_name copy (removed for the shared one)
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude ledger, fixture scrub and the verify --record write's text checks, and its hook/MCP payload types — edits: text_fault, has_email, screen_text_fault
@task_file n=6 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene walk check over committed fixtures — edits: the walk filters, claude_fixtures, check_mcp
@task_file n=7 path=tests/support/hygiene.rs mark=edits
what: the shared fixture hygiene violation classes — edits: Violation (an email class)
@task_file n=8 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes trusted-state checks — edits: check_instance, check_stamps, check_path, check_read, unix::reading
@task_file n=9 path=crates/viola-state/src/fs.rs mark=edits
what: the private state file opens — edits: open_private_append
@task_file n=10 path=src/obs.rs mark=edits
what: the private diagnostics file, its open and check — edits: check_diagnostics
@task_file n=11 path=tests/tui_pty_seam.rs mark=edits
what: the PTY resize check on a real openpty — edits: the resize test's key write
@task_file n=12 path=tests/tui_channel_fds.rs mark=edits
what: the channel fd check on the live child table — edits: the self-dump fds receipt, the linux cfg split, the fd count assertion
@task_file n=13 path=tests/cli_links.rs mark=edits
what: the viola link replay check — edits: e5_the_replay_reads_the_link_the_wrapper_wrote
@task_file n=14 path=tests/security_negatives_server_identity.rs mark=edits
what: the server identity security negative tests — edits: digit_runs, digit_runs_reads_whole_numbers_only
@task_file n=15 path=tests/security_negatives_socket_dir.rs mark=edits
what: the socket directory security negative tests — edits: digit_runs, digit_runs_reads_whole_numbers_only
@task_file n=16 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negative tests — edits: digit_runs
@task_item n=1 part=needs
Every refusal cause prints a hint, including unverified-cli and unknown on send and each not-delivered detail of viola answer.
@task_item n=2 part=needs
Each new hint is the last stderr line, names the next verb, never quotes prompt text nor names a path, and never names viola release.
@task_item n=3 part=needs
The send unable line of the readback mirror and its word column: the hint follows it, the mirror line itself keeps its layout and has no colour.
@task_item n=4 part=needs
The CLI refusal line form and the shared refusal word column stay as they are; a hint is a human-output line, no new refusal reason, exit code or --json field.
@task_item n=5 part=needs
viola answer output: the answered and unknown-dialog lines and the not-delivered refusals with their hints.
@task_item n=6 part=needs
viola run start parses its name as a ViolaName through the same check send uses; the name is never case-transformed and a target is always a wrapped name.
@task_item n=7 part=needs
The strict-modes refusal on the frame verbs, viola list, viola plugin install and viola revive (state files writable by another user), and the strict-modes-failed --json detail, now also fire on a symlinked trusted-state path.
@task_item n=8 part=needs
The viola verify --record write refuses a fixture carrying an email: the recorded-fixture-not-clean refusal and the --record output line.
@task_item n=9 part=needs
The viola home not private refusal of viola verify, reached through the strict and private-file checks a symlink now fails.
@task_item n=10 part=needs
The viola link output and its replay: the guard test reads the link the wrapper wrote back from the log.
@task_item n=11 part=needs
Results stay on stdout and the rest on stderr; plain output modes print the hints with no styling.
@task_item n=12 part=needs
The MCP tool refusal result carries the same refusal cause as the CLI; a hint added on the CLI adds no MCP field.
@task_standing n=1
convention: every new human-output line (a hint) is part of the human-output contract, plain ASCII in uncoloured modes and printed on stderr
home: surface-cli
@task_standing n=2
convention: every new or reworked test reads output through the CLI exit codes and refusal line forms the contracts fix, never a new code
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, so its absence row is not reached.
@task_item n=2 part=not_needed
The viola list board layout rows (column widths, name truncation, stale rows, colour depth, empty board, unwrapped rack, BAY line cell order): no board rendering changes.
@task_item n=3 part=not_needed
CLI token rendering rows (colour, spacing, radius, motion, callsign, label and data roles): no styling is added.
@task_item n=4 part=not_needed
The viola release --budget output and the closed budget gate on the board: the budget gate is not removed here, that is its own entry.
@task_item n=5 part=not_needed
viola wait output rows (waiting line, dialog, dialog-expired, timed out, turn-ended): wait is not changed.
@task_item n=6 part=not_needed
viola --help and per-verb usage rows, verb groups and hidden verbs: no verb is added or hidden.
@task_item n=7 part=not_needed
viola revive --list rows, preflight order, session-live check and passed start output: revive's start flow is not changed beyond the strict-modes refusal.
@task_item n=8 part=not_needed
Spinner, banner, progress glyph, footer, prompt and pager decisions: no progress or interactive output is added.
@task_item n=9 part=not_needed
viola last output and the cursor byte offset: the last verb is not touched.
@task_item n=10 part=not_needed
Wheel verb output (pause, release) and the spoken phraseology of wheel, handoff and dialog verbs: not reached.
