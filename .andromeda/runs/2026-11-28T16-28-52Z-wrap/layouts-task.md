@task_prose part=session
The fleet console gained its actions: a selected row (jk or arrows, only wrapped rows) and every verb by one key, each run as a child of the viola CLI verb itself (own exe, --home, no shell, one child at a time), its output drawn as result lines and its refusal as the verb's own exit.
Keys l last, p pause, r release, f focus, v revive (runs up), s send, m switch, o open, a answer; release and revive ask a y/n confirm, escape or n leaves it, Ctrl-C ends every mode.
s, m and o open one editing line in the key line's place: the text echoed as typed, its width counted per scalar (1, 2 or 0 columns), past the edge the prompt, then ..., then the longest fitting tail; a answers a pending permission (y allow, n deny) or plan (y approve, n revise), and on a question draws one line sending the operator to the session.
The key line now reads q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open, 98 columns in full; measured at the 80-column floor it is cut after m swi, so a answer and o open do not show there (left to the operator as a fork, as built while unanswered).
Scrolling follows the selection with a ... <n> above line; the board item gained the pending dialog's id, kept out of list --json and the MCP list item; the diagnostics log gained a console-action line and a count on console-key; the scaffold's single key line q quit and its three-word key reading were retired.
Proven over fakes only; the chunk stays GATED on the operator's live reading.
@task_file n=1 path=src/console/action.rs
what: the console's action runner: the verbs it can run, each action as a CLI verb child of the own exe under --home with no shell, its stdin text, and its output lines bounded and escaped
@task_file n=2 path=src/console/keys.rs
what: the console's key reader: the closed key classes decoded from raw stdin, the escape wait and the per-read cap
@task_file n=3 path=src/console/mod.rs
what: the fleet console's loop and state: its modes (list, confirm, editing line, answer prompt), the selected row, one child at a time, the key log per read
@task_file n=4 path=src/console/view.rs
what: the console frame's layout: the fleet body lines, the selection and its scroll window with the more and above lines, the result rows, the fitted key line
@task_file n=5 path=src/human/console.rs
what: the console's fixed copy: the key line, the confirm, editing and answer prompts, the question line, the running line and the editing line's width rule
@task_file n=6 path=src/human/fleet.rs
what: the console's fleet lines drawn from the board: each wrapped row's name, its selection gutter and its pending dialog
@task_file n=7 path=src/human.rs
what: the human output module's re-exports of the console and fleet copy
@task_file n=8 path=src/cmd/console.rs
what: the viola console verb's entry: starts the console with its action runner
@task_file n=9 path=src/cmd/list.rs
what: the viola list verb: a board item literal gaining the dialog id, its output unchanged
@task_file n=10 path=src/human/board/test_support.rs
what: the board rendering's test fixtures: a board item literal gaining the dialog id
@task_file n=11 path=crates/viola-core/src/obs.rs
what: the closed list of diagnostics events and their fields, now with console-action and the console-key count
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board read of the fleet: each item's pending dialog kind and now its id, not serialised
@task_file n=13 path=schemas/diag-line.v1.json
what: the diagnostics line schema: the event list, the console-action arm and the console-key arm's words and count
@task_file n=14 path=schemas/diag-detail.v1.json
what: the diagnostics detail schema's event list, now with console-action
@task_file n=15 path=tests/contract_diag_schema.rs
what: the contract check of the diagnostics schemas against the closed event list
@task_file n=16 path=tests/tui_console.rs
what: the end-to-end check of the fleet console under a PTY over the fakes: selection, scroll, every action, the editing line, the answer prompt, Ctrl-C from every mode
@task_file n=17 path=Cargo.toml
what: the workspace manifest: the unicode-width dependency added to the root bin
@task_file n=18 path=Cargo.lock
what: the dependency lock: the root package's unicode-width line, no crate added
@task_item n=1 part=touches
decision/console-frame-floor: its last row is no longer q quit alone but the full key line, q quit first; at the 80-column floor the line is cut after m swi so a answer and o open are not shown; the row must say whether the floor's line shows every letter (an open operator fork, as built while unanswered).
@task_item n=2 part=touches
decision/console-own-surface: the console is no longer view-only; it takes a selection, a confirm, an answer prompt and one editing line, and its key line now holds every binding; the editing line is a named exception to plain ASCII output (UTF-8 echoed as typed, counted 1, 2 or 0 columns a scalar).
@task_item n=3 part=touches
decision/board-vs-strip: the console now acts on the selected row through the strip verbs (pause, release, focus, last, send, switch, open, answer, up), stale where it keeps the console a reading of the board alone.
@task_item n=4 part=touches
decision/cli-holds-every-action: the console holds no action of its own; every key runs the CLI verb as a child, so the rule holds and gains the console as a caller.
@task_item n=5 part=touches
decision/no-prompt-or-pager: the console now draws a y/n confirm for release and revive, an answer prompt and an editing line; an exception on the console surface, the CLI verbs still take no interactive prompt.
@task_item n=6 part=touches
decision/no-persistent-footer: the console's last row, the key line or the editing line in its place, stands on every frame; the console's own surface, not a CLI verb's footer.
@task_item n=7 part=touches
decision/console-fleet-body-lines: the body lines gain a two-column selection gutter marking the selected wrapped row alone, a ... <n> above line when scrolled down, and an action's result rows laid in the lowest rows with the selection kept above them.
@task_item n=8 part=touches
decision/console-redraw-on-change: the frame now also redraws on a selection move, a mode change, a child's start (the running line) and its finish (its result lines).
@task_item n=9 part=touches
decision/console-agents-read-off-loop: the action child runs off the key loop too, one at a time; a key for another action while one runs reads busy.
@task_item n=10 part=touches
contract/cli-verbs/console: viola console now acts: every listed key, the confirm, the editing line and the answer prompt, with Ctrl-C ending every mode.
@task_item n=11 part=touches
decision/board-row-escaping: the child's output lines and the editing line's echo are drawn through the same row escaping.
@task_item n=12 part=touches
decision/targets-are-wrapped-names: the selection moves over wrapped rows only and skips unwrapped rows and the caption, so every console action targets a wrapped name.
@task_item n=13 part=touches
contract/cli-verbs/send and decision/prompt-text-from-stdin-or-file: the console's s hands the typed line to viola send on stdin unchanged; an empty line is the verb's empty-text refusal, exit 13 (contract/send-refusals/empty-text, contract/cli-exit-codes/13).
@task_item n=14 part=touches
contract/cli-verbs/switch: the console's m splits the typed words, an effort word as --effort=, any other as --model=, never as flags; a bad word is the verb's exit 2.
@task_item n=15 part=touches
contract/cli-verbs/open and contract/viola-open/parent-default: the console's o runs open with the typed name as one element and --parent the selected row's name, never the default.
@task_item n=16 part=touches
contract/viola-answer-output and contract/mcp-tools/answer: the console's a runs viola answer with the board's dialog id, y allow or n deny on a permission, y approve or n revise on a plan; the answer verb's output is drawn as result lines.
@task_item n=17 part=touches
decision/permission-request-question-answerable: the console does not answer a question; on a question a draws one line, name then question - answer it in the session (f), and runs nothing.
@task_item n=18 part=touches
contract/viola-send-output/unable and decision/shared-refusal-word-column: a refused action's lines are the verb's own output drawn in the console's result rows, its exit logged.
@task_item n=19 part=touches
contract/cli-verbs/focus, contract/cli-verbs/last, contract/cli-verbs/up, contract/wheel-verb-output/pause and contract/wheel-verb-output/release: each now has the console as a caller (f, l, v, p, r), its output drawn as result lines.
@task_fact n=1 slug=surface-console
fact: The console's key line reads q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open, q quit first; at the 80-column floor it is cut after m swi, so a answer and o open are not shown there, as built while the operator's fork stands open.
@task_fact n=2 slug=surface-console
fact: The editing line (send, switch, open) takes the key line's place and takes no second cut: prompt then the text echoed as typed through row escaping, each scalar 1, 2 or 0 columns; past the edge the prompt, then ..., then the longest fitting tail, a straddling wide scalar and a width-0 head left out; the one exception to plain ASCII on the console.
@task_fact n=3 slug=surface-console
fact: The selection is a two-column gutter on the selected wrapped row alone; it keeps its name when the fleet changes, a gone name takes the row at its place clamped, and the body scrolls to follow it with a ... <n> above line and the more line.
@task_fact n=4 slug=surface-console
fact: An action's output lines are laid in the lowest rows of the body with the selection kept above them; a running child shows one running line until it finishes.
@task_fact n=5 slug=surface-console
fact: Every console mode is left by one key (n or escape from a confirm, escape from the editing line and the answer prompt) and Ctrl-C ends the console from every mode.
@task_fact n=6 slug=surface-console
fact: The console's prompts are fixed: send name: , switch name: , open child of name: , answer name permission? y allow  n deny, answer name plan? y approve  n revise; a with nothing pending is unbound.
@task_fact n=7 slug=surface-console
fact: Absence: the console does not adopt a session; adopt from the console is owned by no route entry and would be a new entry on the operator's word.
@task_item n=1 part=untouched
The web surface absence: the session adds no page.
@task_item n=2 part=untouched
viola verify output and its refusals: the verify verb is not reached.
@task_item n=3 part=untouched
viola link and unlink output: neither verb is called by the console nor changed.
@task_item n=4 part=untouched
The frame verbs' exit-21 hints and the exit-21 --json detail rows: no hint or detail was added or reworded.
@task_item n=5 part=untouched
viola wait output: the console does not wait and the verb is unchanged.
@task_item n=6 part=untouched
The CLI deep links absence: the console adds no link or URL.
@task_item n=7 part=untouched
viola --help and per-verb usage output: no verb, flag or usage line was added.
