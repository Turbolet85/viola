@task_prose part=session
The session removed the status-line interception whole: viola no longer writes a per-session settings.json override, no longer passes --settings to the child at start, no longer reads a status-line source, and no longer records statusline_command in the instance snapshot.
The viola hook statusline arm, its /bin/sh -c shell-out with its 5 s deadline, and the hidden hook statusline --capture --shell probe arm are gone; the word statusline now maps to no hook event and takes the unknown-event exit (0, nothing read or written). With that, viola runs no shell at all.
verify loses Run B's probe settings and the two ledger rows statusline-override and statusline-shell: the relied set goes from 23 to 21 rows, and the stamps drop those two row ids. --settings stays a flag of the agent crate, used only by verify's Runs C and D.
The fake agent loses its statusline-echo mode, --statusline-stdin and its statusline receipt kind; it still accepts --settings and ignores the value. The harness loses boot --statusline-echo. The StatusLine fixtures, their schema and two hook_stdin seeds are deleted. The diag-line schema loses the subject statusline-shell, the hook_event statusline and the field shell_exit_status.
Every own-format v stays 1, as the founder decided: the snapshot reader skips the retired key, and a stamps file holding the retired row ids reads on the relied rows alone.
@task_file n=1 path=crates/viola-agent-claude/src/statusline.rs
what: the agent crate's status-line module (deleted): the user's statusLine command, the per-session settings override document, the /bin/sh -c shell argv
@task_file n=2 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm (deleted): the statusline wrapper, its shell-out under the statusline deadline and its --capture --shell probe arm
@task_file n=3 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline-echo mode and statusline-stdin replay (deleted)
@task_file n=4 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded StatusLine fixture variant of the claude CLI 2.1.287 (deleted)
@task_file n=5 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded StatusLine fixture variant of the claude CLI 2.1.302 (deleted)
@task_file n=6 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook_stdin fuzz corpus seed for the statusline payload (deleted)
@task_file n=7 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook_stdin fuzz corpus seed for the statusline payload (deleted)
@task_file n=8 path=schemas/claude-statusline-fixture.v1.json
what: the schema of the recorded StatusLine fixtures (deleted)
@task_file n=9 path=tests/hook_statusline.rs
what: the integration test target of viola hook statusline (deleted)
@task_file n=10 path=tests/support/statusline.rs
what: the test-support helpers for the statusline tests (deleted)
@task_file n=11 path=crates/viola-agent-claude/src/ledger.rs
what: the per-behaviour CLI capability ledger: its rows, the relied set (now 21), the probes and the capture file names
@task_file n=12 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate root holding the claude flag literals (--plugin-dir and, moved here, --settings)
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's tests pinning the relied-row verdict vector
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamps tests, now including a stamps file that holds the two retired row ids
@task_file n=15 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger's probe-run tests (its statusline cases removed)
@task_file n=16 path=crates/viola-agent-claude/src/hook.rs
what: the agent crate's hook payload shapes and hook-to-kind mapping
@task_file n=17 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot and its writer and reader (statusline_command dropped; unknown keys skipped on read)
@task_file n=18 path=src/cmd/hook.rs
what: the viola hook subcommand: the event, dialog and capture arms and the unknown-event exit
@task_file n=19 path=src/cmd/run.rs
what: viola run's start path (the status-line source read, the override write and the snapshot record removed)
@task_file n=20 path=src/run/mod.rs
what: the wrapper's child launch: the flags passed to the child (now --plugin-dir then the caller's args)
@task_file n=21 path=src/cmd/verify.rs
what: viola verify, the sole stamps writer: its probe runs and --record fixture variants
@task_file n=22 path=src/cmd/verify/typed.rs
what: verify's typed probe inputs (the statusline settings input removed)
@task_file n=23 path=src/cmd/verify/typed/runs.rs
what: verify's probe runs B, C and D (Run B's probe settings removed; C and D still pass --settings)
@task_file n=24 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary: its mode words, options and receipt kinds
@task_file n=25 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the subject enum, the hook_event enum and the process-exit fields
@task_file n=26 path=fixtures/viola-0.1.0/retired-words.json
what: the fixture of retired words read by the compatibility tests (the retired snapshot key, ledger rows, hook word, source file and diag subject)
@task_file n=27 path=fixtures/viola-0.1.0/snapshot.json
what: the instance snapshot fixture in the 0.1.0 shape, still holding statusline_command
@task_file n=28 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness test binary's CLI (boot loses --statusline-echo)
@task_file n=29 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot of a wrapped session (the statusline-echo planting removed)
@task_file n=30 path=crates/viola-e2e/src/harness/session.rs
what: the harness session options
@task_file n=31 path=crates/viola-e2e/src/harness/run.rs
what: the harness gate runner: its LEDGER_ROWS and LIVE_PASS expectations (21 rows)
@task_file n=32 path=.config/nextest.toml
what: the test runner config: the ci kill-line override filter's test binaries
@task_file n=33 path=Cargo.toml
what: the workspace manifest's [[test]] target list (hook_statusline removed)
@task_file n=34 path=tests/hook_fail_open.rs
what: the hook fail-open tests, now including the retired hook word read as an unknown event that writes nothing
@task_file n=35 path=tests/cli_instance_state.rs
what: the instance-state tests: run writes no settings file and leaves the user settings as found
@task_file n=36 path=tests/cli_verify/runs.rs
what: verify's run tests and their summary counts (21 rows)
@task_item n=1 part=touches
decision/no-shell-out: its except (the statusline command as the one shell-out) loses its subject. viola now runs no shell at all, so the except is dropped and the rule stands absolute.
@task_item n=2 part=touches
Retire the statusline decisions: decision/statusline-only-shell-out, decision/statusline-deadline (the 5 s STATUSLINE_DEADLINE), decision/statusline-hook-exempt, decision/statusline-wrapper-not-a-hook, decision/statusline-check-before-log, decision/refused-statusline-check-no-trace and decision/statusline-wrapped-per-session.
@task_item n=3 part=touches
Retire the status-line source rows: decision/statusline-source-in-viola-home, decision/statusline-source-precedence, decision/statusline-source-follows-home and contract/statusline-source/command. viola now reads no statusline-source.json and no statusLine of the user's settings.
@task_item n=4 part=touches
Retire the settings-override rows: decision/per-session-settings-override, decision/settings-override-safe-path-only, contract/settings-override/plain-path-only, contract/settings-override/status-line and contract/instance-settings-json/statusline. A start now writes no settings.json in the instance dir.
@task_item n=5 part=touches
Child argv: retire contract/child-argv/settings and contract/claude-child-argv/settings, because a start hands the child no --settings. The rows contract/child-argv/plugin-dir, contract/child-argv/caller-arguments and their claude-child-argv twins now read --plugin-dir then the caller's args.
@task_item n=6 part=touches
decision/claude-flag-literals-in-agent-crate: --settings stays a flag of the agent crate, now at the crate root beside --plugin-dir. Its callers are verify's Runs C and D alone; run is no longer one. Change the wording only where the row names run.
@task_item n=7 part=touches
Instance snapshot: retire contract/instance-snapshot/statusline-command and narrow contract/instance-snapshot/data. The snapshot drops statusline_command, and the reader still skips unknown keys. Re-check contract/instance-snapshot/writer and contract/instance-snapshot/spawn-order and decision/run-start-order wherever they list the override write or the source read.
@task_item n=8 part=touches
decision/protocol-version-bump-rule: its except is widened to this removal (the snapshot's statusline_command and the stamps' two row ids), and every own-format v stays 1, on the founder's word. contract/snapshot-envelope/v and contract/protocol-version/v hold v at 1.
@task_item n=9 part=touches
viola hook statusline: retire contract/hook-statusline/capture, contract/hook-statusline/exit, contract/hook-statusline/refused-check, contract/hook-statusline/stdout, contract/hook-statusline-output/stdout and contract/hook-to-kind-map/hook-statusline. The word statusline now takes the unknown-event exit 0 and reads and writes nothing.
@task_item n=10 part=touches
The name name/other/hook-statusline is freed. The hook takes nine events and no tenth word. Check name/command/hook and contract/viola-hook-exit-codes/0 wherever they name statusline.
@task_item n=11 part=touches
Capture arms go from two to one: decision/probe-capture-arms-inert and contract/hook-capture-arm/exit and contract/hook-capture-arm/payload-file name a single arm, hook <event> --capture. Retire contract/probe-capture/statusline-shell-file and contract/statusline-capture-shell-file/verdict.
@task_item n=12 part=touches
Relied ledger rows go from 23 to 21. The row ids statusline-override and statusline-shell and ShellVerdict are gone. This reaches decision/cli-capability-ledger, decision/capability-ledger-gate, decision/version-verified-all-relied-rows, contract/ledger-stamps/rows and contract/viola-verify-output/step-line and contract/viola-verify-output/summary-line wherever they name those ids or the count 23.
@task_item n=13 part=touches
verify: Run B's probe settings, the typed/statusline captures and the --record status-line variants are gone. This reaches decision/verify-child-set, decision/verify-record-only-clean, decision/verify-probe-dirs-drop-guarded and decision/verify-run-e-resumes-run-b-first-session wherever they name Run B's settings or the statusline captures.
@task_item n=14 part=touches
Hook output and fail-open rules lose their statusline clause: decision/hook-output-one-crate, decision/hooks-fail-open, decision/hook-fails-open, decision/hook-exits-on-untrusted-instance, decision/hook-without-trusted-instance-exits-silently, contract/hook-output/stdout and decision/hook-never-writes-stderr, wherever they except or name the statusline wrapper.
@task_item n=15 part=touches
Snapshot replay readers go from 6 to 5, because the statusline hook's serve is gone. This reaches decision/hook-fails-open-on-replayed-snapshot, decision/snapshots-read-through-replay and decision/replayed-snapshot-liveness wherever they list hook statusline as a reader.
@task_item n=16 part=touches
Fixtures: the StatusLine fixture family, its schema and their workspace.repo leave. decision/recorded-fixtures-scrubbed and decision/fixture-keeps-own-repository-remote lose any except naming the StatusLine variants' workspace.repo.
@task_item n=17 part=touches
Diagnostics: the diag-line subject statusline-shell, the hook_event statusline and the process-exit field shell_exit_status are removed (deadline and deadline_hit stay). This reaches contract/diag-line/format, contract/diagnostics-lines/process-log-line, contract/diagnostics-role-files/hook-name and decision/codes-only-process-logs wherever they name them.
@task_item n=18 part=touches
decision/bounded-readers and decision/os-gated-tests-named and decision/os-gated-tests-name-reason: the statusline reader caps (SHELL_CAP, the statusline deadline) and the OS-gate rows naming the statusline tests are gone.
@task_item n=19 part=touches
Fake agent: the mode word statusline-echo and the option --statusline-stdin are gone, so the mode words go from 2 to 1 (agents stays). --settings is still accepted and ignored. This reaches decision/fake-agent-test-seams and decision/fake-agent-after-double-dash wherever they list them.
@task_item n=20 part=touches
decision/no-tmux-or-shell-dependency now holds without exception, since viola invokes no /bin/sh. decision/global-statusline-untouched stays true and becomes stronger, because viola adds no settings layer at all. Flag it only if its except names the override.
@task_item n=21 part=touches
decision/no-live-claude-in-ci and decision/tests-against-fake-agent: the harness boot loses --statusline-echo, and the ci kill-line filter loses the hook_statusline binary. Check them only where a row names those.
@task_fact n=1 slug=established-decisions
fact: viola intercepts no status line: it writes no settings layer for a session, hands the child no --settings at start, reads no user statusLine source and runs no status-line command, so a wrapped session's status line is exactly as unwrapped.
@task_fact n=2 slug=conventions
fact: The retired hook word statusline maps to no hook event: viola hook statusline takes the unknown-event exit 0 and reads and writes nothing.
@task_fact n=3 slug=standard-contracts
fact: A ledger stamps file that still holds the retired row ids statusline-override and statusline-shell reads verified on the 21 relied rows alone; a snapshot still holding statusline_command reads whole, with that key skipped.
@task_item n=1 part=untouched
The MCP server, its tools, list results and exit codes: no MCP surface moved.
@task_item n=2 part=untouched
The wheel, send, paste and confirmation decisions and the refusal reasons and order: no send or wheel behaviour changed.
@task_item n=3 part=untouched
The wrapper channel's JSON-RPC methods, frames, MAX_FRAME and fault codes, and the event kinds and event data: no channel or event shape changed.
@task_item n=4 part=untouched
The Hyprland seam, the PTY seam and the vt100 readiness gate.
@task_item n=5 part=untouched
The board and list payloads, session liveness and the revive refusals and their codes.
@task_item n=6 part=untouched
The dependency policy and crate graph: no crate or dependency was added or removed, and the lockfile did not change.
@task_item n=7 part=untouched
The pinned copy, plugin install, socket-dir checks, strict-modes rules and the CLI exit-code tables of the frame verbs.
