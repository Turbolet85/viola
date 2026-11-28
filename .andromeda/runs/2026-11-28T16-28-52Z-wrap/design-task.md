@task_prose part=session
The session gave the fleet console its actions: a selected row, every verb by keyboard, each run as a child CLI verb (pause, release, focus, last, revive as up, send, switch, open, answer), its output drawn as result lines and refusals as the verb's own unable lines.
It added a selection gutter over wrapped rows, an above line when the list scrolls, confirm prompts for release and revive, one editing line for send, switch and open, and an answer prompt for a pending permission or plan, with a question line that runs nothing.
The console key line now reads q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open; measured at the 80-column floor it is cut after m swi, so a answer and o open do not show there (left to the operator as a fork, as built).
The editing line echoes the typed text as typed, UTF-8 included, with control characters escaped, widths by unicode-width, and past the edge the prompt, then ..., then the longest tail that fits.
It added the console-action log line and a count on the console-key line, grew the console-key word set from 3 to 14 and the closed event list from 18 to 19; the board item gained a pending dialog id that list, list --json and the MCP list item do not show.
It retired the scaffold's one-class key reading and the key line that held q quit alone; the chunk stays GATED on the operator's live reading.
@task_file n=1 path=src/console/action.rs
what: the fleet console's action runner: the verbs it can run, each as a child of the own viola executable with --home, no shell, the instance names removed from its environment, send's text and answer's body on stdin, each output stream bounded and every line escaped
@task_file n=2 path=src/console/keys.rs
what: the fleet console's key reader: the bounded raw-stdin decoder and its closed key classes (Ctrl-C, arrows, Enter, Escape, Backspace, a character, unbound), the escape wait and the read cap
@task_file n=3 path=src/console/mod.rs
what: the fleet console's run loop and state: its modes (list, confirm, editing line, answer prompt), the selected row, one action at a time, busy, the per-read key fold into the console-key log lines, the console-action log line per finished child
@task_file n=4 path=src/console/view.rs
what: the fleet console's view: the frame of fleet lines, the selection and its follow and move rules, the scroll window with its more line and above line, the result rows, the fitted key line
@task_file n=5 path=src/human/console.rs
what: the fleet console's fixed human copy: the key line, the confirm prompt, the running line, the editing prompts, the answer prompts, the question line, and the editing line's width rule
@task_file n=6 path=src/human/fleet.rs
what: the fleet console's fleet lines drawn from the board: each wrapped row's name, its selection gutter and its pending dialog kind and id
@task_file n=7 path=src/human.rs
what: the human-output module's re-exports of the console and fleet-line helpers
@task_file n=8 path=src/cmd/console.rs
what: the viola console verb's entry: it builds the action runner over the own executable and home and starts the console
@task_file n=9 path=src/cmd/list.rs
what: the viola list board command; its board item literal gains the pending dialog id, its output unchanged
@task_file n=10 path=src/human/board/test_support.rs
what: the board rendering's test support; its board item literal gains the pending dialog id
@task_file n=11 path=crates/viola-core/src/obs.rs
what: the closed list of diagnostic log events, now with console-action, and the console-key event's doc naming its count
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board read of the fleet: each item gains the pending dialog's id from the snapshot, kept out of serialisation beside the dialog kind
@task_file n=13 path=schemas/diag-line.v1.json
what: the diagnostic line schema: the event enum, the console-action arm with verb and exit_code, the console-key arm's grown key word set and its count field
@task_file n=14 path=schemas/diag-detail.v1.json
what: the diagnostic detail schema's event enum, now with console-action
@task_file n=15 path=tests/contract_diag_schema.rs
what: the diagnostic schema contract check over the closed event list
@task_file n=16 path=tests/tui_console.rs
what: the fleet console's end-to-end checks under an outer terminal: selection, scroll, every action verb, confirms, the editing line, the answer prompt, the question line, Ctrl-C from every mode
@task_file n=17 path=Cargo.toml
what: the workspace and root binary dependency list, gaining unicode-width
@task_file n=18 path=Cargo.lock
what: the dependency lock, gaining only the root package's unicode-width line
@task_item n=1 part=touches
decision/list-is-the-board goes stale or needs an addition: the fleet console now acts on a selected strip through the CLI verbs, so the board is no longer read-only display alone.
@task_item n=2 part=touches
contract/fleet-console-keys/unbound is stale: j k, the arrows, l p r f v s m a o, y n and Escape are now bound; any other key is still unbound, and a with nothing pending reads unbound.
@task_item n=3 part=touches
contract/fleet-console-keys/q: q still quits, now from the list mode, and the key line it heads now lists every binding with q quit first.
@task_item n=4 part=touches
contract/fleet-console-keys/ctrl-c: Ctrl-C now ends the console from every mode (list, confirm, editing line, answer prompt), the terminal restored.
@task_item n=5 part=touches
contract/fleet-console-view/more-line gains its counterpart: a ... <n> above line when the selection scrolls the list down; no more line at the end, no above line near the top.
@task_item n=6 part=touches
contract/fleet-console-view/line-cut: the key line is cut at the frame width, at 80 columns after m swi so a answer and o open do not show; the editing line is fitted by its own width rule and takes no second cut.
@task_item n=7 part=touches
decision/fleet-console-not-verb-output: the console now draws the verbs' own output and unable lines as its result rows; whether those lines keep their verb-output form inside the console is for the row to say.
@task_item n=8 part=touches
decision/no-interactive-prompt-or-pager: the console now holds a y/n confirm, an answer prompt and one typed editing line; an exception or a scope statement for the console is owed.
@task_item n=9 part=touches
decision/prompt-text-from-stdin-or-file: send's text can now be typed in the console's editing line, which hands it to viola send on stdin unchanged.
@task_item n=10 part=touches
decision/cli-ascii-only and decision/ascii-only-human-output: the editing line echoes the typed text as typed, non-ASCII included (a Cyrillic line proven), a named exception to ASCII-only output.
@task_item n=11 part=touches
decision/escape-controls-in-human-output and decision/list-rows-escape-all-controls: every line of an action child's output and the editing echo pass through the same row escaping.
@task_item n=12 part=touches
decision/confirmations-carry-context: release and revive ask a confirm that names the verb and the session; y runs it, n or Escape runs nothing.
@task_item n=13 part=touches
decision/pause-release-transfer-control: pause and release can now be pressed from the console on the selected row, release behind a confirm.
@task_item n=14 part=touches
contract/cli-refusal-line/fixed-unable-form and contract/send-readback/unable: a refused action's own unable lines are drawn in the console as its result, the verb's exit code logged.
@task_item n=15 part=touches
contract/send-readback/prompt and contract/viola-send-mirror/read-back: a console send's readback is the verb's own, drawn as the console's result; nothing announces success before the verb returns.
@task_item n=16 part=touches
decision/dialog-pending-cocked-strip and decision/permission-request-question-held: the answer prompt opens on a cocked strip's permission or plan; a question is not answered from the console but sends the human to the session.
@task_item n=17 part=touches
contract/cli-verb-output/answer: the console answers a permission with allow or deny and a plan with approve or revise through viola answer with the board's dialog id.
@task_item n=18 part=touches
decision/bare-strips-never-target: the console's selection moves over wrapped rows only, skipping unwrapped rows and the caption.
@task_item n=19 part=touches
decision/stable-session-slots and decision/no-auto-sorting-strips: the selection keeps its name when the fleet changes, and a gone name takes the row at its place, clamped.
@task_item n=20 part=touches
decision/no-spinner and decision/busy-not-animated: a running child is shown by one static running line, and another action key while it runs reads busy.
@task_item n=21 part=touches
decision/truncate-only-unwrapped-name and decision/no-hardcoded-widths-or-wrapping: the editing line past the frame edge is cut to the prompt, ..., and the longest fitting tail, a second truncation besides the unwrapped NAME.
@task_item n=22 part=touches
decision/lines-never-wrapped: the editing line and the key line are each cut to the frame width, never wrapped.
@task_fact n=1 slug=surface-console
fact: The fleet console binds j and k and the arrows to move the selection, and l last, p pause, r release, f focus, v revive (viola up), s send, m switch, a answer, o open, each run on the selected row as its CLI verb in a child, one at a time.
@task_fact n=2 slug=surface-console
fact: The selected row is marked by its gutter alone; the selection moves over wrapped rows only and none stands on a board with no wrapped row.
@task_fact n=3 slug=surface-console
fact: Release and revive ask a confirm naming the verb and the session; y runs it, n or Escape leaves it and runs nothing.
@task_fact n=4 slug=surface-console
fact: s, m and o open one editing line with the prompt send <name>: , switch <name>: or open child of <name>: ; Enter hands the text to the verb, Escape cancels and runs nothing.
@task_fact n=5 slug=surface-console
fact: switch's typed words are values, an effort word as the effort and any other word as the model, never flags; open's typed text is the child's name as one element, its parent the selected session.
@task_fact n=6 slug=surface-console
fact: The editing line echoes the text as typed, UTF-8 included, controls escaped, each scalar 1, 2 or 0 columns by unicode-width; past the edge it draws the prompt, ..., and the longest tail that fits, a straddling wide scalar left out.
@task_fact n=7 slug=surface-console
fact: a on a pending permission opens answer <name> permission? y allow  n deny, on a plan answer <name> plan? y approve  n revise; Escape leaves it answering nothing.
@task_fact n=8 slug=surface-console
fact: a on a pending question draws the one line <name>  question - answer it in the session (f) and runs nothing; no question or option text is read or shown.
@task_fact n=9 slug=surface-console
fact: An action child's output lines are the console's result rows, each escaped; a refusal shows as the verb's own unable lines, never a console-made message.
@task_fact n=10 slug=surface-console
fact: While a child runs a static running line stands and any other action key reads busy; no second child starts.
@task_fact n=11 slug=surface-console
fact: The key line reads q quit  jk move  l last  p pause  r release  f focus  v revive  s send  m switch  a answer  o open, q quit first; at the 80-column floor it is cut after m swi, as built while the operator's fork on it stands unanswered.
@task_fact n=12 slug=surface-console
fact: Every console mode is left by one key (n or Escape from a confirm, Escape from the editing line and the answer prompt), and Ctrl-C ends the console from every mode.
@task_fact n=13 slug=surface-console
fact: The console has no adopt action; adopt from the console is owned by no route entry.
@task_item n=1 part=untouched
Colour: the colour palette, the amber accent, the CLI colour tokens, colour depth and the colour decision order; the console's new lines are plain text with no colour.
@task_item n=2 part=untouched
Typography, spacing, depth strategy, border radius and the rendered-page and web-surface absences: no page and no web surface were added.
@task_item n=3 part=untouched
CLI exit codes and the exit-21 causes: no code was added or moved; the console only logs a verb's own exit code.
@task_item n=4 part=untouched
The refusal, output and json contracts of run, revive, up, verify, link, unlink, focus, last, wait and the wrapper fault detail: no verb's own output changed.
@task_item n=5 part=untouched
The command names and viola --help verb groups: no CLI verb was added, renamed or freed; the console's letters run the existing verbs.
@task_item n=6 part=untouched
The refusal hints and peer reply screening: no hint and no screened code word was added or changed.
@task_item n=7 part=untouched
The brand direction, expression level, style presets and the self-validation protocol tests: no decision of style was changed.
@task_item n=8 part=untouched
viola console's own start refusals (no terminal, home failing its read rule, home widened mid-run): unchanged.
