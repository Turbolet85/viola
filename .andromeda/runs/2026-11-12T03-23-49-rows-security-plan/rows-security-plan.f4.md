@row_add n=1 row=contract/instance-unreachable-detail/strict-modes-failed area=error-handling rec=1,2
boundary: instance-unreachable detail
item: strict-modes-failed
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the instance's strict-modes check refused, and no frame was sent.
@row_add n=2 row=contract/instance-unreachable-detail/endpoint-lost area=error-handling rec=1,2
boundary: instance-unreachable detail
item: endpoint-lost
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the wrapper was lost with the call open.
@row_add n=3 row=contract/instance-unreachable-detail/server-verify-failed area=error-handling rec=1,3,2
boundary: instance-unreachable detail
item: server-verify-failed
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the peer euid decision refused past the connect, and no frame was sent; a peer euid refusal has no word of its own and reads server-verify-failed.
@row_add n=4 row=contract/instance-unreachable-detail/socket-dir-refused area=error-handling rec=1,4,2
boundary: instance-unreachable detail
item: socket-dir-refused
promise: A cause of exit 21 in the CLI --json document and MCP's isError result: the socket directory of the recorded endpoint was refused, and no frame was sent; on a CLI frame verb it is during connect, one process-exit at WARN and no channel-request.
@row_add n=5 row=decision/channel-closed-types area=input-validation rec=5
name: Channel closed types
rule: Channel frame fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id is u64, and link and unlink driver and driven names go through ViolaName::try_new.
@row_add n=6 row=decision/capture-answers-closed-probe-ids area=input-validation rec=6
name: Capture answers from closed probe ids
rule: --answers is read only with --capture; each answer file is read through take(64) and matched exactly against the closed ProbeAnswer ids, the body built only through dialog::decision_body.
why: Ratified: the hook answers, never a key.
@row_add n=7 row=decision/verify-children-direct-spawn area=threat-model-summary rec=8
name: Verify children spawned directly
rule: viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals.
why: Ratified live rulings, each given after the widening was shown.
