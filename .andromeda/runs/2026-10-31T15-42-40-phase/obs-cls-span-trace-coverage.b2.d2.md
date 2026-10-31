# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the wait.block and run.wait_dispatch span attributes and woken_kind values; no exit-cause code or refusal on them, and the chunk adds no span to wait
span-trace-coverage#7.3 · yes · wait/last refusal order (instance check before any role file, then liveness pre-check, then server check), process-exit detail instance-dead and server-verify-failed at WARN, the no-line instance refusal and MCP's instance-unreachable; client.rs edits and the exit-21 rows for wait and last
span-trace-coverage#7.4 · yes · a mid-call disconnect logged once by the client (the endpoint-lost cause the chunk names), and server-verify-failed closing as the client's process-exit outcome line before any frame
span-trace-coverage#8.1 · yes · answer's instance check (refuse_untrusted) before any role file and the server check before channel.request(answer), both edited in client.rs; a refused check sends no frame
span-trace-coverage#8.2 · no · run.dialog_register, run.dialog_await and hook.decision_emit span attributes; the dialog path is not reached and no exit-21 or exit-1 value sits on them
span-trace-coverage#8.3 · yes · CLI answer's server-verify-failed process-exit (exit 21, during connect, WARN, no channel line) and its refused-instance exit 21 with the instance pair or instance-unreachable document and no line; MCP answer's no-line refusal
span-trace-coverage#8.4 · no · dialog hold end, dialog-expired and the hook's fail-open decision cleanup; the hook adds no cause and no exit-21 or exit-1 code here
span-trace-coverage#9.1 · yes · pause.client and release.client run the server check before channel.request, with no frame on refusal; the server check in client.rs is edited
span-trace-coverage#9.2 · no · run.wheel_transition span attributes (wheel_from, wheel_to, cause); the chunk adds no span and touches no wheel code
span-trace-coverage#9.3 · yes · pause/release server-verify-failed refusal made before any frame: no channel line, and a process-exit exit 21 with detail server-verify-failed, during connect, at WARN
span-trace-coverage#9.4 · no · wheel point-in-time span cleanup and the events.ndjson wheel audit record; no exit-cause code and no reached unit
span-trace-coverage#10.1 · no · budget governor must-trace spans; the budget path is not re-opened and no exit-cause code arises
span-trace-coverage#10.2 · no · budget governor span attributes (state.budget_write, statusline.shell_out, run.budget_eval); untouched by the chunk
span-trace-coverage#10.3 · no · statusline hook-invoked, hook-decision and process-start lines; src/cmd/hook.rs is not reached and the catalogue adds no hook cause
span-trace-coverage#10.4 · no · statusline shell-out deadline and cleanup; untouched, and the hook adds no cause
span-trace-coverage#11.1 · no · E2 R8 env-strip names on process-start claude-child; the strip is not reached and no exit-cause code arises
span-trace-coverage#12.1 · no · E3 implicit link record inside a taken send or answer; it writes nothing on a refused call, the negative table asserts no product line, and the chunk adds no link record
span-trace-coverage#13.1 · no · the driven wrapper's explicit link/unlink methods, their -32602 replies and no-line rule; that server handler is not reached and carries no exit-21 or exit-1 code
span-trace-coverage#14.1 · yes · revive's state-recovered line then process-exit exit 1 detail cwd-missing, a catalogue value for exit 1 that revive's edited document after a passed preflight must hold
span-trace-coverage#15.1 · no · torn-append state-recovered line in events.ndjson; no exit-cause code and no reached unit
span-trace-coverage#16.1 · no · ledger read in run's version gate and its parse-rejected ledger-stamps line; the version gate is an untouched unit of run.rs and the line carries a parser detail, not an exit-21 or exit-1 code
span-trace-coverage#17.1 · yes · verify's refused home tree (exit 1, home pair, no log, no line) and its interrupted process-exit detail for exit 1, plus plugin install's exit 21 refusal with no log, all in verify.rs and the exit catalogue
