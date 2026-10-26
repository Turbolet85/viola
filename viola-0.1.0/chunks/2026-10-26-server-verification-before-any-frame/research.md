# Codebase Research — 2026-10-26-server-verification-before-any-frame

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 14 (plus one code-graph query, `tree-query-2026-10-26-server-verification-before-any-frame.json` in the run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (Shims, boot, status / cleanup / logs, Fake agent, Host pressure, Exemptions, and the 18 Session Additions); the ones that apply here: 2026-09-27 (a nextest `test(/…/)` filter takes the full path, `binary(<stem>)` for a whole root file), 2026-09-29 (a local red is judged against a same-day control, never closed by a green re-run), 2026-10-11 (`schema-check` reads kept homes only, so the new cases' lines are read by their own schema assertion and CI's G4), 2026-10-20 (`target/debug/viola` is re-pointed per feature set). No live rig and no real `claude` is driven by this chunk.
- **Platform issues consulted:** none — no runner-only bullet: Setup 5a read `6bdfdda8` in progress, no red folded.
- **External inputs:** `inputs#I1` — the overseer's phase directive (forced sample at P2, evidence kept, a widening held for the founder's live word); `inputs#I2` — the operator's invocation. Every fact the plan turns on beside these lives in this repository or in the locked dependency source (`interprocess 2.4.4` in `~/.cargo/registry`, read as a library's behaviour, not an input).

