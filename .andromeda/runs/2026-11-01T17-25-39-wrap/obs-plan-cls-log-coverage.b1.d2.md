# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the process-start field row names fields and the subject enum, which the session did not move
log-coverage#1.2 · no · the process-exit field row names fields without a detail value list, so the new code lands on #2.1, #2.2 and #2.8 instead
log-coverage#1.3 · no · channel-request fields are untouched, and a dropped peer sent no frame so makes no channel call
log-coverage#1.4 · no · channel-response detail stays closed at the three -32602 faults, since the schema moved only the process-exit and hook-decision detail enums at schemas/diag-line.v1.json:78 and :53
log-coverage#1.5 · no · send-issued, send-confirmed and send-refused are not written for a refused socket directory, because no frame exists
log-coverage#1.6 · no · the session adds no dialog record, and the refused hook path writes hook-invoked and hook-decision only
log-coverage#1.7 · no · the hook-invoked and hook-decision field row and its hook_event enum did not move, and the new detail code is carried at the fail-open list #2.4
log-coverage#1.8 · no · panic fields are untouched by the session
log-coverage#1.9 · no · release-from-driver is untouched by the session
log-coverage#1.10 · no · state-recovered details are untouched, as no store format or snapshot changed
log-coverage#1.11 · no · the parse-rejected parser and detail lists are untouched, since the schema moved only the process-exit and hook-decision enums
log-coverage#2.1 · yes · stale · the exit-1 cause list gains socket-dir-refused from run's and revive's start check (F2), and line 29's count of 19 process-exit detail values moves to 20 at schemas/diag-line.v1.json:78
log-coverage#2.2 · yes · stale · the exit-21 cause list gains socket-dir-refused from the eight CLI frame verbs (F3), and server-verify-failed's reach moves from Linux and Windows to macOS as well, where it means an euid mismatch alone
log-coverage#2.3 · no · exit 20 wrapper-fault is untouched, and the accept-loop drop writes no line and no exit (F6)
log-coverage#2.4 · yes · stale · the fail-open hook-decision list grows from seven codes to eight with socket-dir-refused (F1, F4), the statusline arm's three-of-seven count moves with it, and server-verify-failed's reach moves to macOS with the euid refusal at the hook seam
log-coverage#2.5 · no · the per-parser parse-rejected detail lists are untouched, and the session adds no parse-rejected site
log-coverage#2.6 · no · the mcp-stdin oversize line is untouched by the session
log-coverage#2.7 · no · RefusalReason details are untouched, since Refused::NotADirectory is a Refused variant and UnreachableCause::SocketDir carries no RefusalReason (F5, F8)
log-coverage#2.8 · yes · add-to · socket-dir-refused (F1) is a filesystem refusal word that writes lines on process-exit exit 1 and 21 and on hook-decision, so it joins the Vector 7 word list beside strict-modes-failed
log-coverage#3.1 · no · the closed subject enum and the spawn pairs are unchanged, since the session adds no spawn and moved only detail enums
