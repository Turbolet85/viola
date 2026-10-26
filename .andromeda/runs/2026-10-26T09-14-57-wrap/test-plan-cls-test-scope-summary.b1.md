# Classified — test-plan · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · yes · add-to · server verification (the connected peer's pid and start time against the one snapshot read) is now built and tested on Linux and Windows, and macOS has no peer pid, so the entity must carry both
test-scope-summary#2.1 · no · the session touches no viola-agent-claude file, and its hook parsing, dialog mapping, R8 strip, version gate and agents parsing are unchanged (the report marks §6 Path 7 and the R8 strip unchanged)
test-scope-summary#3.1 · no · the exit set (0/1/2/10–14/20/21) and the --json documents hold, since an impostor still exits 21 with the unchanged instance-unreachable document, and the revive and list tests are not reached
test-scope-summary#4.1 · no · the exit codes, the one JSON document and the unable-plus-last-line-hint human shape hold, since an impostor's exit 21 keeps the unchanged document and adds only a new hint text
test-scope-summary#5.1 · no · the run pump and the wheel and prompt-submitted events the tui surface reads are not reached by the change
test-scope-summary#5.2 · no · the wrapper channel's methods, response shapes and -32xxx codes are unchanged, and the viola-channel client gains only the server_pid accessor, so its sync-client driver and signals hold
test-scope-summary#5.3 · no · an impostor on the four MCP frame tools still returns isError instance-unreachable with detail null and no frame written, and no tool is added, so the stdio driver and signals hold
test-scope-summary#6.1 · no · the five-command harness is the §3 Test Harness Contract, which the report marks untouched, and the new tests run inside the existing cargo test step
test-scope-summary#7.1 · yes · add-to · the fake agent's --hold-stdout-internal holder mode now also serves as the held recorded-pid process in the server-identity negatives and the handler tests, a data role the strategy must carry beside the child role, which stands
test-scope-summary#8.1 · no · run's start sequence and its collision check are unchanged (run.rs is not reached, and the report marks §6 Path 1 untouched)
test-scope-summary#8.2 · no · on the recorded wrapper the identity check passes before the first send frame, so the accepted send, send-issued then prompt-submitted order, the exit 13 refusal and the MCP send result hold; an impostor is a separate negative this path does not list
test-scope-summary#9.1 · no · pause and release pass the identity check on the recorded wrapper, so manual-pause, the release back to the driver and the -32602 release-from refusal (exit 20) hold unchanged
test-scope-summary#9.2 · no · the budget governor and statusline arm are untouched (the report marks §6 Path 6 untouched and statusline.rs not reached), and send and release pass the check on the recorded wrapper, so budget-paused and release --budget hold
test-scope-summary#10.1 · no · the report marks the chaos suites untouched (no case added or moved), so the stale-heartbeat and live-pid cases hold even though liveness.rs gains server_verdict and a LiveEndpoint return type
test-scope-summary#11.1 · no · the ETXTBSY fix is a test-only change that plants each set's binary before its threads start in the drift contract test, and the suite's fixture-matching assertions are unchanged
