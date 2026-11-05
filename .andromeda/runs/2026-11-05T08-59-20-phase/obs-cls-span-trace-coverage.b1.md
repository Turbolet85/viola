@answer n=1 entity=span-trace-coverage#1.1 v=yes
why: names the CLIENT spans wait.block and last.client (the wait and last verbs item 10 keeps) and pty.spawn (the PTY seam item 12 keeps)
@answer n=2 entity=span-trace-coverage#1.2 v=yes
why: mcp.tool_call (the MCP server-verification path, item 10) and hook.handle (the hook contract, item 14) are span owners the chunk edits
@answer n=3 entity=span-trace-coverage#1.3 v=yes
why: run.* spans are the run start sequence items 9 and 13 name, and hook.decision_emit sits on the hook path item 14 keeps
@answer n=4 entity=span-trace-coverage#2.1 v=yes
why: the cli verb dispatch spans and refused-check no-log rules cover wait and last, whose server verification the chunk edits (item 10)
@answer n=5 entity=span-trace-coverage#2.2 v=yes
why: child process-start and process-exit from the handle-wait thread are the PTY seam behaviour item 12 keeps once TerminateProcess leaves viola-pty
@answer n=6 entity=span-trace-coverage#2.3 v=yes
why: the hook exit-0 and silent no-op rules are the hook contract item 14 says the removal must not move
@answer n=7 entity=span-trace-coverage#2.4 v=yes
why: channel-request and channel-response lines are the Linux log contract item 11 holds fixed, in viola-channel whose Windows transport arms the chunk removes
@answer n=8 entity=span-trace-coverage#2.5 v=yes
why: the accept-loop peer-euid filter is the peer-euid comparison item 10 keeps, and any macOS arm beneath it leaves with the chunk
@answer n=9 entity=span-trace-coverage#2.6 v=yes
why: mcp.tool_call and the codes-only reject lines are what the MCP server-verification path item 10 names writes on refusal
@answer n=10 entity=span-trace-coverage#3.1 v=yes
why: the wait and last server check inside the dispatch span, ending exit 21 with server-verify-failed, is the path item 10 keeps; the macOS euid-only carve-out leaves with its arm
@answer n=11 entity=span-trace-coverage#3.2 v=yes
why: the client euid read and peer_admitted refusal are the peer-euid comparison item 10 keeps; the Windows no-euid arm leaves
@answer n=12 entity=span-trace-coverage#3.3 v=yes
why: the captured-span refusal cases per frame verb and for the hook are tests the chunk keeps, and the refused_inside helper's macOS gate is one the chunk changes
@answer n=13 entity=span-trace-coverage#3.4 v=yes
why: the socket-directory captured-span cases and the Seen recorder sit beside the macOS-gated refused_inside helper, which the chunk's gate removal touches
@answer n=14 entity=span-trace-coverage#3.5 v=yes
why: the instance check's exit 21 and the instance-unreachable strict-modes-failed document are named by item 10 as unchanged on Linux
@answer n=15 entity=span-trace-coverage#3.6 v=yes
why: the socket-dir-refused exit 21 on the frame verbs is a detail item 10 names as staying unchanged
@answer n=16 entity=span-trace-coverage#3.7 v=yes
why: the socket-directory human pair is the fixed two-line refusal pair item 13 keeps through human.rs refuse
@answer n=17 entity=span-trace-coverage#3.8 v=yes
why: the hook's shared connect and server check, with its macOS euid-only carve-out, is the hook path item 14 keeps and the chunk's macOS arm removal edits
@answer n=18 entity=span-trace-coverage#3.9 v=yes
why: hook-decision socket-dir-refused on the dialog path is a detail item 10 names and the hook contract of item 14 covers
@answer n=19 entity=span-trace-coverage#3.10 v=yes
why: the -32602 closed word on channel-response is a detail of the catalog item 11 keeps for every Linux value
@answer n=20 entity=span-trace-coverage#3.11 v=yes
why: the MCP frame tools' instance check, socket-directory check and server verification, with the macOS carve-out, are the MCP paths item 10 names
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: CLI reply_of and MCP of_reply screening is shared code that neither the removal edits nor any scope item names, a re-check every change owes
@answer n=22 entity=span-trace-coverage#4.1 v=yes
why: the run start span sequence is the run start scenario item 9 names; the run.conpty_sideload step (Windows) leaves and the Linux chain stays
@answer n=23 entity=span-trace-coverage#4.2 v=yes
why: run.conpty_sideload attributes, the pty_backend closed words conpty-sideload and conpty, and the Windows named-pipe endpoint_kind are companions item 9 names for removal
@answer n=24 entity=span-trace-coverage#4.3 v=yes
why: sideload_fallback on process-start and its closed schema are companions item 9 names; the run start's refusal details stay under items 11 and 13
@answer n=25 entity=span-trace-coverage#4.4 v=yes
why: process-exit from the handle-wait thread, never on reader EOF, with exit_source kill-fallback is the PTY seam behaviour item 12 keeps
