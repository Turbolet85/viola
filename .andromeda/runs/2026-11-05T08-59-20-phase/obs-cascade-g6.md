@carried n=1 label=obs-plan/metric-coverage fact=fct-dhdxvyfp
why: The perf job keeps only its Linux leg, which must still derive hook latency from the hyperfine max that gate --require perf reads.
quote: | Hook latency per `hook_event` (Scenario 4/6; perf trigger) | `viola.hook.duration_ms` ← `hook-decision.duration_ms` | histogram | ms | raw samples; gate = hyperfine `max` |
@carried n=2 label=obs-plan/metric-coverage fact=fct-zkvtfud4
why: Removing viola-pty's Windows TerminateProcess fallback must leave the process-exit fields the child-exit counter reads unchanged on Linux, exit_source's handle-wait and kill-fallback included.
quote: | Child / probe exits | `viola.child.exit` ← `process-exit{subject, child_exit_status, exit_source}` | counter | 1 | n/a |
@carried n=2 run=2 label=obs-plan/log-coverage fact=fct-umidjbc4
quote: | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
@carried n=2 run=3 label=obs-plan fact=fct-3epowryi
quote: **PTY seam** (viola-pty): spawn gives `process-start{subject:"claude-child"}`. Exit comes from the handle-wait thread; kill-fallback use is logged as `exit_source:"kill-fallback"`.
@carried n=3 label=obs-plan/metric-coverage fact=fct-wl54ld47
why: Every gate green on Linux requires the Linux test and perf jobs left after the CI removal to keep counting zero panic lines through G2.
quote: | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |
@carried n=4 label=obs-plan/metric-coverage fact=fct-l3f7hrdu
why: The macOS carve-out leaves with the macOS arm, so this row's OS parenthetical names removed code and is an expected amendment the plan names, while the Linux server-verify-failed and socket-dir-refused lines it counts stay as written.
quote: The CLI's server-verification refusal (Linux, Windows and macOS, where it means an euid mismatch alone) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, and its socket-directory refusal one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN there, each counted by the Counter row's `detail` grouping
@carried n=5 label=obs-plan/log-coverage fact=fct-qzbqiq2k
why: The exit-21 server-verify-failed line must stay unchanged on Linux, while its Windows and macOS wording names code the chunk removes and is P3's to list as an expected amendment.
quote: `server-verify-failed` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) on a server of another euid (Unix), on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`
@carried n=6 label=obs-plan/log-coverage fact=fct-qiql34nd
why: The hook keeps its contract and its fail-open server-verify-failed line on Linux, while the macOS carve-out wording names code the chunk removes.
quote: `server-verify-failed` is written by `viola hook` on a server of another euid (Unix), on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone): `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`
@carried n=7 label=obs-plan/metric-coverage fact=fct-tinnk5cr
why: The strict-modes check loses its Windows arm, and on Linux a wait it refuses must still write no line, as the scope keeps the strict-modes-failed details unchanged.
quote: A CLI `wait` refused by the instance strict-modes check (`client::refuse_untrusted`: exit 21, the instance pair or the `instance-unreachable` document) writes no line at all, neither a `process-exit` nor a `channel-request`
@carried n=7 run=2 label=obs-plan/log-coverage fact=fct-mivk5eld
quote: The other refused strict-modes checks write no line either — no `process-start`, no `process-exit`, no `strict-modes-failed`: `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted` (exit 21, the instance pair, or under `--json` the `instance-unreachable` document)
@carried n=7 run=3 label=obs-plan/log-coverage fact=fct-mivk5eld
quote: `socket-dir-refused` is a closed `detail` code on `process-exit` and on `hook-decision`: the socket directory of a start, or a recorded endpoint's directory, is refused as a symlink, as no directory, for another owner or for a foreign mode bit.
@carried n=8 label=obs-plan/log-coverage fact=fct-yb5jpaot
why: The pty_backend ConPTY values, endpoint_kind's named-pipe reading and the sideload_fallback field name the ConPTY sideload code the chunk removes, so P3 lists each and P4 decides it against the masters' change, while openpty and every Linux field stay.
quote: `os`, `pid`, `child_pid`, `endpoint_kind`, `pty_backend` (`conpty-sideload|conpty|openpty`), `cli_version`, `cli_verified`, `env_stripped_count`, `env_stripped_known`, `env_kept` (claude-child: the kept `CLAUDE*` names, comma-joined, names only), `sideload_fallback` (claude-child, Windows, present only on a degrade: `hash-mismatch|unreadable|load-failed|not-built`, or `strict-modes-failed`, the last when the write-rule check `pin_companions` makes before its held open refuses `bin/<key>/conpty/` or a companion in it, the child then on the inbox ConPTY with its exit unchanged and nothing written; D-36) |
@carried n=8 run=2 label=obs-plan fact=fct-wk2jyutc
quote: This section only **adds** fields and the D-01, D-03, D-05 and D-06 enum values; it renames and removes nothing.
@carried n=9 label=obs-plan/log-coverage fact=fct-dsupowm6
why: The closed 20-value process-exit detail enum must keep every Linux word, while the Windows Ctrl-C / Ctrl-Break wording and the .cmd / .bat refusal's batch-script-child are Windows text P3 lists and P4 decides.
quote: And `interrupted`: a `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked ends with one `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` in `cli-<name>.ndjson` under a `VIOLA_NAME` (no file with no instance, D-06), after every spawn pair it opened is closed (chunk 2026-10-13-interrupted-verify-cleanup). `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 20 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening).
@carried n=9 run=2 label=obs-plan/log-coverage fact=fct-dsupowm6
quote: `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, `socket-dir-refused` (the socket directory of `viola run` or `viola revive`'s start refused or not creatable, before any snapshot, child or bind)
@carried n=9 run=3 label=obs-plan fact=fct-26ufyipy
quote: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`)
@binding n=1
**metric-coverage ↔ log-coverage**: the `viola.child.exit` counter reads `process-exit`'s `exit_source`, which the PTY seam writes from the handle-wait thread or as `kill-fallback`; removing viola-pty's Windows TerminateProcess fallback must leave both values and the handle-wait origin as they are on Linux (obs-plan/metric-coverage fct-zkvtfud4) (obs-plan/log-coverage fct-umidjbc4) (obs-plan fct-3epowryi).
@binding n=2
**metric-coverage ↔ log-coverage**: the Counter row's `detail` grouping and `viola.wait.outcome` count the exit-21 `server-verify-failed` and `socket-dir-refused` lines the CLI verbs write and the hook's fail-open line; the «Linux, Windows and macOS, where it means an euid mismatch alone» wording in all three names the macOS carve-out this chunk removes, so the plan names it as one expected obs-plan amendment of the wrap, while the Linux lines, levels and files stay unchanged (obs-plan/metric-coverage fct-l3f7hrdu) (obs-plan/log-coverage fct-qzbqiq2k) (obs-plan/log-coverage fct-qiql34nd).
@binding n=3
**metric-coverage ↔ log-coverage**: a wait or frame verb refused by the strict-modes check writes no line, so no metric counts it, and a refused socket directory writes its own `socket-dir-refused` line; dropping the Windows arm of these checks and the `cfg(unix)` gate left without a sibling must keep both the absence and the line (obs-plan/metric-coverage fct-tinnk5cr) (obs-plan/log-coverage fct-mivk5eld).
@binding n=4
**log-coverage ↔ the body and schemas/diag-line.v1.json**: the schema closes `process-start`'s `pty_backend` and `sideload_fallback` and the 20-value `process-exit` `detail` enum, and §6 only adds and removes nothing; any ConPTY or Windows word the chunk drops from code or schema must be matched by the wrap's obs-plan amendment that the plan names as expected, and every Linux value stays (obs-plan/log-coverage fct-yb5jpaot) (obs-plan/log-coverage fct-dsupowm6) (obs-plan fct-wk2jyutc).
@binding n=5
**metric-coverage ↔ the body's CI and SLO gates**: the hook-latency histogram is gated on hyperfine `max` and the panic counter must be 0; with the Windows and macOS jobs removed, both are measured on the Linux `test` and `perf` jobs alone, which keep `gate --require perf` and G2 (obs-plan/metric-coverage fct-dhdxvyfp) (obs-plan/metric-coverage fct-wl54ld47) (obs-plan fct-733e2isu) (obs-plan fct-wgmneptw).
