@task_prose part=session
The fleet console, until now a view of the board, now acts on its selected row by keyboard: pause, release, focus, revive (through up), last, send, switch, open and answer, each by running viola's own CLI verb as a child process, one at a time, its output read back as result lines and its refusal shown as an unable line with the verb's exit code.
Release and revive ask y or n first; s, m and o open one editing line (send text, switch words, a child name for open); a opens an answer prompt for a pending permission or plan, or draws one line on a question pointing to focus.
The action child is the console's own exe with --home, no shell, VIOLA_NAME and VIOLA_DIR removed, each output stream bounded at MAX_FRAME and every line escaped; the key reader reads closed key classes, 64 bytes a read, and the editing text is bounded at MAX_FRAME bytes.
The board item gains the pending dialog's id, read from the live snapshot, kept out of list --json and the MCP list item; the process log gains a console-action event (verb, exit code) and the console-key line gains a per-read count; the closed obs event list goes 18 to 19.
unicode-width is pinned as a direct dependency (already in the lock through vt100); no IPC method, endpoint, socket, port, config key or environment variable was added; the matrix capability stays GATED on the operator's live reading.
Retired: the scaffold's one-read-one-key-class reader and the key line that held only q quit.
@task_file n=1 path=src/console/action.rs
what: added — the fleet console's action runner: each console verb as a child of viola's own CLI, its argv and stdin, its bounded readback lines
@task_file n=2 path=src/console/keys.rs
what: edited — the fleet console's bounded raw key reader and its closed key classes
@task_file n=3 path=src/console/mod.rs
what: edited — the fleet console's loop and modes: selection, confirm, editing line, answer prompt, one action at a time, its key and action log lines
@task_file n=4 path=src/console/view.rs
what: edited — the fleet console's frame view: the selected row, the scroll window and its above and more lines, the fitted key line
@task_file n=5 path=src/human/console.rs
what: edited — the console's human copy: the key line, the confirm, editing and answer prompts, the question line, the editing line's width rule
@task_file n=6 path=src/human/fleet.rs
what: edited — the human fleet rows drawn from the board: the gutter, the selected row, each wrapped row's pending dialog kind and id
@task_file n=7 path=src/human.rs
what: edited — the human writer's module root and its re-exports (companion)
@task_file n=8 path=src/cmd/console.rs
what: edited — the viola console subcommand's entry, which builds the action runner over the own exe and the home
@task_file n=9 path=src/cmd/list.rs
what: edited — the viola list subcommand (companion: a board item literal gains the dialog id)
@task_file n=10 path=src/human/board/test_support.rs
what: edited — test support for the human board rendering (companion: a board item literal)
@task_file n=11 path=crates/viola-core/src/obs.rs
what: edited — the closed list of obs events of the process logs and their field docs
@task_file n=12 path=crates/viola-state/src/board.rs
what: edited — the board read model: one item per instance, from its snapshot or its replay
@task_file n=13 path=schemas/diag-line.v1.json
what: edited — the diagnostics process-log line schema: its event enum and per-event field arms
@task_file n=14 path=schemas/diag-detail.v1.json
what: edited — the diagnostics detail-file line schema and its event enum
@task_file n=15 path=tests/contract_diag_schema.rs
what: edited — the contract test holding the diagnostics schemas to the closed obs event list
@task_file n=16 path=tests/tui_console.rs
what: edited — the end-to-end tests of the fleet console under an outer PTY over the fake agent
@task_file n=17 path=Cargo.toml
what: edited — the workspace manifest and its one place of pinned third-party versions
@task_file n=18 path=Cargo.lock
what: edited — the dependency lockfile (one dependency line, no crate added)
@task_item n=1 part=touches
decision/terminal-fleet-console: the console is no longer a view alone; it acts on the selected row by keyboard through every verb named above.
@task_item n=2 part=touches
The project-intent overview says the keyboard-driven terminal fleet console is planned, not built; it is now built and proven over the fakes, its live half gated on the operator's reading.
@task_item n=3 part=touches
decision/new-capabilities-as-verbs: the console's actions add no subcommand and no channel verb; each runs an existing CLI verb, which this rule should read beside.
@task_item n=4 part=touches
decision/human-controls-cli-only, decision/pause-link-unlink-cli-only, decision/release-cli-only, decision/switch-cli-only and decision/focus-has-no-driver-form: the console reaches pause, release, switch and focus only as children of the CLI verb, so they stay CLI-only; a row may now name the console as a caller.
@task_item n=5 part=touches
decision/no-shell-out and decision/hyprctl-direct-spawn: the console spawns its action child directly with a closed argv and no shell, a second direct-spawn site.
@task_item n=6 part=touches
contract/child-environment/viola-name, contract/child-environment/viola-dir and decision/env-strip-names-only: the action child has VIOLA_NAME and VIOLA_DIR removed, so a console started inside a wrapped session acts as a plain CLI caller, not as that instance.
@task_item n=7 part=touches
decision/bounded-readers and contract/wrapper-channel-frames/max-frame: the action child's stdout and stderr are each read through MAX_FRAME, the editing text is bounded at MAX_FRAME bytes, the key reader takes 64 bytes a read with a CSI cap of 32.
@task_item n=8 part=touches
decision/control-characters-refused, decision/untrusted-upstream-text and decision/upstream-text-is-content: the action child's output lines and the editing line's echo are each escaped before drawing.
@task_item n=9 part=touches
decision/option-text-never-in-human-line: the console's question line names only the instance; no question or option text is drawn or logged.
@task_item n=10 part=touches
contract/answer-response/permission, contract/answer-response/plan, contract/answer-response/question and decision/permission-answer-without-suggestions: the console answers a permission y allow or n deny and a plan y approve or n revise through viola answer with a behavior body on stdin; a question is not answered from the console.
@task_item n=11 part=touches
contract/cli-verbs/send, contract/cli-verbs/switch, contract/cli-verbs/open and contract/cli-verbs/answer: the console drives each; switch words become --model= or --effort= values, never flags; open passes the typed name as one element with --parent set to the selected row; send's text goes on stdin unchanged.
@task_item n=12 part=touches
contract/cli-verbs/pause, contract/cli-verbs/release, contract/cli-verbs/focus, contract/cli-verbs/last and contract/cli-verbs/up: the console runs each on the selected row; revive from the console runs up.
@task_item n=13 part=touches
decision/board-read-model-shape and contract/instance-snapshot/pending-dialog: the board item gains the pending dialog's id beside its kind, read from the snapshot's pending_dialog, absent on a replayed snapshot and on an unwrapped item, never serialised.
@task_item n=14 part=touches
contract/list-payload/dialog-pending and the contract/list-item rows: list --json and the MCP list item are unchanged by the new board field; a picker should confirm these rows still hold.
@task_item n=15 part=touches
decision/codes-only-process-logs, decision/codes-only-role-logs and decision/process-logs-hold-codes-only: the console-action line carries verb and exit_code alone; a console-key line carries a key word and a count, never a key byte, typed text, name or argument.
@task_item n=16 part=touches
decision/obs-event-only-emitter and contract/diag-line/format: console-action is a new obs event of viola-core with its schema arm, and the console-key arm gains count; the closed obs event list goes 18 to 19.
@task_item n=17 part=touches
contract/diagnostics-role-files/cli-name: each action child writes its own cli-<name> log, as any CLI verb does.
@task_item n=18 part=touches
decision/workspace-dependency-pins and decision/cargo-deny-dependency-policy: unicode-width is pinned exactly at 0.2.2 in the workspace dependencies and taken by the root bin; no crate enters the graph; cargo deny is green.
@task_item n=19 part=touches
decision/single-human-writer: all new console copy (the key line, the confirm, editing and answer prompts, the question line, the running line) sits in the one human writer of the root bin.
@task_item n=20 part=touches
contract/event-prompt-submitted/origin, decision/byte-source-wheel-claim and decision/human-prompt-confirms-wheel-take: a send typed in the console reaches the instance as a CLI send, recorded with origin driver, not as a human keystroke in the wrapped terminal.
@task_item n=21 part=touches
contract/cli-exit-codes/13, contract/cli-exit-codes/2 and contract/human-stderr/refusal: the console draws the verb's own refusal as result lines and logs its exit code; an empty send gives exit 13, a bad switch word exit 2.
@task_item n=22 part=touches
decision/modular-monolith-no-daemon and decision/no-daemon-disk-is-shared-truth: an action child is not tied to the console's life and finishes after a quit; it is a short-lived CLI verb, no daemon.
@task_fact n=1 slug=established-decisions
fact: The fleet console acts only by running viola's own CLI verbs as child processes (its own exe with --home, no shell, VIOLA_NAME and VIOLA_DIR removed), one child at a time, a further action key while one runs reading busy.
@task_fact n=2 slug=established-decisions
fact: A console action's child is not tied to the console's life: quitting the console leaves a running verb to finish on its own.
@task_fact n=3 slug=standard-contracts
fact: The console answers only a pending permission (y allow, n deny) or plan (y approve, n revise) through viola answer; a pending question is answered in the session, the console drawing one line that points to focus, and with nothing pending the answer key is unbound.
@task_fact n=4 slug=project-directory-structure
fact: The board item carries the pending dialog's id beside its kind, taken from the live snapshot only and kept out of list --json and the MCP list item.
@task_fact n=5 slug=occupied-resources
fact: The console's process log writes one console-action line per finished child holding only the verb and its exit code, and on the editing line at most one key line per class per read holding a count, never a typed character.
@task_fact n=6 slug=stack-and-technologies
fact: unicode-width is a direct, exactly pinned dependency of the root bin, used for terminal column widths of the console's editing line.
@task_item n=1 part=untouched
The wrapper channel protocol: no channel method, frame shape, fault code, refusal reason or protocol version was added or changed.
@task_item n=2 part=untouched
The MCP server's tools: no tool added, and the send, wait, last and answer tools and their results are unchanged.
@task_item n=3 part=untouched
Hook contracts, hook tiers, the hook-to-kind map and the dialog hold: no hook changed; the console answers through the existing answer verb.
@task_item n=4 part=untouched
The normalised event kinds and the events.ndjson event data: no event kind was added; the new console-action event is a process-log obs event, a different list.
@task_item n=5 part=untouched
The PTY wrapper's send gate, readiness gate, paste bound and wheel classifier internals: the console reaches them only through the existing send and release verbs.
@task_item n=6 part=untouched
The capability ledger, verify, the ledger stamps and the CLI version gate: none moved.
@task_item n=7 part=untouched
The Hyprland compositor seam and its window rows: focus and open are reached only through their own verbs.
@task_item n=8 part=untouched
config.json keys, VIOLA_ and AGENT_RUN_ environment names, socket and endpoint names: none added or changed.
@task_item n=9 part=untouched
CI workflows, the pre-push gate, coverage floors and the test harness surface: no change to any of them.
@task_item n=10 part=untouched
Revive, up, adopt and the run start order: their internals are unchanged; the console only calls them.
@task_item n=11 part=untouched
The absences of a web or graphical frontend, a daemon and a listening network port: the console stays a terminal surface with no port.
