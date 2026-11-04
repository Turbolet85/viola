@carried n=1 label=obs-plan/span-trace-coverage fact=fct-mw7zqeom
why: The openpty-backed wrapper on Linux and macOS must give the child's process-exit from the handle-wait thread, with exit_source handle-wait or kill-fallback, never on reader EOF, as the Windows wrapper does.
quote: The handle-wait thread emits `process-exit{subject:"claude-child", child_exit_status, exit_source:"handle-wait"|"kill-fallback"}`, never on reader EOF (ConPTY chaos class). `run` then emits `process-exit{subject:"self", exit_code, duration_ms}`.
@carried n=1 run=2 label=obs-plan fact=fct-itztmsse
quote: the child's `process-exit` comes from the handle-wait thread, never on reader EOF.
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-lohdixpq
why: The send's server check must keep the macOS carve-out the scope names: on macOS an euid mismatch alone, no peer pid compared, and no test reading it as a comparison.
quote: A server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request`, and the path ends there (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-xfymn57j
why: The pty seam's write leg on openpty must carry the same pty.paste_write attributes as on ConPTY: the typed text's length only, never the text.
quote: `pty.paste_write`: `text_bytes` (the length of the typed text: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters), `paste_mode="bracketed"`;
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-gghzntp4
why: The send's client-side server-verification refusal must write the same closed process-exit on Linux, macOS and Windows, which the one-schema re-assertion reads.
quote: client-side server-verification refusal, made after the client's own refusals and before any frame (Linux, Windows and macOS, on macOS an euid mismatch alone): no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, and MCP `send` writes no line;
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-u2zbdxyd
why: The exit-21 during words and the send's typed exit set must hold on every OS, the scope adding no exit code.
quote: An exit-21 `process-exit`'s `during` is `call` for `endpoint-lost` alone; `instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during` `connect`.
@carried n=5 run=2 label=obs-plan/span-trace-coverage fact=fct-u2zbdxyd
quote: `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`)
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-l4ierws2
why: wait / last on Linux and macOS must run the socket-directory check, the connect, the euid decision and the server check in this order before any channel.request, macOS comparing no peer pid.
quote: The server check runs inside `wait.block` / `last.client`, after the client's own refusals, the socket-directory check, the connect and the euid decision, and before `channel.request`; a refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request` (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=6 run=2 label=obs-plan/span-trace-coverage fact=fct-3omdhkir
quote: `wait.block` closes on the response, on the connection error, or on a socket-directory or server-verification refusal made before any frame (no `channel.request` opened; the client's `process-exit` its outcome line).
@carried n=6 run=3 label=obs-plan fact=fct-q67chfay
quote: a serving process that is not the recorded wrapper (Linux and Windows) or runs under another euid (Linux and macOS) ends a CLI `wait` / `last` before any frame, with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside the dispatch span and no `channel-request` line, and MCP's returns `instance-unreachable` with `detail` `server-verify-failed` and no line (on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out)
@carried n=7 label=obs-plan/span-trace-coverage fact=fct-hluu43pb
why: wait / last exit 21 must carry server-verify-failed or socket-dir-refused with during connect on Linux and macOS, the same closed words as on Windows.
quote: `detail:"server-verify-failed"` with `during:"connect"` when the serving process is of another euid or not the recorded wrapper, or the check cannot be made (Linux, Windows and macOS, on macOS an euid mismatch alone), or `detail:"socket-dir-refused"` with `during:"connect"` when the recorded endpoint's socket directory is refused.
@carried n=8 label=obs-plan/span-trace-coverage fact=fct-u5crvqku
why: The dialog hook and answer on Linux and macOS must fail open with no frame on a refused directory, euid or server check, macOS comparing no peer pid.
quote: A refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made opens neither: the dialog hook emits no decision (fail open), and `answer` sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=9 label=obs-plan/span-trace-coverage fact=fct-ffumo3bh
why: The dialog hook's failed server check must log the same fail-open pair with detail server-verify-failed on every OS.
quote: A failed server check is one of those fail-open paths before `dialog_id`: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised`.
@carried n=10 label=obs-plan/span-trace-coverage fact=fct-a7vqqg3h
why: pause / release on Linux and macOS must run the socket-directory, euid and server checks before any frame, macOS comparing no peer pid.
quote: `pause.client` / `release.client` run the socket-directory check, the connect, the euid decision and then the server check before `channel.request(pause|release)`; a refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request` and sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=11 label=obs-plan/span-trace-coverage fact=fct-wovktszp
why: pause / release exit 21 must carry server-verify-failed with during connect on every OS, the same closed words the one schema re-asserts.
quote: a server-verification refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client's `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN;
@carried n=12 label=obs-plan/span-trace-coverage fact=fct-gqw4imf3
why: The statusline-shell pair is a behaviour of Unix alone, so a test reading it must carry that reason where it stands, and a Windows run must write no such pair rather than an OS-only word.
quote: `process-start` / `process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}`, only when the snapshot records a `statusline_command` and a shell is built (never on Windows, where `shell_argv` is `None`);
@carried n=13 label=obs-plan/span-trace-coverage fact=fct-6xa7lf3f
why: The openpty child spawn on Linux and macOS must strip CLAUDE names by the Unix rule and log them on process-start claude-child as names only.
quote: The strip removes every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except the persistent set (Windows `HKCU`/`HKLM` `Environment` value names; Unix `config.json` `claude_env_keep`), and always removes the 11 names of the identity floor `IDENTITY_FLOOR`, measured on the Windows host, whatever the persistent set says
@carried n=13 run=2 label=obs-plan fact=fct-xnfonqro
quote: `env_stripped_count`, `env_stripped_known` and `env_kept` on `process-start{subject:"claude-child"}`; names only, never values, and no env map is passed to a span or event.
@carried n=14 label=obs-plan/span-trace-coverage fact=fct-wev2ceaa
why: The snapshot-replay recovery line must keep the same detail words and fields on every OS, as the task re-asserts.
quote: `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v", file:"snapshot.json", v_seen}` (D-03).
@carried n=14 run=2 label=obs-plan fact=fct-iyk74prg
quote: `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one line per call
@carried n=15 label=obs-plan/span-trace-coverage fact=fct-wgyuisdz
why: The torn-line heal line must keep the same detail word and fields on every OS, as the task re-asserts.
quote: `state-recovered{detail:"torn-line-healed", file:"events.ndjson", offset}` (D-03), at WARN with no `corr`, written by the process whose append healed.
@carried n=15 run=2 label=obs-plan fact=fct-ghtvc6rx
quote: the appender that heals writes one `state-recovered{detail:"torn-line-healed"}` line per heal; the reader neither heals nor logs.
@carried n=16 label=obs-plan/span-trace-coverage fact=fct-6z2owz4q
why: A ledger the Unix strict-modes check refuses must log the same parse-rejected word strict-modes-failed as on Windows and count as unverified.
quote: An unreadable or unparsable ledger, or one the strict-modes check refuses (`run`'s version gate reads it through `viola_state::stamps::read_stamps_strict`), gives `parse-rejected{parser:"ledger-stamps", detail:"unreadable"|"malformed"|"strict-modes-failed", count}` in `run-<name>.ndjson` and is treated as unverified (`cli_verified:false`).
@carried n=16 run=2 label=obs-plan fact=fct-g4daqt4h
quote: a missing stamp, or an unreadable, malformed or strict-modes-refused ledger, counts as unverified (`cli_verified:false`); the path and stamp contents never appear.
@carried n=17 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
why: In the Unix suite verify must run only against the fake agent, with its PTY runs on the openpty seam and its relied row count read per OS.
quote: In CI, `viola verify` runs only against the fake agent — its print mode plus the five interactive PTY runs (Run E, the resume run, included) over `--screens --turn-stop --trusted-root <workspace root> --dialogs --framing` — at the recorded version 2.1.287:
@carried n=17 run=2 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
quote: a row is relied unless it is one of the three statusline rows where `statusline::WRAPPED_HERE` is false; 24 on Unix, 21 on Windows
@carried n=17 run=3 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
quote: A `verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) starts no further spawn, kills and reaps the child it is running
@carried n=17 run=4 label=obs-plan fact=fct-bharnesn
quote: in CI `viola verify` runs only against the fake agent
@binding n=1
**span-trace-coverage run-start cleanup ↔ log-coverage PTY seam wrapper**: the openpty wait and kill legs must give the claude-child process-exit from the handle-wait thread with exit_source handle-wait or kill-fallback (obs-plan/span-trace-coverage fct-mw7zqeom), the same boundary event the log-coverage PTY seam row requires on every OS (obs-plan fct-3epowryi).
@binding n=2
**span-trace-coverage server checks (send, wait / last, answer, pause / release) ↔ the scope's macOS carve-out and the log-coverage detail catalog**: on Linux and macOS every frame verb and the dialog hook must refuse before any frame with server-verify-failed or socket-dir-refused, during connect, macOS comparing euid alone and no test reading it as a comparison (obs-plan/span-trace-coverage fct-lohdixpq), (obs-plan/span-trace-coverage fct-l4ierws2), (obs-plan/span-trace-coverage fct-u5crvqku), (obs-plan/span-trace-coverage fct-a7vqqg3h); MCP's refusal stays instance-unreachable with no line (obs-plan fct-pqz5mgd5).
@binding n=3
**span-trace-coverage closed words ↔ ci-integration G4 and pii default-deny**: the state-recovered, parse-rejected and process-exit detail words the Unix suite writes must pass G4 against both schemas on ubuntu-latest and macos-latest as on windows-2025 (obs-plan/span-trace-coverage fct-wev2ceaa), (obs-plan/span-trace-coverage fct-6z2owz4q), (obs-plan fct-htkftgtl), (obs-plan fct-llmksgvm).
@binding n=4
**span-trace-coverage E2 strip ↔ pii never-log floor**: the Unix child spawn's strip must log CLAUDE names only, never values, under the same never-log floor as Windows (obs-plan/span-trace-coverage fct-6xa7lf3f), (obs-plan fct-xvmeqbmc).
@binding n=5
**span-trace-coverage statusline shell-out ↔ one diagnostics schema**: the statusline-shell pair exists only where a shell is built, so a Windows run writes no pair and no OS-only word, and a test reading it names that reason (obs-plan/span-trace-coverage fct-gqw4imf3), (obs-plan fct-dao4nifd).
@binding n=6
**span-trace-coverage verify ↔ the fake-agent boundary and slo budgets**: verify on Unix runs only against the fake agent, and its home-level lines fall under G2, G4 and the secret scan like any other (obs-plan/span-trace-coverage fct-f3gi2vhc), (obs-plan fct-bharnesn), (obs-plan fct-zze2ezbr).
