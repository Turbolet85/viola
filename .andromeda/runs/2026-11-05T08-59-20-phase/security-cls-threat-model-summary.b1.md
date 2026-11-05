@answer n=1 entity=threat-model-summary#1.1 v=no
why: events.ndjson retention and content volume; the chunk changes no event log, and the scope names no rule over it
@answer n=2 entity=threat-model-summary#1.2 v=no
why: user-content volume in events.ndjson and the permission payloads; the chunk changes no event or dialog payload
@answer n=3 entity=threat-model-summary#1.3 v=yes
why: the statusline_command shell-out and the Unix-only settings override that the scope makes unconditional name this config
@answer n=4 entity=threat-model-summary#1.4 v=no
why: transient CLAUDE* credential stripping from the child environment; the chunk does not touch the env strip or any credential path
@answer n=5 entity=threat-model-summary#1.5 v=yes
why: the NEVER-log floor on the snapshot cwd (0600 snapshot only, no log line); the scope keeps that floor as it is
@answer n=6 entity=threat-model-summary#2.1 v=yes
why: the Unix IPC rules (socket-dir check, euid peer check, closed refusals) the scope keeps whole while the Windows arms leave
@answer n=7 entity=threat-model-summary#3.1 v=no
why: CLI send text and paste-terminator handling; the chunk changes no send or PTY input path on Linux
@answer n=8 entity=threat-model-summary#4.1 v=no
why: the human terminal stdin classifier; scope item not_needed 6 holds, no PTY input path changes on Linux
@answer n=9 entity=threat-model-summary#5.1 v=yes
why: hook fail-open (exit 0, no body) and the strict-modes check before any home read, which the scope keeps and the statusline shell-out it names
@answer n=10 entity=threat-model-summary#6.1 v=yes
why: the MCP tools' strict-modes, socket-dir and euid checks before the frame, which the client verification path the chunk keeps must honour
@answer n=11 entity=threat-model-summary#7.1 v=no
why: MCP stdin line cap at MAX_FRAME; the chunk changes no MCP transport or tool input handling
@answer n=12 entity=threat-model-summary#8.1 v=yes
why: the strict-modes checks on the home and instance (check_home_tree, check_instance, check_read) that the Unix arms must keep refusing exactly
@answer n=13 entity=threat-model-summary#9.1 v=yes
why: the strict-modes home-tree check whose Windows owner-and-DACL arm leaves while the Unix owner and mode arm stays
@answer n=14 entity=threat-model-summary#10.1 v=no
why: the pinned-binary placement race and its ETXTBSY handling; the chunk changes no placement path and the scope names none of it
@answer n=15 entity=threat-model-summary#11.1 v=yes
why: the npm-shim to claude.exe resolution that the scope removes with the .cmd/.bat refusal, and the PATH spawn that stays on Linux
@answer n=16 entity=threat-model-summary#12.1 v=yes
why: the cargo deny, Cargo.lock, deny-sync and channel-tokio-free rules over the manifests and deny.toml the scope edits
@answer n=17 entity=threat-model-summary#13.1 v=yes
why: the ci.yml and nightly.yml job set, the per-OS release and perf jobs, and the ubuntu supply-chain job the scope reshapes
