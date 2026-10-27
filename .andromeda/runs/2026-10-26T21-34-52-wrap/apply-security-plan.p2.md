# Apply — security-plan · part 2 of 4

## Verdicts
- fanout-security-plan--cascade-g1.md 7 · applied · .andromeda/security-plan-detail/authentication-authorization.md:25#1, .andromeda/security-plan-detail/authentication-authorization.md:25#2
- fanout-security-plan--cascade-g1.md 8 · applied · .andromeda/security-plan-detail/authentication-authorization.md:25#3, .andromeda/security-plan-detail/authentication-authorization.md:26#1, .andromeda/security-plan-detail/authentication-authorization.md:26#2, .andromeda/security-plan-detail/authentication-authorization.md:26#3, .andromeda/security-plan-detail/authentication-authorization.md:27#1
- fanout-security-plan--cascade-g1.md 9 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#2
- fanout-security-plan--cascade-g1.md 10 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#4
- fanout-security-plan--cascade-g1.md 11 · applied · .andromeda/security-plan-detail/authentication-authorization.md:28#1
- fanout-security-plan--cascade-g1.md 12 · applied · .andromeda/security-plan-detail/authentication-authorization.md:27#3
- fanout-security-plan--cascade-g1.md 13 · applied · .andromeda/security-plan-detail/authentication-authorization.md:34#1
- fanout-security-plan--cascade-g1.md 14 · applied · .andromeda/security-plan-detail/authentication-authorization.md:34#2
- fanout-security-plan--cascade-g1.md 15 · applied · .andromeda/security-plan-detail/authentication-authorization.md:35#1
- fanout-security-plan--cascade-g1.md 16 · applied · .andromeda/security-plan-detail/authentication-authorization.md:35#2, .andromeda/security-plan-detail/authentication-authorization.md:35#3
- fanout-security-plan--cascade-g1.md 17 · applied · .andromeda/security-plan-detail/authentication-authorization.md:35#4
- fanout-security-plan--cascade-g1.md 18 · applied · .andromeda/security-plan-detail/authentication-authorization.md:35#5
- fanout-security-plan--cascade-g1.md 19 · applied · .andromeda/security-plan-detail/authentication-authorization.md:36#1
- fanout-security-plan--cascade-g1.md 20 · applied · .andromeda/security-plan-detail/authentication-authorization.md:36#2, .andromeda/security-plan-detail/authentication-authorization.md:36#3
- fanout-security-plan--cascade-g1.md 21 · applied · .andromeda/security-plan-detail/authentication-authorization.md:37#1
- fanout-security-plan--cascade-g1.md 22 · applied · .andromeda/security-plan-detail/authentication-authorization.md:37#2
- fanout-security-plan--cascade-g1.md 23 · applied · .andromeda/security-plan-detail/authentication-authorization.md:38#1, .andromeda/security-plan-detail/authentication-authorization.md:40#1
- fanout-security-plan--cascade-g1.md 24 · applied · .andromeda/security-plan-detail/authentication-authorization.md:39#1
- fanout-security-plan--cascade-g1.md 25 · applied · .andromeda/security-plan-detail/authentication-authorization.md:40#2
- fanout-security-plan--cascade-g1.md 26 · applied · .andromeda/security-plan-detail/authentication-authorization.md:41#1
- fanout-security-plan--cascade-g1.md 27 · applied · .andromeda/security-plan-detail/authentication-authorization.md:41#2
- fanout-security-plan--cascade-g1.md 28 · applied · .andromeda/security-plan-detail/authentication-authorization.md:42+#1
- fanout-security-plan--cascade-g1.md 29 · applied · .andromeda/security-plan-detail/authentication-authorization.md:63#1
- fanout-security-plan--cascade-g1.md 30 · applied · .andromeda/security-plan-detail/authentication-authorization.md:66#1, .andromeda/security-plan-detail/authentication-authorization.md:66#2
- fanout-security-plan--cascade-g1.md 31 · applied · .andromeda/security-plan-detail/authentication-authorization.md:67#1
- fanout-security-plan--cascade-g1.md 32 · applied · .andromeda/security-plan-detail/authentication-authorization.md:69#1
- fanout-security-plan--cascade-g1.md 33 · applied · .andromeda/security-plan-detail/authentication-authorization.md:70#1

