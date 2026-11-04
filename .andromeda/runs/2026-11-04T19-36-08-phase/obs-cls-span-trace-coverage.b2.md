@answer n=1 entity=span-trace-coverage#5.1 v=no
why: pin_copy placement and its ok or pinned-hash-mismatch outcome are run-start steps the Unix chunk does not change, no scope line names them, and they open no span or line

@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the send must-trace path runs the socket-directory check and the server check before channel.request, with the macOS euid-only carve-out (needs 6, 7)

@answer n=3 entity=span-trace-coverage#6.2 v=yes
why: pty.paste_write text_bytes and paste_mode on the PTY seam are the paste-logs-text_bytes-only need (needs 2); the run span attributes are unchanged

@answer n=4 entity=span-trace-coverage#6.3 v=yes
why: the send log lines and the client-side server-verify and socket-directory refusals ending exit 21 at WARN bind the Unix channel telemetry (needs 6, 7) and the closed detail words

@answer n=5 entity=span-trace-coverage#6.4 v=yes
why: send.client exit codes include exit 21 with server-verify-failed and socket-dir-refused during connect on every OS (needs 5, 6, 7)

@answer n=6 entity=span-trace-coverage#7.1 v=yes
why: wait and last run the server check inside wait.block and last.client before channel.request, with the macOS euid-only carve-out (needs 6, 7)

@answer n=7 entity=span-trace-coverage#7.2 v=no
why: wait.block and run.wait_dispatch attributes are not named by the scope, and the wait code is one the chunk uses unchanged, so it is a re-check every change owes

@answer n=8 entity=span-trace-coverage#7.3 v=yes
why: wait and last channel-request and channel-response lines carry corr and conn on both sides, and the exit 21 server-verify-failed and socket-dir-refused lines bind (needs 6, 7)

@answer n=9 entity=span-trace-coverage#7.4 v=yes
why: wait.block closes on a socket-directory or server-verify refusal made before any frame, so no channel.request opens (needs 6, 7)

@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the dialog hook runs its server check before channel.request(hook.dialog) and the driver runs its own before channel.request(answer), with the macOS carve-out (needs 6, 7, 19)

@answer n=11 entity=span-trace-coverage#8.2 v=no
why: dialog_register, dialog_await and decision_emit span attributes are not named by the scope, and the Unix hook contract covers stdout and stderr, not spans

@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: the fail-open hook-decision lines for server-verify-failed and socket-dir-refused, the silent refused-instance exit and the CLI answer exit 21 lines bind (needs 6, 7, 19)

@answer n=13 entity=span-trace-coverage#8.4 v=yes
why: the hook exits 0 with empty stdout at the hold's end and at its backstop, under the Unix hook contract (needs 19)

@answer n=14 entity=span-trace-coverage#9.1 v=yes
why: pause and release run the socket-directory check and the server check before any channel.request, with the macOS carve-out (needs 6, 7)

@answer n=15 entity=span-trace-coverage#9.2 v=no standing=S3
why: span convention (skip_all, static name, explicit allow-list, never content in span fields); the wheel attribute list is not named by the scope

@answer n=16 entity=span-trace-coverage#9.3 v=yes
why: pause and release server-verify and socket-directory refusals ending exit 21 before any channel line, and the release-from-driver and invalid-params lines, bind the Unix channel telemetry (needs 6, 7)

@answer n=17 entity=span-trace-coverage#9.4 v=no
why: the cleanup restates a point-in-time span and the wheel event as audit record, which the Unix work does not change and no scope line names

@answer n=18 entity=span-trace-coverage#10.1 v=no
why: budget must-trace spans are not named by the scope, and the Unix hook contract covers stdout and stderr, not span paths

@answer n=19 entity=span-trace-coverage#10.2 v=no standing=S3
why: span convention (skip_all, static name, explicit allow-list, never content in span fields); the budget attribute list is not named by the scope

@answer n=20 entity=span-trace-coverage#10.3 v=yes
why: statusline-shell process-start and process-exit lines exist off Windows only, and the hook-decision detail words bind the Unix hook contract (needs 19, 5)

@answer n=21 entity=span-trace-coverage#10.4 v=yes
why: the statusline hook passes stdout through unlogged, stays inside its 5 s deadline, and prints nothing and runs no shell on oversize or malformed stdin (needs 19)

@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: the Unix spawn strips inherited CLAUDE names by exact match and writes env_stripped_count, env_stripped_known and env_kept on the claude-child process-start line (needs 2, 4)

@answer n=23 entity=span-trace-coverage#12.1 v=no
why: the implicit link is run state that the scope does not name, with no span and no line of its own, and its channel pair is covered by the send and answer entries

@answer n=24 entity=span-trace-coverage#13.1 v=no
why: explicit link and unlink serving is run state the chunk does not change, and its invalid-params detail word is neither OS-only nor named by the scope

@answer n=25 entity=span-trace-coverage#14.1 v=yes
why: state-recovered snapshot-replayed and snapshot-unsupported-v keep the same detail words on every OS (needs 21); its span-shape clause is standing S3 and is not carried