## Files inspected
- `crates/viola-state/src/liveness.rs` (full) — `live_endpoint` (:74) is the CLI verbs' and the MCP tools' shared pre-check: `read_snapshot`, then `classify(beat_age, same_process)` = `Live`, then the snapshot's `endpoint`. It returns the endpoint only, so a caller has no recorded `pid` / `started_at` to compare a server with. `same_process` (:64) parses `started_at` RFC 3339 to whole seconds and compares it with `process_start_time(pid)` (:48, sysinfo, a zombie is none). `own_start` (:58) renders a pid's start as the snapshot writes it.
- `src/cmd/client.rs` (full) — the CLI verbs' shared outputs: `unreachable(json, during, human)` (:125) prints the `instance-unreachable` document or the verb's stderr pair and writes `process-exit{exit_code:21, detail:"instance-dead", during}` at WARN — its `detail` is a literal, so a second exit-21 cause has no way to say itself. `unable_unreachable` (:144) prints the `not-running` hint for `wait` / `last` / `answer` / `pause` / `release`. No `read_snapshot(` call in this file (`grep -c 'read_snapshot(' src/cmd/client.rs` = 0).
- `src/cmd/send.rs` (:205–260) — `deliver`: `refusal_of(text)` (control-character, empty-text, exit 13) runs first, then `live_endpoint`, then `out.open()` and one `request` = `Client::connect(endpoint, "cli")?.request("send", …)` (:253). The pre-frame order the check must keep: client refusals → pre-check → connect → (check) → frame.
- `src/cmd/wait.rs` (:43–89), `src/cmd/last.rs` (:30–61), `src/cmd/answer.rs` (:103–147), `src/cmd/pause.rs` (:44–87), `src/cmd/release.rs` (:18–75) — each `live_endpoint` → `Client::connect(endpoint, "cli")?.request(method, …)`; `release` goes through `pause::wheel_call` (`src/cmd/release.rs:18`); `wait` prints `waiting:` only after its pre-check and before its request.
- `src/cmd/hook.rs` (:140–260, :480–520, :560–800) — `handle` (event path): `hook-invoked` first, stdin cap, `normalise`, then `deliver` (:493): `read_snapshot(&instance.dir).and_then(|s| s.endpoint)` (no liveness, no identity) → `Client::connect_by(endpoint, "hook", deadline)` → `notify("hook.event")`; a `false` makes SessionEnd take `append_session_end` (:511) and the event `channel-unreachable`. `handle_dialog`: `classify`, then `ask` (:245): the same snapshot read → `connect_by` → `request_until("hook.dialog", …, reply_by)`. The unit fixture `served` (:737) writes a snapshot with `pid: std::process::id()` and `started_at: "s"`.
- `crates/viola-mcp/src/tools.rs` (:315–350) — `frame`: `live_endpoint` → `TokioClient::connect(endpoint, "mcp").await` → `request`; `result::unreachable()` (`crates/viola-mcp/src/result.rs:28`) is the `isError` `instance-unreachable` body; no line is written on that path today (`grep -n 'obs_event!' crates/viola-mcp/src/*.rs` = one site, `tools.rs:300`, the client-side `send-refused`).
- `crates/viola-channel/src/client.rs` (:1–320) — `Client::connect_by` (:38) = `open_by` → `Client{reader, conn}`; Windows `open_by` (:248) adopts `open_handle`'s SQOS + overlapped `CreateFileW` handle through `PipeStream::try_from` then `Stream::from`; Unix `open_by` (:306) is `Stream::connect`. No identity read exists on either client.
- `crates/viola-channel/src/tokio_client.rs` (:1–110) — `TokioClient::connect` (:30) → `open` (Unix `Stream::connect`, Windows the same `client::open_handle` adopted by the Tokio `PipeStream`).
- `crates/viola-channel/src/endpoint.rs` (:1–80) — the per-user socket dir and endpoint path; no directory check before a client connect (`grep -rn 'symlink_metadata\|lstat' crates/viola-channel/src` = 0 hits).
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/interprocess-2.4.4/src/os/unix/local_socket/peer_creds.rs` (:1–110) — `PeerCreds::pid()` is `Some` on Linux (`SO_PEERCRED`'s `pid`) and `None` on macOS (`xucred` has no pid); `euid()` is `Some` on both.
- `…/interprocess-2.4.4/src/os/windows/named_pipe/{local_socket/stream.rs,stream/impl.rs,stream/impl/handle.rs}` — `peer_creds()` on Windows is `peer_process_id()`, which reads `server_process_id()` (`GetNamedPipeServerProcessId`) on a client end; an adopted handle learns its side from `GetNamedPipeInfo`'s `PIPE_SERVER_END` flag (`handle.rs:16`), so the SQOS-adopted client handle reads the server's pid. The same holds for the Tokio flavour (`tokio/stream/impl.rs:96`, `tokio/stream/impl/handle.rs:29`). Read in source; not run on Windows here.
- `src/run/snapshot.rs`, `src/cmd/run.rs` (:365–395) — `run.collision_check` reads the snapshot and classifies it; it opens no connection and writes no frame.
- `src/human/hints.rs` (:26–120) — `send_hint` has one exit-21 hint, `not-running`; the server-verification hint design names (`the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, `.andromeda/design-system-detail/surface.md` cli-send) is not built; `no_hint_names_release` walks a closed cause list.
- `crates/viola-mcp/tests/handlers.rs` (:14–130) — `TestWrapper` is a `viola_channel::Server` in the test process; its snapshot is written through `write_snapshot` with `pid` = the test process and `started_at` = `own_start(pid)`, its heartbeat through `touch_heartbeat` — so a verified client passes against it, and the same shape with a recorded pid that is NOT the serving process is an impostor.
- `tests/hook_fail_open.rs` (:170–300) — the event matrix and the dialog-tier matrix, each case exit 0 with empty stderr and stdout under `< 1.0 s`; `Env::Stopped` and the no-snapshot case read `channel-unreachable`.
- `tests/security_negatives_channel.rs` (:1–60, the test list) — `#![cfg(windows)]`: the SQOS identification case and the listener's remote-rejection and DACL cases, each beside a control.
- `.claude/rules/verification-harness.md` (full) — above.

