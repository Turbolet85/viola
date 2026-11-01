# Apply — architecture · part 3 of 4

## Verdicts
- fanout-architecture--cascade-g3.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:8#1
- fanout-architecture--cascade-g3.md 2 · applied · .andromeda/architecture-detail/occupied-resources.md:10+#1
- fanout-architecture--cascade-g3.md 4 · applied · .andromeda/architecture-detail/occupied-resources.md:14#1, .andromeda/architecture-detail/occupied-resources.md:14#2
- fanout-architecture--cascade-g3.md 5 · applied · .andromeda/architecture-detail/occupied-resources.md:30#1
- fanout-architecture--cascade-g3.md 6 · applied · .andromeda/architecture-detail/occupied-resources.md:36#1
- fanout-architecture--cascade-g3.md 7 · applied · .andromeda/architecture-detail/occupied-resources.md:42#1
- fanout-architecture--cascade-g3.md 8 · applied · .andromeda/architecture-detail/occupied-resources.md:42#2
- fanout-architecture--cascade-g3.md 9 · applied · .andromeda/architecture-detail/occupied-resources.md:122+#1
- fanout-architecture--cascade-g3.md 10 · applied · .andromeda/architecture-detail/occupied-resources.md:124+#1
- fanout-architecture--cascade-g3.md 11 · applied · .andromeda/architecture-detail/occupied-resources.md:125#1
- fanout-architecture--cascade-g3.md 12 · applied · .andromeda/architecture-detail/occupied-resources.md:155#1
- fanout-architecture--cascade-g3.md 13 · applied · .andromeda/architecture-detail/occupied-resources.md:163#1
- fanout-architecture--cascade-g3.md 14 · applied · .andromeda/architecture-detail/occupied-resources.md:175#1
- fanout-architecture--cascade-g3.md 15 · applied · .andromeda/architecture-detail/occupied-resources.md:187#1
- fanout-architecture--cascade-g3.md 16 · applied · .andromeda/architecture-detail/occupied-resources.md:189+#1
- fanout-architecture--cascade-g3.md 17 · applied · .andromeda/architecture-detail/occupied-resources.md:220+#1
- fanout-architecture--cascade-g3.md 18 · applied · .andromeda/architecture-detail/occupied-resources.md:233+#1
- fanout-architecture--cascade-g3.md 19 · applied · .andromeda/architecture-detail/cross-cutting-patterns.md:35+#1

## Edits
- line: .andromeda/architecture-detail/occupied-resources.md:8
  old: «`$TMPDIR` root. The socket is chmod 0600»
  new: «`$TMPDIR` root. `run` and `revive`'s start check that directory before any snapshot, child or `Server::bind`, creating it only when absent and checking it again, and the CLI frame verbs, `hook` and `mcp`'s frame tools check the recorded `endpoint`'s directory before they connect; a symlink, a non-directory, a foreign owner or any group or other bit is refused (`socket-dir-refused`) and left as found, the check never changing its mode. The socket is chmod 0600»
- after: .andromeda/architecture-detail/occupied-resources.md:10
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-ipc-endpoints -->»
  add: «»
  add: «- A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it and no removal is owed, a choice: a harmless file in a 0700 directory.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/occupied-resources.md:14
  old: «A mismatch or a check that cannot be made sends no frame»
  new: «Ahead of that comparison each checks the recorded `endpoint`'s directory (`viola_state::strict::check_endpoint_dir`) before it connects, a refusal reading `socket-dir-refused`, and then, on Unix, the connected peer's euid (`viola_channel::peer_admitted` over `Client::server_euid` / `TokioClient::server_euid`), a foreign euid or one it cannot read reading `server-verify-failed`. A mismatch or a check that cannot be made sends no frame»
- line: .andromeda/architecture-detail/occupied-resources.md:14
  old: «macOS reports no peer pid and stays on the recorded gap.»
  new: «macOS reports no peer pid and stays on the recorded gap, owned by no route entry: there the pid and start-time comparison is absent and `server-verify-failed` means an euid mismatch alone.»
- line: .andromeda/architecture-detail/occupied-resources.md:30
  old: «one of the ten codes, §Standard Contracts»
  new: «one of the eleven codes, §Standard Contracts»
- line: .andromeda/architecture-detail/occupied-resources.md:36
  old: «no live endpoint is recorded or the serving process is not the recorded wrapper»
  new: «no live endpoint is recorded, the recorded endpoint's directory is refused (`socket-dir-refused`), or the serving process is not the recorded wrapper or, on Unix, runs under another euid»