## Edits
- line: .andromeda/security-plan-detail/authentication-authorization.md:25
  old: «**Interim gap (ratified), its strict-modes half only:** on Linux and Windows `viola hook` verifies the serving process before its `hook.event` frame (`src/cmd/hook.rs`'s shared `connect`: `read_snapshot`, then `connect_by`, then `verify_server`; macOS the recorded gap), but reads that snapshot without the strict-modes check, owed to the route entry "Home and code-bearing file integrity" (Decisions Log `2026-09-28`). **Second interim gap (the founder's ruling, live, 2026-10-04, relayed by the overseer):** its strict-modes half owed to the route entry "Home and code-bearing file integrity": CLI `viola send` writes its `send` frame after a liveness-only pre-check — `read_snapshot`, then the snapshot's pid + start time alive and the heartbeat live (`classify(beat_age, same_process)` = `live`) with an `endpoint` present, else exit 21 — and then, on Linux and Windows, checks the serving process's identity before the frame (`client::connect`: the peer pid and its start time against the snapshot's `pid` and `started_at` from the same `live_endpoint` read), but not the snapshot's strict-modes. The residual, a same-user process squatting a stale endpoint name, is closed on Linux and Windows and open on macOS; the pipe DACL and the 0700 socket directory still apply. It does not borrow the `hook.event` exception.»
  new: «**Strict-modes check first, no dated gap:** every client runs the instance's strict-modes check before it reads the snapshot (`viola hook`'s event and dialog paths through `canonical` and `trusted` in `src/cmd/hook.rs` before obs init; CLI `send` through `client::refuse_untrusted` after its text read), then reads it through `read_snapshot_or_replay`, only a parsed snapshot giving reference values, then on Linux and Windows checks the serving process's identity before the frame (macOS the recorded gap): `viola hook` through its shared `connect` (`connect_by`, then `verify_server`); CLI `send` after a liveness-only pre-check (`live_endpoint`: the snapshot's pid + start time alive and the heartbeat live (`classify(beat_age, same_process)` = `live`) with an `endpoint` present, else exit 21) through `client::connect` (the peer pid and its start time against the snapshot's `pid` and `started_at` from the same `live_endpoint` read). The residual, a same-user process squatting a stale endpoint name, is closed on Linux and Windows and open on macOS; the pipe DACL and the 0700 socket directory still apply.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:25
  old: «closed `send` params) and this gap.»
  new: «closed `send` params).»
