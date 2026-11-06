@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the CLIENT span-name list names no OS arm, and the session retires no span name in it
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span names channel.dispatch, mcp.tool_call, hook.handle and hook.statusline are not retired by the session
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the INTERNAL span names run.*, state.*, hook.decision_emit and pty.paste_write stay, and the ConPTY arm behind the pty seam names no span here
@answer n=4 entity=span-trace-coverage#2.1 v=no
why: the cli short-lived verbs row names no OS arm, and its verbs, detail words and Unix strict-modes refusals stand on Linux
@answer n=5 entity=span-trace-coverage#2.2 v=no
why: the viola run row names portable-pty 0.8.1 (kept pinned) and failure capture at the pty seam, which removing the ConPTY arm does not alter
@answer n=6 entity=span-trace-coverage#2.3 v=no
why: the viola hook row names no OS arm, and its hook events, detail words and instance check are unchanged
@answer n=7 entity=span-trace-coverage#2.4 v=no
why: the ipc-internal row claims only that interprocess emits no log and the channel-request fields, which removing the named pipe does not alter
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: the accept loop still filters each stream by peer_admitted over its peer euid on Linux, so the row's claim holds
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: rmcp's tracing target stays OFF in the sink, and the mcp.tool_call row names no OS arm
@answer n=10 entity=span-trace-coverage#3.1 v=yes mode=contradict
why: the session removes the Windows pid comparison and the macOS euid-only carve-out, so "runs on Linux and Windows" and the three-OS reach of server-verify-failed no longer hold
@answer n=11 entity=span-trace-coverage#3.2 v=yes mode=contradict
why: the session removes the Windows no-euid arm and the PEER_EUID_REPORTED constant that this clause cites
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the per-verb captured-span case names and the shared helper cmd::client::refused_inside keep their names and roles, and this entity states no OS gate
@answer n=13 entity=span-trace-coverage#3.4 v=yes mode=contradict
why: the session removes the macOS code that gated refused_inside, so the recorder standing out of a macOS-gated helper no longer holds
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: the instance check's refusal path (exit 21, the instance pair, the strict-modes-failed document) is the Unix path and stays, and the session removes only the Windows owner/DACL arm
@answer n=15 entity=span-trace-coverage#3.6 v=no
why: the socket-dir-refused exit and detail are unchanged, since the session keeps the NotADirectory, Symlink and Unreadable refusals
@answer n=16 entity=span-trace-coverage#3.7 v=no
why: the socket-directory refusal's human pair names no OS arm and the session does not alter it
@answer n=17 entity=span-trace-coverage#3.8 v=yes mode=contradict
why: the session removes the Windows pid comparison and the macOS carve-out, so "runs on Linux and Windows" and the macOS euid-only reach no longer hold
@answer n=18 entity=span-trace-coverage#3.9 v=no
why: the hook's socket-dir-refused line and its dialog-path placement are unchanged by the session
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: the -32602 closed word, the ResponseLine Drop and the fault_of reads name no OS arm, and the session does not alter the protocol fault
@answer n=20 entity=span-trace-coverage#3.11 v=yes mode=contradict
why: the session removes the Windows pid comparison and the macOS carve-out on which the mcp server check's three-OS server-verify-failed reach relies
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the reply screening through is_code_word and sanitised_fault adds no span or line, and the session does not alter it
@answer n=22 entity=span-trace-coverage#4.1 v=yes mode=contradict
why: the session removes the ConPTY sideload, so the must-trace run.conpty_sideload (Windows) span no longer exists
@answer n=23 entity=span-trace-coverage#4.2 v=yes mode=contradict
why: the session removes run.conpty_sideload, the batch-script-child refusal and the named-pipe and ConPTY values of endpoint_kind and pty_backend, so those attributes now read only unix-socket and openpty
@answer n=24 entity=span-trace-coverage#4.3 v=yes mode=contradict
why: the session removes the batch-script-child detail, the sideload_fallback process-start property and its closed enum in the schema
@answer n=25 entity=span-trace-coverage#4.4 v=yes mode=contradict
why: the session removes the windows-sys TerminateProcess kill fallback that emitted exit_source kill-fallback, and the ConPTY reader-EOF parenthetical names a removed arm
