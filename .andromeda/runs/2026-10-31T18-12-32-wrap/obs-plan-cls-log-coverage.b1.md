# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the process-start field row is unchanged, and the session adds no process-start field or spawn.
log-coverage#1.2 · no · the process-exit row names detail and during as fields without their words, and during stays connect|call; the word lists sit in #2.1 and #2.2.
log-coverage#1.3 · no · channel-request is on the channel wire, which the session does not reach (nothing in viola-channel).
log-coverage#1.4 · no · channel-response's three -32602 detail words are closed by diag-line.v1.json, which moved only at line 78 (process-exit), so the claim still holds on the changed schema.
log-coverage#1.5 · no · the send-issued, send-confirmed and send-refused fields are unchanged; the session changes send's exit-21 document, not its log fields.
log-coverage#1.6 · no · the dialog-raised and dialog-answered fields are not touched by the session.
log-coverage#1.7 · no · hook-invoked and hook-decision are untouched: hook.rs is not in the diff and the seven hook-decision words did not move.
log-coverage#1.8 · no · the panic fields are untouched; the session adds no panic path or field.
log-coverage#1.9 · no · release-from-driver is a channel event, and the channel wire and its logging are not reached.
log-coverage#1.10 · no · state-recovered: snapshot replay and torn-line healing code is not in the diff.
log-coverage#1.11 · no · the parse-rejected parser list and detail words are unchanged, and diag-line.v1.json moved only at line 78, so its parse-rejected constraint holds.
log-coverage#2.1 · yes · contradict · a stale holder now logs stale-heartbeat, not already-live (F3), and the closed process-exit detail set grows from 13 to 19 with F1 and the verify codes of F2 beside interrupted.
log-coverage#2.2 · yes · add-to · exit 21's process-exit causes now also include endpoint-lost (a wrapper lost with a call open) and unwrapped-name, the five UnreachableCause words of F11 and the catalog of F1.
log-coverage#2.3 · no · exit 20 wrapper-fault: the session adds no exit-20 cause and moves no exit.
log-coverage#2.4 · no · the hook-decision fail-open codes stand: hook.rs is not in the diff and no hook arm's word changed.
log-coverage#2.5 · no · the parse-rejected details are unchanged, the session adds no parse-rejected word, and the schema's constraint holds on the changed file.
log-coverage#2.6 · no · the mcp-stdin oversize line and its once-per-session parse-rejected come from the stdin reader, which the session does not change; MCP's change maps refusals to causes in result.rs and tools.rs.
log-coverage#2.7 · no · the RefusalReason details are verbatim, and the session adds no RefusalReason.
log-coverage#2.8 · yes · contradict · exit 21's --json documents (list, plugin install, link, unlink, the frame verbs) and MCP's isError body now carry the cause as detail, so the bare instance-unreachable and detail-null wording no longer holds (F10, F11).
log-coverage#3.1 · no · the eight verify spawn pairs keep their subjects, child_exit_status and duration_ms; the new verify codes sit on verify's own subject-self process-exit, and the subject enum is unchanged.