- line: .andromeda/security-plan-detail/authentication-authorization.md:25
  old: «**Third interim gap here, the fifth dated gap overall (the founder's ruling, live, 2026-10-04, relayed by the overseer):** its strict-modes half owed to the route entry "Home and code-bearing file integrity": CLI `viola wait` and `viola last` write their `wait` / `last` frame after the same liveness-only pre-check (else exit 21 `instance-unreachable`, `during:"connect"`), then, on Linux and Windows, check the serving process's identity before the frame through the same `client::connect`, but not the snapshot's strict-modes. The residual is the same, closed on Linux and Windows and open on macOS; they borrow neither the `hook.event` exception nor `send`'s.»
  new: «CLI `viola wait` and `viola last` run the instance check first (`client::refuse_untrusted`), then write their `wait` / `last` frame after the same liveness-only pre-check (else exit 21 `instance-unreachable`, `during:"connect"`), then, on Linux and Windows, check the serving process's identity before the frame through the same `client::connect`. The residual is the same, closed on Linux and Windows and open on macOS.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:26
  old: «under the channel row's controls (frame cap, depth, version, the `wait` / `last` params) and this gap.»
  new: «under the channel row's controls (frame cap, depth, version, the `wait` / `last` params).»
- line: .andromeda/security-plan-detail/authentication-authorization.md:26
  old: «**Fourth interim gap here, the sixth dated gap overall (the founder's ruling F1, live, 2026-10-04, relayed by the overseer, the residual shown):** its strict-modes half owed to the route entry "Home and code-bearing file integrity": CLI `viola answer` writes its `answer` frame after the same liveness-only pre-check (`live_endpoint`; else exit 21 `instance-unreachable`), and `viola hook pre-tool-use|permission-request` writes its `hook.dialog` frame after its payload shape check (`classify`), on the endpoint its event path reads; on Linux and Windows both check the serving process's identity before the frame (`answer` through `client::connect`, `hook.dialog` through `src/cmd/hook.rs`'s shared `connect`), and neither checks the snapshot's strict-modes. The residual, a process squatting the endpoint answering a dialog (a PermissionRequest `allow` included), is closed on Linux and Windows and open on macOS. These frames borrow neither the `hook.event` exception nor `send`'s nor `wait` / `last`'s.»
  new: «CLI `viola answer` runs the instance check first (`client::refuse_untrusted`), then writes its `answer` frame after the same liveness-only pre-check (`live_endpoint`; else exit 21 `instance-unreachable`), and `viola hook pre-tool-use|permission-request`, past its instance check (`trusted`), writes its `hook.dialog` frame after its payload shape check (`classify`), on the endpoint its event path reads; on Linux and Windows both check the serving process's identity before the frame (`answer` through `client::connect`, `hook.dialog` through `src/cmd/hook.rs`'s shared `connect`). The residual, a process squatting the endpoint answering a dialog (a PermissionRequest `allow` included), is closed on Linux and Windows and open on macOS.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:26
  old: «dialog-answers-by-dialog-id, under the channel row's controls and this gap. **Fifth interim gap here, the seventh dated gap overall (the founder's ruling F-W1, live, 2026-10-04, relayed by the overseer, the pause-swallow residual shown):** its strict-modes half owed to the route entry "Home and code-bearing file integrity": CLI `viola pause` and `viola release` write their `pause` / `release` frame after the same liveness-only pre-check (`live_endpoint`; else exit 21 `instance-unreachable`); on Linux and Windows both then check the serving process's identity before the frame through `client::connect`, and neither checks the snapshot's strict-modes.»
  new: «dialog-answers-by-dialog-id, under the channel row's controls. CLI `viola pause` and `viola release` run the instance check first (`client::refuse_untrusted`), then write their `pause` / `release` frame after the same liveness-only pre-check (`live_endpoint`; else exit 21 `instance-unreachable`); on Linux and Windows both then check the serving process's identity before the frame through `client::connect`.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «These frames borrow none of the six earlier gaps. `pause` and `release` are served by the wrapper as of chunk 2026-10-04-the-wheel, under the channel row's controls and this gap.»
  new: «`pause` and `release` are served by the wrapper as of chunk 2026-10-04-the-wheel, under the channel row's controls.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «**Sixth interim gap here, the eighth dated gap overall (the founder's own answer to P4 fork Q1, live, 2026-10-17, relayed by the overseer, `inputs#I2` of chunk 2026-10-17-mcp-server-for-drivers):** its strict-modes half owed to the route entry "Home and code-bearing file integrity": `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the same liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`); on Linux and Windows each tool then checks the serving process's identity after `TokioClient::connect` and before its frame (`server_verified`, the start-time read on `tokio::task::spawn_blocking`), and none checks the snapshot's strict-modes. These frames borrow none of the seven earlier gaps.»
  new: «`viola mcp`'s `send` / `wait` / `last` / `answer` tools run `check_instance` on the home and the instance directory first, then write their frames through the Tokio client after the same liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`); on Linux and Windows each tool then checks the serving process's identity after `TokioClient::connect` and before its frame (`server_verified`, the start-time read on `tokio::task::spawn_blocking`).»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «No line or output holds a pid, start time, path or endpoint.»
  new: «No line or output holds a pid, start time, path or endpoint. **On a strict-modes refusal** (the check comes before the liveness and server checks): no frame and no line is written, no `strict-modes-failed` line included; the CLI frame verbs exit 21 with the instance-state `unable:` / `hint:` pair (under `--json` the `instance-unreachable` document, `detail: null`); `viola hook` exits 0 with no body; MCP returns `instance-unreachable` with `detail: null`.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:27
  old: «`run`'s collision check (`src/cmd/run.rs:383`) reads the snapshot through `read_snapshot` and judges the recorded wrapper by its `pid` + OS start time and the heartbeat;»
  new: «`run`'s collision check (`src/cmd/run.rs:418`) takes the caller's one read through `read_snapshot_or_replay` (`run` after its home-tree check; `revive`'s preflight reads once and hands that read to both its collision check and its recorded-directory reading) and judges a parsed snapshot's recorded wrapper by its `pid` + OS start time and the heartbeat, a replayed one by the beat alone (a `beat_age` within `STALE_AFTER` is `already-live`, else free), an error or an absent snapshot free;»
- line: .andromeda/security-plan-detail/authentication-authorization.md:28
  old: «`viola-state` liveness (`live_endpoint`, `server_verdict`, `verify_server`), called by the root bin's `cmd::client::connect` and `cmd::hook` and by `viola-mcp`'s `frame`;»
  new: «`viola-state` liveness (`live_endpoint`, `server_verdict`, `verify_server`), called by the root bin's `cmd::client::connect` and `cmd::hook` and by `viola-mcp`'s `frame`; `viola-state` `strict::check_instance` (called by the root bin's `cmd::client::refuse_untrusted` and `cmd::hook`'s `trusted`, and by `viola-mcp`'s `frame`) and `replay::read_snapshot_or_replay` (read by `live_endpoint` and `cmd::hook`'s `connect`);»
- line: .andromeda/security-plan-detail/authentication-authorization.md:34
  old: «which reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order,»
  new: «which holds the home, the instance directory and its `diagnostics/` to the read rule, `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds,»
- line: .andromeda/security-plan-detail/authentication-authorization.md:34
  old: «proven by `cfg(unix)` cases, with no Windows case that widens an instance tree's DACL yet, owed to the entry "Home and code-bearing file integrity";»
  new: «proven by `cfg(unix)` cases and by `cfg(windows)` cases (an instance tree granted Users write refused beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read refused, a plain home made before viola refused `Writable`), green on `windows-2025` as measured at ci#40403389629;»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «it is none of the dated interim gaps; proven by `cfg(unix)` cases only), `link` / `unlink`»
  new: «it is none of the dated interim gaps; proven by `cfg(unix)` cases only; `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused, naming the instance (`ScanRefused::Instance`), once it holds one, an inconsistency that is a CARRY owed to the route entry "Unix endpoint and home hardening"), `link` / `unlink`»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «(`viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file»
  new: «(`viola_state::strict::check_instance` on the absolute home and `instances/<driven>` through `client::refuse_untrusted`, which the six frame verbs `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` call first too on `instances/<name>`, each before `client::start`, so before any role file»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «leaves the directory's mode as found; it is none of the dated interim gaps; proven by `cfg(unix)` cases only), `mcp`»
  new: «leaves the directory's mode as found; it is none of the dated interim gaps; proven for `link` / `unlink` by `cfg(unix)` cases only, for the six frame verbs on all three OSes), `mcp`»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «its frame tools then read `snapshot.json`'s `endpoint` under the eighth dated gap, without the check)»
  new: «its frame tools then run `check_instance` on the home and the instance directory before `live_endpoint`, a refusal returning `instance-unreachable` with `detail: null`, no frame and no line)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «every `hook <event>` including `hook statusline` (which runs»
  new: «every `hook <event>` (its event and dialog paths run the same instance check as `hook statusline`, through `canonical` and `trusted` in `src/cmd/hook.rs`, before the diagnostics level is read and before obs init: a refused or absent instance exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append; the two `--capture` arms stay ahead of the check) including `hook statusline` (which runs»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «it is none of the dated interim gaps), and every CLI client verb that resolves an instance's `endpoint`;»
  new: «it is none of the dated interim gaps; on every hook path a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no instance, so the hook writes nothing), and every CLI client verb that resolves an instance's `endpoint`;»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «interim gap, ratified: until the Epoch 6 entry "Home and code-bearing file integrity" lands, `viola hook <event>` reads `snapshot.json`'s `endpoint` without this check, Decisions Log `2026-09-28`; a second, ratified the same way: `viola verify`'s `update_stamps` reads `ledger/stamps.json` without it until the same entry adds it to this entry-point set, Decisions Log `2026-09-28`, stamps read (`viola run`'s version gate left this gap with chunk 2026-10-04-dialog-answers-by-dialog-id: it reads through»
  new: «`viola verify`'s `update_stamps` (`viola_state::strict::check_stamps` before `create_private_dir(ledger/)` and again under its lock, a refusal `StampsError::StrictModes` that reads and writes nothing), `viola run`'s version gate (it reads through»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «a third, ratified by the founder live: on Windows x64 `viola run` loads the ConPTY companions `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` under a `FILE_SHARE_READ`-only held handle and a full SHA-256 re-hash against the compiled pins, without the Windows owner/DACL check, until the same entry adds the `conpty/` folder and both files to the Windows set below, Decisions Log `2026-09-29`; a fourth, the founder's ruling, live, 2026-10-04, relayed by the overseer: `viola send` reads `snapshot.json`'s `endpoint` (and its liveness fields) without this check until the same entry adds it; a fifth, the founder's ruling, live, 2026-10-04, relayed by the overseer: `viola wait` and `viola last` read them the same way, without this check, until the same entry adds them;»
  new: «on Windows x64, `viola run`'s ConPTY companions `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (`pin_companions` runs `viola_state::strict::check_present` on the `conpty/` folder and on each companion that exists before the `FILE_SHARE_READ`-only held open and the full SHA-256 re-hash against the compiled pins; a refusal writes nothing and leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`);»
- line: .andromeda/security-plan-detail/authentication-authorization.md:37
  old: «a sixth, the founder's ruling F1, live, 2026-10-04, relayed by the overseer: `viola answer` reads them the same way, without this check, until the same entry adds it — `viola hook pre-tool-use|permission-request` stays under the first; a seventh, the founder's ruling F-W1, live, 2026-10-04, relayed by the overseer: `viola pause` and `viola release` read them the same way, without this check, until the same entry adds them; an eighth, the founder's own answer to P4 fork Q1, live, 2026-10-17, relayed by the overseer (`inputs#I2` of chunk 2026-10-17-mcp-server-for-drivers): `viola mcp`'s `send` / `wait` / `last` / `answer` tools read them the same way, through `viola_state::liveness::live_endpoint`, without this check, until the same entry adds them — `mcp`'s own start»
  new: «`mcp`'s own start»
- line: .andromeda/security-plan-detail/authentication-authorization.md:37
  old: «and the installed `command` gets no re-hash, its integrity owed to the route entry "Home and code-bearing file integrity")»
  new: «and the installed `command` gets no re-hash, `marketplace/` being off the integrity set)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:38
  old: «**Read rule:** also refuse when the home or `instances/<name>/` grants any group or other permission (`mode & 0o077 != 0`).»
  new: «**Read rule:** also refuse when the home, `instances/<name>/` or its `diagnostics/` grants any group or other permission (`mode & 0o077 != 0`; `strict::unix_read_verdict`).»
