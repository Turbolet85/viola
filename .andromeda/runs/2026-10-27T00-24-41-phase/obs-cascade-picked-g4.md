# obs · picked accounting · group 4 of 5

## Picked
span-trace-coverage#1.3 · not in scope · span kinds are recorded per span and not emitted in v1; `run.pin_copy` keeps its INTERNAL kind, a re-check every change owes
span-trace-coverage#2.1 · carried · the cli row rules `viola verify`'s dispatch span and its `process-start` / `process-exit{exit_code, detail}` into `cli-<name>.ndjson`, and the chunk edits verify's `pin_exe` call
span-trace-coverage#2.2 · carried · the `viola run` row rules the manual start-sequence and `state.*` spans the pin write sits under, and the chunk edits the start's `pin_bytes` call
span-trace-coverage#2.3 · carried · the `viola hook` row rules the hook surface the race breaks: a hook that runs keeps exit 0 and its sink fallback, and the witness reads its lines
span-trace-coverage#4.1 · carried · `run.pin_copy` is a must-trace span in the start chain the edited pin step sits inside
span-trace-coverage#4.2 · carried · `run.pin_copy`'s closed `outcome`, `run.conpty_sideload`'s outcomes (the Windows `pin_companions` arm the scope may edit) and the never-`pinned_bin` rule rule over the edited units
span-trace-coverage#4.3 · carried · the start's failure `detail` set (a lost race's new exit-1 cause), `sideload_fallback` and the `start_records` readiness stage the two-sided witness reads
span-trace-coverage#4.4 · not in scope · `run.start`'s close and the handle-wait thread's child exit are untouched by the absent-copy write; a re-check every change owes
span-trace-coverage#7.1 · not in scope · the dialog → `answer` span chain; no dialog path changes
span-trace-coverage#7.3 · not in scope · the dialog path's log fields; no dialog path changes
span-trace-coverage#9.3 · not in scope · the budget governor's log fields; the chunk touches no budget gate or status line
span-trace-coverage#10.1 · not in scope · the R8 strip's names on `process-start{subject:"claude-child"}`; the strip is not edited and no strip field changes
span-trace-coverage#16.1 · carried · `viola verify`'s spans, its `cli-<name>.ndjson` pair, its home-tree check before the log and the stamped-home boot that pins through verify first rule the edited `pin_exe` call and the race's test-side workaround
metric-coverage#1.1 · not in scope · the counter derivation over role files; no metric is added and a new `detail` code is counted by the existing derivation unchanged
metric-coverage#1.3 · carried · the `liveness` gauge read through `agent-run status` is where the two-sided witness reads each boot's outcome
log-coverage#1.1 · carried · `process-start`'s closed `subject` and `sideload_fallback` values must hold if the exe's Windows arm takes `pin_companions`' wait
log-coverage#1.2 · carried · `process-exit`'s `detail` field carries any new exit-1 cause of the absent-copy write
log-coverage#1.7 · carried · `hook-invoked` / `hook-decision{hook_event:"session-start"}` are the hook lines the race witness waits on
log-coverage#1.8 · carried · `panic_location` must stay workspace-relative under any path remap or profile key the reproducible-build measurement leads the founder to rule
log-coverage#2.1 · carried · the `process-exit` exit-1 `detail` catalog and the schema's 13-value count move if the fix adds a cause
log-coverage#2.4 · not in scope · the hook's fail-open `detail` codes; the race fails the pinned path's exec before any hook arm runs, and no hook code is edited
log-coverage#2.7 · carried · Vector 7's `pinned-hash-mismatch` on `process-exit` exit 1 must hold when a concurrent start re-hashes a present copy, and the refused home-tree check stays lineless
