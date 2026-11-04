@answer n=1 entity=log-coverage#1.1 v=yes
why: pty_backend takes openpty on Unix and subject claude-child is the PTY spawn the chunk adds (needs 1, 2).
@answer n=2 entity=log-coverage#1.2 v=yes
why: exit_source handle-wait and kill-fallback are the child exit the Unix PTY seam logs (needs 2).
@answer n=3 entity=log-coverage#1.3 v=yes
why: Unix channel telemetry logs channel-request with conn on both sides (needs 7).
@answer n=4 entity=log-coverage#1.4 v=yes
why: Unix channel telemetry logs channel-response with conn, detail and error_code on both sides (needs 7).
@answer n=5 entity=log-coverage#1.5 v=yes
why: paste logs text_bytes, the field the scope names (needs 2).
@answer n=6 entity=log-coverage#1.6 v=yes
why: the schema re-assert on every OS covers the dialog field row (needs 4, 5; schemas/diag-line.v1.json).
@answer n=7 entity=log-coverage#1.7 v=yes
why: the hook contract runs under the Unix suite and its hook-invoked and hook-decision fields must match on every OS (needs 19).
@answer n=8 entity=log-coverage#1.8 v=yes
why: the panic record's panic_location and thread fields bind the panic hook on every OS (needs 10).
@answer n=9 entity=log-coverage#1.9 v=yes
why: the release line's conn and from fields are the channel telemetry the Unix release path carries (needs 7; schema re-assert).
@answer n=10 entity=log-coverage#1.10 v=yes
why: state-recovered keeps the same detail words and file names on every OS (needs 21).
@answer n=11 entity=log-coverage#1.11 v=yes
why: the parser and detail closed words are the catalog the schema re-assert holds, including the Unix channel-frame and hook-stdin parsers (needs 5).
@answer n=12 entity=log-coverage#2.1 v=yes
why: run's and revive's pre-child Unix refusals (socket-dir-refused, no-session, cwd-missing, home-tree pair) are named by the chunk (needs 5, 8, 9).
@answer n=13 entity=log-coverage#2.2 v=yes
why: the Unix CLI frame verbs write server-verify-failed and socket-dir-refused, and the macOS euid-only check binds (needs 5, 6).
@answer n=14 entity=log-coverage#2.3 v=yes
why: wrapper-fault is a closed detail word of the schema the chunk re-asserts on every OS (needs 5).
@answer n=15 entity=log-coverage#2.4 v=yes
why: the hook fail-open codes, including socket-dir-refused and server-verify-failed on Unix, are the hook contract the Unix suite runs (needs 5, 6, 19).
@answer n=16 entity=log-coverage#2.5 v=yes
why: the per-parser detail words are the closed catalog the schema re-assert holds, including the Unix channel-frame and hook-stdin parsers (needs 5, 7).
@answer n=17 entity=log-coverage#2.6 v=no standing=S2
why: every new log line goes through obs_event! with the closed event enum and catalog-named typed fields only.
@answer n=18 entity=log-coverage#2.7 v=no
why: the arch RefusalReason words are OS-independent and the scope names only paste's text_bytes, so the refusal words are a re-check on a send component the chunk only uses.
@answer n=19 entity=log-coverage#2.8 v=yes
why: the Unix home-tree and socket-dir refusals and their strict-modes-failed and socket-dir-refused words are named by the scope (needs 5, 8, 9).
@answer n=20 entity=log-coverage#3.1 v=yes
why: verify-pty-probe, agents-probe and the closed subject enum cover the Unix PTY spawns and the fake-agent verify runs (needs 2, 5, 22).