- line: .andromeda/security-plan-detail/authentication-authorization.md:39
  old: «On failure `run` and `mcp` exit 1;»
  new: «On failure `run`, `verify` and `mcp` exit 1 (`viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, a refusal printing the home pair with no step line and no summary, and maps its `update_stamps` refusal `StampsError::StrictModes` to the same pair);»
- line: .andromeda/security-plan-detail/authentication-authorization.md:40
  old: «that grants read (`FILE_READ_DATA`, `GENERIC_READ` or `GENERIC_ALL`;»
  new: «that grants read (`READ_RIGHTS` = `0x9000_0001`: `FILE_READ_DATA`, `GENERIC_READ` or `GENERIC_ALL`, judged by `strict::windows_read_verdict`;»
- line: .andromeda/security-plan-detail/authentication-authorization.md:40
  old: «so a readable directory exposes `events.ndjson` and `diagnostics/`.»
  new: «so a readable directory exposes `events.ndjson` and `diagnostics/`. A refusal names the property it refused: `Refused` holds `Readable` ("a trusted state path is readable by another user") for the read rule and `Symlink` ("a trusted state path is a symbolic link") for a symbolic link, each message fixed, so neither borrows `Writable`.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:41
  old: «The outcomes match Unix.»
  new: «The outcomes match Unix but one: an unstattable home (a path under a regular file) is a strict-modes refusal on Unix (`verify` exits 1 with the home pair, the frame verbs 21 with the instance pair), while on Windows such a path stats `NotFound`, passes as absent, and the catch-site line stands.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:41
  old: «This covers pre-existing and later-modified homes, not only creation.»
  new: «This covers pre-existing and later-modified homes, not only creation. The witnesses: `tests/security_negatives_strict_modes.rs` (the six frame verbs, a start under umask 000, the Windows protected DACL read back, a read-granted pre-existing home refusing `run`, single writers with the wrapper stopped) and `tests/chaos_snapshot_replay.rs` (the replayed readers, E5 steps 1–3), green on all three OSes as measured at ci#40403389629.»
- after: .andromeda/security-plan-detail/authentication-authorization.md:42
  anchor: «  - **Stack reference:** `viola-state` home resolution; tempfile 3.27.0 `persist` via `viola_state::fs::replace_private`; std `OpenOptionsExt`; windows-sys 0.61.2»
  add: «- **Requirement:** `~/.viola/` access control, the home tree at start»
  add: «  - **Implementation:** `viola_state::strict::check_home_tree(home, name, key)` checks the home (read rule), then with a name `check_instance`'s set, then `ledger/`, `bin/` and `plugin/`, then with a key `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (write rule), each that exists, and returns `TreeRefused { HomeLevel, InstanceLevel }`. `run` and `revive`'s start run it through `checked_exe` (the exe's bytes read once, its `pin_key` taken) before the wrapper log opens; a refusal exits 1 with the home or the instance `unable:` / `hint:` pair (`refuse_tree`; `revive --json` the `strict-modes-failed` document), no log, no line and no child.»
  add: «  - **Stack reference:** `viola-state` `strict` (`check_home_tree`, `TreeRefused`); root bin `src/cmd/run.rs` (`checked_exe`, `refuse_tree`), `src/cmd/revive.rs`»