- line: .andromeda/architecture-detail/occupied-resources.md:42
  old: «document on stdout). `verify` exits 1 with the home-state pair»
  new: «document on stdout). They exit 21 the same way, past the instance check, on a refused socket directory (`--json`: the `instance-unreachable` document with `detail` `socket-dir-refused`; human: `send_hint` cause `socket-dir`, `hint:` last), and with `detail` `server-verify-failed` on a server of another euid. `verify` exits 1 with the home-state pair»
- line: .andromeda/architecture-detail/occupied-resources.md:42
  old: «(`revive --json` keeps its `strict-modes-failed` document).»
  new: «(`revive --json` keeps its `strict-modes-failed` document). Both also exit 1 on a refused socket directory, before any snapshot, child or `Server::bind`, a failed create reading the same code (`revive --json`: the `socket-dir-refused` document); a `Server::bind` failure is no part of that code.»
- after: .andromeda/architecture-detail/occupied-resources.md:122
  anchor: « `run` (and `viola revive`'s start arm, which logs as process `run` into `run-<name>.ndjson` with `run`'s detail sink once its strict-modes check has passed; `run`'s start and `revive`'s start arm also run the home-tree check through `checked_exe` / `refuse_tree` before `open_wrapper_log`; a refused check, on either `revive` arm or `run`'s start, opens no process log and writes no line), `hook` and the `cli` verbs `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `link`, `unlink`, `revive --list` and `plugin install` have producers today (`cli-<name>.ndjson`: `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` — `<name>` the target instance, the driven one for `link` / `unlink` — write `process-start`, the client `channel-request` / `channel-response` lines (`wait`'s carry `after` / `timeout_ms` / `outcome`) and `process-exit{subject:"self"}`, exit 21 with `detail:"instance-dead"` + `during:"connect"`, or `detail:"server-verify-failed"` + `during:"connect"` when the serving process is not the recorded wrapper (then no `channel-request` line), at WARN, with no pid, start time, path or endpoint, exit 21 with `detail:"unwrapped-name"` + `during:"connect"`, or `detail:"endpoint-lost"` + `during:"call"` for a wrapper lost with the call open, exit 20 with `detail:"wrapper-fault"`; a refused instance check of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` (`client::refuse_untrusted`) opens no role file and writes no line; `list`, only under a parsed `VIOLA_NAME` once its scan passed, one `agents-probe` `process-start` / `process-exit` pair per `claude agents --json` read and one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer (no row text, id, `cwd` or path; chunk 2026-10-16-unwrapped-sessions-on-the-board), and `viola revive`'s start arm writes its `session-live` reading's `agents-probe` pair into `run-<name>.ndjson` and, on a `session-live` refusal, one `process-exit{subject:"self", exit_code:1, detail:"session-live"}`; `viola revive <name> --list`, only under a parsed `VIOLA_NAME` and only after its strict-modes check passed, writes `process-start` and `process-exit{subject:"self"}` (`exit_code` 0 with no detail, or `exit_code` 1 `detail:"no-session"` on an empty chain) into the CALLER's `cli-<VIOLA_NAME>.ndjson`, and with no name opens none (chunk 2026-10-17-cli-machine-contract);»
  add: « a refused socket directory is none of the refused checks above: a CLI frame verb writes `process-exit{subject:"self"}`, exit 21 with `detail:"socket-dir-refused"` + `during:"connect"`, at WARN and no `channel-request` line, `server-verify-failed` also covering a server of another euid, and `run` and `revive`'s start write one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`;»
