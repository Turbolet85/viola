# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the dialog spans and their attributes sit after obs init, which the new trusted check leaves in place, and a refused instance registers no dialog
span-trace-coverage#7.3 · yes · contradict · a refused instance now exits 0 before obs init with no line, so a refused fail-open path before dialog_id can no longer emit the hook-invoked{corr:null} line this row says every such path emits
span-trace-coverage#7.4 · yes · contradict · a refused instance now exits 0 before obs init with no line, so the fail-open hook-invoked and hook-decision pair this row gives before dialog_id no longer appears on that path
span-trace-coverage#8.1 · no · pause and release still run the server check before channel.request, and the new refusal exits earlier with no frame, so the claim holds on the changed files
span-trace-coverage#8.2 · no · the run.wheel_transition attributes and its worker-thread placement are untouched by the session
span-trace-coverage#8.3 · no · the wheel contract at its send-refused line is re-read and holds, and the server-verify refusal keeps its own line while the instance refusal is a separate earlier exit
span-trace-coverage#8.4 · no · the point-in-time wheel span and the wheel product event are unchanged, and the session adds no wheel transition
span-trace-coverage#9.1 · no · the budget chain is unchanged on a readable snapshot, and a replayed snapshot carries no statusline_command so statusline.shell_out never runs on that path
span-trace-coverage#9.2 · no · the budget span attributes and read_budget's no-reading, no-line rule are unchanged, and the new read rule refuses into that same no-reading path
span-trace-coverage#9.3 · no · the statusline arm's lines, refused-instance silence and capture-arm silence hold as landed, and a replayed snapshot records no statusline_command so no statusline-shell line appears
span-trace-coverage#9.4 · no · STATUSLINE_DEADLINE, the stdout pass-through and the oversize or malformed silence in hook.rs are unchanged, and the new checks sit before them
span-trace-coverage#10.1 · no · the R8 strip and the claude-child process-start fields are unchanged, and the session adds only the start's tree check before the wrapper log opens
span-trace-coverage#11.1 · no · the implicit link is recorded in run's link set, which the change does not reach, and a refused call never reaches that wrapper
span-trace-coverage#12.1 · no · the served link and unlink methods in run's link set are not reached, and no -32602 path or channel method changed
span-trace-coverage#13.1 · yes · add-to · F1, F2 and F3 join it (state.snapshot_recover placement under run.start and hook.handle, revive's one preflight read, collision_check judging a replay by heartbeat alone), and the readers it names as still on read_snapshot now read through read_snapshot_or_replay
span-trace-coverage#14.1 · no · the torn-append healing and its state-recovered line are not reached, as the report's Untouched list states
span-trace-coverage#15.1 · no · the run.version_gate ledger read, its strict-modes parse-rejected line and cli_verified:false are re-read and unmoved, and the new refusal is verify's write side
span-trace-coverage#16.1 · yes · add-to · verify's home-tree refusal opens no cli log and writes no line (exit 1 with the home pair), so it joins this row's no-line exemption beside plugin install's refused check
