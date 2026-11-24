@task_prose part=session
The session added the CLI verb viola up [NAME...] [--dry-run] [--json]: after a reboot it re-opens each instance whose heartbeat predates the boot, parents first, skipping live and stale ones, one terminal each through revive, and prints one plain line per instance.
Its output is one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, no SGR, one write per line; --json gives one document {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}} with no hint.
Its outcome words are a closed set: live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken; exit 0 when every line is live, already-live, stale-heartbeat or would-open, else 1; exit 21 on the board's strict refusal; exit 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
It added a new refusal code boot-time-unread with its own unable/hint pair, an up not-live hint line on stderr after the lines, the setup group of viola --help now lists up, and viola revive gained a hidden --origin flag that places the window on the recorded workspace.
Test-only: a shared teardown for stand-in terminal process trees replaced two kill loops (closing a coverage-profile flake), and the TUI fd-holding e2e guard got a settle wait inside its existing 2 s deadline; no assertion moved. up reuses open's existing live wait bound and mints no new time bound.
@task_file n=1 path=src/cmd/up.rs
what: the viola up verb — fleet selection after a reboot, parents-first order, the per-instance output lines, the --json document, its exit codes and refusals
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb table and dispatch, gaining the up verb
@task_file n=3 path=src/cmd/revive.rs
what: the viola revive verb, gaining a hidden --origin flag that places the revived window on the recorded workspace
@task_file n=4 path=src/cmd/open.rs
what: the viola open verb; its terminal spawn and its live wait (the open live bound) now shared with up
@task_file n=5 path=src/human/verbs.rs
what: the human output line writers for verbs — gains the up line writer and a hint line writer, one write per line
@task_file n=6 path=src/human/hints.rs
what: the fixed unable/hint pair literals per code word — gains the boot-time-unread pair and the up not-live hint
@task_file n=7 path=src/human.rs
what: the human output module and its exported fixed strings
@task_file n=8 path=src/bin/viola-fake-agent/foot.rs
what: the stand-in foot terminal used by tests, now recording each spawn
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the stand-in agent's own unit cases
@task_file n=10 path=crates/viola-state/src/liveness.rs
what: the liveness classification of an instance, gaining the host boot time read and a heartbeat-before-boot check
@task_file n=11 path=crates/viola-state/src/replay.rs
what: the event-log replay reader, gaining a session-ended-after-last-start read
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board scan of instances under the home, gaining a read accessor
@task_file n=13 path=schemas/diag-line.v1.json
what: the diagnostic line schema; its process-exit detail word list gains boot-time-unread
@task_file n=14 path=tests/cli_output_plain.rs
what: the plain-output literal checks — the viola --help literal and its served-verbs list, now with up in the setup group
@task_file n=15 path=tests/contract_error_bodies.rs
what: the error-body check that every refusal document has a writer and a witness; gains five up documents
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the bounded-readers guard listing every bounded input source; gains the boot time source and the events end-after-start read
@task_file n=17 path=tests/cross_exit_causes.rs
what: the exit-cause table check pairing each refusal word with its exit code; gains boot-time-unread at exit 1
@task_file n=18 path=tests/cli_revive.rs
what: the viola revive integration cases, gaining the --origin window placement cases
@task_file n=19 path=tests/cli_machine_contract.rs
what: the --json machine-output contract check, gaining the up document
@task_file n=20 path=tests/contract_diag_schema.rs
what: the diagnostic line schema check
@task_file n=21 path=tests/tui_channel_fds.rs
what: the e2e guard that the wrapped child under viola run holds no file of viola's home; gains a settle wait inside its existing 2 s deadline
@task_file n=22 path=tests/support/mod.rs
what: the shared test support module list
@task_file n=23 path=tests/support/terminal.rs
what: the one shared teardown that ends a stand-in terminal's process tree from its leaves, start-time-checked
@task_file n=24 path=tests/cli_open.rs
what: the viola open integration cases; its terminal guard now uses the shared teardown
@task_file n=25 path=tests/cli_up.rs
what: the viola up integration cases — order, placement, skip rules, outcome words, dry run, --json, refusals
@task_file n=26 path=scripts/r4-up.sh
what: the test-only live rig for up's real-window gate (R4), with a rehearsal form; not run live
@task_item n=1 part=touches
The viola --help served-verb rows: the setup group now reads open, run, revive, up, verify, plugin install; rows holding the usage line, group labels or served verbs only are read against it (contract/cli-help/usage-line, contract/cli-help/group-labels, contract/cli-help/served-verbs-only).
@task_item n=2 part=touches
A new refusal code word boot-time-unread (exit 1, its own unable/hint pair, a --json document, a process-exit detail word): rows on the refusal format, unable line, hint line, one hint per code, reason code beside unable and the exit-code item of the refusal output.
@task_item n=3 part=touches
The rule that every error body has a writer and a witness: up adds five refusal documents, two witnessed by existing cases by name and boot-time-unread by its unit pair only, since no case can hide the host boot time.
@task_item n=4 part=touches
The fixed CLI strings pinned by literal and fixed copy from design: the up lines, the boot-time-unread pair and the up not-live hint are new pinned literals (design words carried PROVISIONAL).
@task_item n=5 part=touches
Output-discipline rows (static linear output, no SGR in plain modes, no SGR off a terminal, control characters escaped, no spinners): up prints one plain ASCII line per instance in one write each, no SGR, no progress output while it waits.
@task_item n=6 part=touches
The rule that --json gives one document with no hint: up's --json is one document and its not-live hint is printed only without --json.
@task_item n=7 part=touches
Every refusal ends with a hint versus up's exit 1 from a not-live line: one stderr hint line after all the lines, not per line; an exit 1 from no-session, cwd-missing, session-live or not-taken alone carries no hint.
@task_item n=8 part=touches
CLI exit code rows 21 and the instance-unreachable strict-modes-failed detail: up exits 21 with no line on the board's strict refusal, and 1 with a document when the home tree check after the scan fails.
@task_item n=9 part=touches
Strict-modes refusal before any write: up scans the board strictly before opening its cli log, and its home-tree refusal writes no line.
@task_item n=10 part=touches
Printed names are the parsed arguments: up's named instances pass through the name parser and narrow the fleet; a name not found prints not-taken.
@task_item n=11 part=touches
Paths not carried: up's lines and document carry no address, pid, path or workspace number, only names, code words and a placement word.
@task_item n=12 part=touches
The revive --json refusal words already-live, stale-heartbeat, no-session, cwd-missing and session-live are reused as up's per-instance outcome words, printed as a line rather than a refusal.
@task_item n=13 part=touches
Time-bound rows (time bounds fall on the driver only, never on the human; the user time limit absence): up waits on open's existing live bound per spawn and mints no new bound; the time-bound lists are deliberately not grown or split this session (held on an earlier fork).
@task_item n=14 part=touches
CLI not keyboard-interactive and no interactive input: up asks nothing and raises no prompt; --dry-run prints the plan and spawns nothing.
@task_item n=15 part=touches
Zero flakiness, no retries: a coverage-profile flake in the open and up integration cases was closed by one shared start-time-checked tree teardown, and the TUI fd guard's flake by a settle wait inside its existing deadline; no retry and no assertion moved.
@task_item n=16 part=touches
Boundary tests sequenced on records: the TUI fd guard now re-reads the child's fd table until it settles within the 2 s deadline instead of asserting the first read.
@task_item n=17 part=touches
Error words in the exit-cause list and the process-exit detail list gain boot-time-unread; rows listing the closed refusal word set are read against it.
@task_fact n=1 slug=1-a11y-scope-summary
fact: viola up output: one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, no SGR, one write per line; non-spawn lines first in name order, then spawned or would-open lines in spawn order.
@task_fact n=2 slug=cognitive-accessibility
fact: viola up outcome words are a closed set — live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken; exit 0 when every line is live, already-live, stale-heartbeat or would-open, else 1.
@task_fact n=3 slug=cognitive-accessibility
fact: viola up --json gives one document {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}} and no hint.
@task_fact n=4 slug=cognitive-accessibility
fact: viola up prints one stderr hint line after all its lines when any line reads not-live, and none under --json; no other up line carries a hint.
@task_fact n=5 slug=cognitive-accessibility
fact: boot-time-unread is a refusal code word: exit 1, its own unable/hint pair and a --json document, written when the host boot time cannot be read; witnessed by its unit pair only.
@task_fact n=6 slug=cognitive-accessibility
fact: viola up refuses with exit 21 and no line on the board's strict refusal, and exit 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
@task_item n=1 part=untouched
viola list board rows (caption, order, colour, stale-row dim, empty board, unwrapped separator, NAME truncation): the board's output is unchanged.
@task_item n=2 part=untouched
SGR-specific visual rows (amber on DIALOG, bold on NAME, dim on stale rows, colour decision inputs): up prints no SGR, and no coloured output changed.
@task_item n=3 part=untouched
TUI passthrough byte rows (human keystrokes, vendor content, focus/mouse/resize sequences, viola terminal bytes, vt100 feed panic, run diagnostics off-terminal): viola run's terminal path has no product change.
@task_item n=4 part=untouched
Dialog, send and wait rows (dialog hold, dialog-expired, send refusals, wait timed_out, MCP wait timeout_ms): none moved.
@task_item n=5 part=untouched
link/unlink, verify, plugin install, hook call and MCP frame tool rows: none of those verbs or tools changed.
@task_item n=6 part=untouched
Screen-reader targets and manual keyboard/Orca passes: no manual pass ran and none changed.
@task_item n=7 part=untouched
CI integration and SLO rows (GitHub Actions workflow, coverage suite, no a11y artifact, gate breach fails build): the CI workflow and gate are unchanged.
@task_item n=8 part=untouched
Absences of a web or graphical frontend, touch surface, rendered page and internationalization: still absent.
@task_item n=9 part=untouched
Structured violation JSON and WCAG mapping rows: unchanged.
