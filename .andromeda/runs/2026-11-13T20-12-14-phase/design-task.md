@task_prose part=chunk
Closes in code or tests the rule lines the rows landing owed the route: every refusal reason prints a hint (unverified-cli and answer's other not-delivered details included); the email check reaches every committed fixture kind and the --record write; the hook, agents and statusline hygiene walks check emails.
run parses its name through the shared name check; trusted-state checks and the diagnostics open refuse a symlink, the diagnostics open narrowing the open file; the hook and MCP payload types derive Redact.
The resize test drops its needless key; the fd test always reads the live child fd table and drops its fd count assertion; one shared digit_runs test helper replaces three copies.
Adds the snapshot-replay guard read off the link the wrapper wrote, with its one-off remove-the-guard control; every coordinate is re-tested at HEAD.
No new refusal reason, exit code, event kind or wire field; a hint is a human-output line.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: viola send verb and its refusal-hint mapping and the shared CLI name check — edits: hint_cause, parse_name
@task_file n=2 path=src/human/mirror.rs mark=edits
what: the human readback mirror's unable line with its optional hint — edits: write_unable
@task_file n=3 path=src/cmd/answer.rs mark=edits
what: viola answer verb and its not-delivered refusal details, each to print a hint
@task_file n=4 path=src/cmd/run.rs mark=edits
what: viola run verb's name parsing — edits: its own parse_name copy, replaced by the shared one
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude agent ledger's fixture text screening (absolute path, user word, email) and the --record fixture write, plus its hook and MCP payload types — edits: text_fault, screen_text_fault, has_email, the hook and MCP payload types
@task_file n=6 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene walks over every committed fixture kind — edits: the walk filters, claude_fixtures, check_mcp, the hook, agents and statusline walks
@task_file n=7 path=tests/support/hygiene.rs mark=edits
what: the shared fixture hygiene violation classes — edits: Violation, gaining an email class
@task_file n=8 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes trusted-state checks — edits: check_instance, check_stamps, check_path, check_read, unix::reading
@task_file n=9 path=crates/viola-state/src/fs.rs mark=edits
what: the private diagnostics file open — edits: open_private_append, restrict
@task_file n=10 path=src/obs.rs mark=edits
what: the diagnostics caller of the private open — edits: check_diagnostics
@task_file n=11 path=tests/tui_pty_seam.rs mark=edits
what: the PTY resize test on a real openpty — edits: the needless key write before the size receipt
@task_file n=12 path=tests/tui_channel_fds.rs mark=edits
what: the channel fd test over the live child fd table — edits: the self-dump fds receipt, the linux cfg split, the fd count assertion
@task_file n=13 path=tests/cli_links.rs mark=edits
what: the link verb tests and the snapshot-replay guard that reads the link the wrapper wrote — edits: e5_the_replay_reads_the_link_the_wrapper_wrote
@task_file n=14 path=tests/support/mod.rs mark=edits
what: the shared test support module, home of the one shared digit_runs helper and its whole-numbers test
@task_file n=15 path=tests/security_negatives_server_identity.rs mark=edits
what: the server-identity security negatives tests — edits: its digit_runs copy and digit_runs_reads_whole_numbers_only test
@task_file n=16 path=tests/security_negatives_socket_dir.rs mark=edits
what: the socket-directory security negatives tests — edits: its digit_runs copy and digit_runs_reads_whole_numbers_only test
@task_file n=17 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negatives tests — edits: its digit_runs copy
@task_item n=1 part=needs
The refusal hints table: every refusal reason carries a hint, unverified-cli and each answer not-delivered detail included (control-character, empty-text, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog).
@task_item n=2 part=needs
The CLI refusal line and its hint line on stderr: the unable line, the two-space form, the fixed unable form, and the hint line that follows it.
@task_item n=3 part=needs
The viola send readback mirror's unable and unconfirmable forms, and the send readback unable row, where the hint now always prints.
@task_item n=4 part=needs
The viola answer verb's output and its not-delivered refusals, each now ending with a hint.
@task_item n=5 part=needs
The carve-outs the new hints must keep: no hint for faults and for unknown, no hint under --json, hints in human mode only, no hint names viola release.
@task_item n=6 part=needs
What a new hint's text must obey: it never quotes the sent text, names no internals (no path, pid or stack), and is ASCII-only human output with control characters escaped.
@task_item n=7 part=needs
The --json failure documents stay clean and unchanged: the hint is a human line only, so no new refusal code, cause or field reaches a document.
@task_item n=8 part=needs
The strict-modes refusals a symlinked trusted-state file or diagnostics file now falls into: the instance-state and home-state unable/hint pairs, exit 21 with the strict-modes-failed cause, and the board's strict-modes refusal.
@task_item n=9 part=needs
The run start refusals and their fixed message form, which a name refused by the shared name check now reaches the same way send does; start refusals show no path or pid.
@task_item n=10 part=needs
The viola verify unclean-fixture refusal, which the email check at the --record write now trips like the absolute-path and user-word checks.
@task_item n=11 part=needs
The CLI exit codes: the chunk adds none and changes none; a refused symlink or name exits on its existing code.
@task_item n=12 part=needs
The silent verbs' catch sites (hook, mcp, run while wrapped): a payload type deriving Redact and the diagnostics open refusal print nothing to the human there.
@task_item n=13 part=needs
Data on stdout and messages on stderr: each new hint goes to stderr beside its refusal line.
@task_item n=14 part=needs
The anti-pattern rows over error and hint wording: no internals in errors or hints, confirmations carry context, no optimistic send confirmation.
@task_item n=1 part=not_needed
The flight-progress strip, readback box signature, handoff and strip-board rows of the brand identity: the chunk draws no strip and changes no board state.
@task_item n=2 part=not_needed
Colour palette, colour depth, the amber and dim rules and the colour decision order: no colour is added or moved.
@task_item n=3 part=not_needed
Typography, spacing and iconography rows (bold NAME column, column gaps, glyphs, the six list columns): no table or glyph changes.
@task_item n=4 part=not_needed
The web-page surface and the rendered-page absence: viola renders no page.
@task_item n=5 part=not_needed
The viola list board's column, width, NAME cut, reading age, skip count and window figure rows: the board's layout is untouched.
@task_item n=6 part=not_needed
The viola revive --list output and its --json exit-1 codes, and the viola wait and viola last output rows: those verbs are not changed.
@task_item n=7 part=not_needed
viola release --budget output and the budget-paused rows: the budget gate and its replay leg stay as they are in this chunk.
@task_item n=8 part=not_needed
The wrapper fault --json detail rows and the peer reply screening rows: no wire field or code word changes.
@task_item n=9 part=not_needed
The --help verb groups and the command name register: no verb is added, renamed or hidden.
@task_item n=10 part=not_needed
The self-validation protocol rows (squint, swap, readback box signature check): they test design direction, not refusal or test code.
