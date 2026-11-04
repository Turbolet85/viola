@answer n=1 entity=log-coverage#1.1 v=yes
why: process-start carries subject claude-child and pty_backend openpty, which the Unix PTY seam logs at spawn (items 1, 2).
@answer n=2 entity=log-coverage#1.2 v=yes
why: process-exit carries exit_source handle-wait and kill-fallback, the exit paths the seam must log from the handle-wait thread (item 2).
@answer n=3 entity=log-coverage#1.3 v=yes
why: channel-request fields are the Unix channel telemetry that must carry corr and conn on both sides (item 7).
@answer n=4 entity=log-coverage#1.4 v=yes
why: channel-response's closed method, detail and error_code words are the Unix channel telemetry words both sides keep (items 5, 7).
@answer n=5 entity=log-coverage#1.5 v=yes
why: send-issued and send-confirmed carry text_bytes, the only paste field the PTY seam logs (item 2).
@answer n=6 entity=log-coverage#1.6 v=yes
why: dialog-raised and dialog-answered fields are part of the one field catalog the chunk re-asserts on every OS (item 4, task prose).
@answer n=7 entity=log-coverage#1.7 v=yes
why: hook-invoked and hook-decision fields and the hook_event closed list are what the Unix hook contract and the one schema keep (items 4, 19).
@answer n=8 entity=log-coverage#1.8 v=yes
why: the panic record fields panic_location and thread are the single-line event panic written by the panic hook on every OS (item 10).
@answer n=9 entity=log-coverage#1.9 v=yes
why: release-from-driver fields conn, from and from_trust sit in the closed-enum field catalog re-asserted on every OS (items 4, 5).
@answer n=10 entity=log-coverage#1.10 v=yes
why: state-recovered detail words and its file and offset fields must match on every OS, the snapshot-replayed and torn-line-healed words named by item 21.
@answer n=11 entity=log-coverage#1.11 v=yes
why: the parse-rejected parser list is a closed word set the one schema re-assertion must hold on every OS, including channel-frame and mcp-stdin (item 5).
@answer n=12 entity=log-coverage#2.1 v=yes
why: the exit-1 detail words socket-dir-refused, no-session, session-live and cwd-missing are the Unix run and revive pre-child refusals (items 5, 8, 9).
@answer n=13 entity=log-coverage#2.2 v=yes
why: the exit-21 detail words server-verify-failed and socket-dir-refused carry the macOS euid-only carve-out and the Unix endpoint refusals (items 5, 6, 7).
@answer n=14 entity=log-coverage#2.3 v=yes
why: wrapper-fault is a word of the closed exit-20 detail list the schema closes, so it must hold on every OS with no OS-only word (item 5).
@answer n=15 entity=log-coverage#2.4 v=yes
why: the hook-decision fail-open codes include server-verify-failed and socket-dir-refused, which the Unix hook path must write under the hook contract (items 5, 7, 19).
@answer n=16 entity=log-coverage#2.5 v=yes
why: the per-parser parse-rejected detail words, including panicked under catch_unwind, are closed words the schema holds on every OS (items 5, 10).
@answer n=17 entity=log-coverage#2.6 v=no standing=S2
why: the mcp-stdin oversize line states obs_event! routing, the S2 convention, and is an existing mcp.ndjson line the chunk does not change; its closed words are carried by the parser list at #1.11.
@answer n=18 entity=log-coverage#2.7 v=no
why: send-refused reasons are an open string in diag-line.v1.json and are decided above the PTY seam, which the chunk leaves unchanged in its refusal words, so no closed word here needs the parity re-assertion.
@answer n=19 entity=log-coverage#2.8 v=yes
why: the filesystem refusals name the Unix home-tree, socket-dir and verify refusals that the chunk writes as fixed stderr pairs or as closed exit-1 and exit-21 lines (items 6, 7, 8, 9).
@answer n=20 entity=log-coverage#3.1 v=yes
why: the spawn pairs and the closed subject enum cover the Unix PTY spawns, including verify's interactive runs that the openpty seam drives (items 1, 2, 22).
