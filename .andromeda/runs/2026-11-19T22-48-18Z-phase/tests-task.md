@task_prose part=chunk
Under viola run, a prompt that a CLI-scheduled wake submits is told apart from a prompt the human typed, so a fired wake does not take the wheel, does not make the driver's next send refuse human-typing, and does not route the next question dialog to the human (capability v2-36).
A prompt the human types still takes the wheel; where the means cannot tell a wake from a human prompt, the conservative side is human.
The means is this phase's to find: the CLI sends no source field on a fired wake's UserPromptSubmit, so the candidates rest on the wrapper's own knowledge (the pty bytes from the human's side, the turn end's scheduled_wakes count) and possibly a relied ledger row with a verify probe.
It carries three open items: a third live reading of the placeholder hint's cell attribute, the intermittent rig rehearsal red acted on only at its next red, and the watch on the harness boot-timeout test's 10 s wall bound, read at each run with no bound moved.
@task_item n=1 part=needs
The wheel's origin decision for a submitted prompt (human, driver, harness): a prompt a fired scheduled wake submits must not be filed origin human nor move the wheel to human, and the wheel/human-input record must not be written for it.
@task_item n=2 part=needs
Human input still takes the wheel: a prompt the human types, and human editing keys, move the wheel to human exactly as before; the human is never blocked by the change.
@task_item n=3 part=needs
The conservative default: when the wrapper cannot tell a wake from a human prompt (no scheduled wake known, human bytes seen, an unreadable signal), the prompt is filed origin human and the wheel goes to human.
@task_item n=4 part=needs
The driver send path after a fired wake: the driver's next send is not refused human-typing (CLI send exit 10, the human-typing refusal on CLI, --json and MCP send) and is confirmed by its prompt-submitted record with origin driver; the send refusal order and the confirmation window stay as they are.
@task_item n=5 part=needs
Question dialogs after a fired wake: the next question dialog is held for the driver (its decision body served to the driver on a stamped CLI) rather than routed to the human; the dialog hold, its expiry and the one-pending rule apply unchanged.
@task_item n=6 part=needs
The normalised prompt-submitted event and its origin field: what origin a wake's prompt carries, and whether the event line or its data gain a value or key; any new value must stay readable by forward-compatible event log readers and a closed enum changes only by decision.
@task_item n=7 part=needs
The UserPromptSubmit hook payload parse: the CLI schema declares an optional source field (schedule_wakeup, loop_wakeup) that live readings on 2.1.325 and 2.1.326 never carried; the parse must tolerate its absence, its presence and any unknown field, and the hook must still fail open within its spine deadline.
@task_item n=8 part=needs
The turn-ended event's scheduled_wakes count (and the Stop event data carrying it) as a possible input to the wake decision: a wake known scheduled at the last turn end.
@task_item n=9 part=needs
The wrapper's pty pump that reads the human's keys: whether any byte came from the human's side before a prompt was submitted while a wake was scheduled; terminal replies, focus, mouse and resize events are not human editing and never take the wheel.
@task_item n=10 part=needs
Unverified or older CLI versions: what they get from the wake means (the unverified-CLI gate degrades to send only and gates answer; the version gate needs every relied ledger row), so an unverified CLI stays on the conservative human side.
@task_item n=11 part=needs
If the means rests on a relied capability ledger row: every ledger row has a verify probe, stamps come only from viola verify against the fake agent, stamps refuse a widened ledger, and the recorded fixture set per CLI version is replayed.
@task_item n=12 part=needs
The fake agent as the rehearsal surface for a fired wake: a turn-script or control step that submits a prompt with no human key and fires UserPromptSubmit, held to the recorded fixtures and configured by argv alone; a fake-agent run is no proof of real-CLI behaviour.
@task_item n=13 part=needs
The wheel's return rules the change must stay inside: the wheel returns to the driver only through viola release or a start, a manual pause forces the human wheel, and a harness-injected turn keeps the wheel.
@task_item n=14 part=needs
Verdicts on the wheel, the send and the held dialog are asserted on logged events and records (wheel, prompt-submitted, send-confirmed, dialog events), never on the child's rendered screen.
@task_item n=15 part=needs
Live readings stay local and off CI: the real claude CLI is never run in CI; this phase makes no live start and reads no operator config, and any live reading is recorded as a reading, never as verification.
@task_item n=16 part=needs
The placeholder hint's cell attribute carry: a third reading on the standing live rig needs a fresh box, keys typed before any send and no send after them, reads every drawn hint cell past the cursor's attributes only, never text; it is recorded as a live rig hint-cell reading.
@task_item n=17 part=needs
The intermittent rig rehearsal red (box read empty after x and del): acted on only at its next red, its cause named from the printed rig summary and the kept after-del frame before the wait is mended; no retry rule, no loosened wait, no sleep.
@task_item n=18 part=needs
The harness boot-timeout watch: the test that a boot never getting ready times out and stops its supervisor is read at each run against its 10 s wall bound; the boot deadline is no performance budget and no bound is moved.
@task_item n=19 part=needs
The wake's prompt text is user content: no log line, diagnostics file or fixture written for the origin decision carries prompt text, and any new log line is codes-only with detail apart.
@task_standing n=1
convention: each new test follows the test function naming rule, table-driven unit cases in rstest, inline unit tests, a root integration binary prefixed by its surface, one shared copy of each helper, and the layer selected by nextest filterset
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test creates and owns its own private home under the e2e home root, seeds its own data, and keeps time behind the injected clock seam
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new fixture or fake-agent recording is synthetic, scrubbed of home paths and usernames, carries no real prompt, and passes the fixture hygiene walk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: each new test waits on logged events, never on sleeps, has no retries, asserts literal oracles on the contract, and runs in parallel with no shared mutable state
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new E2E test runs over the fake agent with the real viola hook, holds a long-lived wrapper in a drop guard, and asserts only cross-process behaviour through records
home: 6-e2e-test-strategy · e2e-test-strategy
@task_standing n=6
convention: each new log line is a structured JSON process log line with its event, corr and level fields, codes only, no secrets
home: log-format
@task_item n=1 part=not_needed
The revive verb: its preflight order, refusal codes, exit codes and --json documents; a fired wake changes nothing on revive.
@task_item n=2 part=not_needed
The viola list board and list --json rows: columns, liveness and staleness, skipped counts, control-character escaping of names.
@task_item n=3 part=not_needed
The wrapper channel's link and unlink methods, link records and link idempotence; the wake decision makes and drops no link.
@task_item n=4 part=not_needed
Start-time security refusals: strict-modes refusals, socket directory checks, server identity checks, squatted endpoints, pinned-executable hash and widened home tree at start.
@task_item n=5 part=not_needed
The MCP server's protocol framing: tools/list tool set, invalid-params field rows, stdin line cap and oversize-before-initialize, unknown tool and internal fault replies.
@task_item n=6 part=not_needed
Local command sends: /clear and /compact confirmation by session, ledger-listed local commands and their unconfirmable rule.
@task_item n=7 part=not_needed
Sent text validation: control characters in pasted text, empty text, typed framing of multi-line text, inner CR and trailing newlines.
@task_item n=8 part=not_needed
Snapshot replay and torn-append healing of the event log and instance snapshot, single writers of state files.
@task_item n=9 part=not_needed
Plugin install and marketplace rows, the generated plugin's file modes; the hook commands the plugin lists are unchanged by a wake.
@task_item n=10 part=not_needed
The harness agent-run status document, session record, pid file and supervisor exit file shapes.
@task_item n=11 part=not_needed
Declared absences: a browser suite, Windows and macOS support, a database, outbound HTTP, a deploy stage.
@task_item n=12 part=not_needed
The CI workflow's shape: actions pinning, workflow permissions, artifact names and retention, weekly advisory runs and nightly fuzzing; the chunk adds no CI job or workflow.
@task_item n=13 part=not_needed
The child environment strip and the claude child's process-start record of kept and stripped variables.
