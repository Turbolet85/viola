@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the session removes no CLIENT span name, and pty.spawn stays on openpty
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span names are unchanged, since the session removes no dispatch, tool or hook span
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the run.* and state.* wildcards still cover every span the session keeps, and no INTERNAL name listed is removed
@answer n=4 entity=span-trace-coverage#2.1 v=no
why: the short-lived verb rows name no retired arm (named pipe, sideload, batch-script refusal, macOS), and their Unix strict-modes and socket-directory claims hold
@answer n=5 entity=span-trace-coverage#2.2 v=no
why: the viola run row names the pty seam and handle-wait child lines, which stay on openpty and portable-pty 0.8.1, and names no kill-fallback or sideload
@answer n=6 entity=span-trace-coverage#2.3 v=no
why: the hook row names no OS arm and no retired control, and its instance-check and capture-arm claims hold
@answer n=7 entity=span-trace-coverage#2.4 v=no
why: the channel-request and channel-response fields are unchanged and no retired named-pipe field is named
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: the Unix accept-loop euid filter stays, the row names no Windows or macOS arm, and its refused-stream claim holds on Linux
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: the mcp.tool_call row names no OS arm, and its writer and target claims are unchanged
@answer n=10 entity=span-trace-coverage#3.1 v=yes mode=contradict
why: the removed Windows and macOS peer-pid arms make 'runs on Linux and Windows' and 'reaches Linux, Windows and macOS' false, leaving Linux alone
@answer n=11 entity=span-trace-coverage#3.2 v=yes mode=contradict
why: the Windows no-euid clause and the PEER_EUID_REPORTED constant leave with the Windows and macOS arms, so the euid-reporting statement no longer holds
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the captured-span case names and the shared test helper cmd::client::refused_inside carry no OS clause, and each case's claim holds on Linux
@answer n=13 entity=span-trace-coverage#3.4 v=yes mode=contradict
why: the macOS gate on refused_inside is retired with the macOS code, so the 'out of the macOS-gated refused_inside' clause no longer holds
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: the six frame verbs' instance check and its exit-21 and strict-modes-failed document are Unix-neutral, and the only moved part, the Windows owner/DACL arm, is not named here
@answer n=15 entity=span-trace-coverage#3.6 v=no
why: the socket-dir-refused detail and its refusal are unchanged, and the clause names no OS
@answer n=16 entity=span-trace-coverage#3.7 v=no
why: the human pair's plain-ASCII and ViolaName rules are unchanged, and the clause names no OS
@answer n=17 entity=span-trace-coverage#3.8 v=yes mode=contradict
why: the hook's 'runs on Linux and Windows' and 'reaches Linux, Windows and macOS' clauses name the retired Windows and macOS arms, so they no longer hold
@answer n=18 entity=span-trace-coverage#3.9 v=no
why: the hook's socket-dir-refused decision and its WARN line are unchanged, and the clause names no OS
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: the -32602 detail word, ProtocolError::cause and fault_of are unchanged, and the clause names no OS
@answer n=20 entity=span-trace-coverage#3.11 v=yes mode=contradict
why: the frame tools' 'runs on Linux and Windows' and 'reaches Linux, Windows and macOS' clauses name the retired arms, so they no longer hold
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the reply screening through is_code_word and sanitised_fault adds no span or line and names no OS
@answer n=22 entity=span-trace-coverage#4.1 v=yes mode=contradict
why: run.conpty_sideload, marked Windows, leaves the must-trace span chain of the run start
@answer n=23 entity=span-trace-coverage#4.2 v=yes mode=contradict
why: the batch-script-child decision, the run.conpty_sideload attributes, the named-pipe endpoint_kind value and the conpty-sideload pty_backend value all leave with the retired arms
@answer n=24 entity=span-trace-coverage#4.3 v=yes mode=contradict
why: batch-script-child leaves the process-exit detail enum, and sideload_fallback leaves the process-start properties
@answer n=25 entity=span-trace-coverage#4.4 v=yes mode=stale
why: the TerminateProcess kill fallback is removed, so the kill-fallback exit_source emission on the handle-wait thread is stale and child exit rests on the handle wait alone
