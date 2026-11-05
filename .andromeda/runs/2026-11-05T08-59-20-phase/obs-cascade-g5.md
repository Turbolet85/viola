@carried n=1 label=obs-plan/span-trace-coverage fact=fct-e6vtcesf
why: The start's must-trace chain must lose the Windows-only `run.conpty_sideload` step with its code and keep every Linux span in its order, and the Unix-only settings override write loses its gate once its only sibling is gone.
quote: `run.collision_check` › `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate` (spawns the `version-probe`) › `channel.bind`
@carried n=1 run=2 label=obs-plan/span-trace-coverage fact=fct-e6vtcesf
quote: the settings override write (`instances/<name>/settings.json` through `replace_private`, Unix only), which sits between the first snapshot and `pty.spawn`
@carried n=1 run=3 label=obs-plan/span-trace-coverage fact=fct-e6vtcesf
quote: a refusal is one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, before any snapshot, child or `Server::bind`, on `viola revive`'s start too
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-smfiay44
why: The sideload span's attributes and the `named-pipe`, `conpty-sideload` and `conpty` words and the `.cmd`/`.bat` refusal name code the chunk removes, so P3 must list each and P4 decide it against the masters' change, with `unix-socket` and `openpty` kept on Linux.
quote: `run.conpty_sideload` (Windows): `outcome` (`loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`; `strict-modes-failed` when the owner/DACL check refuses the `conpty/` dir or a companion, `PinError::StrictModes`, nothing written), `search_restricted` (bool)
@carried n=2 run=2 label=obs-plan/span-trace-coverage fact=fct-smfiay44
quote: `channel.bind`: `endpoint_kind` (`named-pipe|unix-socket`);
@carried n=2 run=3 label=obs-plan/span-trace-coverage fact=fct-smfiay44
quote: `pty.spawn`: `pty_backend` (`conpty-sideload|conpty|openpty`, from `viola_pty::pty_backend()`; `conpty-sideload` after a successful pre-load), `env_stripped_count`.
@carried n=2 run=4 label=obs-plan/span-trace-coverage fact=fct-smfiay44
quote: the `.cmd`/`.bat` refusal (`batch-script-child`) is decided earlier, by viola-side program resolution (`resolve_program`, after obs init and before the strip plan and `pty.spawn`)
@carried n=2 run=5 label=obs-plan fact=fct-itztmsse
quote: never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-4g2at3oh
why: The `sideload_fallback` field on `process-start` and its closed property in `schemas/diag-line.v1.json` name code the chunk removes, so P3 must list them for P4's decision while the Linux exit-1 `detail` set and every Linux schema value stay.
quote: plus, on Windows when the `run.conpty_sideload` outcome is not `loaded`, `sideload_fallback` (`hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`, D-36).
@carried n=3 run=2 label=obs-plan/span-trace-coverage fact=fct-4g2at3oh
quote: In `schemas/diag-line.v1.json`, `pty_backend` is `{ "type": "string" }`, while `sideload_fallback` is closed in the `process-start` properties to `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built` (`diag-detail.v1.json` carries neither).
@carried n=3 run=3 label=obs-plan/span-trace-coverage fact=fct-4g2at3oh
quote: with `detail` ∈ `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|socket-dir-refused`
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-mw7zqeom
why: Once the Windows TerminateProcess fallback leaves viola-pty, the Linux child exit must still come from the handle-wait thread with `exit_source` `handle-wait` or `kill-fallback`, never on reader EOF.
quote: The handle-wait thread emits `process-exit{subject:"claude-child", child_exit_status, exit_source:"handle-wait"|"kill-fallback"}`, never on reader EOF (ConPTY chaos class).
@carried n=4 run=2 label=obs-plan fact=fct-itztmsse
quote: the child's `process-exit` comes from the handle-wait thread, never on reader EOF.
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-lohdixpq
why: The macOS carve-out leaves with the macOS arm, while on Linux the send path's peer-pid and euid comparison, `server-verify-failed`, exit 21 and its `during:"connect"` must stay as they are.
quote: (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=5 run=2 label=obs-plan/span-trace-coverage fact=fct-gghzntp4
quote: client-side server-verification refusal, made after the client's own refusals and before any frame (Linux, Windows and macOS, on macOS an euid mismatch alone): no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, and MCP `send` writes no line;
@carried n=5 run=3 label=obs-plan/span-trace-coverage fact=fct-u2zbdxyd
quote: `detail:"server-verify-failed"` with `during:"connect"` (the serving process is of another euid or is not the recorded wrapper, or the check cannot be made; no frame sent; Linux, Windows and macOS, on macOS an euid mismatch alone)
@carried n=5 run=4 label=obs-plan/span-trace-coverage fact=fct-u2zbdxyd
quote: `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`)
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-l4ierws2
why: The task names the wait / last server-verification path: the macOS carve-out leaves, and on Linux the comparison, the exit-21 `server-verify-failed` line and `wait.block`'s close on a refusal must stay unchanged.
quote: a refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request` (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=6 run=2 label=obs-plan/span-trace-coverage fact=fct-hluu43pb
quote: `detail:"server-verify-failed"` with `during:"connect"` when the serving process is of another euid or not the recorded wrapper, or the check cannot be made (Linux, Windows and macOS, on macOS an euid mismatch alone)
@carried n=6 run=3 label=obs-plan/span-trace-coverage fact=fct-3omdhkir
quote: `wait.block` closes on the response, on the connection error, or on a socket-directory or server-verification refusal made before any frame (no `channel.request` opened; the client's `process-exit` its outcome line).
@carried n=6 run=4 label=obs-plan fact=fct-q67chfay
quote: a serving process that is not the recorded wrapper (Linux and Windows) or runs under another euid (Linux and macOS) ends a CLI `wait` / `last` before any frame, with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside the dispatch span and no `channel-request` line, and MCP's returns `instance-unreachable` with `detail` `server-verify-failed` and no line (on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out)
@carried n=7 label=obs-plan/span-trace-coverage fact=fct-u5crvqku
why: The dialog hook's and `answer`'s server checks lose the macOS carve-out with the macOS arm, while on Linux a failed check must still fail open with its `hook-decision` line, keeping the hook's exit-0 contract.
quote: A refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made opens neither: the dialog hook emits no decision (fail open), and `answer` sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=7 run=2 label=obs-plan/span-trace-coverage fact=fct-ffumo3bh
quote: A failed server check is one of those fail-open paths before `dialog_id`: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised`.
@carried n=8 label=obs-plan/span-trace-coverage fact=fct-a7vqqg3h
why: The pause / release server check loses the macOS carve-out with the macOS arm, while its Linux refusal must still send no frame and end in the exit-21 `server-verify-failed` line.
quote: a refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made, opens no `channel.request` and sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
@carried n=8 run=2 label=obs-plan/span-trace-coverage fact=fct-wovktszp
quote: a server-verification refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client's `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN;
@carried n=9 label=obs-plan/span-trace-coverage fact=fct-6xa7lf3f
why: The R8 strip's Windows registry and case-insensitive arms leave with the `cfg(windows)` code, while the Unix exact-match strip, its `claude_env_keep` set, the identity floor and the names-only E2 fields on `process-start` must stay.
quote: The strip removes every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except the persistent set (Windows `HKCU`/`HKLM` `Environment` value names; Unix `config.json` `claude_env_keep`), and always removes the 11 names of the identity floor `IDENTITY_FLOOR`, measured on the Windows host
@carried n=9 run=2 label=obs-plan fact=fct-xnfonqro
quote: `env_stripped_count`, `env_stripped_known` and `env_kept` on `process-start{subject:"claude-child"}`; names only, never values, and no env map is passed to a span or event.
@carried n=10 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
why: Verify's relied-row count, its stop signals and its Windows-close cleanup name per-OS arms the chunk removes or ungates, so the Linux count of 24 and the Unix stop handling must stay, and in CI `viola verify` must still run only against the fake agent.
quote: `MM` the relied count (`LedgerRow::relied()`, `ALL` filtered in its order: a row is relied unless it is one of the three statusline rows where `statusline::WRAPPED_HERE` is false; 24 on Unix, 21 on Windows, the 22nd row `resume-session`, the 23rd `agents-join` and the 24th `local-command-compact` relied on every OS)
@carried n=10 run=2 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
quote: A `verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) starts no further spawn
@carried n=10 run=3 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
quote: In CI, `viola verify` runs only against the fake agent
@carried n=10 run=4 label=obs-plan fact=fct-bharnesn
quote: in CI `viola verify` runs only against the fake agent;
@binding n=1
**span-trace-coverage ↔ log-coverage (the line schemas)**: removing the sideload span, `sideload_fallback` and the `conpty-sideload` / `conpty` backend words must be decided together with `schemas/diag-line.v1.json`'s closed `process-start` property, every Linux value kept, and G4 must still report zero conformance failures on the Linux jobs (obs-plan/span-trace-coverage fct-4g2at3oh) (obs-plan fct-2v5pxi7x).
@binding n=2
**span-trace-coverage ↔ triggers (Vector 7 and multi-platform-exporter-compat)**: the sideload's `strict-modes-failed` line and the per-OS wording of the scenarios are the wrap's expected amendments, which the plan must name, alongside the trigger that names a `conpty/` companion's strict-modes line and the trigger for the same sink on every OS (obs-plan/span-trace-coverage fct-smfiay44) (obs-plan fct-wrd7y3z2) (obs-plan fct-dao4nifd).
@binding n=3
**span-trace-coverage ↔ scope entities (viola-pty)**: the handle-wait and `kill-fallback` exit rule must hold once the windows-sys `TerminateProcess` fallback named on the viola-pty entity leaves (obs-plan/span-trace-coverage fct-mw7zqeom) (obs-plan fct-t2iihkln).
@binding n=4
**span-trace-coverage ↔ the server-verification paths of every frame verb**: the macOS carve-out must leave one rule across send, wait / last, the dialog hook with `answer`, and pause / release at once, with no env, flag or feature seam forcing a refusing peer-euid decision (obs-plan/span-trace-coverage fct-lohdixpq) (obs-plan/span-trace-coverage fct-l4ierws2) (obs-plan/span-trace-coverage fct-u5crvqku) (obs-plan/span-trace-coverage fct-a7vqqg3h).
