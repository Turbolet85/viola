@task_prose part=session
The session built the fleet console's actions: every verb run by keyboard from a selected row (pause, release, last, focus, revive, send, switch, open, answer), each as a child of viola's own CLI verbs, its readback drawn and a refusal shown as the verb's own exit.
The action child is the own exe with --home, the verb and the name, no shell, VIOLA_NAME and VIOLA_DIR removed, one child at a time, each output stream read through take(MAX_FRAME) and every line escaped; no IPC method, endpoint, socket, port or env var was added.
The console key reader became a decoder of sequences (ESC_WAIT 50 ms, READ_CAP 64 a read, CSI cap 32) with closed classes Interrupt, Up, Down, Enter, Escape, Backspace, Char, Unbound; an editing line for send, switch and open is bounded at MAX_FRAME bytes, and an answer prompt for a permission or a plan reads only the board's dialog kind and id.
Measured: the console-key word set grew 3 to 14 with a count field, the closed event list 18 to 19 with a new console-action line holding verb and exit_code only; a 200-byte paste reached the console in at most four reads; a CR inside a read ends the editing line and drops the rest of that read.
Retired: the scaffold's one-class-a-read key reading and its three-word key set. The board item gained a dialog id kept out of list --json and the MCP list item; unicode-width =0.2.2 was pinned, already in the lock, no crate added.
matrix v2-15 stays GATED on the operator's live reading; nothing was proven against a real claude or real terminals.
@task_file n=1 path=src/console/action.rs
what: added — the console's action runner: spawns each verb as a child of the own exe with --home, strips the parent identity variables, no shell, bounds each child stream at MAX_FRAME, escapes each output line, and builds send, switch, open and answer argv and stdin with the closed answer behaviours
@task_file n=2 path=src/console/keys.rs
what: edited — the console key reader: the bounded raw stdin read (READ_CAP), the escape-sequence decoder with its wait and sequence cap, and the closed key classes
@task_file n=3 path=src/console/mod.rs
what: edited — viola console's loop: the modes (list, confirm, editing line, answer prompt), one child at a time, the editing text bound, and the console-key and console-action diagnostics lines
@task_file n=4 path=src/console/view.rs
what: edited — the console view: the selection over wrapped rows, the scroll window and the key line fitting
@task_file n=5 path=src/human/console.rs
what: edited — the console's human-mode copy: confirm, editing and answer prompts, the question line, the running line and the editing echo's width rule through the control-escaping helper
@task_file n=6 path=src/human/fleet.rs
what: edited — the fleet rows of human output: the selection gutter and each wrapped row's pending dialog kind and id
@task_file n=7 path=src/human.rs
what: edited — the human output module's re-exports (companion), including the control-escaping helper
@task_file n=8 path=src/cmd/console.rs
what: edited — the viola console command entry: builds the action runner from the own exe and the home after its terminal and home checks
@task_file n=9 path=src/cmd/list.rs
what: edited — viola list's board item construction (companion): the item gains the dialog id, list output unchanged
@task_file n=10 path=src/human/board/test_support.rs
what: edited — the board test support's item literal (companion)
@task_file n=11 path=crates/viola-core/src/obs.rs
what: edited — the core crate's closed diagnostics event list and its event docs: the new console-action event and the console-key count
@task_file n=12 path=crates/viola-state/src/board.rs
what: edited — the board read over the strict-checked snapshots: an item's pending dialog id, none for a replayed or unwrapped item, never serialised
@task_file n=13 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema: the console-action arm (verb enum, exit_code) and the console-key arm's closed key words and count
@task_file n=14 path=schemas/diag-detail.v1.json
what: edited — the diagnostics detail schema's closed event enum
@task_file n=15 path=tests/contract_diag_schema.rs
what: edited — the diagnostics schema contract test's closed event list
@task_file n=16 path=tests/tui_console.rs
what: edited — the console end-to-end tests over the fakes under their own PTY and test home
@task_file n=17 path=Cargo.toml
what: edited — the workspace dependency pins: unicode-width added at an exact version
@task_file n=18 path=Cargo.lock
what: edited — the root lockfile under the cargo-deny gate: one dependency line, no crate added
@task_item n=1 part=touches
decision/console-key-reader-bounded is stale: the reader now decodes sequences across reads (ESC_WAIT 50 ms, READ_CAP 64, CSI cap 32), its closed classes are Interrupt, Up, Down, Enter, Escape, Backspace, Char, Unbound, and an editing line's text is bounded at MAX_FRAME bytes with a char past it counted unbound; the closed console-key word set moved 3 to 14.
@task_item n=2 part=touches
The console is no longer read-only: it starts children. decision/console-terminal-and-home-check and contract/strict-modes-refusal/console rule over a console that now acts through the verbs; each child re-runs its own verb's checks.
@task_item n=3 part=touches
decision/parent-identity-env-stripped: the console's action child has VIOLA_NAME and VIOLA_DIR removed, so a child verb never takes the console's own instance identity; indirectly decision/pause-release-from-viola-name and decision/driver-release-refused, as a console release runs without a VIOLA_NAME from.
@task_item n=4 part=touches
decision/console-agents-child-outlives-quit gains a sibling: an action child may also outlive a console quit (no parent-death tie).
@task_item n=5 part=touches
A new product reader of child output: decision/child-output-bounded, decision/reads-capped-at-max-frame, decision/max-frame-shared-cap, decision/max-frame-cap and decision/bounded-readers-census (every product reader bounded or named) take each action child's stdout and stderr through take(MAX_FRAME).
@task_item n=6 part=touches
decision/human-output-escapes-controls and decision/escape-terminal-controls: every action child line and the editing echo are drawn through the control-escaping helper; decision/upstream-text-is-content and decision/upstream-text-is-never-a-command over child output drawn as content only.
@task_item n=7 part=touches
Direct spawn without a shell: decision/verify-children-direct-spawn, decision/open-terminal-direct-spawn and decision/session-id-argv-no-shell are the convention the console's runner now follows, each argument one argv element.
@task_item n=8 part=touches
decision/answer-behavior-closed-enum: the console answers by a closed behaviour (allow, deny, approve, revise) sent as a body on the answer verb's stdin with the board's dialog id; decision/dialog-answers-need-verify-stamp holds through the answer verb it runs.
@task_item n=9 part=touches
Send from the console: the typed line reaches viola send on stdin unchanged, so decision/paste-validation-on-both-sides, decision/paste-text-control-character-set, decision/send-text-is-one-bracketed-paste, decision/empty-typed-text-refused and contract/send-refusal/empty-text (the empty line read as exit 13) apply through the verb.
@task_item n=10 part=touches
Switch from the console: typed words become --effort= or --model= values, never flags, so decision/switch-method-behind-ipc-controls, decision/effort-word-check-in-agent-crate, decision/launch-flags-checked and decision/launch-flags-refused-after-dashes apply through the verb (a bad word read as exit 2).
@task_item n=11 part=touches
decision/open-closed-argument-allowlist: the console's open passes the typed name as one element with --parent and the separator then claude.
@task_item n=12 part=touches
decision/cli-input-trust-boundary and decision/cli-input-checked-before-frame: the console's actions cross into the CLI verbs and inherit their checks rather than framing anything itself; decision/new-listener-needs-log-entry and decision/new-env-read-needs-log-entry hold with no new method, listener or env read.
@task_item n=13 part=touches
decision/question-option-text-crosses-boundary: the console reads no question or option text; a question shows one fixed line and runs nothing.
@task_item n=14 part=touches
decision/user-content-only-to-diagnostics and decision/never-log-claude-values: the console-key and console-action lines carry no key byte, typed text, name, argument or child output; a console-key count is a count of keys only.
@task_item n=15 part=touches
Board dialog id: decision/snapshot-strict-checked-before-trust and decision/board-home-read-rule over the new item field read from the strict-checked snapshot; decision/replayed-snapshot-inert and decision/replayed-snapshot-yields-nothing, a replayed item carrying none.
@task_item n=16 part=touches
contract/cli-exit-codes/21: the plan's claim that pause on a dead instance gives 21 was disproved (a gone instance draws no row); the console's refused case reads a frozen, stale wrapper's exit.
@task_item n=17 part=touches
Dependency pins: decision/exact-dependency-pins, decision/exact-third-party-pins, decision/cargo-deny-gate and decision/crates-io-only over unicode-width added at =0.2.2, a crate already in the lock, cargo deny green.
@task_item n=18 part=touches
decision/focus-raises-only-ancestry-windows and decision/revive-cwd-spawn-only over the console's f and v keys, which run focus and up as children.
@task_fact n=1 slug=input-validation
fact: viola console runs every action as a child of its own executable with --home, the verb and the name, no shell, one child at a time, each output stream bounded at MAX_FRAME and each line escaped before it is drawn.
@task_fact n=2 slug=data-protection
fact: A console action child may outlive the console's quit; no parent-death tie is set.
@task_fact n=3 slug=input-validation
fact: The console hands typed text to its verbs as data only: send's text on stdin unchanged, switch's words as --effort= or --model= values never flags, open's typed name as one argv element, answer's body a closed behaviour word on stdin.
@task_fact n=4 slug=input-validation
fact: The console's editing line holds at most MAX_FRAME bytes, a char past it counted unbound; Enter, Escape and Interrupt end the read and the rest of that read is dropped.
@task_fact n=5 slug=data-protection
fact: The console's answer prompt reads only the board's pending dialog kind and id, never dialog, question or option text; a question draws one fixed line and starts no child.
@task_fact n=6 slug=data-protection
fact: The board item's pending dialog id is never serialised: it stays out of list --json and the MCP list item.
@task_fact n=7 slug=data-protection
fact: The console-action diagnostics line holds the verb word and exit code only, and a console-key line a closed key word and a key count only; no name, argument, typed text or child output reaches any line.
@task_item n=1 part=untouched
API security: the rmcp feature set and its banned HTTP and auth features; the MCP server's tools and inputs are unchanged.
@task_item n=2 part=untouched
Secret management: credential stripping from the claude child, the artifact canary scan, live rig credentials and stored-secret absences.
@task_item n=3 part=untouched
The IPC channel and its sockets: socket directory checks, peer euid checks, server verification, channel frames, methods and error codes; no channel method was added or changed.
@task_item n=4 part=untouched
CI and workflow security: actions SHA pins, workflow permissions, zizmor, scheduled fuzzing and advisory runs, the release build and attestations.
@task_item n=5 part=untouched
viola verify and its probes, the live rigs, fixture recording and scrubbing, and their refusal codes.
@task_item n=6 part=untouched
The hook path: hook stdin caps, hook fail-open, hook decision details and the trust record.
@task_item n=7 part=untouched
Pinned binary placement and re-hash, plugin file rewriting, event log rotation and growth, and the revive --json codes.
