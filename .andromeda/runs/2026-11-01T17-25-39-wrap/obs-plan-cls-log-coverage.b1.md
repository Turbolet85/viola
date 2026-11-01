# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the process-start field row lists no field the session changes, and the new refusal writes only a process-exit
log-coverage#1.2 · no · process-exit's field names and `during` values are unchanged; the new word is a detail value, which lives in the exit-1 and exit-21 lists
log-coverage#1.3 · no · channel-request is not written for a refused frame verb or a dropped peer, since no frame is sent
log-coverage#1.4 · no · channel-response's closed detail set is the three -32602 faults, which the session does not move
log-coverage#1.5 · no · send-issued, send-confirmed and send-refused are unchanged; a refused send writes process-exit exit 21 instead
log-coverage#1.6 · no · dialog-raised and dialog-answered fields are unchanged; the refused hook writes only hook-invoked and hook-decision
log-coverage#1.7 · no · the hook-invoked and hook-decision field list is unchanged; the new refusal reuses the existing detail field, whose codes live in 2.4
log-coverage#1.8 · no · the panic line and its fields are untouched, and the session names no panic hook change
log-coverage#1.9 · no · release-from-driver is written only for a delivered release, and a refused release sends no frame
log-coverage#1.10 · no · state-recovered's details and snapshot fields are not touched by the session
log-coverage#1.11 · no · the parse-rejected parser list is unchanged, and the session adds no parser
log-coverage#2.1 · yes · stale · the closed process-exit detail enum moves from 19 to 20 and run's and revive's start gain the socket-dir-refused exit-1 cause (F1, F2)
log-coverage#2.2 · yes · contradict · server-verify-failed's reach moves from Linux and Windows to macOS too, and the exit-21 list of five gains socket-dir-refused (F1)
log-coverage#2.3 · no · the exit-20 wrapper-fault cause is unchanged, since a refused peer is dropped on the accept thread with no line and no fault
log-coverage#2.4 · yes · contradict · the hook's server-verify-failed reach moves to macOS, and the fail-open list of seven gains socket-dir-refused as a hook-decision, so seven becomes eight (F1, F4)
log-coverage#2.5 · no · the parse-rejected details per parser are unchanged, and the enums that move are process-exit's and hook-decision's
log-coverage#2.6 · no · the mcp-stdin oversize line is unchanged, and the session changes no stdin parsing
log-coverage#2.7 · no · the RefusalReason details are unchanged, since the new Refused::NotADirectory and UnreachableCause::SocketDir carry no RefusalReason and write no send-refused
log-coverage#2.8 · yes · add-to · socket-dir-refused (F1) joins the filesystem-refusal group as a Vector 7 refusal that writes its own line, unlike the lineless strict-modes checks
log-coverage#3.1 · no · the child and shell spawn pairs are unchanged, the new refusal is a subject self process-exit, and the subject enum that diag-line.v1.json closes does not move
