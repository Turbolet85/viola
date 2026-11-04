@answer n=1 entity=threat-model-summary#1.1 v=yes
why: events.ndjson carries prompt text and final replies that the chunk's events.ndjson schema and the user-content floor cover on every OS
@answer n=2 entity=threat-model-summary#1.2 v=yes
why: permission input, plan and question text land in events.ndjson and hook.dialog, which the events.ndjson schema and the user-content floor cover
@answer n=3 entity=threat-model-summary#1.3 v=yes
why: the statusline_command in snapshot.json and the per-session statusline settings override are the Unix-only statusline shell-out the chunk writes
@answer n=4 entity=threat-model-summary#1.4 v=yes
why: CLAUDE* parent-identity values are R8-stripped from the child and must reach no log, diagnostic, snapshot or events line on any OS
@answer n=5 entity=threat-model-summary#1.5 v=yes
why: the snapshot's recorded cwd stands only in the 0600 snapshot and on no log line, which the chunk's NEVER-log floor names
@answer n=6 entity=threat-model-summary#2.1 v=yes
why: the wrapper IPC endpoint, socket directory, peer euid and client verify and the instance-unreachable details are what the chunk implements on Unix
@answer n=7 entity=threat-model-summary#3.1 v=yes
why: the send bytes (bracketed-paste framing, CR normalisation, exact delivery-confirmation equality) are carried by the pty seam's write verb the scope names and read by the Unix fake-agent send tests
@answer n=8 entity=threat-model-summary#4.1 v=yes
why: the stdin classifier's closed non-editing (wheel) list is named by the chunk, and the v1-40 witness under the outer PTY reads this stdin
@answer n=9 entity=threat-model-summary#5.1 v=yes
why: hook stdin fail-open to exit 0 with no body, the strict-modes check before any hook read and the hook statusline shell-out are named by the chunk
@answer n=10 entity=threat-model-summary#6.1 v=yes
why: the MCP send, wait, last and answer tools run the strict-modes, socket-dir and peer euid checks before a frame, which the chunk requires of every Unix client
@answer n=11 entity=threat-model-summary#7.1 v=no
why: the MCP stdin CappedLines reader is neither named by the scope nor edited or driven by the chunk, so its oversize rule is a re-check every change owes
@answer n=12 entity=threat-model-summary#8.1 v=yes
why: the CLI --home and VIOLA_DIR choice, the strict-modes refusals and the --json internal-error document are named by the chunk's exit-code, --json and home checks
@answer n=13 entity=threat-model-summary#9.1 v=yes
why: the ~/.viola/ home tree's strict-modes check before the wrapper log opens is the Unix home strict-modes the chunk implements, and its Windows DACL half is a Windows control the chunk leaves in place
@answer n=14 entity=threat-model-summary#10.1 v=yes
why: the pinned viola is placed at 0700 under the chunk's home strict-modes, so the write-handle-closed-before-name placement that avoids ETXTBSY binds that placement
@answer n=15 entity=threat-model-summary#11.1 v=yes
why: viola run's child resolution to an absolute path, its version probe and the pinned folder's PATH order are named by the portable-pty spawn the chunk adds
@answer n=16 entity=threat-model-summary#12.1 v=no standing=S3
why: the convention that each new CI job or step pins its actions by full commit SHA with a version comment, which the SHA-pinned action list states, and the chunk adds no dependency
@answer n=17 entity=threat-model-summary#13.1 v=yes
why: the ci.yml jobs (per-OS tests, the per-OS perf job and the release check refusing test-only features) are the file and gates the chunk changes
