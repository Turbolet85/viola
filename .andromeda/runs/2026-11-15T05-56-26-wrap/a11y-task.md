@task_prose part=session
The session removed the status-line interception whole: viola no longer reaches into the child's status line at all.
Gone: the per-session settings override and the child's --settings, the status-line source read, the snapshot's statusline_command, the viola hook statusline arm with its /bin/sh -c shell-out and its 5 s deadline, and the hidden hook statusline --capture --shell probe arm.
The word statusline now maps to no hook event: viola hook statusline takes the unknown-event exit (0, nothing read or written); the hook keeps one capture arm and nine events.
Verify's relied ledger rows went 23 to 21 (statusline-override and statusline-shell retired); its step lines read [NN/21] and the summary 21 pass.
The fake agent lost its statusline-echo mode, the harness lost boot --statusline-echo, and the log lost the subject statusline-shell, the hook_event word statusline, the field shell_exit_status and the spans hook.statusline and statusline.shell_out.
With the shell-out gone viola runs no shell at all; every test of the interception went, including the TUI statusline passthrough case; a wrapped session's status line is now exactly the unwrapped one by construction.
@task_file n=1 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm — the status-line hook that ran the user's command, passed its output byte for byte and held a 5 s deadline (deleted)
@task_file n=2 path=crates/viola-agent-claude/src/statusline.rs
what: the status-line shell-out and settings-override builder — the /bin/sh -c call and the override document (deleted)
@task_file n=3 path=src/cmd/hook.rs
what: viola hook — the event and dialog hook calls, their event-word dispatch and the unknown-event silent exit 0
@task_file n=4 path=tests/hook_statusline.rs
what: the viola hook statusline suite — its exit status, byte-for-byte output and deadline cases (deleted)
@task_file n=5 path=tests/support/statusline.rs
what: the test support for the status-line hook cases (deleted)
@task_file n=6 path=tests/hook_fail_open.rs
what: the hook fail-open suite — the hook calls' exit and write-nothing witnesses, now holding the retired-word unknown-event case
@task_file n=7 path=tests/tui_passthrough.rs
what: the viola run TUI passthrough boundary suite — zero viola bytes on the terminal, human keystrokes, vendor content; lost its statusline passthrough case
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the fake agent the boundary tests drive — its mode words and options (statusline-echo and --statusline-stdin gone)
@task_file n=9 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline-echo mode and its statusline receipt (deleted)
@task_file n=10 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the e2e harness CLI — boot and its options (--statusline-echo gone)
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the harness run — its live verify pass reading, now 21 ledger rows and [NN/21] step lines
@task_file n=12 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot of a wrapped session in a test home
@task_file n=13 path=crates/viola-e2e/src/harness/session.rs
what: the harness session options
@task_file n=14 path=crates/viola-agent-claude/src/ledger.rs
what: the viola verify ledger — the relied rows each verify step line names, now 21
@task_file n=15 path=crates/viola-agent-claude/src/lib.rs
what: the claude flag literals of the agent crate (--settings, --plugin-dir)
@task_file n=16 path=src/cmd/verify.rs
what: viola verify — its step lines, summary and recorded variants
@task_file n=17 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs B, C and D (Run B's status-line probe settings gone)
@task_file n=18 path=src/cmd/run.rs
what: viola run start — no status-line source read, no settings override write, no snapshot status-line command
@task_file n=19 path=src/run/mod.rs
what: the child launch of viola run — the child's argv, now --plugin-dir then the caller's args
@task_file n=20 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot the wrapper writes and readers replay (statusline_command gone)
@task_file n=21 path=schemas/diag-line.v1.json
what: the run diagnostics log-line schema — subjects, hook_event words, process-exit fields
@task_file n=22 path=tests/cli_verify.rs
what: the viola verify output suite — step lines and the summary line, now 21 rows
@task_file n=23 path=tests/cli_verify/runs.rs
what: the viola verify run cases and their summary counts
@task_file n=24 path=tests/cli_instance_state.rs
what: the viola run instance-state suite — the instance dir and the user settings left as found
@task_file n=25 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract check — its status-line rows gone
@task_file n=26 path=tests/contract_os_gated_tests.rs
what: the OS-gated tests list — its status-line rows gone
@task_file n=27 path=tests/contract_diag_schema.rs
what: the diagnostics log-schema contract check
@task_file n=28 path=tests/contract_fake_agent_drift.rs
what: the fake-agent drift check against recorded fixtures
@task_file n=29 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk
@task_file n=30 path=tests/cli_fake_agent.rs
what: the fake agent suite — the hangup case's Stop hook now a plain absolute command
@task_file n=31 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded StatusLine fixture of claude 2.1.287 (deleted)
@task_file n=32 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded StatusLine fixture of claude 2.1.302 (deleted)
@task_file n=33 path=schemas/claude-statusline-fixture.v1.json
what: the StatusLine fixture schema (deleted)
@task_file n=34 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook stdin fuzz seed for the status-line hook (deleted)
@task_file n=35 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook stdin fuzz seed for the status-line hook (deleted)
@task_file n=36 path=fixtures/viola-0.1.0/retired-words.json
what: the retired-words fixture — the retired hook word, snapshot key, ledger rows, source file and log subject
@task_file n=37 path=fixtures/viola-0.1.0/snapshot.json
what: the 0.1.0 instance snapshot fixture, holding the retired statusline_command key
@task_file n=38 path=Cargo.toml
what: the workspace manifest and its test targets (hook_statusline gone)
@task_file n=39 path=.config/nextest.toml
what: the nextest profiles and the ci kill-line filter (binary(hook_statusline) gone)
@task_item n=1 part=touches
contract/viola-hook-statusline/exit retires whole: there is no viola hook statusline arm; the word statusline takes the unknown-event exit 0 and reads and writes nothing.
@task_item n=2 part=touches
decision/statusline-output-passed-byte-for-byte retires whole: viola no longer runs or passes through any status-line output; the child's status line is the user's own, untouched.
@task_item n=3 part=touches
decision/time-bounds-never-on-human loses the status-line hook's 5 s deadline (STATUSLINE_DEADLINE) from whatever it lists or excepts.
@task_item n=4 part=touches
decision/time-bounds-fall-on-the-driver loses the status-line deadline from its list of bounds; the dialog hold and the driver's bounds stay.
@task_item n=5 part=touches
absence/user-time-limit and decision/no-unwarned-timeouts: any clause naming the status-line command's deadline or its kill loses its subject.
@task_item n=6 part=touches
decision/wrapped-tui-reads-as-unwrapped: the status line now reads exactly as unwrapped by construction (no settings layer, no hook); a clause naming the wrapped status line or its passthrough is stale; the live wrapped-vs-unwrapped status-line reading is owed as a carry.
@task_item n=7 part=touches
decision/zero-viola-bytes-on-the-terminal, contract/tui-passthrough/viola-terminal-bytes, contract/tui-passthrough/own-bytes and contract/tui-passthrough/vendor-content: any except or note for status-line output written through viola's hook loses its subject; the TUI statusline passthrough case left the suite.
@task_item n=8 part=touches
decision/single-vendor-content-boundary: the status-line output was a second path of user or vendor content through viola; that path is gone.
@task_item n=9 part=touches
decision/control-character-escaping, decision/c0-c1-escaped and decision/control-bytes-escaped: any exception for status-line bytes passed unescaped loses its subject.
@task_item n=10 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: the log subject statusline-shell, the hook_event word statusline, the field shell_exit_status and the spans hook.statusline and statusline.shell_out are gone.
@task_item n=11 part=touches
contract/viola-verify/step-line and contract/viola-verify/interrupted-pair: verify now reads 21 relied rows, step lines [NN/21], summary 21 pass; the ids statusline-override and statusline-shell left the ledger order — touched only where a cell holds 23 or those ids.
@task_item n=12 part=touches
contract/hook-call/server-verify-failed, contract/hook-call/socket-dir-refused and contract/hook-call/strict-modes-failed: the hook keeps the event and dialog calls alone, nine events and one capture arm; any clause naming the statusline arm or two capture arms is stale.
@task_item n=13 part=touches
decision/boundary-tests-sequenced-on-records and decision/boundary-check-never-inspects-claude-content: the fake agent's statusline-echo mode and statusline receipt kind and the harness's boot --statusline-echo are gone; a row naming them as a boundary driver is stale.
@task_item n=14 part=touches
decision/live-readings-never-gate: a live wrapped-vs-unwrapped status-line reading is now owed as a route carry, gating nothing.
@task_item n=15 part=touches
decision/tui-boundary-assertions-only and decision/tui-boundary-only: the TUI boundary viola touches shrank — no status-line hook, no settings layer on the child.
@task_item n=16 part=touches
decision/screen-reader-target-tui and decision/manual-orca-pass: a manual check of the wrapped status line read by Orca, if named, now reads the user's own status line unchanged.
@task_item n=17 part=touches
contract/log-identity/process: a process tag for the viola hook statusline process, if listed, is freed.
@task_fact n=1 slug=keyboard-test-harness
fact: viola adds no settings layer to the child and writes nothing into its status line: a wrapped session's status line is the user's own, exactly as unwrapped, by construction.
@task_fact n=2 slug=cognitive-accessibility
fact: viola hook statusline is no hook event: the word takes the unknown-event exit 0, reading and writing nothing, and holds no deadline.
@task_fact n=3 slug=1-a11y-scope-summary
fact: viola runs no shell and intercepts no status line: no status-line command run, timed or passed through.
@task_item n=1 part=untouched
Colour and SGR rows (dialog amber, name bold, stale-row dim, word beside every SGR, no colour on failure words): nothing of colour moved.
@task_item n=2 part=untouched
The viola list board, its caption, order, empty board, unwrapped separator and name truncation: the board is unchanged.
@task_item n=3 part=untouched
viola revive refusals and revive --list, and viola link and unlink output and exit codes: none changed.
@task_item n=4 part=untouched
viola --help rows (usage line, group labels, handoff row, served verbs): no verb was added or removed from help; the hook statusline arm was a hook event, not a verb.
@task_item n=5 part=untouched
CLI refusal layout, hint and code-word rows, exit codes 2 and 10 to 21, the --json failure and fault documents: no refusal, code or exit code moved.
@task_item n=6 part=untouched
viola send, wait and dialog answer rows (input-not-ready, no-prompt-submitted, timed_out, dialog-expired, dialog_hold_s, MCP wait timeout_ms): the dialog hold and wait bounds are unchanged.
@task_item n=7 part=untouched
Human keystrokes, the wheel, the stdin non-editing list and focus, mouse and resize sequences: the wheel and keystroke path are unchanged.
@task_item n=8 part=untouched
CI integration, SLO invariants, anti-pattern and tier rows, structured violation JSON, the screen-reader setup rows and the web, touch and i18n absences: no gate, budget or tool changed.
