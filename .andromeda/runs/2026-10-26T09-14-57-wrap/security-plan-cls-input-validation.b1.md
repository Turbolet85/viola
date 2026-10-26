# Classified — security-plan · input-validation.b1

## Answers
input-validation#1.1 · no · the paste-text validation is reported untouched and the client's exit-13 refusals stay first (F3), so the check still precedes any frame
input-validation#1.2 · no · the dialog free-text validation is reported untouched and the exit-13 refusals in viola answer stay first (F3)
input-validation#1.3 · no · the report adds no IPC method, endpoint or frame, and the viola-channel change is only the client's server_pid and its re-export (F1)
input-validation#1.4 · yes · stale · the event and dialog paths of src/cmd/hook.rs gained the server check and its server-verify-failed fail-open (F4), the same paths that carry this row's dialog classify and Stop data rules
input-validation#1.5 · yes · stale · the four frame handlers in viola-mcp/src/tools.rs gained the server check (F5) in the same handlers that take and check these raw arguments
input-validation#1.6 · yes · contradict · the frame verbs now verify the server after the liveness pre-check, so this row's liveness-only wording no longer holds, while its strict-modes gap stands
input-validation#1.7 · no · the config.json readers live in files the session did not reach and the budget gate is reported untouched
input-validation#1.8 · no · the persistent-environment name read lives in src/run/env.rs, which the session did not reach, and the R8 strip is reported untouched
input-validation#1.9 · no · the pump-delay seam lives in src/cmd/run.rs, which the session did not reach
input-validation#1.10 · no · the panic seam is named only in src/cmd/hook/seam.rs, unreached, and its call before the stdin lock sits ahead of the event and dialog paths the check touched
input-validation#1.11 · no · the colour facts are read in src/human/style.rs and src/cmd/list.rs, neither reached by the session
input-validation#1.12 · yes · add-to · F8, every reader stays on read_snapshot with fork 1 re-pinned to Home and code-bearing file integrity, and the other-readers count is three with src/cmd/client.rs holding no read
input-validation#1.13 · no · the notify tailing code is not among the reached files and the report names no tail or watcher change
input-validation#1.14 · no · the child-output reads and their parse live in viola-agent-claude and the list, revive and verify runs, none reached, and the report marks revive's preflight and verify's runs untouched
input-validation#1.15 · no · the vt100 feed and gate live in src/run/gate.rs, which the session did not reach
input-validation#1.16 · no · program resolution lives in src/run/mod.rs and viola-agent-claude, neither reached, and the session's server check sits in the four frame handlers, not the list read
input-validation#1.17 · no · the session-id argv lives in src/cmd/revive.rs, resume_args and verify's Run E, none reached, and the report marks their preflight and runs untouched
input-validation#1.18 · no · the statusline source is reported untouched and lives in src/cmd/run.rs, which the session did not reach
input-validation#1.19 · no · the statusline shell-out is reported untouched, and the server check sits on the event and dialog paths, not the statusline arm that holds STATUSLINE_DEADLINE
input-validation#1.20 · no · the Windows DLL search and ConPTY re-hash are reported untouched and live in src/conpty.rs, src/main.rs and src/cmd/run.rs, none reached
input-validation#1.21 · no · the entity is N/A, since the session adds no upload, webhook or database surface
