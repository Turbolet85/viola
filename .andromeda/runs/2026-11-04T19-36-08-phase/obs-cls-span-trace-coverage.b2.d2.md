@answer n=1 entity=span-trace-coverage#5.1 v=no
why: pin-copy placement is a span-level outcome that writes no line or field, and no named surface or task need reaches it.
@answer n=2 entity=span-trace-coverage#6.1 v=yes
why: the send chain's server check and macOS euid-only carve-out sit before channel.request, which the Unix endpoint and carve-out needs (6 and 7) must honour.
@answer n=3 entity=span-trace-coverage#6.2 v=yes
why: pty.paste_write text_bytes and paste_mode are the paste log the Unix PTY seam must write (need 2).
@answer n=4 entity=span-trace-coverage#6.3 v=yes
why: the send log lines and their closed refusal words, server-verify and socket-dir refusals included, are the events.ndjson and process-log schema the scope re-asserts on every OS.
@answer n=5 entity=span-trace-coverage#6.4 v=yes
why: the process-exit detail words server-verify-failed and socket-dir-refused and the exit-21 during values form the closed detail catalog (need 5).
@answer n=6 entity=span-trace-coverage#7.1 v=yes
why: the wait and last server check and its macOS carve-out sit before channel.request (needs 6 and 7).
@answer n=7 entity=span-trace-coverage#7.2 v=no
why: wait.block and run.wait_dispatch attributes are span-only, outside the named schema surfaces, and the chunk adds no span on this path.
@answer n=8 entity=span-trace-coverage#7.3 v=yes
why: the wait and last channel-request and channel-response fields and outcome words are process-log fields the schema re-asserts, instance-unreachable included.
@answer n=9 entity=span-trace-coverage#7.4 v=yes
why: wait.block closes on a socket-directory or server-verification refusal before any frame, with its process-exit line, which is the Unix refusal path (needs 6 and 7).
@answer n=10 entity=span-trace-coverage#8.1 v=yes
why: the dialog hook's server check inside hook.handle and the driver's answer check, with the macOS carve-out, bind the Unix fail-open path (needs 6 and 19).
@answer n=11 entity=span-trace-coverage#8.2 v=no
why: run.dialog_register, run.dialog_await and hook.decision_emit attributes are span-only, outside the named schema surfaces.
@answer n=12 entity=span-trace-coverage#8.3 v=yes
why: the dialog-raised, dialog-answered, hook-invoked and hook-decision fields and fail-open detail words, server-verify-failed and socket-dir-refused included, are log fields that the schema and the hook contract (need 19) require.
@answer n=13 entity=span-trace-coverage#8.4 v=yes
why: the dialog-expired and dialog-answered events.ndjson fields and the hook's exit 0 with empty stdout at expiry are the events schema and the hook contract (need 19).
@answer n=14 entity=span-trace-coverage#9.1 v=yes
why: pause and release's socket-directory check, euid decision and server check before any frame, with the macOS carve-out, are the Unix endpoint paths (needs 6 and 7).
@answer n=15 entity=span-trace-coverage#9.2 v=no
why: run.wheel_transition attributes are span-only and outside the named schema surfaces.
@answer n=16 entity=span-trace-coverage#9.3 v=yes
why: the send-refused human-typing line, the release channel lines, release-from-driver and the pause and release refusal lines are process-log and events fields the schema re-asserts.
@answer n=17 entity=span-trace-coverage#9.4 v=no
why: the entity gives a point-in-time span and an audit pointer to the wheel event, with no field, word or unit the chunk must hold.
@answer n=18 entity=span-trace-coverage#10.1 v=no
why: the statusline and budget span tree is span-only and the chunk does not name it.
@answer n=19 entity=span-trace-coverage#10.2 v=no
why: budget span attributes are span-only and outside the named schema surfaces.
@answer n=20 entity=span-trace-coverage#10.3 v=yes
why: hook-invoked, parse-rejected, the statusline-shell process lines and hook-decision detail words are process-log fields, and the Unix statusline shell-out logs them under the hook contract (need 19).
@answer n=21 entity=span-trace-coverage#10.4 v=yes
why: the hook's stdout passthrough, empty output on oversize or malformed stdin, and the deadline detail are the Unix hook contract (need 19) and a closed detail word.
@answer n=22 entity=span-trace-coverage#11.1 v=yes
why: the E2 process-start fields on claude-child and the Unix exact-match strip are what the chunk's Unix spawn logs (need 2).
@answer n=23 entity=span-trace-coverage#12.1 v=yes
why: the events.ndjson link line for an implicit pair, and the send or answer channel pair that carries it, are the events schema on every OS.
@answer n=24 entity=span-trace-coverage#13.1 v=yes
why: the link and unlink channel-response detail invalid-params and the events.ndjson link and unlink lines are closed words and fields the schema re-asserts.
@answer n=25 entity=span-trace-coverage#14.1 v=yes
why: the state-recovered detail words and fields, the same on every OS (need 21), and the revive start arm's line (need 9) bind; its span sentence is standing S3 and not carried.
