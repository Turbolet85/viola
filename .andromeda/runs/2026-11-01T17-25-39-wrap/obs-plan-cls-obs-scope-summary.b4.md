# Classified — obs-plan · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · no · the mcp file sink, its path and its home check through resolved_home are unchanged; the new socket-dir check sits on the frame path, not the exporter
obs-scope-summary#22.4 · no · service identity (process mcp, instance from VIOLA_NAME) is unchanged
obs-scope-summary#22.5 · yes · add-to · MCP's frame socket-dir refusal (F5) returns instance-unreachable with a sixth closed cause socket-dir-refused, so the list of five extends
obs-scope-summary#23.1 · no · init steps 1 to 6 and the step-3 home checks are unchanged; the socket-dir check runs in the start or dispatch after process-start and writes its own process-exit, so it is not an init step
obs-scope-summary#24.1 · no · sink rules (one file per role, no stdout, mcp stderr fallback) are unchanged
obs-scope-summary#25.0 · yes · add-to · a refused dialog-path hook-decision (socket-dir-refused, F4) carries a null corr with no dialog_id yet, which the hook-decision corr rule must carry
obs-scope-summary#26.1 · no · diagnostics/ folder and file modes and log location are unchanged per the report; the socket-dir check is a separate check with its own word
obs-scope-summary#27.1 · no · the run surfaces (cli run, channel bind, persistent stores) are unchanged
obs-scope-summary#27.2 · no · the new run start step carries no span (F2), so the must-trace span chain and its log events still hold; the refusal's process-exit falls under "process-exit on failure"
obs-scope-summary#27.3 · yes · add-to · run and revive's start gain socket-dir-refused (exit 1, F2) as an exit-1 detail code, so the distinct-code list extends
obs-scope-summary#27.4 · no · source references only
obs-scope-summary#28.1 · no · send's surface chain is unchanged
obs-scope-summary#28.2 · no · the send success chain (send.client, channel.request, readiness gate, paste, confirm) is unchanged; a refused send writes a spanless process-exit and the list does not claim refusal branches
obs-scope-summary#28.3 · no · send-line fields are unchanged; the refusal writes process-exit, not a send-refused line
obs-scope-summary#28.4 · no · source references only
obs-scope-summary#29.1 · no · wait's surface chain is unchanged
obs-scope-summary#29.2 · no · wait's must-trace chain lists no pre-frame steps, and the new check adds a spanless step
obs-scope-summary#29.3 · no · wait's outcome fields and mid-wait exit-21 endpoint-lost case are unchanged; the connect-stage refusal writes a process-exit, not a wait line
obs-scope-summary#29.4 · no · source references only
obs-scope-summary#30.1 · no · the dialog surface chain is unchanged
obs-scope-summary#30.2 · no · the dialog must-trace chain (hook.handle to decision_emit, answer.client) is unchanged; the refused hook never reaches channel.request and adds no span
obs-scope-summary#30.3 · yes · stale · server-verify-failed's reach moves from Linux and Windows to Linux, Windows and macOS (euid mismatch), so the "on Linux and Windows" reading moves
obs-scope-summary#30.4 · no · source references only
obs-scope-summary#31.1 · no · the run and driver surface chain is unchanged
obs-scope-summary#31.2 · no · the send-refused, pause and release chain is unchanged; the socket-dir refusal writes a process-exit with no channel-request
