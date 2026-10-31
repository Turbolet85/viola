# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · no list field moved, and src/run/agents.rs still reads resolve_program unchanged
standard-contracts#2.0 · no · the envelope and the replay fallback live in viola-state, which the session did not edit
standard-contracts#3.0 · no · the stamps envelope and its strict read live in viola-state, unedited, and AgentError only gained variants (F8)
standard-contracts#4.1 · no · viola-state was not edited and StampsError keeps its code
standard-contracts#5.1 · no · the link set lives in src/run/links.rs and viola-state's replay, neither reached, and the snapshot's links field is unchanged
standard-contracts#5.2 · no · the link and remove rules live in src/run/links.rs, not reached, and the calls' wire is unchanged
standard-contracts#5.3 · no · already_linked is the set's own answer in src/run/links.rs, not reached, and the wire form is unchanged
standard-contracts#5.4 · no · unlink's {} answer and the absent-pair rule live in src/run/links.rs, not reached, and are unchanged
standard-contracts#5.5 · no · a -32602 still writes no events line, the channel-response detail is a diag line the session only closed, and the link -32603 path stays held by the unit layer (F13)
standard-contracts#6.1 · no · the pid and start-time comparison did not move, and the session's client edits are the reply screen and the fault reading
standard-contracts#7.1 · no · the strict-modes check is viola-state's and unedited, and a refused check still writes no frame or line
standard-contracts#8.1 · no · the reply's wire form is unchanged, and its details unconfirmable, budget-paused, five-hour and seven-day are code words the new screen passes (F2, F3)
standard-contracts#9.1 · no · the -32602 body stays data null, which the new reader gives as invalid params (F4), and the session's viola-core addition is is_code_word, leaving the wake set alone
standard-contracts#10.1 · no · a from of any shape still changes nothing, and the request side and the answer wire are unchanged
standard-contracts#11.1 · no · the hook.event wire, its re-validation and its -32603 append-failure answer are unchanged
standard-contracts#12.1 · no · resolve_program is named unchanged and this line states neither its return nor Refusal::NotFound, and instance-unreachable is unchanged
standard-contracts#13.1 · no · the prompt-submitted shape and its text and origin rules are not what the session changed in viola-agent-claude, which retired Refusal and widened AgentError
standard-contracts#14.1 · no · the link and unlink success documents and the refused instance-unreachable document are unchanged on the wire
standard-contracts#15.1 · yes · add-to · F5 joins it as a new document, a channel fault outside the five codes printing wrapper-fault on --json with the fixed message wrapper fault and data null, beside this instance-unreachable one
standard-contracts#16.1 · no · the statusline arm's product rules in src/cmd/hook/statusline.rs are unchanged, the session adding only cap-driving unit cases there
standard-contracts#17.1 · no · no heartbeat, pid or start-time rule moved, so the server identity read is unchanged
standard-contracts#18.1 · no · resolve_program is named unchanged and this line states neither its return nor Refusal::NotFound, and the claude agents read is unchanged
