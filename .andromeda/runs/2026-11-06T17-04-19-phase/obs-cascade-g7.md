@carried n=1 label=obs-plan/log-coverage fact=fct-yb5jpaot
why: The chunk must re-word this row through the store's whole-fact form so that `pty_backend` names `openpty` alone, `sideload_fallback` is gone, and the row holds the header's two cells.
quote: | Event | Additive fields |
  |-------|-----------------|
  | `process-start` | `subject` (`self|claude-child|version-probe|verify-probe|verify-pty-probe|agents-probe|statusline-shell`), `service_name`, `version`, `os`, `pid`, `child_pid`, `endpoint_kind`, `pty_backend` (`conpty-sideload|conpty|openpty`), `cli_version`, `cli_verified`, `env_stripped_count`, `env_stripped_known`, `env_kept` (claude-child: the kept `CLAUDE*` names, comma-joined, names only), `sideload_fallback` (claude-child, Windows, present only on a degrade: `hash-mismatch|unreadable|load-failed|not-built`, or `strict-modes-failed`, the last when the write-rule check `pin_companions` makes before its held open refuses `bin/<key>/conpty/` or a companion in it, the child then on the inbox ConPTY with its exit unchanged and nothing written; D-36) |
@carried n=1 run=2 label=obs-plan/log-coverage fact=fct-j4k5a564
quote: A closed word of `schemas/diag-line.v1.json` that no build can emit leaves the schema with its emitter: `os` is closed at `linux`.
@carried n=1 run=3 label=obs-plan/log-coverage fact=fct-lhnmh6dy
quote: `schemas/diag-line.v1.json` closes `subject` as `$defs.subject.enum` = `self|claude-child|version-probe|verify-probe|verify-pty-probe|agents-probe|statusline-shell`
@carried n=1 run=4 label=obs-plan fact=fct-wk2jyutc
quote: This section only **adds** fields and the D-01, D-03, D-05 and D-06 enum values; it renames and removes nothing.
@carried n=2 label=obs-plan/log-coverage fact=fct-umidjbc4
why: After the `portable-pty` bump the child's `process-exit` must still carry `child_exit_status` and `exit_source` from the handle-wait thread or the kill fallback, the exit path the killed-wrapper reading (CARRY 11) rests on.
quote: | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
@carried n=2 run=2 label=obs-plan/metric-coverage fact=fct-zkvtfud4
quote: | Child / probe exits | `viola.child.exit` ← `process-exit{subject, child_exit_status, exit_source}` | counter | 1 | n/a |
@carried n=2 run=3 label=obs-plan fact=fct-3epowryi
quote: **PTY seam** (viola-pty): spawn gives `process-start{subject:"claude-child"}`. Exit comes from the handle-wait thread; kill-fallback use is logged as `exit_source:"kill-fallback"`. Paste gives `text_bytes` only.
@carried n=2 run=4 label=obs-plan fact=fct-7e4jvssp
quote: | `rmcp`, `portable_pty`, `sysinfo`, `interprocess` | `OFF` (D-11; failures are captured at the viola seam as viola events) |
@carried n=3 label=obs-plan/log-coverage fact=fct-hb5clvjs
why: Paste through the bumped PTY seam and the live sessions' sends must log the typed text's length as `text_bytes` only, never the text.
quote: `text_bytes` (the typed text's length: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters)
@carried n=4 label=obs-plan/log-coverage fact=fct-qb47tiuw
why: The live read of a dialog hold near the 3600 s ceiling must take its evidence from these dialog lines' `deadline_hit` and `duration_ms`, never from the question or answer text.
quote: | `dialog-raised` / `dialog-answered` | `dialog_kind`, `hook_event`, `from`, `from_trust`, `deadline_hit`, `duration_ms` |
@carried n=4 run=2 label=obs-plan/log-coverage fact=fct-vrjzzaok
quote: `stdin_bytes`, `invoked_at`, `decision_emitted`, `deadline_hit`, `budget_written`, `detail`, `duration_ms` |
@carried n=5 label=obs-plan/log-coverage fact=fct-mivk5eld
why: Whatever wording the founder rules for the socket-directory refusal pair, `socket-dir-refused` must stay one closed `detail` code written on exactly these lines per surface, its stderr pair and its code kept consistent across every list that names it.
quote: `socket-dir-refused` is a closed `detail` code on `process-exit` and on `hook-decision`: the socket directory of a start, or a recorded endpoint's directory, is refused as a symlink, as no directory, for another owner or for a foreign mode bit. The check takes the read verdict of the strict-modes checks above yet writes lines under its own word: `viola run` and `viola revive`'s start one `process-exit{subject:"self", exit_code:1}`, the eight CLI frame verbs one `process-exit{exit_code:21, during:"connect"}` at WARN with no `channel-request`, `viola hook` one `hook-decision` at WARN; only `viola mcp`'s `frame` writes no line, `isError` its record.
@carried n=5 run=2 label=obs-plan/log-coverage fact=fct-dsupowm6
quote: `socket-dir-refused` (the socket directory of `viola run` or `viola revive`'s start refused or not creatable, before any snapshot, child or bind)
@carried n=5 run=3 label=obs-plan/log-coverage fact=fct-dsupowm6
quote: `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-05-linux-only-code-and-ci).
@carried n=5 run=4 label=obs-plan/log-coverage fact=fct-qzbqiq2k
quote: `process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`.
@carried n=5 run=5 label=obs-plan/log-coverage fact=fct-qiql34nd
quote: `hook-decision` fail-open codes (security Vector 4): `oversize-stdin`, `malformed-json`, `channel-unreachable`, `server-verify-failed`, `socket-dir-refused`, `strict-modes-failed`, `unverified-cli`, `deadline`.
@carried n=5 run=6 label=obs-plan/metric-coverage fact=fct-l3f7hrdu
quote: its socket-directory refusal one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN there, each counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for either.
@carried n=5 run=7 label=obs-plan fact=fct-26ufyipy
quote: a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`)
@carried n=6 label=obs-plan/metric-coverage fact=fct-wl54ld47
why: The done-check must close with zero panic lines as G2 counts them, the `viola.panic` count held at 0.
quote: | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |
@carried n=6 run=2 label=obs-plan/log-coverage fact=fct-hq2iiwoa
quote: | `panic` | `panic_location` (workspace-relative, or `<crate>-<ver>/src/...` for dependencies), `thread`. Payload and backtrace go to `detail-<process>.ndjson` only |
@binding n=1
**log-fields `process-start` row ↔ log-details schema closure**: cutting `pty_backend` to `openpty` and removing `sideload_fallback` (obs-plan/log-coverage fct-yb5jpaot) must take any closed word no build emits out of `schemas/diag-line.v1.json` with its emitter, as `os` closed at `linux` did (obs-plan/log-coverage fct-j4k5a564), so that G4 conformance stays green. The closed `subject` enum on the same row stays unchanged (obs-plan/log-coverage fct-lhnmh6dy). The removal touches an obs-added field only, never a field of the tests' log format, so the section's add-only rule on that format holds (obs-plan fct-wk2jyutc).
@binding n=2
**log-fields `process-exit` `exit_source` ↔ metric-coverage `viola.child.exit` ↔ log-coverage PTY seam**: after `portable-pty` leaves `=0.8.1`, the child's exit must still come from the handle-wait thread, with `kill-fallback` logged on the fallback (obs-plan/log-coverage fct-umidjbc4, obs-plan fct-3epowryi). Then `viola.child.exit` keeps reading its three inputs (obs-plan/metric-coverage fct-zkvtfud4). The `portable_pty` target must stay `OFF`, with failures captured at the viola seam (obs-plan fct-7e4jvssp).
@binding n=3
**log-details `socket-dir-refused` ↔ metric-coverage Counter ↔ §3 harness start refusals**: a ratified or re-worded socket-directory pair (obs-plan fct-26ufyipy) must leave one closed code that is consistent across the exit-1 list (obs-plan/log-coverage fct-dsupowm6), the exit-21 list (obs-plan/log-coverage fct-qzbqiq2k), the hook fail-open codes (obs-plan/log-coverage fct-qiql34nd), the per-surface lines (obs-plan/log-coverage fct-mivk5eld) and the metric note (obs-plan/metric-coverage fct-l3f7hrdu). Any change to the code word must also move the schema's 19-value `detail` enum.
@binding n=4
**metric-coverage `viola.panic` ↔ log-fields `panic` row**: the wrap's zero-panic count (obs-plan/metric-coverage fct-wl54ld47) is G2's count over `event:"panic"` lines, and the seam exemption is matched on their `panic_location` (obs-plan/log-coverage fct-hq2iiwoa). The remnant removal must therefore leave the hook seam's exempted location intact.
@binding n=5
**log-fields dialog lines ↔ log-fields hook lines**: the live read of a hold near the 3600 s ceiling must be evidenced by `dialog-raised` / `dialog-answered` `duration_ms` and `deadline_hit` (obs-plan/log-coverage fct-qb47tiuw), together with the dialog hook's `hook-invoked` / `hook-decision` fields (obs-plan/log-coverage fct-vrjzzaok). No question, answer or tool-input text may be logged.
