@row_set n=1 row=contract/process-exit-detail/exit-1 cell=promise seen=1 fact=F2 detector=D-obs-instrumentation
value: already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, from revive's preflight no-session, session-live, cwd-missing, and from viola open terminal-not-found and not-live; each one process-exit line with no child started, bar not-live's terminal.
basis: fct-wrqnldxo, fct-xxzbm355
because: The process-exit detail enum gains terminal-not-found and not-live, both exit-1 causes written by viola open, so the closed exit-1 list must name them.
@row_set n=2 row=contract/run-start-spans/span-chain cell=promise seen=2 fact=F3 detector=D-obs-instrumentation
value: run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT); under --origin, run.window follows the trust step, before state.snapshot_write.
basis: fct-xxzbm355, fct-ofc4imyq
because: The run start gains the run.window span, entered only under the hidden --origin flag after the trust step and before the slots and the first snapshot, so the span chain must hold it.
@row_set n=3 row=contract/diag-subject/hyprctl-probe cell=promise seen=1 fact=F6 detector=D-obs-instrumentation
value: Subject of the process-start and process-exit pair that the Hyprland seam writes around each hyprctl spawn, among them viola open's parent window read and the wrapper's window step at a start under --origin.
basis: fct-uspycncu, fct-m35lqujh
because: The Hyprland seam's product callers move from none to two, so hyprctl-probe pairs are now written by product verbs and the row must say which.
@row_set n=4 row=contract/process-log/child-and-probe-lifecycle cell=promise seen=2 detector=D-obs-instrumentation
value: process-start and process-exit are logged for the claude child, for the claude agents --json and --version probes, for verify's print-mode sender, and for the terminal viola open spawns.
basis: fct-m35lqujh, fct-wrqnldxo
because: viola open spawns a terminal logged by a process-start and process-exit pair with subject terminal, so the list of logged spawns is incomplete without it.
@row_set n=5 row=contract/strict-modes-refusal-exit/1 cell=promise seen=1 fact=F8 detector=none
value: run, revive's start, verify and open exit 1 on a strict-modes refusal; revive and open under --json print the strict-modes-failed document.
basis: fct-xxzbm355
because: viola open refuses a strict-modes check before any log with exit 1 and, under --json, the strict-modes-failed document, so the exit-1 refusal row must name it.
@row_add n=1 row=contract/diag-subject/terminal area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: diag-line subject enum
item: terminal
promise: Subject of the process-start and process-exit pair of the foot terminal window that viola open spawns, written into the cli role file of the opened name.
basis: fct-wrqnldxo, fct-xxzbm355
because: The diag-line subject enum gains terminal, and each subject of that closed enum stands as a row of its own.
@row_add n=2 row=contract/run-span-attributes/run-window area=span-trace-coverage detector=D-obs-instrumentation
boundary: viola run start span attributes
item: run.window
promise: run.window carries the field outcome alone, never an address, pid or workspace.
basis: fct-xxzbm355, fct-m35lqujh
because: The new run.window span declares only the field outcome, and each start span's attributes stand as a row of their own.
@row_add n=3 row=contract/cli-log-file/open area=span-trace-coverage fact=F4 detector=D-obs-instrumentation
boundary: CLI process-log file
item: viola open
promise: Logs as cli into the opened instance's cli-<name>.ndjson, its work inside the cli.open dispatch span.
basis: fct-xxzbm355
because: The new verb viola open opens the cli.open dispatch span and logs into the cli role file of the name it opens, which no standing row of the cli log files names.
@row_add n=4 row=decision/window-line-untraced area=span-trace-coverage fact=F5 detector=D-obs-instrumentation
name: Window line has no trace of its own
rule: The window line a start under --origin writes after its launch line is log-only product audit data: it wakes no wait, opens no span and is never copied into a process-log line.
state: provisional
basis: fct-m35lqujh, fct-wrqnldxo, fct-xxzbm355
because: The chunk adds a window event line kept untraced as the launch line, a decision future work on the start's product lines must respect.
@row_add n=5 row=contract/run-start-events/window-line area=obs-scope-summary detector=D-obs-instrumentation
boundary: run start events in events.ndjson
item: window line
promise: A start made under --origin writes one window line {address, workspace, placement, origin} with source wrapper after its launch line.
basis: fct-wrqnldxo, fct-xxzbm355
because: The start's product order gains a window line after launch under --origin, and the standing order row cannot take it within its bound.
@row_add n=6 row=decision/origin-word-self-reported area=pii-scrubbing-compliance fact=F7 detector=D-obs-pii
name: Origin word self-reported, not an identity
rule: The origin word (cli or agent) on a start's window record is self-reported by its caller and never treated as an identity.
state: provisional
basis: fct-ofc4imyq, fct-xxzbm355
because: The window record gains an origin word the caller states for itself, and reading it as an identity would contradict the self-reported rule.
@row_add n=7 row=decision/window-record-confined area=pii-scrubbing-compliance detector=D-obs-pii
name: Window record kept out of process logs
rule: A start's window record (address, workspace, placement, origin) stands only in events.ndjson and the snapshot, its address, workspace and origin also on list --json and the MCP list result, never in a process-log line.
state: provisional
basis: fct-wrqnldxo, fct-m35lqujh, fct-ofc4imyq
because: The chunk records a host window address and workspace per start and keeps them to the product surfaces, with no address, pid or workspace in a process-log line.
