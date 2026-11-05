@answer n=1 entity=threat-model-summary#1.1 v=no
why: prompt and reply logging to events.ndjson and its retention are not reached; the chunk changes no event write path
@answer n=2 entity=threat-model-summary#1.2 v=no
why: tool-call, plan and dialog payloads on hook.dialog are not reached; the chunk changes no dialog path
@answer n=3 entity=threat-model-summary#1.3 v=yes
why: the statusline shell-out, the only shell-out, and the Unix-only settings override are made unconditional, and this entity states what they run and write
@answer n=4 entity=threat-model-summary#1.4 v=no
why: the R8 strip of CLAUDE* variables from the child environment stays as it is on Linux; the removed Windows registry read is not what this entity states
@answer n=5 entity=threat-model-summary#1.5 v=no standing=S5
why: the cwd clause (absolute path on no log line, error body or stdout) is the NEVER-log floor, which each new log line owes and the chunk adds none
@answer n=6 entity=threat-model-summary#2.1 v=yes
why: the Windows named-pipe arms leave while the Unix socket-directory, peer euid, peer pid and start-time checks and the closed refusals must stay whole, and the macOS carve-out leaves
@answer n=7 entity=threat-model-summary#3.1 v=no
why: the send text path and delivery confirmation are not edited, and the chunk changes no send input path
@answer n=8 entity=threat-model-summary#4.1 v=no
why: the stdin classifier and its closed non-editing list are unchanged on Linux, and no PTY input path changes
@answer n=9 entity=threat-model-summary#5.1 v=yes
why: hooks must exit 0 with no body on any failure, and each hook path's strict-modes check and the statusline shell-out keep their Linux behaviour
@answer n=10 entity=threat-model-summary#6.1 v=yes
why: the MCP send, wait, last and answer tools keep the strict-modes check, the socket-directory check and the peer euid refusal before any frame
@answer n=11 entity=threat-model-summary#7.1 v=no
why: the MCP stdin CappedLines reader at MAX_FRAME is not reached and no scope item names it
@answer n=12 entity=threat-model-summary#8.1 v=yes
why: the strict-modes checks check_home_tree, check_instance and check_read lose their Windows arms, and the exit 21 and exit 1 refusals must hold on Linux
@answer n=13 entity=threat-model-summary#9.1 v=yes
why: the home-tree strict-modes check loses its Windows owner-and-DACL arm, and the conpty companion check in pin_companions leaves with sideload.rs
@answer n=14 entity=threat-model-summary#10.1 v=no
why: the first placement of the pinned binary by place_private_new is not edited by any scope item
@answer n=15 entity=threat-model-summary#11.1 v=yes
why: the scope removes the Windows npm-shim to claude.exe resolution and the .cmd and .bat refusal that this entity states for the claude spawn
@answer n=16 entity=threat-model-summary#12.1 v=yes
why: cargo deny over deny.toml, the Cargo.lock pin and the SHA-pinned actions and rustup toolchains all change with the manifest and workflow edits
@answer n=17 entity=threat-model-summary#13.1 v=yes
why: the ci.yml three-OS matrix, the nightly Windows and macOS jobs and the per-OS release and perf steps are what the scope edits
