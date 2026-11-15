@task_prose part=session
The session removed the status-line interception whole: no per-session settings.json override, no --settings on the child that run starts, no status-line source read, no statusline_command in the snapshot.
The viola hook statusline arm, its /bin/sh -c shell-out with its 5 s deadline, and the hidden hook statusline --capture --shell probe arm are gone; the word statusline now takes the unknown-event exit (0, nothing read or written). With it viola runs no shell at all.
verify's Run B probe settings and the two ledger rows statusline-override and statusline-shell are retired (relied rows 23 to 21); --settings stays a flag of the agent crate, used by verify's Runs C and D alone.
The fake agent loses its statusline-echo mode, --statusline-stdin and its statusline receipt kind; the harness loses boot --statusline-echo; the StatusLine.default-1.json fixtures, their schema and two hook_stdin fuzz seeds are deleted.
The diag-line schema loses the subject statusline-shell, the hook_event word statusline and the field shell_exit_status; the snapshot envelope v stays 1 and readers still skip unknown keys.
New tests pin it: the child launch passes no settings flag, run writes no settings file in the instance dir, the retired hook word is an unknown event that writes nothing, a snapshot holding the retired command reads whole skipping it. CI green 7/7.
@task_file n=1 path=crates/viola-agent-claude/src/statusline.rs
what: the statusline module (deleted): the /bin/sh shell argv, the user status-line command read from the user settings, the settings override document and the probe document
@task_file n=2 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm (deleted): the shell-out of the status-line command under its deadline and the hidden --capture --shell probe arm
@task_file n=3 path=src/cmd/hook.rs
what: viola hook's event dispatch: the hook events it takes, the unknown-event silent exit, the one capture arm
@task_file n=4 path=src/cmd/run.rs
what: run's start path: the status-line source read, the per-session settings override write and its snapshot record (all removed)
@task_file n=5 path=src/run/mod.rs
what: the child launch: the argv handed to the claude child, now --plugin-dir then the caller's args with no settings path
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate root holding the claude flag literals; SETTINGS_FLAG moved here beside PLUGIN_DIR_FLAG
@task_file n=7 path=crates/viola-agent-claude/src/ledger.rs
what: the verify ledger: the relied rows (21), the probes and the stamps' row ids; the statusline-override and statusline-shell rows and ShellVerdict removed
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the hook event names and hook payload parsing of the agent crate
@task_file n=9 path=src/cmd/verify.rs
what: viola verify: the probe runs, Run B's probe settings, the status-line captures and their read cap (removed)
@task_file n=10 path=src/cmd/verify/typed/runs.rs
what: verify's Runs C and D, the remaining callers of --settings
@task_file n=11 path=src/cmd/verify/typed.rs
what: verify's typed probe inputs, losing the statusline settings input
@task_file n=12 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot: its fields, written only by the wrapper and read after the strict-modes check; statusline_command removed, unknown keys skipped on read
@task_file n=13 path=crates/viola-state/src/replay.rs
what: the replayed-snapshot readers
@task_file n=14 path=crates/viola-state/src/strict.rs
what: the strict-modes check over the home tree and trusted state files
@task_file n=15 path=crates/viola-state/src/board.rs
what: the board scan behind viola list
@task_file n=16 path=src/run/version_gate.rs
what: run's version gate reading the verify stamps
@task_file n=17 path=src/run/dialog.rs
what: the wrapper's dialog path and its deadlines
@task_file n=18 path=schemas/diag-line.v1.json
what: the diagnostics log line schema: subjects, hook_event words and the process-exit fields
@task_file n=19 path=schemas/claude-statusline-fixture.v1.json
what: the schema of the recorded StatusLine fixtures (deleted)
@task_file n=20 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded StatusLine fixture holding the workspace.repo field (deleted)
@task_file n=21 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded StatusLine fixture holding the workspace.repo field (deleted)
@task_file n=22 path=fixtures/viola-0.1.0/retired-words.json
what: the viola-format fixture naming the retired words: snapshot key, ledger rows, hook word, source file, diag subject
@task_file n=23 path=fixtures/viola-0.1.0/snapshot.json
what: the viola-format 0.1.0 snapshot fixture, holding the retired statusline_command
@task_file n=24 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook stdin fuzz seed for the statusline event (deleted)
@task_file n=25 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook stdin fuzz seed for the statusline event (deleted)
@task_file n=26 path=src/bin/viola-fake-agent.rs
what: the fake agent test seam: its mode words and options; --settings accepted and ignored
@task_file n=27 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline-echo mode and statusline receipt (deleted)
@task_file n=28 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness boot command and its options
@task_file n=29 path=crates/viola-e2e/src/harness/run.rs
what: the harness's ledger rows and live verify pass at 21 rows
@task_file n=30 path=tests/hook_statusline.rs
what: the hook statusline tests (deleted)
@task_file n=31 path=tests/support/statusline.rs
what: the test support for the statusline hook (deleted)
@task_file n=32 path=tests/hook_fail_open.rs
what: the hook fail-open tests, now with the retired hook word as an unknown event writing nothing
@task_file n=33 path=tests/cli_instance_state.rs
what: the instance state tests: run writes no settings file in the instance dir and leaves the user settings as found
@task_file n=34 path=tests/contract_bounded_readers.rs
what: the bounded-readers census test over every product reader
@task_file n=35 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over recorded fixtures
@task_file n=36 path=tests/contract_os_gated_tests.rs
what: the OS-gated tests census
@task_file n=37 path=tests/security_negatives_strict_modes.rs
what: the strict-modes security negative tests
@task_file n=38 path=.config/nextest.toml
what: the ci kill-line override filter naming test binaries
@task_item n=1 part=touches
Every row whose subject is the status-line interception retires: the statusline command integrity, trust via strict modes, only-from-strict-home, the source read untrusted, source read only, source selection, deadline, payload fails silent, shell-out, capture arm, override plain path, and no statusline command in logs.
@task_item n=2 part=touches
The rows holding the status-line command as the only shell-out (statusline-only-shell-out, single-shell-out) are broken: viola now runs no shell at all, so the exception loses its subject.
@task_item n=3 part=touches
The settings override rows (settings-override-plain-path, statusline-override-plain-path-only) retire: run writes no per-session settings.json and passes the child no --settings.
@task_item n=4 part=touches
The hook unknown-event silent exit: its statusline clause drops; statusline is now one more unknown event word, exit 0, nothing read or written.
@task_item n=5 part=touches
The hook capture arms: two capture arms become one (hook event --capture); the hidden statusline --capture --shell probe arm is gone, so rows naming the two capture arms or the isolated capture arms are set to one.
@task_item n=6 part=touches
Absolute viola path in commands: the hook, MCP and statusline commands become hook and MCP commands only; no statusline command is written.
@task_item n=7 part=touches
The replayed-snapshot rows (inert, yields nothing, fallbacks) and the classified snapshot read: the snapshot loses statusline_command, and hook statusline's replay of it is gone, so the replay readers drop one.
@task_item n=8 part=touches
The shared MAX_FRAME cap: its consumer list loses the status-line readers (the source read and the hook statusline stdin); the bounded-readers census likewise loses them.
@task_item n=9 part=touches
The fixture hygiene and scrub rows (fixture-hygiene, fixture-scrub-and-refusal): their except naming the StatusLine variants' workspace.repo loses its subject; no fixture holds a workspace.repo now.
@task_item n=10 part=touches
The rows on viola writing nothing under the Claude home and in Claude's config: stay true and stronger; run no longer reads the default home's settings.json as a status-line source; a wording naming that read changes.
@task_item n=11 part=touches
The verify probe rows (probe captures transient, probe dirs cleanup, probes confined to own dir, trusted runs): Run B's probe settings and the status-line captures are gone; verify's relied set is 21 rows.
@task_item n=12 part=touches
The snapshot written only by the wrapper and the snapshot strict-checked before trust: the snapshot holds no command to run any more; its reference values narrow.
@task_item n=13 part=touches
The threat model's W1 (the override), W2 (the /bin/sh -c shell-out) and W3 (the two status-line sources) retire, with the data-class and trust-boundary rows that list them.
@task_item n=14 part=touches
The child argv: the child launch takes no settings path; rows describing the child's flags or the session id on argv that list --settings for run are set.
@task_item n=15 part=touches
The test seams of the fake agent: its statusline-echo mode and --statusline-stdin are gone; the remaining mode word is agents.
@task_item n=16 part=touches
The diagnostics never-log rows naming the statusline command or the shell exit: the subject statusline-shell and the field shell_exit_status leave the log schema.
@task_item n=17 part=touches
The parser fuzz scope: the hook_stdin corpus loses its two statusline seeds.
@task_item n=18 part=touches
The protocol version rule: the snapshot envelope and the stamps keep v 1 though the snapshot drops a key and the stamps drop two row ids; readers skip unknown keys.
@task_fact n=1 slug=threat-model-summary
fact: viola has no status-line interception: it writes no settings override, hands the child no --settings, reads no status-line source and runs no status-line command.
@task_fact n=2 slug=input-validation
fact: viola runs no shell at all: every child it starts is spawned directly from an argv, with no /bin/sh -c anywhere.
@task_fact n=3 slug=input-validation
fact: viola hook has one capture arm, hook event --capture; the word statusline maps to no hook event and takes the unknown-event exit 0 with nothing read or written.
@task_fact n=4 slug=input-validation
fact: the snapshot holds no command; a snapshot written by 0.1.0 with statusline_command reads whole, the key skipped, and runs nothing.
@task_item n=1 part=untouched
Dependency security: no dependency, lockfile, toolchain pin, deny policy or workflow changed.
@task_item n=2 part=untouched
The rmcp feature set, the MCP tools and their input checks: no MCP handler or tool changed.
@task_item n=3 part=untouched
The channel socket, its per-user directory, the peer euid checks and the server verification before any frame: unchanged.
@task_item n=4 part=untouched
Paste and send text validation, the control-character sets and the bracketed paste: unchanged.
@task_item n=5 part=untouched
Secret management: the CLAUDE credential strip from the child, the identity floor, the canary scan of CI uploads: unchanged.
@task_item n=6 part=untouched
The pinned binary and plugin folder rows (placement, re-hash, rewrite each start): unchanged.
@task_item n=7 part=untouched
The revive codes, the link and unlink verbs, the wheel and the channel error contracts: unchanged.
@task_item n=8 part=untouched
The Hyprland recorder, the event log rotation and the logs owner-only rows: unchanged.