- line: .andromeda/security-plan-detail/authentication-authorization.md:63
  old: «taken with the `endpoint` from one `read_snapshot` call:»
  new: «taken with the `endpoint` from one `read_snapshot_or_replay` call made after the instance's strict-modes check, only a parsed snapshot giving reference values (a replayed or absent one is no `LiveEndpoint` for the CLI frame verbs and `viola mcp`, and `NotSent::Unreachable`, failing open, for `viola hook`'s `connect`):»
- line: .andromeda/security-plan-detail/authentication-authorization.md:66
  old: «No dated gap: both verbs run `viola_state::strict::check_instance` on the absolute home»
  new: «No dated gap: both verbs run `viola_state::strict::check_instance` through `client::refuse_untrusted` (shared with the six frame verbs) on the absolute home»
- line: .andromeda/security-plan-detail/authentication-authorization.md:66
  old: «in the same process, so they borrow none of the eight dated gaps.»
  new: «in the same process, the order every CLI frame verb takes.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:67
  old: «root bin `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`);»
  new: «root bin `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`) and `src/cmd/client.rs` (`refuse_untrusted`, where the `check_instance` call sits);»
- line: .andromeda/security-plan-detail/authentication-authorization.md:69
  old: «and `link` / `unlink`'s strict-modes refusal (exit 21) comes before their liveness and server checks.»
  new: «and every frame verb's strict-modes refusal comes before its liveness and server checks: `client::refuse_untrusted` runs `check_instance` first (`send` after its text read), a refusal exiting 21 with the instance-state `unable:` / `hint:` pair, or under `--json` the same `instance-unreachable` document, opening nothing under the home and writing no line.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:70
  old: «root bin `src/cmd/client.rs` (`connect`, `Unreachable::ServerUnverified`);»
  new: «root bin `src/cmd/client.rs` (`connect`, `Unreachable::ServerUnverified`, `refuse_untrusted`);»

