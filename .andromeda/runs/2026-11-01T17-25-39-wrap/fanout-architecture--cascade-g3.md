proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: '## Occupied Resources'
    change: 'After the sentence ending «never the `/tmp` or `$TMPDIR` root.» add: «`run` and `revive`''s start check that directory before any snapshot, child or `Server::bind`, creating it only when absent and checking again, and every frame-writing client checks the recorded `endpoint`''s directory before it connects; a symlink, a non-directory, a foreign owner or any group or other bit is refused (`socket-dir-refused`) and left as found: viola never narrows it.»'
    sidecar: 'res-ipc-endpoints: the per-user socket directory is checked at the bind and at every client, refused rather than narrowed.'
    rationale: 'The report''s Symbols / APIs lands `check_socket_dir`, `check_endpoint_dir` and `socket_dir_ready`, and the check never chmods (report.md:11, report.md:14-17); the row states the directory''s mode and not that it is verified.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: '## Occupied Resources'
    change: 'Add a new row after this line: «- A force-killed wrapper''s socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it and no owner is proposed, a recorded choice of a harmless file in a 0700 directory (`inputs#I3` row 3).»'
    sidecar: 'res-ipc-endpoints: a force-killed wrapper''s leftover socket file recorded as a choice with no owner.'
    rationale: 'The P5 card''s Beyond-the-plan row 3 records the choice (report.md:51), and the harness rule removes no leftover (report.md:20, report.md:42).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:51
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 265
    section: '## Occupied Resources'
    change: 'After «in a per-user 0700 directory, never the `/tmp` or `$TMPDIR` root,» add «checked at the bind and at every client and, when loosened, refused (`socket-dir-refused`) rather than narrowed,»; the rest of the line stands.'
    sidecar: 'Body, IPC endpoints: the broad Unix line names the verified socket directory.'
    rationale: 'The broad line the IPC group details carries the same change as the group''s row (report.md:11, report.md:14-17); the report''s Expected amendments name architecture §Occupied Resources (report.md:63).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:63
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: '## Occupied Resources'
    change: 'Before «A mismatch or a check that cannot be made sends no frame» add: «Ahead of that comparison each checks the recorded `endpoint`''s directory (`viola_state::strict::check_endpoint_dir`) before it connects, a refusal reading `socket-dir-refused`, and then, on Unix, the connected peer''s euid (`viola_channel::peer_admitted` over `Client::server_euid` / `TokioClient::server_euid`), a foreign euid or one it cannot read reading `server-verify-failed`.» After «macOS reports no peer pid and stays on the recorded gap» add «, owned by no route entry; `server-verify-failed` on macOS means an euid mismatch alone».'
    sidecar: 'res-ipc-endpoints: clients check the socket directory and the peer euid before the server check; the macOS pid gap stays named with no route owner.'
    rationale: 'The three client seams take the checks in that order (report.md:15-17), `server-verify-failed` now reaches macOS as an euid mismatch alone (report.md:31), and a row stating macOS compares no peer pid stays true with no route owner (report.md:33).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 30
    section: '## Occupied Resources'
    change: 'Change «one of the ten codes, §Standard Contracts» to «one of the eleven codes, §Standard Contracts».'
    sidecar: 'res-binaries: `revive --json`''s exit-1 codes move from ten to eleven.'
    rationale: 'The report''s moved counts name this very site: `revive --json`''s exit-1 codes ten to eleven, `occupied-resources.md:30` «the ten codes» (report.md:29).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 36
    section: '## Occupied Resources'
    change: 'Change «21 when the instance check refuses, no live endpoint is recorded or the serving process is not the recorded wrapper» to «21 when the instance check refuses, no live endpoint is recorded, the recorded endpoint''s directory is refused (`socket-dir-refused`), or the serving process is not the recorded wrapper or, on Unix, runs under another euid».'
    sidecar: 'res-binaries: `link` and `unlink` exit 21 on a refused socket directory and on a server of another euid.'
    rationale: '`src/cmd/client.rs` `connect` takes both checks and keeps its 8 callers, `link` and `unlink` among them (report.md:15).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: '## Occupied Resources'
    change: 'After the sentence ending «on stdout).» add: «They exit 21 the same way, past the instance check, on a refused socket directory (`--json`: the `instance-unreachable` document with `detail` `socket-dir-refused`; human: `send_hint` cause `socket-dir`, `hint:` last), and with `detail` `server-verify-failed` on a server of another euid.»'
    sidecar: 'res-binaries: the six frame verbs exit 21 on a refused socket directory and on a server of another euid.'
    rationale: '`CallError::SocketDirRefused` reads `Unreachable::SocketDir` and a refused euid `CallError::ServerUnverified` (report.md:15); the sixth cause and the hint cause (report.md:19, report.md:28, report.md:71).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: '## Occupied Resources'
    change: 'After «(`revive --json` keeps its `strict-modes-failed` document).» add: «Both also exit 1 on a refused socket directory, before any snapshot, child or `Server::bind`, a failed create reading the same code (`revive --json`: the `socket-dir-refused` document); a `Server::bind` failure stays what it was.»'
    sidecar: 'res-binaries: `run` and `revive`''s start exit 1 on a refused socket directory.'
    rationale: '`start` calls `refused_with(launch.json, "socket-dir-refused", …)` before any snapshot, child or `Server::bind`, and `revive`''s start shares it (report.md:14); a failed create and a refused check read one code (report.md:95).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 122
    section: '## Occupied Resources'
    change: 'Add a new continuation line after this line, leaving it as it reads: « A refused socket directory is not the refused checks above: a CLI frame verb writes `process-exit{subject:"self"}`, exit 21 with `detail:"socket-dir-refused"` + `during:"connect"`, at WARN and no `channel-request` line, `server-verify-failed` also covering a server of another euid; `run` and `revive`''s start write one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`.»'
    sidecar: 'res-filesystem: the frame-verb and start lines of a refused socket directory, and the euid under `server-verify-failed`.'
    rationale: 'The Coverage rows state both lines (report.md:70-71), and a refused euid is `CallError::ServerUnverified`, read as `server-verify-failed` (report.md:15).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: '## Occupied Resources'
    change: 'Add a new continuation line after this line, leaving it as it reads: « On a refused socket directory `hook` writes one `hook-decision{detail:"socket-dir-refused"}` at WARN, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`, its `server-verify-failed` line also covering a server of another euid; `mcp` writes no line for a frame tool whose socket directory is refused, `isError` being the record.»'
    sidecar: 'res-filesystem: the hook and MCP records of a refused socket directory.'
    rationale: 'The hook''s line and MCP''s absent line are stated in Symbols / APIs and Coverage (report.md:16-17, report.md:72-73).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 125
    section: '## Occupied Resources'
    change: 'At the end of the line add: « `socket-dir-refused` is a word of it and of `hook-decision`''s `detail` enum.»'
    sidecar: 'res-filesystem: `socket-dir-refused` joins both `detail` enums of the line schema.'
    rationale: '`schemas/diag-line.v1.json` gains the word on both enums (report.md:24, report.md:30).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:24
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 155
    section: '## Occupied Resources'
    change: 'Change «`viola_state::board::scan` runs `check_path` on the home and on `instances/`, each that exists» to «`viola_state::board::scan` runs `check_read` on the home and `check_path` on `instances/`, on every OS, each that exists»; the rest of the line stands.'
    sidecar: 'res-filesystem: `viola list`''s scan holds the home to the read rule.'
    rationale: '`scan`''s `check_if_present` takes its check, the home through `check_read` and `instances/` staying `check_path` on every OS, with order, the whole-board refusal and no-open unchanged (report.md:18, report.md:32).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 163
    section: '## Occupied Resources'
    change: 'Change the last sentence to: «`Refused` holds `Readable` for a read-rule refusal, `Symlink` for a symbolic link `check_diagnostics` or `check_socket_dir` meets and `NotADirectory` for a socket directory path that is no directory, each with a fixed message.»; «holds two rules» stands.'
    sidecar: 'res-filesystem: `Refused` gains the fixed-message `NotADirectory`, and `Symlink` a second reader.'
    rationale: 'The new variant and the check''s symlink refusal are in Symbols / APIs (report.md:11); strict-modes'' two rules stay true (report.md:32).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 175
    section: '## Occupied Resources'
    change: 'At the end of the line add: « The socket directory check at `client::connect`, hook `connect` and `mcp`''s `frame` is no part of this: it comes after the instance check, on the recorded `endpoint`, and a refusal may write a line (`diagnostics/`, above).»'
    sidecar: 'res-filesystem: the connect-time socket directory check placed after the instance check and apart from its no-line rule.'
    rationale: 'The check reads the recorded endpoint at each `connect` and after `live_endpoint` in `frame` (report.md:15-17), and its refusal writes a line on the frame-verb and hook paths (report.md:71-72), where this row''s refusals write none.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 187
    section: '## Occupied Resources'
    change: 'Replace the row with: «- `viola list` holds the home to the read rule on every OS: `board::scan` reads it through `check_read` with or without an instance, so a 0750 home, or on Windows one granting another SID read, refuses the whole board (`ScanRefused::Home`). The rule on every OS is the overseer''s answer as the founder''s delegate, PROVISIONAL, the founder''s own word owed at Epoch 7''s boundary batch (`inputs#I2` Q3).»'
    sidecar: 'res-filesystem: `viola list`''s two home rules retired, reconciled to the read rule; the CARRY on "Unix endpoint and home hardening" discharged.'
    rationale: 'The Retired bullet names `viola list`''s two rules on the home and the owner''s dated gaps (report.md:26); the board moved to the read rule (report.md:18, report.md:32, report.md:112), its authority the delegate''s (report.md:45, report.md:52).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 187
    section: '## Occupied Resources'
    change: 'Add a new row after this line: «- `viola_state::board::scan` holds the home to `check_read` and `instances/` to `check_path` on every OS, while `read_budget_stamped`, `mcp`''s start and `plugin install` keep the write rule on the home.»'
    sidecar: 'res-filesystem: which readers of the home take which rule after the board''s move.'
    rationale: 'Stated in Symbols / APIs (report.md:18).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 218
    section: '## Occupied Resources'
    change: 'Add a new row after this line: «- Test-only: harness `cleanup`''s `endpoint_gone` on Unix goes through `crates/viola-e2e/src/harness/cleanup.rs` `unconnectable`, where `UnixStream::connect` refused or `NotFound` reads gone, `ok:true` after a force-killed wrapper with no leftover file removed, `holder_gone`''s record, above, standing beside this measured rule.»'
    sidecar: 'res-filesystem: the measured `cleanup` rule for `endpoint_gone` on Unix, beside `holder_gone`.'
    rationale: 'The harness rule and its surface (report.md:20, report.md:42), the disproved file-absence reading (report.md:57), and the plan''s `holder_gone`''s record beside the measured `cleanup` (report.md:63).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:20
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 231
    section: '## Occupied Resources'
    change: 'Add a new row after this line: «- Test-only, outside the viola home: the test-owned socket roots under `/tmp`: `tests/support/home.rs` `SocketRoot`, `SOCKET_ROOT_VAR`, `Wrapper::boot_on_socket_root` and `first_life_on` give a test its own socket root, read from `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, used by `tests/security_negatives_socket_dir.rs`. `viola` never names them.»'
    sidecar: 'res-filesystem: the test-owned socket roots registered.'
    rationale: 'The test helpers (report.md:21), and their row in §Occupied Resources named by Expected amendments (report.md:63).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:21
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 35
    section: '## Cross-cutting Patterns'
    change: 'Add a new sub-bullet after this line: «  - The `unable:` / `hint:` pair for a socket directory names only the parsed `ViolaName`, no path, pid or uid, is plain ASCII with `hint:` last and no SGR, adds `send_hint` cause `socket-dir` in `src/human/hints.rs`, and stands as a planning form, its wording owed to the founder at Epoch 7''s boundary batch.»'
    sidecar: 'crosscut-sanitised-error-surfaces: the socket directory pair and its owed wording.'
    rationale: 'The pair''s Coverage row (report.md:76), the hint cause (report.md:19), and the wording owed as a planning form (report.md:49, report.md:52).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
    fact: F12
note: 'occupied-resources.md:122 and :155 already stand over the 2 000-char line cap, so the first takes its additions as new lines and the second a minimal in-line edit; the body''s Local endpoint trust boundary line (per-connection proof) has no picked entity in this group, so it is not proposed here.'
