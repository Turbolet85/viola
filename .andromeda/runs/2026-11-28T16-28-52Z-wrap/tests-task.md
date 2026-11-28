@task_prose part=session
The fleet console gained its actions: every verb by keyboard from a selected row (pause, release, focus, revive, last, send, switch, answer, open), each run as a child of the own viola binary through the CLI verbs, its readback or refusal drawn as lines.
Part 2 added the editing line (send, switch, open) and the answer prompt (permission y allow / n deny, plan y approve / n revise, a question drawn as one line that runs nothing).
A new console-action diagnostics line (verb, exit_code) joined the closed event list, 18 to 19; the console-key word set grew 3 to 14 and its line gained a count, one line per class per read on the editing line, never the typed text.
The board item gained a skip-serialised dialog id read from the snapshot's pending dialog; list, list --json and the MCP list item are unchanged.
Measured: at 80 columns the key line is cut after m swi; a 200-byte paste reaches the console in at most four reads; a CR inside a read ends the editing line and drops the rest of that read.
Proven over the fakes only; matrix v2-15 stays GATED on the operator's live reading; the pre-push coverage merge went red once on a truncated fake-agent profile with every test passed, green on re-fire, recorded open.
@task_file n=1 path=src/console/action.rs
what: the console's action runner: the verb set, the action child spawn (own exe, --home, verb and name, instance env names removed, no shell), each child stream bounded at MAX_FRAME and escaped line by line, the answer behaviours
@task_file n=2 path=src/console/keys.rs
what: the console's key reader: the bounded raw stdin decoder, the closed key classes, the escape wait and read cap
@task_file n=3 path=src/console/mod.rs
what: the console's run loop and state: modes (list, confirm, editing, answer), one child at a time, the per-read key fold, its unit cases
@task_file n=4 path=src/console/view.rs
what: the console's frame view: the selection, scroll with the above and more lines, the fitted key line
@task_file n=5 path=src/human/console.rs
what: the console's human-mode copy: key line, confirm, editing and answer prompts, the question line, the running line, the editing line width rule
@task_file n=6 path=src/human/fleet.rs
what: the fleet board's human-mode lines: the gutter, the selected row mark, the row's pending dialog
@task_file n=7 path=src/human.rs
what: the human-mode output module's re-exports
@task_file n=8 path=src/cmd/console.rs
what: the viola console verb: builds the action runner and starts the console
@task_file n=9 path=src/cmd/list.rs
what: the viola list verb: a board item literal gains the dialog id
@task_file n=10 path=src/human/board/test_support.rs
what: the human board's shared test constructor of board items
@task_file n=11 path=crates/viola-core/src/obs.rs
what: the closed diagnostics event list and its log-format event names
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board read: each item from the snapshot or the replay, now with the pending dialog's id, never serialised
@task_file n=13 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the event enum, the console-action arm, the console-key arm with its key words and count
@task_file n=14 path=schemas/diag-detail.v1.json
what: the diagnostics detail schema: the event enum
@task_file n=15 path=tests/contract_diag_schema.rs
what: the diagnostics schema contract check: the closed event list held against both schemas
@task_file n=16 path=tests/tui_console.rs
what: the console's E2E suite through an outer PTY over the fake agent: selection, scroll, each verb, send, switch, open, answer, Ctrl-C from every mode
@task_file n=17 path=Cargo.toml
what: the workspace and root bin dependency list: unicode-width added at an exact pin already in the lock
@task_file n=18 path=Cargo.lock
what: the dependency lock: only the root package's new dependency line, no crate added
@task_item n=1 part=touches
decision/harness-log-fields-owned-by-test-plan: the harness-grepped log fields gain the console-action line (verb, exit_code) and the console-key line's count field.
@task_item n=2 part=touches
decision/closed-enums-need-decisions-log: two closed enums moved, the diagnostics event list 18 to 19 (console-action) and the console-key word set 3 to 14 (question added in Part 2); console-action's verb enum is a new closed set of 9 words.
@task_item n=3 part=touches
decision/diagnostics-schema-conformance and decision/diagnostics-schema-validation: the line schema gains the console-action arm and a conditional rule that a console-key text word requires count; the detail schema's event enum grows.
@task_item n=4 part=touches
decision/no-secrets-in-logs, decision/logs-carry-no-secrets, decision/secrets-never-in-logs: the console writes no key byte, typed text, name, argument, child output, question or option text into any line; only key classes, counts, verb and exit code.
@task_item n=5 part=touches
decision/bounded-readers, decision/bounded-readers-keep-the-cap, decision/no-unbounded-json-depth: two new external readers, the console's raw key reader (read cap 64, CSI cap 32, editing text bound at MAX_FRAME) and each action child's stdout and stderr (taken to MAX_FRAME).
@task_item n=6 part=touches
decision/human-output-escapes-control and decision/human-output-control-escaping: every action child output line and the editing line's echo go through the row escaper.
@task_item n=7 part=touches
decision/child-spawn-test-layers and decision/child-env-strip: a new child spawn, the console's action child (own exe, --home, no shell, VIOLA_NAME and VIOLA_DIR removed), unit-tested on its command and env.
@task_item n=8 part=touches
decision/tui-outer-pty-driver, decision/tui-verdicts-from-records, decision/no-screen-scraping-in-e2e, decision/e2e-layer-by-binary-prefix: the console E2E suite grew to 34 cases through the outer PTY, its verdicts read from console, wrapper and event records.
@task_item n=9 part=touches
decision/console-restore-read-on-own-terminal: Ctrl-C now ends the console from every mode (list, confirm, editing, answer), exit 0 and cooked after, read on its own terminal.
@task_item n=10 part=touches
decision/live-failing-reading-not-verification, decision/fake-agent-not-proof-of-real-cli, decision/no-human-in-loop-verification: the console actions are proven over the fakes only, the live half of v2-15 stays GATED on the operator's reading (real focus and terminals, real claude replies and dialogs, the editing line under his terminal and input method, the key line at his tile width).
@task_item n=11 part=touches
decision/window-raise-proven-live and decision/window-reopen-proven-live: the console's f focus runs over the fake compositor and v revive over the fake foot stand-in; real raise and re-open stay for the live reading.
@task_item n=12 part=touches
decision/dialog-answer-mapping, contract/hook-decision/permission, contract/hook-decision/plan-revise: the console's answer drives viola answer; its E2E verdicts read allow for a permission by y and deny for a plan by n (a revise of a permission-raised plan denies it).
@task_item n=13 part=touches
contract/viola-send/records, contract/viola-send/empty-text, contract/cli-exit-codes/13: the console's send hands its line to viola send; the E2E reads the send records, exit 0, and exit 13 for an empty line; a Cyrillic line lands as typed with origin driver.
@task_item n=14 part=touches
contract/cli-human-lines/unable and contract/cli-human-lines/read-back: the console draws a verb's readback and its refusal lines as the verb wrote them, escaped.
@task_item n=15 part=touches
decision/human-layouts-pinned-by-literal and decision/cli-fixed-strings-pinned-by-literal: the console key line, confirm, editing and answer prompts and the question line are new fixed strings pinned by literal asserts, the key line cut at 80 columns pinned too.
@task_item n=16 part=touches
decision/board-on-the-read-rule, decision/pending-dialog-before-its-line, decision/snapshot-replay-fields: the board item gains a dialog id read from the snapshot's pending dialog, none on a replayed or unwrapped item, skipped in serialisation.
@task_item n=17 part=touches
decision/bounded-remove-the-guard-witness: two new remove-the-guard controls (text-fold, flag-value) hold, beside Part 1's four.
@task_item n=18 part=touches
contract/coverage-failure/llvm-cov-exit, contract/coverage-failure/llvm-cov-summary-missing, decision/killed-test-child-uninstrumented: pre-push fired red once with every test passed, the coverage merge refusing a truncated fake-agent profile; green on re-fire of the unchanged tree, recorded open with its cause unproven.
@task_item n=19 part=touches
decision/cargo-deny-supply-chain-gate and decision/no-advisory-ignored: a new direct dependency, unicode-width at an exact pin of the version already locked; cargo deny green, no crate added.
@task_item n=20 part=touches
decision/no-sleep-synchronization and decision/waits-key-on-events: the new console E2E cases wait on logged key, action, frame and event lines.
@task_fact n=1 slug=log-format
fact: The console-action line is written once per finished action child, carrying only verb (a closed set of 9 words) and exit_code, both required; no name, argument or output.
@task_fact n=2 slug=log-format
fact: On the console's editing line one key read writes at most one console-key line per class (text, backspace, unbound), each with a count of 1 or more, never a line per character and never the typed text; a text word requires count.
@task_fact n=3 slug=e2e-test-strategy
fact: The console's answer E2E cases raise their dialog by a test-run viola hook over a recorded payload and read the hook's own decision body and one dialog-answered line, not a fake-agent dialog script.
@task_fact n=4 slug=e2e-test-strategy
fact: The console's refused-action case runs over a frozen, stale wrapper, since a gone instance draws no row and cannot be selected.
@task_fact n=5 slug=e2e-test-strategy
fact: The console's actions are proven only over the fakes; real focus, real terminals, real claude replies and dialogs, the editing line under a real input method and the key line at the operator's tile width are proven only by the gated live reading of v2-15.
@task_item n=1 part=untouched
The MCP surface (tools list, tool results, invalid-params fields, frame tools) is not reached: the console acts through the CLI verbs alone, and the MCP list item is unchanged.
@task_item n=2 part=untouched
The wrapper channel, its JSON-RPC errors, link and unlink methods and channel logs are not reached: no IPC method, endpoint, socket or port was added.
@task_item n=3 part=untouched
list --json, its items and the status polled fields are unchanged; the new dialog id is never serialised.
@task_item n=4 part=untouched
The test harness commands (boot, run, status, cleanup, supervise), their documents, exit codes and session record are not changed.
@task_item n=5 part=untouched
CI workflows, artifacts, permissions, pinned actions and the gate breach set are not changed; CI ran 7/7 green.
@task_item n=6 part=untouched
Fuzzing, mutation testing and the performance budgets and their gates are not reached.
@task_item n=7 part=untouched
Recorded fixtures, the fake agent's command line, receipts, scripts and the fixture hygiene walk are not changed.
@task_item n=8 part=untouched
viola run and revive start refusals, exit 1 causes and the run start events are not reached; the run boot path is untouched.
@task_item n=9 part=untouched
Hook exec-form entry points, hook deadlines and fail-open rules are not changed.
