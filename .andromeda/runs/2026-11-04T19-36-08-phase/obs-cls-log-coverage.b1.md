@answer n=1 entity=log-coverage#1.1 v=yes
why: process-start with subject claude-child and pty_backend openpty is what the Unix PTY seam logs at spawn (item 2)
@answer n=2 entity=log-coverage#1.2 v=yes
why: process-exit exit_source handle-wait or kill-fallback is written from the handle-wait thread and the kill fallback (item 2)
@answer n=3 entity=log-coverage#1.3 v=yes
why: channel-request must carry corr and conn on both sides of the Unix channel endpoint telemetry (item 7)
@answer n=4 entity=log-coverage#1.4 v=yes
why: channel-response carries conn, closed detail words and error codes on both sides of the Unix channel telemetry (item 7)
@answer n=5 entity=log-coverage#1.5 v=yes
why: send-issued and send-confirmed fields include text_bytes, which the paste rule of the PTY seam logging keeps as the only text measure (item 2)
@answer n=6 entity=log-coverage#1.6 v=yes
why: dialog line fields are part of the one diagnostics schema the chunk re-asserts on every OS and G4 checks (item 5, schema re-read)
@answer n=7 entity=log-coverage#1.7 v=yes
why: hook-invoked and hook-decision with the closed hook_event catalog are the hook contract under the Unix suite (items 5, 19)
@answer n=8 entity=log-coverage#1.8 v=yes
why: panic record fields panic_location and thread bind the single-line panic record and the catch_unwind sites on every OS (item 10)
@answer n=9 entity=log-coverage#1.9 v=yes
why: release-from-driver is a closed event of the enum the chunk re-asserts, and its conn field is the Unix channel telemetry field (items 5, 7)
@answer n=10 entity=log-coverage#1.10 v=yes
why: state-recovered detail words snapshot-replayed and torn-line-healed must keep the same words on every OS (item 21)
@answer n=11 entity=log-coverage#1.11 v=yes
why: parse-rejected parser and detail catalog includes channel-frame, hook-stdin and vt100-feed, which the chunk's channel, hook and PTY seam work touches (items 2, 5, 7, 19)
@answer n=12 entity=log-coverage#2.1 v=yes
why: exit 1 words socket-dir-refused, no-session and cwd-missing are the Unix pre-child refusals of run and the revive start arm (items 5, 8, 9)
@answer n=13 entity=log-coverage#2.2 v=yes
why: exit 21 words server-verify-failed (with the macOS euid-only carve-out), socket-dir-refused and strict-modes-failed are named by the Unix scope (items 5, 6)
@answer n=14 entity=log-coverage#2.3 v=yes
why: wrapper-fault is a closed exit-20 detail in the catalog the chunk re-asserts and G4 checks on every OS, so the closed word must stay the same (item 5)
@answer n=15 entity=log-coverage#2.4 v=yes
why: hook-decision fail-open codes socket-dir-refused, server-verify-failed and strict-modes-failed are the Unix-reached hook details and the hook contract (items 5, 19)
@answer n=16 entity=log-coverage#2.5 v=yes
why: parse-rejected per-parser detail list is the closed catalog covering vt100-feed under catch_unwind, channel-frame and hook-stdin oversize (items 5, 7, 10, 19)
@answer n=17 entity=log-coverage#2.6 v=yes
why: the mcp-stdin oversize parse-rejected names the mcp.ndjson destination and closed parser and detail words that the log file and catalog re-assertion covers (items 5, 20)
@answer n=18 entity=log-coverage#2.7 v=no
why: refusal details are the arch send refusal words, an open string in diag-line.v1.json and not named by the scope, so no closed word or seam behaviour must be honoured
@answer n=19 entity=log-coverage#2.8 v=yes
why: filesystem refusal details strict-modes-failed and symlink-ignored are the Unix home-tree and revive start refusals and the closed catalog words (items 5, 8, 9)
@answer n=20 entity=log-coverage#3.1 v=yes
why: subject closed enum and the agents-probe and verify-pty-probe spawn pairs are logged through the Unix viola-pty seam and the closed subject catalog (items 2, 5)