## Patterns
[[pattern]]
id = "sec2-01"
fixed = 'strict-modes half owed to the route entry'

[[pattern]]
id = "sec2-02"
fixed = 'reads that snapshot without the strict-modes check'

[[pattern]]
id = "sec2-03"
regex = '(but not|neither checks|none checks) the snapshot.s strict-modes'

[[pattern]]
id = "sec2-04"
regex = 'controls[^.]{0,80} and this gap'

[[pattern]]
id = "sec2-05"
regex = 'borrow (neither|none of) the'

[[pattern]]
id = "sec2-06"
fixed = 'reads the snapshot through `read_snapshot` and'

[[pattern]]
id = "sec2-07"
fixed = 'with no Windows case that widens an instance tree'

[[pattern]]
id = "sec2-08"
fixed = 'each that exists and in that order'

[[pattern]]
id = "sec2-09"
fixed = 'under the eighth dated gap, without the check'

[[pattern]]
id = "sec2-10"
regex = 'without (this|the) check,? until the (same|Epoch 6) entry'

[[pattern]]
id = "sec2-11"
fixed = 'reads `ledger/stamps.json` without'

[[pattern]]
id = "sec2-12"
fixed = 'without the Windows owner/DACL check'

[[pattern]]
id = "sec2-13"
fixed = 'its integrity owed to the route entry'

[[pattern]]
id = "sec2-14"
fixed = 'from one `read_snapshot` call'

[[pattern]]
id = "sec2-15"
fixed = 'strict-modes refusal (exit 21) comes before their liveness'

[[pattern]]
id = "sec2-16"
fixed = 'The outcomes match Unix. This covers'
