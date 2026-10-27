# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the sideload_fallback values hold, since pin_companions and replace_private_shared are unchanged in the reached pin.rs and the process-start fields gain nothing
log-coverage#1.2 · no · the process-exit fields are unchanged, and the lost-placement refusal reaches the same run refusal site, which logs the same detail
log-coverage#1.3 · no · the channel-request fields are untouched, since viola-channel is not reached and a pinned-copy placement logs no channel frame
log-coverage#1.4 · no · the channel-response fields are untouched, since viola-channel is not reached and no response path moved
log-coverage#1.5 · no · the send-issued, send-confirmed and send-refused fields are untouched, since no confirmed-send, wheel or budget path was changed
log-coverage#1.6 · no · the dialog-raised and dialog-answered fields are untouched, since no dialog-answer path was changed
log-coverage#1.7 · no · the hook-invoked and hook-decision fields are untouched, since no hook arm file is among the reached files and the fake agent's spawn_os_error receipt is test-only
log-coverage#1.8 · no · the panic fields are untouched, since the report adds no panic line, seam or log path
log-coverage#1.9 · no · the release-from-driver fields are untouched, since no release path is among the reached files
log-coverage#1.10 · no · the state-recovered details hold, since the report adds no detail code or field, the schema is unchanged, and the reached fs.rs change is only the new place_private_new
log-coverage#1.11 · no · the parse-rejected parser list holds, since no parser was added and the schema is unchanged
log-coverage#2.1 · no · the exit-1 detail list holds, since the lost-placement path reaches the same run refusal site with the same pinned-hash-mismatch detail and the start order is re-read unchanged
log-coverage#2.2 · no · the exit-21 details hold, since the reached strict.rs change is only a cfg(windows) test oracle and its product verdicts and lines are unchanged
log-coverage#2.3 · no · the exit-20 wrapper-fault detail holds, since the report names no change to its path and adds no exit code
log-coverage#2.4 · no · the hook-decision fail-open codes hold, since no hook arm file is among the reached files and the strict.rs product verdicts are unchanged
log-coverage#2.5 · no · the parse-rejected details hold, since the ledger-stamps strict-modes verdict is unchanged in strict.rs and the detail catalog is unchanged
log-coverage#2.6 · no · the refusal details are RefusalReason values in viola-core, which is not reached, and the report adds no refusal
log-coverage#2.7 · no · the filesystem-refusal lines hold, since the strict.rs product verdicts are unchanged and the pinned-hash-mismatch exit-1 line is the same refusal site with no new line
log-coverage#3.1 · no · the child and shell spawn subjects and their logging are unchanged, since verify.rs is unchanged and the fake agent's receipt is test-only