## Graph impact (from the code-graph query; `tree-query-2026-10-26-server-verification-before-any-frame.json`)
- **`live_endpoint`** — product callers `src/cmd/send.rs:219` (`deliver`), `src/cmd/wait.rs:49`, `src/cmd/last.rs:36`, `src/cmd/answer.rs:116` (`deliver`), `src/cmd/pause.rs:60` (`wheel_call`, which `release` calls), `crates/viola-mcp/src/tools.rs:330` (`frame`); test callers `crates/viola-state/src/liveness.rs:153`, `:165`, `:170`. A changed return type threads through all six product sites.
- **`connect_by`** — product callers `src/cmd/hook.rs:249` (`ask`), `src/cmd/hook.rs:497` (`deliver`); `Client::connect` (:34) wraps it. Product `Client::connect` sites: `src/cmd/{send,wait,last,answer,pause,release}.rs` (one each); `TokioClient::connect`: `crates/viola-mcp/src/tools.rs:333`; `crates/viola-e2e/src/harness/cleanup.rs:139` connects only to prove an endpoint gone (test-only harness, no frame — no change).
- **`read_snapshot`** — product callers under `src/` (the rule: `read_snapshot(` under `src/`, outside test modules): `src/cmd/hook.rs:246` (`ask`), `src/cmd/hook.rs:494` (`deliver`), `src/cmd/hook/statusline.rs:142` (`serve`), `src/cmd/run.rs:379` (`collision_check`) — 4; plus `crates/viola-state/src/liveness.rs:75` (`live_endpoint`) and `crates/viola-state/src/board.rs:272` (`board::read`, the classified read) in crates.
- **`same_process` / `process_start_time`** — `same_process` called by `live_endpoint` (:77) and `run.collision_check` (:380); `process_start_time` by `own_start`, `same_process` and the fake agent's registry (`src/bin/viola-fake-agent.rs:799`).

## Patterns detected
- **One shared client pre-check in `viola-state`** (`crates/viola-state/src/liveness.rs:74`): the CLI verbs and the MCP tools already share it; the identity reference values (`pid`, `started_at`) live in the same snapshot it reads, so the verification's comparison sits beside it, not per verb.
- **Endpoint mechanics in `viola-channel`, identity facts from the OS** (`client.rs:248`, `tokio_client.rs:96`): both clients reach the same stream type, whose `peer_creds()` gives the server's pid (Windows, Linux) or none (macOS); `viola-channel` has no `viola-state` edge, so the start-time read and the comparison stay above it.
- **A refusal made before any frame logs only its outcome** (`src/cmd/send.rs` `refused_client_side`; `src/cmd/client.rs:125`): no `channel-request` line for a frame not written.
- **Fail-open as a `detail` on `hook-decision`** (`src/cmd/hook.rs:171–176`): the event path returns the code it took; SessionEnd's direct append runs whenever delivery is false.
- **The test-process server as the recorded wrapper** (`crates/viola-mcp/tests/handlers.rs` `booted`; `src/cmd/hook.rs:737` `served`): a snapshot written through the writer naming the serving process — the shape a verification positive keeps and an impostor case varies (a live recorded pid that is not the server).

## Conventions to follow
- **Closed codes only**: `process-exit` `detail` `server-verify-failed` and `hook-decision` `detail` `server-verify-failed` are already members of `schemas/diag-line.v1.json`'s closed enums (`:53`, `:78`), so no schema change; `--json` keeps `{"v":1,"error":"instance-unreachable","detail":null}` (architecture §Standard Contracts) and MCP its `instance-unreachable` body.
- **Pre-frame order**: `send`'s `control-character` / `empty-text` and `answer`'s free-text refusal (exit 13) stay first (`src/cmd/send.rs:212`, `src/cmd/answer.rs:104`); MCP's `invalid-params` and client-side `send-refused` stay first (`crates/viola-mcp/src/tools.rs`).
- **Hook bounds**: the event path's connect keeps `CONNECT_DEADLINE` (750 ms) below `SPINE_DEADLINE`, the dialog path its `DIALOG_HOLD_MAX + 5 s` reply bound (`src/cmd/hook.rs` units `connect_deadline_is_below_the_spine_deadline`, `dialog_deadline_is_the_longest_hold_plus_the_reply_grace`).
- **Tests**: root integration file prefixes (`security_negatives_`, `hook_`, `cli_`), rstest `#[case::label]`, homes under `target/e2e-home`, no hand-written snapshot in a booted-wrapper test — an impostor's snapshot is the writer's (`write_snapshot`), never a booted wrapper's file rewritten (test-plan §7 Seed strategies, §11 Test Data).

