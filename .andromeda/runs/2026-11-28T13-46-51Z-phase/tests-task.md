@task_prose part=chunk
The viola console gains actions over the fleet board view: a selection of one board row and the verbs reached from it by keyboard alone.
Each action acts through the CLI verb it names, with the same outcome and the same refusal as that verb typed at a shell, and no second code path (matrix v2-15: open a child, focus, send, answer, last, pause, release, revive; and the v2-31 console leg, the model and effort switch).
A send shows its readback, which is the send frame verb's own reply. A refusal shows as the verb's unable line on the console's own surface. Every mode the actions add is left by keyboard alone.
The scaffold's decoder, which reads one class per read, is replaced by a decoder of key sequences where the actions need it. The console's terminal restore case is re-asserted on the console's own terminal.
@task_item n=1 part=needs
The board selection: one row of the viola console board is selected and named by the board's item (the listed instance), never by the drawn text. A cut line ends with an ellipsis and an unwrapped name is not cut on its own, so the drawn text cannot identify the row.
@task_item n=2 part=needs
An unwrapped row's name is never a target, and a frame verb reaches only the wrapper its instance snapshot records. A console action from a selected row is bound by both rules, through the CLI verb it calls.
@task_item n=3 part=needs
Actions through the CLI verbs: each console verb has the same outcome, refusal, refusal order and exit cause as the CLI verb at a shell. That covers the frame verbs checking the instance first, the strict-modes and socket-directory refusals, and the instance-unreachable causes. There is no second code path for an action.
@task_item n=4 part=needs
The send action's readback is the send frame verb's own reply: the confirmed read-back line, the unconfirmable line or the refusal (for example not-delivered, human-typing, turn-running, manual-pause, empty-text). It is never the board's STATUS cell, which comes from the agents listing at most every 5 s and reads unknown after a failed read.
@task_item n=5 part=needs
Refusals on the console surface: an action's refusal appears as the verb's human unable line, worded as the CLI's human lines word it. Human output escapes control characters, and error text holds no path or upstream text.
@task_item n=6 part=needs
The send action's text entry: the text sent goes through the send verb. Control characters in pasted text are refused, the text is checked at the client and at the wrapper, multi-line text is one prompt, and a leading slash is never taken as a prompt. Empty text is refused before the wheel is read.
@task_item n=7 part=needs
The answer action reaches viola answer: the dialog answer mapping, the unknown-dialog and unverified-cli refusals, and the refusal when the human holds the wheel are what the console shows.
@task_item n=8 part=needs
The pause and release actions and the wheel: pause forces the human wheel. The wheel returns only through viola release, and a release from a driver is refused. The console is driven by the human and may reach release as a human verb.
@task_item n=9 part=needs
The last and revive actions reach viola last and viola revive: the newest turn message, and revive's refusal codes and exit causes as the console's result and unable lines.
@task_item n=10 part=needs
The open-a-child and focus actions reach the CLI verbs that open and raise a session window. A real window opening, placement or raise is proven only by a gated live reading, never in this chunk's first pass.
@task_item n=11 part=needs
The switch action (v2-31 console leg): the model and effort switch is reached from the console by keyboard alone, through the viola switch CLI verb, which is CLI-only. Its taking effect on the next turn is the real CLI's, carried elsewhere.
@task_item n=12 part=needs
The key sequence decoder replaces the one-class-per-read decoder: 0x03 anywhere is still an interrupt, exactly q still quits, and anything unbound stays unbound. As a parser surface it owes property tests on its invariants.
@task_item n=13 part=needs
No key trap: every mode an action adds (a selection, a text entry, any confirm) is left by a key. The console's interrupt and quit still restore the terminal.
@task_item n=14 part=needs
Console restore, re-asserted: the line mode is raw while the frame stands and cooked after the quit, read on the console's own terminal. It is never asserted that the mode after equals the mode before.
@task_item n=15 part=needs
Reaching a row: rows past the body show only as one more-rows line and no scroll exists, so a row reached by keyboard needs the view to move to it. Beyond that, the view's content changes only as far as a selection and an action's result need.
@task_item n=16 part=needs
The strict-modes refusal of viola console stays as it is, whichever verbs the console now reaches.
@task_item n=17 part=needs
How console tests are proved: a keystroke driver over the console's own pseudo-terminal reads state, never the rendered screen. A key sent after a resize waits for the new size. Verdicts come from records and logged events, never from screen content.
@task_item n=18 part=needs
Console tests keep the test-scoped PATH and never read the host's claude sessions, real ~/.claude or ~/.viola. They run over the fake agent, never the real claude CLI.
@task_item n=19 part=needs
Live reading: no live reading is made. A capability clause that needs one is claimed gated, and any reading is recorded as read, never as verification.
@task_item n=20 part=needs
Wheel effects of the console's own input: focus, mouse and resize never take the wheel, and terminal replies are not editing. Any human-input wheel record a console action causes is the verb's own record.
@task_standing n=1
convention: every new test creates its own private home and starts viola tied to it, with inherited session and agent variables dropped
home: integration-test-strategy
@task_standing n=2
convention: every new test waits on logged events and never syncs by sleep or retry
home: 2-test-strategy
@task_standing n=3
convention: every new test follows the test naming rule and puts its unit cases in rstest tables, inline with the code
home: 2-test-strategy
@task_standing n=4
convention: every new test's oracles are written as literals, and human-mode output is pinned by literal asserts
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: every new test input is synthetic, carries a canary, and holds no real data
home: 7-test-data-fixtures
@task_standing n=6
convention: every new test runs in parallel with no shared mutable state and no process-wide env mutation
home: 11-test-anti-patterns-never-do-these
@task_standing n=7
convention: every new log line is a structured JSON line that carries no secrets
home: log-format
@task_standing n=8
convention: every new test counts toward the coverage floors and the zero-flakiness budget
home: 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
The MCP server surface and its tools, tool results and invalid-params fields: console actions go through the CLI verbs, not MCP.
@task_item n=2 part=not_needed
Hook exec entry points, hook decision bodies and hook deadlines: the chunk adds no hook behaviour.
@task_item n=3 part=not_needed
viola run start and passthrough: the child spawn, the child environment strip, pinned-exe hashing, plugin rewrite and the start events. The chunk starts no wrapper of its own.
@task_item n=4 part=not_needed
viola verify, stamps, fixture recording and its scrub refusals: the chunk records no fixture and writes no stamp.
@task_item n=5 part=not_needed
The harness command contracts (boot, cleanup, status, supervise, the pre-push stages and the mutation run document): the chunk changes no harness command.
@task_item n=6 part=not_needed
CI artifact names and per-OS artifact contracts: the chunk adds no CI job or artifact.
@task_item n=7 part=not_needed
Performance budgets for hooks and hyperfine invocation: the console actions are not timed hooks.
@task_item n=8 part=not_needed
The wrapper channel's served link and unlink methods and its JSON-RPC error bodies: the console reaches them only through the CLI verbs' own handling.
@task_item n=9 part=not_needed
Absences of a database, outbound HTTP, a browser suite and Windows or macOS support: none is reached.
