@answer n=1 entity=span-trace-coverage#5.1 v=no
why: the pin-copy placement and its outcome set are not named by the scope, which names only run's Unix pre-child refusals and socket_dir_ready
@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the send's server check, socket-directory check and macOS euid-only carve-out bind before channel.request (scope items 6 and 7)
@answer n=3 entity=span-trace-coverage#6.2 v=yes
why: pty.paste_write text_bytes and paste_mode bind through scope item 2, since paste logs text_bytes only
@answer n=4 entity=span-trace-coverage#6.3 v=yes
why: send's refusal lines for server-verify-failed and socket-dir-refused, written before any channel line, carry the Unix closed detail words (scope items 5 and 6)
@answer n=5 entity=span-trace-coverage#6.4 v=yes
why: send.client's exit 21 process-exit with detail server-verify-failed or socket-dir-refused during connect is the Unix closed detail catalog (scope item 5)
@answer n=6 entity=span-trace-coverage#7.1 v=yes
why: wait and last run the server check and socket-directory check before channel.request, with the macOS euid-only carve-out (scope item 6)
@answer n=7 entity=span-trace-coverage#7.2 v=no
why: wait.block and run.wait_dispatch span attributes are not named by the scope, and spans are outside the diagnostics schema it re-asserts
@answer n=8 entity=span-trace-coverage#7.3 v=yes
why: wait and last log lines carry process-exit exit 21 with server-verify-failed (macOS euid-only) and socket-dir-refused during connect (scope item 6)
@answer n=9 entity=span-trace-coverage#7.4 v=yes
why: wait.block closes on a socket-directory or server-verification refusal made before any frame, which the Unix endpoint telemetry must hold (scope items 6 and 7)
@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the dialog hook's and the driver's server check and socket-directory check run before each frame, with the macOS euid carve-out (scope items 6 and 7)
@answer n=11 entity=span-trace-coverage#8.2 v=no
why: dialog span attributes are not named by the scope, and spans are outside the diagnostics schema it re-asserts
@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: dialog and hook log lines, including the fail-open server-verify-failed and socket-dir-refused decisions, carry the Unix closed detail words (scope items 5 and 7)
@answer n=13 entity=span-trace-coverage#8.4 v=yes
why: the dialog-expired and dialog-answered events.ndjson lines and their field set (no from) are read by the one schema the chunk re-asserts on every OS (scope item 5)
@answer n=14 entity=span-trace-coverage#9.1 v=yes
why: pause and release run the socket-directory check and server check before channel.request, with the macOS euid carve-out (scope items 6 and 7)
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: run.wheel_transition span attributes are not named by the scope, and spans are outside the diagnostics schema it re-asserts
@answer n=16 entity=span-trace-coverage#9.3 v=yes
why: the pause and release log lines carry server-verify-failed and socket-dir-refused refusals, and release-from-driver's closed detail (scope items 5 to 7)
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: it states the wheel event's audit role and the point-in-time span, which is neither a field, a closed word nor a check the chunk re-asserts
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: hook.statusline and budget span structure are not named by the scope, which names no span or attribute of them
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: statusline and budget span attributes are not named by the scope, and spans are outside the diagnostics schema it re-asserts
@answer n=20 entity=span-trace-coverage#10.3 v=yes
why: the statusline hook's Unix log lines (statusline-shell process-start and exit, parse-rejected, hook-decision details, no line without VIOLA_NAME) bind through scope item 19 and the one schema
@answer n=21 entity=span-trace-coverage#10.4 v=yes
why: the statusline hook's stdout passes through unchanged and is never logged, which is the Unix hook contract of scope item 19 (stdout only status-line output, stderr empty)
@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: the CLAUDE* strip is exact on Unix with the config.json claude_env_keep set, and it shapes the env and process-start line of the claude-child spawn the Unix pty seam makes (scope item 2)
@answer n=23 entity=span-trace-coverage#12.1 v=yes
why: the implicit link writes a link product line and no process-log line or event value, which the closed event enum and events.ndjson schema re-asserted on every OS must hold (scope item 5)
@answer n=24 entity=span-trace-coverage#13.1 v=yes
why: the driven wrapper's link and unlink channel lines carry corr and conn and the invalid-params detail on -32602, under the closed detail catalog and channel telemetry (scope items 5 and 7)
@answer n=25 entity=span-trace-coverage#14.1 v=yes
why: state-recovered snapshot-replayed and snapshot-unsupported-v lines keep their detail words on every OS, and the revive start arm writes one (scope items 9 and 21)