- after: .andromeda/architecture-detail/occupied-resources.md:124
  anchor: « with no name, or on a refused scan, `list` opens no `cli-*.ndjson`; `mcp.ndjson` (chunk 2026-10-17-mcp-server-for-drivers), written by `viola mcp` once its home and `diagnostics/` passed `check_path` (a refusal opens no role file and writes no line): `process-start{subject:"self"}`, the Tokio client's `channel-request` / `channel-response` lines (none for a frame tool whose instance check or server check fails, and a refused instance check writes no line at all), one client-side `send-refused` per refused `send` text, the `list` tool's `agents-probe` pair and the board's `state-recovered` / `parse-rejected` lines (the `config-json` rejections included), one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN after a session a stdin line past `MAX_FRAME` ended (no byte of the line), and no self `process-exit`: none on stdin EOF or at that cut (`command grep -n 'ObsEvent::ProcessExit' -r src crates --include=*.rs`), and none when Claude Code ends it by a signal, as measured live on claude 2.1.287 at that chunk; `hook-<name>.ndjson`: `hook-invoked`, `hook-decision`, `parse-rejected` (a failed server check writes `hook-decision{detail:"server-verify-failed"}` at WARN, on the dialog path after `hook-invoked{corr:null}` and with `corr:null, decision_emitted:false`; a refused or absent instance writes no line on any path, its check (`trusted`) coming before `read_diagnostics_level` and obs init); `hook`'s drift reports go only to the instance's `diagnostics/detail-hook.ndjson`). One line per `write`; dir 0700, files 0600 on Unix; line format owned by obs and fixed by `schemas/diag-line.v1.json`. Content-bearing detail stays in the instance detail files `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (the instance files, below).»
  add: « On a refused socket directory `hook` writes one `hook-decision{detail:"socket-dir-refused"}` at WARN, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`, its `server-verify-failed` line also covering a server of another euid; `mcp` writes no line for a frame tool whose socket directory is refused, `isError` being the record.»
- line: .andromeda/architecture-detail/occupied-resources.md:125
  old: «and `unclean-fixture` among its words.»
  new: «and `unclean-fixture` among its words. `socket-dir-refused` is a word of it and of `hook-decision`'s `detail` enum.»
- line: .andromeda/architecture-detail/occupied-resources.md:155
  old: «`viola_state::board::scan` runs `check_path` on the home and on `instances/`, each that exists»
  new: «`viola_state::board::scan` runs `check_read` on the home and `check_path` on `instances/`, on every OS, each that exists»
- line: .andromeda/architecture-detail/occupied-resources.md:163
  old: «`Refused` holds `Readable` for a read-rule refusal and `Symlink` for a symbolic link `check_diagnostics` meets, each with a fixed message.»
  new: «`Refused` holds `Readable` for a read-rule refusal, `Symlink` for a symbolic link `check_diagnostics` or `check_socket_dir` meets and `NotADirectory` for a socket directory path that is no directory, each with a fixed message.»
- line: .andromeda/architecture-detail/occupied-resources.md:175
  old: «(the home pair, or the instance pair for an instance-level refusal, and no child).»
  new: «(the home pair, or the instance pair for an instance-level refusal, and no child). The socket directory check at `client::connect`, hook `connect` and `mcp`'s `frame` is no part of this: it comes after the instance check, on the recorded `endpoint`, and its refusal writes the lines `diagnostics/`, above, states.»
- line: .andromeda/architecture-detail/occupied-resources.md:187
  old: «- `viola list`'s two home checks disagree: `board::scan`'s own `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused (`ScanRefused::Instance`) once it holds one. Their reconciliation is owed to a CARRY on the working-route entry "Unix endpoint and home hardening".»
  new: «- `viola list` holds the home to the read rule on every OS: `board::scan` reads it through `check_read` with or without an instance, so a 0750 home, or on Windows one granting read to a SID `trusted_sid` rejects, refuses the whole board (`ScanRefused::Home`).»
- after: .andromeda/architecture-detail/occupied-resources.md:189
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home, beside `board::scan`'s read rule, above.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:220
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- Test-only: harness `cleanup`'s `endpoint_gone` on Unix reads through `unconnectable` (`crates/viola-e2e/src/harness/cleanup.rs`): `UnixStream::connect` refused or `NotFound` reads gone, so `cleanup` reads `ok:true` after a force-killed wrapper, removing no leftover file; `holder_gone`, above, holds the same reading for `Wrapper::kill`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/occupied-resources.md:233
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-filesystem -->»
  add: «»
  add: «- Test-only, outside the viola home: the test-owned socket roots under `/tmp`. `tests/support/home.rs` `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on` give a test its own socket root, `SOCKET_ROOT_VAR` being `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, used by `tests/security_negatives_socket_dir.rs`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/cross-cutting-patterns.md:35
  anchor: «  - Every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.»
  add: «  - The `unable:` / `hint:` pair for a socket directory names only the parsed `ViolaName`, no path, pid or uid, is plain ASCII with `hint:` last and no SGR, its hint `send_hint`'s cause `socket-dir` in `src/human/hints.rs`.»

## Patterns
[[pattern]]
id = "arch3-01"
fixed = 'one of the ten codes'

[[pattern]]
id = "arch3-02"
fixed = 'runs `check_path` on the home and on `instances/`'

[[pattern]]
id = "arch3-03"
fixed = 'for a symbolic link `check_diagnostics` meets'

[[pattern]]
id = "arch3-04"
fixed = 'two home checks disagree'
