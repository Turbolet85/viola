@task_prose part=session
A live round against the real claude CLI 2.1.325, run locally on a test home: no product behaviour, IPC method, event kind, flag, config key, crate or dependency changed.
It measured the dialog hold at its 3600 s ceiling live: the hook raised a question, the wrapper wrote dialog-expired after 3600013 ms with the hook still waiting, and the founder ruled the 3600 s ceiling stands; its value is unchanged and only its PROVISIONAL qualifier left the constant's doc comment.
It stamped 2.1.325 by a live verify on a test home only (21 of 21 relied ledger rows pass); the live home is not stamped, nothing was placed on 2.1.325, and the verify's fixture recording was refused whole because the test home's directory was named home and the absolute-path scrub caught /home/ in a payload.
It measured that the real CLI erases the input box on DEL (the fake agent keeps a typed DEL), that 2.1.325's folder-trust dialog has unnumbered options with the selector on the non-trust row, and that on a start whose trust keys pass through the wrapper a wheel human-input line precedes session-start and a later human key adds no wheel line.
Matrix v2-03 is not verified this round and is to be un-claimed; v2-02 (the hold near the ceiling) is met. The round's rigs are evidence scripts outside the product, the test suites and the release build.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core crate's shared constants, among them DIALOG_HOLD_MAX, the dialog hold's ceiling of 3600 s that bounds the configured dialog hold; only its doc comment changed, the value and its references did not
@task_item n=1 part=touches
Dialog hold expiry: the 3600 s ceiling a row may state, or mark as measured only to about 660 s, is now measured live at the ceiling itself on claude 2.1.325 (wrapper expiry after 3600013 ms, hook still waiting); the value stands unchanged.
@task_item n=2 part=touches
Dialog hold read once at start: the round ran the config key dialog_hold_s at 3600 on a test home; no key was added and the read-once rule is unchanged.
@task_item n=3 part=touches
Dialog timeouts outlast the longest hold: the longest hold (3600 s) a row bounds against is now a live-measured value, unchanged; the hook's own backstop beyond it stands, since the hook was still waiting at expiry.
@task_item n=4 part=touches
The dialog-expired event data and the question event: measured live, a question with source hook then dialog-expired with source wrapper one hold later; the question dialog stays on the screen after the wrapper's expiry, as the hook's reply was null.
@task_item n=5 part=touches
The first record of a start and the run start events order: on a live start whose folder-trust keys pass through the wrapper, a wheel human-input line is written before session-start, so a reading of lines 1-2 as the start's wheel line then session-start does not hold on such a start.
@task_item n=6 part=touches
The wheel human-input record and human input takes the wheel: once the wheel is already human from trust keys, a later human key adds no wheel line of its own.
@task_item n=7 part=touches
A reused instance name: its events.ndjson keeps the earlier start's lines at its head (cursors are byte offsets, never rewritten), so a start's own lines are not lines 1-2 of the log.
@task_item n=8 part=touches
Fake-agent input keystroke contract and the fake agent is not proof of real-CLI behaviour: the real CLI 2.1.325 erases the input box on DEL, while the fake agent keeps a typed DEL in its prompt; left unchanged on the overseer's word and carried.
@task_item n=9 part=touches
Recorder scrub, dirty payload refuses the recording, the MCP and verify fixture scrub refusal absolute-path, and the verify recording refusal message: a live verify from a test home whose directory is named home was refused whole with SessionStart absolute-path; the refusal code stands, the test home's naming tripped it.
@task_item n=10 part=touches
Recorded fixtures per CLI version, every CLI-version fixture set replayed, fixtures refreshed by live verify, and one default CLI version: no fixture set for 2.1.325 was recorded or placed; the 2.1.302 fixture set is untouched; the recorded 2.1.325 set is a carry.
@task_item n=11 part=touches
Stamps written only by verify, stamps only from verify against the fake agent, and verify is the only stamp writer: 2.1.325 was stamped by a live verify against the real CLI on a test home (stamped 2.1.325, 21 pass, 0 fail), never on the live home.
@task_item n=12 part=touches
Unverified CLI version gate and version gate needs every relied row: the relied set of 21 rows all pass on 2.1.325 on the test home; the live home stays unstamped for 2.1.325.
@task_item n=13 part=touches
Real claude CLI only locally, live verify never in CI, no real CLI in CI, and live readings are not a test layer: the whole round was a local live reading by operator-named entries, never in CI; CI on the commit ran 7 of 7 checks green over the fake agent.
@task_item n=14 part=touches
Test wrapper boots run trusted with screens: claude 2.1.325's folder-trust dialog in a fresh cwd stands before any hook, with two unnumbered option rows and the selector on the non-trust row by default.
@task_item n=15 part=touches
Dev-host test homes on tmpfs, E2E home base may be a link, and test-home deletion through the dev-host link: a rig resolving the target/e2e-home link landed outside the root; the path must be kept as spelled; the kept live test home under target/e2e-home is the operator's to delete.
@task_item n=16 part=touches
The send input-not-ready contract: a send witness reads it unread where no box stood, not-delivered, or x-stood for a human wheel line, and a confirmed send reads the box empty; input-not-ready is not a residue.
@task_item n=17 part=touches
Compatibility tests across CLI versions and multi-version compatibility layers: a new CLI version, claude 2.1.325, was stamped and measured (box_only_guards 3, ready_settle_ms 2339, turn_settle_ms 603, compact_latency_ms 7038) without a fixture set placed.
@task_fact n=1 slug=7-test-data-fixtures
fact: The fake agent keeps a typed DEL in its prompt, while the real claude CLI (measured on 2.1.325) erases the input box on DEL; a rehearsal over the fake reads only the box standing, never the erase.
@task_fact n=2 slug=6-e2e-test-strategy
fact: On a start whose folder-trust keys pass through the wrapper, a wheel human-input line precedes session-start and a later human key adds no wheel line; a criterion reading the start's lines 1-2 as its wheel line then session-start holds only on a cwd trusted beforehand.
@task_fact n=3 slug=6-e2e-test-strategy
fact: A reused instance name keeps the earlier start's lines at the head of its events.ndjson, cursors being byte offsets never rewritten; a reader of a start's first lines reads that start's own lines, never line 1 of the log.
@task_item n=1 part=untouched
CI gates, coverage floors, mutation thresholds, performance budgets and supply-chain gates: no threshold, job or tool moved; CI ran green on its standing checks.
@task_item n=2 part=untouched
MCP tools, the wrapper channel and its JSON-RPC errors, CLI exit codes, refusal codes and --json documents: no method, tool, code, word or flag was added or changed.
@task_item n=3 part=untouched
The test harness contract (boot, run, cleanup, gate, pre-push, secret scan, status, session record): the round's rigs are evidence scripts outside the harness and changed none of its commands or documents.
@task_item n=4 part=untouched
The bans on parsing the rendered child screen and on screen assertions: the rigs read frames off a VT screen model as evidence outside the product and the test suites; no product code or test reads the screen.
@task_item n=5 part=untouched
The list board's wrapped and unwrapped rows and names: the user's status row read the same wrapped and unwrapped, a matrix note fact, and no list contract moved.
@task_item n=6 part=untouched
Log format, log corr fields, process log lines and diagnostics schema: no log line, field or event kind changed.
@task_item n=7 part=untouched
Socket, home and strict-modes permission checks, peer checks and environment strips: no security control or check changed; the rig keys and the status-line hash read belong to another plan.