## New files to create
- `tests/security_negatives_server_identity.rs` — the cross-OS (`cfg(not(target_os = "macos"))`) impostor negatives: a test-owned server on an endpoint whose writer-made snapshot records a live pid that is not the server's, driven by the CLI verbs and both hook paths, beside the control where the recorded pid is the server's

## Files to modify
- `crates/viola-channel/src/client.rs` — the connected stream's server pid, read before any frame
- `crates/viola-channel/src/tokio_client.rs` — the same read on the Tokio client
- `crates/viola-state/src/liveness.rs` — the pre-check hands the recorded identity to its callers, and the pure verification decision
- `src/cmd/client.rs` — the exit-21 outcome names its cause (`instance-dead` or `server-verify-failed`) and its hint
- `src/human/hints.rs` — the server-verification hint
- `src/cmd/send.rs` — verified before its `send` frame
- `src/cmd/wait.rs` — verified before its `wait` frame
- `src/cmd/last.rs` — verified before its `last` frame
- `src/cmd/answer.rs` — verified before its `answer` frame
- `src/cmd/pause.rs` — `wheel_call` verified before the `pause` / `release` frame
- `src/cmd/release.rs` — its `request` takes the verified connection
- `src/cmd/hook.rs` — `deliver` and `ask` verify before `hook.event` / `hook.dialog`; the unit fixture records the serving process's real start time
- `crates/viola-mcp/src/tools.rs` — `frame` verifies before its Tokio-client frame
- `crates/viola-mcp/tests/handlers.rs` — the MCP impostor case and its control
- `tests/hook_fail_open.rs` — the mismatch rows of the event and dialog-tier matrices

## Sweeps
- `read_snapshot(` under `src/`, outside test modules: 4 hits (`hook.rs:246`, `hook.rs:494`, `statusline.rs:142`, `run.rs:379`) · 2 changed (`hook.rs` ×2, behind the verification, the reader itself unchanged) · 2 no-change (`statusline.rs` writes no frame; `run.rs` opens no connection) — re-derived: `grep -rn 'read_snapshot(' src` read against each hit's module. The five readers the freight names count `src/cmd/client.rs`, which holds no call any more.
- `live_endpoint(` over the workspace: 9 hits · 6 product changed (above) · 3 tests in `liveness.rs` (kept, moved with the signature).
- `Client::connect` / `TokioClient::connect` product sites: 8 · 8 changed (the six CLI verbs' `request`, the hook's two) + `tools.rs:333` · `cleanup.rs:139` no-change (an endpoint-gone probe, test-only, writes no frame).
- `started_at: "` in Rust sources: 10 hits · 1 changed (`src/cmd/hook.rs:752`, the hook client's fixture: `"s"` is no start time, so a verified `ask` would fail open against it) · 9 no-change (wrapper-side, `revive` and `statusline` fixtures: none drives a frame-writing client) — `grep -rn 'started_at: "' --include=*.rs src crates tests`.

## Open questions
- Where the strict-modes half of the dated gaps lands (with this chunk, or with "Home and code-bearing file integrity", whose line carries the same verbs' strict-modes CARRYs) → blocks: plan-decision (leaned at P4 from the route's own freight).
- What a client does with an unreadable or newer `snapshot.json` (CARRY 7) and whether the five readers move to the replay (CARRY 8) → blocks: plan-decision (the founder's to rule, held).
- Whether a pid + start-time check changes CARRY 2's Windows window (a hook reaching an exiting wrapper's pipe): the exiting wrapper IS the recorded process, so a passing check is the correct verdict there; whether sysinfo still lists a terminated process whose pipe handle is open is unmeasured on Windows → blocks: implementation-scope (named, not closed, at P4).
