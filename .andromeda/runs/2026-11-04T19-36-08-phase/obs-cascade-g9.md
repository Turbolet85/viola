@carried n=1 label=obs-plan/log-coverage fact=fct-yb5jpaot
why: The Unix wrapper's child spawn must log process-start with subject claude-child and pty_backend openpty, and a Windows-only field such as sideload_fallback must read as key absence on Unix, never an OS-only word.
quote: `process-start` | `subject` (`self|claude-child|version-probe|verify-probe|verify-pty-probe|agents-probe|statusline-shell`), `service_name`, `version`, `os`, `pid`, `child_pid`, `endpoint_kind`, `pty_backend` (`conpty-sideload|conpty|openpty`), `cli_version`, `cli_verified`, `env_stripped_count`, `env_stripped_known`, `env_kept` (claude-child: the kept `CLAUDE*` names, comma-joined, names only), `sideload_fallback` (claude-child, Windows, present only on a degrade
@carried n=1 run=2 label=obs-plan fact=fct-qaeihrqm
quote: `corr`: per the tests table; null is represented by key absence (Section 3, Null encoding)
@carried n=2 label=obs-plan/log-coverage fact=fct-umidjbc4
why: The openpty seam's child exit must log process-exit with exit_source handle-wait or kill-fallback on Unix exactly as the ConPTY seam does, never from reader EOF.
quote: | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
@carried n=2 run=2 label=obs-plan fact=fct-3epowryi
quote: **PTY seam** (viola-pty): spawn gives `process-start{subject:"claude-child"}`. Exit comes from the handle-wait thread; kill-fallback use is logged as `exit_source:"kill-fallback"`. Paste gives `text_bytes` only.
@carried n=3 label=obs-plan/log-coverage fact=fct-tnco7mng
why: channel-request over the Unix socket endpoint must carry the same fields, conn on both sides, as over the named pipe.
quote: | `channel-request` | `method`, `conn` / `srv_conn`, `from`, `from_trust`, `sender`, `v`, `after`, `timeout_ms` |
@carried n=3 run=2 label=obs-plan fact=fct-qmzbmwv2
quote: On Unix a peer the wrapper's `accept_loop` refuses by euid is dropped on the accept thread, before its worker thread and before any frame, and writes no line by rule (no frame to key a line on, `channel-response`'s `detail` closed); the client reads end-of-file.
@carried n=4 label=obs-plan/log-coverage fact=fct-ikvvw35l
why: channel-response's closed method, result_class and detail words must read identical on every OS, G4 reading them green on both Unix runners.
quote: | `channel-response` | `method` (only a name on the channel's closed method list, `link` and `unlink` among them; any other name logs no `method` key), `conn` / `srv_conn`, `result_class` (`ok|refusal|error`), `refusal`, `detail` (`unsupported-version` / `invalid-params` / `release-from-driver`, closed by `schemas/diag-line.v1.json`: the three `-32602` faults, on the server line and the client line alike, and absent on every other `channel-response` line)
@carried n=5 label=obs-plan/log-coverage fact=fct-hb5clvjs
why: Paste over openpty must log text_bytes only, and its CR LF normalisation must give the same count on Unix as on Windows.
quote: `text_bytes` (the typed text's length: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters)
@carried n=6 label=obs-plan/log-coverage fact=fct-qb47tiuw
why: The dialog paths now run in the Unix fake-agent suite, and their lines must keep these fields and closed words under G4 on both Unix OSes.
quote: | `dialog-raised` / `dialog-answered` | `dialog_kind`, `hook_event`, `from`, `from_trust`, `deadline_hit`, `duration_ms` |
@carried n=7 label=obs-plan/log-coverage fact=fct-vrjzzaok
why: The hook contract is re-read under the Unix suite, and its lines must carry the same closed hook_event words on every OS.
quote: | `hook-invoked` / `hook-decision` | `hook_event` (`session-start|user-prompt-submit|pre-tool-use|permission-request|stop|session-end|notification|post-tool-use|post-tool-use-failure|statusline`), `stdin_bytes`, `invoked_at`, `decision_emitted`, `deadline_hit`, `budget_written`, `detail`, `duration_ms` |
@carried n=8 label=obs-plan/log-coverage fact=fct-hq2iiwoa
why: The panic record re-asserted on every OS must carry panic_location in the same workspace-relative form on Unix and Windows, its payload and backtrace in the detail file only.
quote: | `panic` | `panic_location` (workspace-relative, or `<crate>-<ver>/src/...` for dependencies), `thread`. Payload and backtrace go to `detail-<process>.ndjson` only |
@carried n=8 run=2 label=obs-plan fact=fct-boem2fct
quote: | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
@carried n=9 label=obs-plan/log-coverage fact=fct-izpoipm6
why: release-from-driver is a closed event row G4 must read green on ubuntu-latest and macos-latest with the same fields as on Windows.
quote: | `release-from-driver` (D-01) | `conn`, `from` (only when a valid `ViolaName`), `from_trust` (`self-reported`) |
@carried n=10 label=obs-plan/log-coverage fact=fct-rn7dqtoc
why: state-recovered's snapshot-replayed and torn-line-healed lines must keep the same detail words on every OS, with file a basename and scanned a last component, never an OS path.
quote: `detail` (`torn-line-healed|snapshot-replayed|snapshot-unsupported-v`), `file` (basename only: `events.ndjson` on `torn-line-healed`, `snapshot.json` on the two snapshot details), `offset` (`torn-line-healed` only: the log's length before the healing append), `v_seen` (`snapshot-unsupported-v` only), `scanned` (the two snapshot details only, written by `read_snapshot_or_replay`: the instance dir's own last component when it parses as a `ViolaName`, schema pattern `^[a-z][a-z0-9-]{0,31}$`, absent otherwise — never a path or a literal `null`
@carried n=11 label=obs-plan/log-coverage fact=fct-yp3xr6e4
why: parse-rejected's closed parser list must hold on every OS, state-entry and ledger-stamps among the parsers the Unix suite reaches.
quote: | `parse-rejected` (D-05) | `parser` (`claude-agents-json|vt100-feed|config-json|ledger-stamps|hook-stdin|channel-frame|mcp-stdin`, `prompt-submitted|paste-text|state-entry`), `detail`, `count` |
@carried n=12 label=obs-plan/log-coverage fact=fct-dsupowm6
why: run's and revive's Unix start refusals must log socket-dir-refused on exit 1 before any snapshot, child or bind, inside the closed 20-value process-exit detail enum the chunk re-asserts.
quote: `internal-error`, `socket-dir-refused` (the socket directory of `viola run` or `viola revive`'s start refused or not creatable, before any snapshot, child or bind)
@carried n=12 run=2 label=obs-plan/log-coverage fact=fct-dsupowm6
quote: a `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked ends with one `process-exit{subject:"self", exit_code:1, detail:"interrupted"}`
@carried n=12 run=3 label=obs-plan/log-coverage fact=fct-dsupowm6
quote: `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 20 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening).
@carried n=12 run=4 label=obs-plan fact=fct-26ufyipy
quote: a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`), a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines
@carried n=13 label=obs-plan/log-coverage fact=fct-qzbqiq2k
why: The Unix suite must read server-verify-failed on exit 21 as macOS's euid mismatch alone, with no pid, start time, path or endpoint in the line.
quote: `server-verify-failed` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) on a server of another euid (Unix), on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson` (`link` / `unlink`: the driven instance's), with no pid, start time, path or endpoint.
@carried n=13 run=2 label=obs-plan fact=fct-ufkep7p5
quote: | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
@carried n=14 label=obs-plan/log-coverage fact=fct-ylj2z2t5
why: wrapper-fault is a member of the closed process-exit detail enum that must read identical on every OS.
quote: `process-exit` exit 20: `wrapper-fault`.
@carried n=15 label=obs-plan/log-coverage fact=fct-qiql34nd
why: Under the Unix suite's hook contract, the fail-open paths must write these closed codes only, server-verify-failed and socket-dir-refused among them on Unix.
quote: `hook-decision` fail-open codes (security Vector 4): `oversize-stdin`, `malformed-json`, `channel-unreachable`, `server-verify-failed`, `socket-dir-refused`, `strict-modes-failed`, `unverified-cli`, `deadline`.
@carried n=15 run=2 label=obs-plan/log-coverage fact=fct-qiql34nd
quote: `server-verify-failed` is written by `viola hook` on a server of another euid (Unix), on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone): `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`
@carried n=16 label=obs-plan/log-coverage fact=fct-vehppv2d
why: parse-rejected's detail must stay inside the closed list diag-line.v1.json constrains on every OS, symlink-ignored and strict-modes-failed among the words Unix reaches.
quote: `symlink-ignored` (`state-entry`), `strict-modes-failed` (`ledger-stamps`: `run`'s version gate refused the stamps file on the strict-modes check, and the read counts as unverified). `diag-line.v1.json` constrains `parse-rejected.detail` to this list (D-31).
@carried n=16 run=2 label=obs-plan/log-coverage fact=fct-iy4rx7dp
quote: `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written at `WARN` into `mcp.ndjson` by `viola_mcp::serve` through `obs_event!`, once per session, after the session ends, only when the stdin reader cut a line past `MAX_FRAME`; it holds no byte of the cut line.
@carried n=17 label=obs-plan/log-coverage fact=fct-mivk5eld
why: A refused Unix socket directory must log under its own closed word socket-dir-refused, one line per arm or none for mcp, never as strict-modes-failed and never with a path.
quote: `socket-dir-refused` is a closed `detail` code on `process-exit` and on `hook-decision`: the socket directory of a start, or a recorded endpoint's directory, is refused as a symlink, as no directory, for another owner or for a foreign mode bit. The check takes the read verdict of the strict-modes checks above yet writes lines under its own word: `viola run` and `viola revive`'s start one `process-exit{subject:"self", exit_code:1}`, the eight CLI frame verbs one `process-exit{exit_code:21, during:"connect"}` at WARN with no `channel-request`, `viola hook` one `hook-decision` at WARN; only `viola mcp`'s `frame` writes no line, `isError` its record.
@carried n=17 run=2 label=obs-plan/log-coverage fact=fct-mivk5eld
quote: The `viola-state` call site that skips a symlinked entry therefore emits `parse-rejected{parser:"state-entry", detail:"symlink-ignored", count}` once per scan, at `warn`, into the scanning process's role file. The entry name and the link target are never logged.
@carried n=18 label=obs-plan/log-coverage fact=fct-lhnmh6dy
why: The closed subject enum must hold on every OS, the claude-child over openpty and verify's PTY probes on Unix writing the same words and process-exit fields as on Windows.
quote: `schemas/diag-line.v1.json` closes `subject` as `$defs.subject.enum` = `self|claude-child|version-probe|verify-probe|verify-pty-probe|agents-probe|statusline-shell`; each of the eight `verify` spawn pairs' `process-exit` carries `child_exit_status` and `duration_ms` (D-35)
@carried n=18 run=2 label=obs-plan fact=fct-7eai74no
quote: **Child / shell spawns:** `process-start` / `process-exit` with a closed `subject`; a `claude agents --json` parse failure gives one `parse-rejected{parser:"claude-agents-json"}` and the value `"unknown"`.
@binding n=1
**log-coverage field and detail catalog ↔ ci-integration G4**: every carried row's fields and closed words must conform to `schemas/diag-line.v1.json` on `ubuntu-latest` and `macos-latest` as on `windows-2025`, G4 its own `if: always()` step reading each G2-scope line (obs-plan/log-coverage fct-dsupowm6) (obs-plan fct-htkftgtl).
@binding n=2
**log-coverage process-start / process-exit PTY seam ↔ scope-entities viola-pty**: the openpty backend must log spawn and exit at the seam only, exit from the handle-wait thread, with the byte stream never logged (obs-plan fct-3epowryi) (obs-plan fct-m24p7svx).
@binding n=3
**log-coverage server-verify-failed ↔ span-trace-coverage wait / last scenario**: on macOS the exit-21 `server-verify-failed` line must mean an euid mismatch alone, and MCP's `instance-unreachable` writes no line (obs-plan/log-coverage fct-qzbqiq2k) (obs-plan fct-q67chfay).
@binding n=4
**log-coverage panic row ↔ slo-invariants zero unlogged panics**: each OS's panic must give exactly one `panic` line in its home-level role file, counted by G2, payload in the detail file only (obs-plan/log-coverage fct-hq2iiwoa) (obs-plan fct-3qkbv2ca).
@binding n=5
**log-coverage catalog ↔ pii-scrubbing default-deny**: a field one OS cannot fill must be absent rather than an OS-only key or word, since a field outside the catalog fails G4 under `unevaluatedProperties: false` (obs-plan/log-coverage fct-yb5jpaot) (obs-plan fct-llmksgvm).
@binding n=6
**log-coverage state-recovered ↔ span-trace-coverage E5**: `snapshot-replayed` must be written by `read_snapshot_or_replay` with the same detail words on every OS (obs-plan/log-coverage fct-rn7dqtoc) (obs-plan fct-iyk74prg).
