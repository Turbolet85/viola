@answer n=1 entity=span-trace-coverage#1.1 v=no
why: the CLIENT span name list is OS-neutral and the chunk adds, renames and drops none of these names, so it is a re-check every change owes
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: the SERVER span name list is OS-neutral and the chunk adds, renames and drops none of these names, so it is a re-check every change owes
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: the INTERNAL kind classification of run.* and state.* is unchanged for the spans that remain, and the removed run.conpty_sideload span leaves by its own removal, not by this rule
@answer n=4 entity=span-trace-coverage#2.1 v=yes
why: the cli verbs' process-start and process-exit lines, their refusal ordering and their detail words are the Linux log contract the chunk re-asserts (task item 11)
@answer n=5 entity=span-trace-coverage#2.2 v=yes
why: viola run's child process-start and process-exit from the handle-wait thread, and the pty seam, are the Linux PTY seam the chunk keeps (task item 12)
@answer n=6 entity=span-trace-coverage#2.3 v=yes
why: the hook's exit-0 path, its instance check ordering and its hook-decision lines are the hook contract the removal must not move (task item 14)
@answer n=7 entity=span-trace-coverage#2.4 v=yes
why: the channel-request and channel-response line fields and the -32602 closed word of section 6 are the Linux wire contract the removal leaves unchanged (task item 11), while the corr and conn join key itself is left out
@answer n=8 entity=span-trace-coverage#2.5 v=yes
why: the accept-loop peer euid filter is the Linux peer-euid decision the chunk keeps, and the macOS carve-out sits beside it (task item 10)
@answer n=9 entity=span-trace-coverage#2.6 v=yes
why: the MCP tool-call span and its codes-only reject lines are the MCP server-verification path the chunk keeps on Linux (task item 10)
@answer n=10 entity=span-trace-coverage#3.1 v=yes
why: the frame verbs' server check, its euid decision and server-verify-failed on Linux stay, and the macOS euid-only carve-out leaves with the macOS arm (task item 10)
@answer n=11 entity=span-trace-coverage#3.2 v=yes
why: the three client euid seams and peer_admitted are the Linux peer-euid comparison the chunk keeps, and the Windows no-euid arm leaves with its cfg(windows) code (task item 10)
@answer n=12 entity=span-trace-coverage#3.3 v=yes
why: the captured-span refusal cases per verb and the shared refused_inside helper are tests the chunk reaches, since the helper's macOS gate is removed
@answer n=13 entity=span-trace-coverage#3.4 v=yes
why: the socket-directory captured-span cases and the refused_seen recorder sit beside the macOS-gated refused_inside, whose gate the chunk removes
@answer n=14 entity=span-trace-coverage#3.5 v=yes
why: the cli instance check, its exit 21 and the instance-unreachable document stay unchanged on Linux (task item 10)
@answer n=15 entity=span-trace-coverage#3.6 v=yes
why: the socket-dir-refused process-exit at WARN on the frame verbs is a Linux refusal the chunk keeps (task item 10)
@answer n=16 entity=span-trace-coverage#3.7 v=yes
why: the plain-ASCII socket-directory human pair is the fixed refusal pair that stays for the Linux refusals (task item 13)
@answer n=17 entity=span-trace-coverage#3.8 v=yes
why: the hook's server check and its hook-decision server-verify-failed and channel-unreachable fallbacks are the hook contract on Linux, and the macOS carve-out leaves with its arm (task items 10 and 14)
@answer n=18 entity=span-trace-coverage#3.9 v=yes
why: the hook's socket-dir-refused hook-decision at WARN is the Linux hook refusal the chunk keeps (task items 10 and 14)
@answer n=19 entity=span-trace-coverage#3.10 v=yes
why: the -32602 detail closed word on channel-response and its fault_of reading are the Linux closed-word contract the chunk keeps (task item 11)
@answer n=20 entity=span-trace-coverage#3.11 v=yes
why: the MCP frame tools' instance check, socket-directory check and server-verify-failed reply are Linux server-verification paths, and the macOS carve-out leaves with its arm (task item 10)
@answer n=21 entity=span-trace-coverage#3.12 v=yes
why: the CLI reply_of and MCP of_reply closed-word screens are the Linux closed-word and wire contract the chunk keeps (task item 11)
@answer n=22 entity=span-trace-coverage#4.1 v=yes
why: the run start sequence names run.conpty_sideload (Windows), a span this chunk removes (task item 9), while the Unix-only settings write and the socket-directory refusal stay
@answer n=23 entity=span-trace-coverage#4.2 v=yes
why: the run.conpty_sideload attributes name Windows code the chunk removes, and the pty_backend and endpoint_kind value sets name the Linux values it keeps (task items 8 and 9)
@answer n=24 entity=span-trace-coverage#4.3 v=yes
why: the sideload_fallback field and its closed words name code the chunk removes, while the process-start and process-exit details stay for Linux (task items 9 and 11)
@answer n=25 entity=span-trace-coverage#4.4 v=yes
why: the handle-wait process-exit with exit_source handle-wait or kill-fallback, never on reader EOF, is the PTY seam the chunk keeps on Linux (task item 12)
