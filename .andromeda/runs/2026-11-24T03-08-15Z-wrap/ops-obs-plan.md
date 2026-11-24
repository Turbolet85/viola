@row_add n=1 row=contract/cli-log-file/focus area=4-span-trace-coverage fact=F1 detector=D-obs-instrumentation
boundary: CLI process-log file
item: viola focus
promise: Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a closed-detail process-exit inside the cli.focus dispatch span, which carries no fields.
basis: fct-yz4mggmo, fct-p6tk5bfr
because: The chunk adds the viola focus verb, instrumented under the manual span cli.focus into the cli role file (provisional on fork F-A), and obs-plan holds a cli-log-file row per such verb beside open's.
@row_add n=2 row=contract/process-exit-detail/exit-1-focus area=6-log-coverage fact=F2 detector=D-obs-instrumentation
boundary: process-exit detail
item: exit 1 from viola focus
promise: viola focus ends exit 1 with one process-exit detail: window-not-found (no window of the session, nothing dispatched), unsupported (not on Hyprland, nothing spawned), unreachable (no compositor answering, provisional), or the hyprctl seam's oversize, malformed or refused.
basis: fct-yz4mggmo, fct-vviqcehe
because: The diag-line schema's process-exit detail enum gains focus's six exit-1 words, and the standing exit-1 row would pass its bound and its no-child-started clause does not hold where focus has spawned hyprctl.
@row_add n=3 row=decision/focus-names-no-window area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
name: Focus names no window, pid or workspace
rule: viola focus never prints or logs the window address, a pid or a workspace, in success or in refusal.
basis: fct-yz4mggmo, fct-p6tk5bfr
because: The chunk's new verb reads window records through hyprctl, and the report states none of the window's identifiers reaches any output or log line.
@row_add n=4 row=decision/focus-ancestry-walk-unlogged area=4-span-trace-coverage fact=F4 detector=none
name: Focus ancestry walk held in memory, never logged
rule: viola focus's walk of the wrapper's process ancestry is held in memory and never written or logged, and a recorded window address outside that ancestry is never focused.
basis: fct-yz4mggmo, fct-gtbzkyct
because: The chunk adds an ancestry walk that chooses the window to raise, and the report states it is kept out of every file and line and bounds which recorded address may be focused.
@row_add n=5 row=decision/focus-no-product-event area=4-span-trace-coverage fact=F5 detector=none
name: Focus is no product event
rule: viola focus writes no snapshot and no events.ndjson line; a focus is no product event and leaves the events log unchanged.
basis: fct-erx67rhe
because: The chunk adds a verb that acts on a session's window, and the report states it adds nothing to the snapshot or the product audit stream.
@row_set n=1 row=decision/instance-check-before-log-init cell=rule seen=2 fact=F6 detector=none
value: Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found.
basis: fct-yz4mggmo, fct-fesrwmoj
because: viola focus runs the home and instance strict-modes check before any log line and refuses with exit 21, so the list of commands checking before log init must name it.
@row_set n=2 row=contract/strict-modes-refusal-exit/21 cell=promise seen=1 detector=none
value: The six frame verbs, link / unlink and focus exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.
basis: fct-yz4mggmo
because: viola focus exits 21 with strict-modes-failed for a home or instance failing strict modes, so the list of verbs taking exit 21 on that refusal must name it.
@row_none n=1 fact=F7 type=4
relied: viola focus makes at most three hyprctl seam reads (clients, a workspace dispatch only for a workspace 1 or above, the window dispatch), each one hyprctl-probe pair under the hyprctl read deadline; contract/diag-subject/hyprctl-probe already holds the pair per spawn.
