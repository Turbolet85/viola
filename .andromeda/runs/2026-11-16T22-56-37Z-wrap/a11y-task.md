@task_prose part=session
The session added one number to a turn end: the turn-ended event data gains the key scheduled_wakes, the count of the session's scheduled wakes, beside background_tasks.
The count is the length of the claude Stop payload's session_crons array alone; no element is read and no wake content crosses; it is null when absent, null or not an array, and null on an API-error (StopFailure) turn end.
It reaches the hops background_tasks reaches and no others: the turn-ended event line, CLI wait --json and the MCP wait tool's structured content; not last, list, the snapshot or the replay.
No human-readable output changed: no CLI line, caption, refusal, hint, colour or terminal byte; wait still wakes on a turn end exactly as before.
No new event kind, IPC method, endpoint, socket, env var, config key, crate or dependency; nothing retired; the live reading of a real wake was not run and stays a carry on the standing live rig entry.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the internal core crate's event kind table, the closed list of each event kind's data keys; turn-ended now names four keys
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the claude hook normaliser, the single vendor-content boundary that maps a claude Stop or StopFailure hook payload to turn-ended event data, with its unit tests
@task_file n=3 path=src/run/wait.rs
what: the wait job, which events wake a parked viola wait driver, with its unit tests
@task_file n=4 path=crates/viola-state/tests/state_events.rs
what: the state crate's event-log suite, the check that a turn-ended data key is a known key
@task_file n=5 path=crates/viola-mcp/tests/handlers.rs
what: the MCP server tool handler suite, the MCP wait tool's structured content
@task_file n=6 path=tests/hook_events.rs
what: the hook-event integration suite, a booted wrapper fed recorded claude hook payloads, with the no-content scan of every home file
@task_file n=7 path=tests/cli_wait_last.rs
what: the viola wait and viola last CLI suite, wait --json output of a turn end
@task_file n=8 path=tests/cli_wheel.rs
what: the human-takes-the-wheel CLI suite; one line added to a recorded turn-end fixture it feeds
@task_item n=1 part=touches
decision/single-vendor-content-boundary: the claude hook boundary now reads one more vendor field, session_crons, as a length only; no vendor content crosses, the rule holds and gains a second count-only field.
@task_item n=2 part=touches
decision/critical-path-cli-board-output: the wait/last output critical path; wait --json of a turn end now carries scheduled_wakes, while the human wait and last output is unchanged.
@task_item n=3 part=touches
contract/viola-wait/waiting-line and contract/viola-wait/timed-out: viola wait output; the waking rule and its human lines are unchanged, only the --json turn-end document gains a key.
@task_item n=4 part=touches
contract/mcp-wait-tool/timeout-ms: the MCP wait tool; its structured content of a turn end now carries scheduled_wakes unchanged, its timeout bound untouched.
@task_item n=5 part=touches
decision/time-bounds-fall-on-the-driver and decision/no-unwarned-timeouts: a wait driver's bound; wait wakes on a turn end whatever its scheduled_wakes, no bound moved and no timer added.
@task_item n=6 part=touches
decision/no-assertions-for-internal-crates: the core crate's event kind table changed; its pin is a crate test, not an a11y assertion.
@task_item n=7 part=touches
decision/a11y-inside-existing-suites and decision/a11y-runs-in-existing-test-suites: seven new tests landed inside existing suites; no new suite, driver or a11y artifact.
@task_item n=8 part=touches
decision/live-readings-never-gate: the live reading of a fired scheduled wake was not run and is carried on the standing live rig entry; nothing gates on it.
@task_item n=9 part=touches
decision/boundary-check-never-inspects-claude-content and decision/no-screen-content-parsing: the new field is read from the hook payload, never from the screen, and its elements are not inspected.
@task_item n=10 part=touches
decision/paths-not-carried and decision/control-character-escaping: the new value is a bare integer or null, carrying no path and no printable text to escape.
@task_item n=11 part=touches
decision/fixed-cli-strings-pinned and decision/fixed-cli-strings-pinned-by-literal: the wait and wheel suites' literal pins of human output still hold unchanged; only a fixture line was added.
@task_item n=1 part=untouched
The board and list rows (caption, order, colour, stale row, unwrapped separator, NAME truncation): no list output changed and the count does not reach list.
@task_item n=2 part=untouched
Refusal, fault and exit-code rows (unable and hint lines, code words, exit codes 2 to 21, --json failure documents, instance-unreachable details): no refusal or code word was added or moved.
@task_item n=3 part=untouched
The TUI passthrough rows and the keyboard harness rows (human keystrokes, the wheel, terminal bytes, focus, mouse and resize sequences, vt100 feed panic): no terminal byte or stdin rule changed.
@task_item n=4 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale, no SGR off a terminal, word beside every SGR): no styled output changed.
@task_item n=5 part=untouched
Screen-reader and manual pass rows (Orca targets, manual keyboard pass), and the absences (web frontend, touch, i18n, user time limit): the session reaches none.
@task_item n=6 part=untouched
CI and gate rows (fail-fast gate, zero flakiness, coverage and doctest, GitHub Actions workflow, structured violation JSON): no gate, threshold or CI leg changed.
@task_item n=7 part=untouched
viola revive, link, unlink, plugin install, verify and help rows: none of those commands or their output changed.
