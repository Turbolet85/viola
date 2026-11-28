@task_prose part=chunk
The `viola console` fleet console gains actions over the board view the fleet-console-view chunk built: a selection of one board row, and the verbs reached from it by keyboard alone.
Each action acts through the CLI verb it names, with the same outcome and the same refusal as that verb typed at a shell, and no second code path.
A send shows its readback, which is the send frame verb's own reply. A refused action shows the verb's `unable` line on the console's own surface. Every mode the actions add is left by keyboard alone, so there is no key trap.
A key decoder that reads key sequences replaces the scaffold's one-class-per-read decoder wherever the actions need it. The console's raw/cooked terminal restore is re-proved on the console's own terminal.
Which verbs are offered, their keys, the selection model, the send's text entry, any confirm and any scroll are open operator forks. Capability clauses that need a live reading are claimed gated.
@task_item n=1 part=needs
The terminal fleet console as the one view: actions are added to the existing board view, and the view's content changes only as far as a selection and an action's result need.
@task_item n=2 part=needs
The selection names its row by the board's item, meaning the instance key (the viola name) of the board's list payload item, never by the drawn text. A cut line ends in `...` and an unwrapped name is not cut separately.
@task_item n=3 part=needs
Rows past the body show only as one `... <n> more` line and no scroll exists yet. A row reached by keyboard needs the view to move to it, which affects how the board is drawn around a selection.
@task_item n=4 part=needs
Each action goes through the CLI verb it names, so the same outcome, exit code and refusal come out of the console as from the shell: the verbs open, focus, send, answer, last, pause, release, revive and switch. There is no second code path for an action.
@task_item n=5 part=needs
Indirect: whether the console calls a verb in-process or as a child run is measured at P3. Either way the console's own code must stay inside the no-shell-out rule, and a child run must be spawned directly with a closed argv. The rule that new capabilities arrive as subcommands or channel verbs still holds: the console adds no verb of its own.
@task_item n=6 part=needs
A send's readback is the send frame verb's reply (`ok`, the unconfirmable ok payload, or a refusal with its closed-set detail). It is never the view's STATUS cell, which comes from the agents thread at most every 5 s and reads `unknown` after a failed read. The readback may take longer than the view's one-second re-read.
@task_item n=7 part=needs
Indirect, through the send verb: every send is confirmed after the fact and is never queued. A send mid-turn or mid-send is refused, and the console must show that refusal rather than retry or hold the send.
@task_item n=8 part=needs
The send's text entry hands its text to the send verb under its rules: line-ending normalisation, control characters refused (never stripped), empty text refused, the paste bound, no trailing newline typed.
@task_item n=9 part=needs
A refused action shows as the verb's human `unable` refusal line on the console's surface. This covers the refusal reasons and their closed detail set, the first-refusal-only order, exit-21 unreachable causes and their human hints, and the wrapper-fault human error line.
@task_item n=10 part=needs
Indirect: text the console shows from upstream (a reply, a refusal, a dialog's option text) is untrusted content. It goes through the sanitised error surfaces, and option text stays out of human lines.
@task_item n=11 part=needs
The human controls are CLI-only: switch, pause, release, link/unlink, and focus (which has no driver form). The console, driven by the human, reaches them through the CLI verb. Switch reaches its console leg this way.
@task_item n=12 part=needs
Wheel rules the console's actions act under: the human always wins the wheel, and the wheel returns only through an explicit CLI release. Release is a human verb (deterred, not enforced, from a driver); whether the console may reach it is read at P3.
@task_item n=13 part=needs
The answer action answers a pending dialog through the answer verb: permission, plan and question answer responses, the unknown-dialog refusal, and permission answers without suggestions.
@task_item n=14 part=needs
The open, focus and revive actions reach their verbs' outcomes and refusals: the open exit codes, the revive refusals and error codes, and the Hyprland seam errors as the focus verb reports them.
@task_item n=15 part=needs
The key decoder reads key sequences instead of one class per read of at most 64 bytes. 0x03 anywhere still interrupts and `q` still quits. Every mode the actions add (selection, text entry, any confirm) has a keyboard way out.
@task_item n=16 part=needs
Terminal restore: the line mode is raw while the frame stands and cooked after the quit, read on the console's own terminal, never as the mode after equalling the mode before.
@task_item n=17 part=needs
Indirect: the console writes its surface through the root bin's one human writer, under the print-macro bans. Colour variables decide only SGR output.
@task_item n=18 part=needs
Indirect: waiting for a readback or an action's result blocks on the verb and never polls the channel. The board's own re-read cadence stays as built.
@task_item n=19 part=needs
How the chunk is proved: the keystroke driver over the console's own pseudo-terminal reads state, never the screen (what state is measured at P3). Every new console test start keeps the test-scoped PATH and never reads host sessions. Tests run only against the fake agent, with no live start, no window, no real claude and no real home read.
@task_item n=20 part=needs
Capability claims: the verbs-from-the-console capability and the console leg of the model-and-effort switch are claimed only as far as the fake-agent tests prove them. A clause that needs a live reading is claimed gated, left to the operator. The switch's next-turn effect stays carried.
@task_standing n=1
convention: each new test runs in its own isolated viola home against the fake agent, never against a live claude
home: established-decisions
@task_standing n=2
convention: each new test's homes, sockets and child processes are owned by the test process and cleaned by owner record, with channel sockets under the short test root outside the product namespace
home: occupied-resources
@task_standing n=3
convention: each new end-to-end test run leaves zero panic lines, and any snapshot assertion runs in check mode only
home: project-directory-structure
@task_standing n=4
convention: each new test that is OS-gated names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=5
convention: each new log line is a codes-only process-log line emitted through the one event emitter, at the diagnostics level as the sole level source
home: cross-cutting-patterns · build-system
@task_standing n=6
convention: each new test seam uses the named fake-agent and harness variables, never a configuration channel the product reads
home: conventions
@task_standing n=7
convention: each new module is wired into the module tree and new code keeps the crates above their coverage floors
home: build-system · ci-cd-approach
@task_item n=1 part=not_needed
MCP server and MCP tool rows (tool results, stdin line caps, call deadlines, the tools list). The console drives the CLI verbs, never the MCP server.
@task_item n=2 part=not_needed
Hook rows: hook tiers, hook contract and exit codes, the hook-to-kind map, hook diagnostics, the dialog hold. The console neither runs nor answers through a hook.
@task_item n=3 part=not_needed
The verify and capability-ledger machinery: ledger stamps, verify refusal and exit codes, trust writer code words, recorded fixtures. The console runs no verify and writes no stamp.
@task_item n=4 part=not_needed
Wrapper-side internals behind the verbs: the PTY seam, the vt100 readiness gate, screen signatures, the child's argv and environment, the pinned binary copy and the run start order. The console only calls the verbs.
@task_item n=5 part=not_needed
Writer rows for wrapper event appends, instance snapshot fields and the events log. The console appends no event and writes no snapshot of its own.
@task_item n=6 part=not_needed
Plugin rows: the embedded plugin, plugin install and its marketplace tree, and the plugin's MCP config. The console installs or loads no plugin.
@task_item n=7 part=not_needed
Deployment and release rows: local install, the release workflow, crates.io publishing, containers and hosting. This chunk ships nothing.
@task_item n=8 part=not_needed
Status-line interception, a usage-budget gate and model API calls. The console reads no usage or status line and calls no model.
