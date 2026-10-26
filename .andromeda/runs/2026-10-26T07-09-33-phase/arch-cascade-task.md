# Task — arch · 2026-10-26-server-verification-before-any-frame

## The chunk
Builds server verification before the first frame in every client that writes a frame to a wrapper's endpoint: CLI `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s `hook.event` (every event) and `hook.dialog` (`pre-tool-use` · `permission-request`); `viola mcp`'s `send` / `wait` / `last` / `answer` tools through the Tokio client.
The check: the connected server's process id (Unix `peer_creds` on the socket, Windows the pipe's server process id) and its sysinfo start time must match the `pid` and `started_at` the instance's `snapshot.json` records; macOS reads euid + dir only, a recorded gap.
On a mismatch or a check that cannot be made: CLI verbs report the instance unreachable (exit 21), MCP tools return `instance-unreachable`, `viola hook` fails open (exit 0, nothing on stdout or stderr, no frame written).
Closes the dated exceptions that let these clients write a frame after a liveness-only pre-check (security.md list, amended at the wrap); whether the strict-modes half of each closes here or with "Home and code-bearing file integrity" is a boundary question for P4.
Keeps today's outcome for an unreadable or newer `snapshot.json` (read as absent, exit 21 / fail open) behind a hold for the founder's word; re-reads the five `read_snapshot` callers and the Windows hook-into-exiting-wrapper window (SessionEnd's direct-append fallback skipped) at P3.

## Reached files
- src/cmd/client.rs · edits the liveness-only pre-check before the CLI verbs' frame
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits the `hook.event` and `hook.dialog` client paths and their snapshot reads
- src/cmd/hook/statusline.rs · re-reads its `read_snapshot` call
- src/cmd/run.rs · re-reads `run`'s collision-check `read_snapshot` call
- crates/viola-mcp/src/tools.rs · edits the `send` / `wait` / `last` / `answer` tools' pre-check
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/endpoint.rs
- crates/viola-state/src/liveness.rs · edits `live_endpoint`'s callers' pre-check basis
- crates/viola-state/src/snapshot.rs · re-reads `read_snapshot` and the classified snapshot read
- crates/viola-state/src/strict.rs · re-reads the existing strict-modes checks
- .claude/rules/security.md · edits the Local trust boundary's dated-exceptions list (at the wrap)

## Needs
- The instance snapshot's `pid`, `started_at` (wrapper's own OS start time, sysinfo, whole seconds, RFC 3339) and `endpoint` fields, the values the verification compares against and the endpoint the client connects to
- Session liveness: pid + process start time decide `gone`, a bare pid unsafe because pids are reused; the heartbeat rule the current liveness-only pre-check rests on
- The snapshot envelope's rule for a snapshot whose `v` is newer or that fails to parse (ignored, state rebuilt by replay, the replay lacking `endpoint`), which the held item keeps as absent for these clients
- The snapshot writer rule: only the wrapper writes `snapshot.json`; clients only read it
- The IPC decision: interprocess local sockets behind the `channel` seam, the sync client for CLI and hook and the Tokio client for `mcp`, Windows clients connecting only through the SQOS Identification open
- The IPC endpoint resources: Windows named pipe `viola-<h12>`, Unix socket in a per-user 0700 dir connected through the snapshot's recorded `endpoint`, never an abstract-namespace name
- The endpoint name rule (FNV-1a short hash of name + home, recorded in the snapshot), which the client's endpoint must still match
- Tokio containment: the verification on the `hook` and CLI paths stays std/blocking; only `viola-mcp` builds a runtime
- The CLI exit code 21 (instance unreachable) and the `instance-unreachable` CLI `--json` document as the mismatch outcome of every CLI verb
- The MCP result convention: an unreachable instance is an `isError: true` result shaped like CLI `--json`, code `instance-unreachable`, no frame written
- The hook contract: hooks fail open, exit 0 with no body, never stderr, exit 2 forbidden; a verification failure in `hook.dialog` leaves the dialog to the human
- The hook transport tiers and SessionEnd's 1.5 s budget with its direct-append fallback, which a refused or failed `hook.event` must still reach (the Windows exiting-wrapper CARRY)
- The channel methods that carry frames (`send`, `wait`, `last`, `answer`, `pause`, `release`, `hook.dialog`, and the id-less `hook.event` notification): the check precedes the first frame of each, `link` / `unlink` not served
- `release` carrying `from` refused `-32602`: unchanged behind the new check, the check adding no new refusal ahead of it on the wrapper
- The viola home and instance resolution the clients use (`--home`, `VIOLA_NAME` + `VIOLA_DIR`, the default home) and the `instances/<name>/snapshot.json` path the check reads
- The error handling rule: fixed-message thiserror variants per crate, anyhow only in the root bin; no path, pid or upstream text in a refusal or MCP `isError` body
- Module boundaries: the server-identity read lives in `viola-channel` / `viola-state`, not in `viola-agent-claude`; crate dependency direction unchanged
- The stack's sysinfo use for process start time, and `cfg` branches per OS (Unix peer creds, Windows pipe server pid, macOS euid + dir gap) each compiled and tested on its runner
- Diagnostic output channels: `hook` writes nothing to stdout or stderr on mismatch; the CLI verbs and `mcp` log the refusal only to their role files, codes only
- The strict-modes check (`viola_state::strict`) and its home-mode ordering rule (check before any log opens), relevant only if P4 takes the strict-modes half here
- `revive`'s and `run`'s own snapshot reads (collision check, `already-live`) as possible clients to classify at P3, writing no frame today

## Standing conventions
- S1 · Each new test runs in its own viola home under `target/e2e-home/viola-test-*/home`, against the fake agent, never the real `claude` in CI · occupied-resources.md
- S2 · A new channel test endpoint takes a test-only name `viola-test-chan-<pid>-<label>` outside the `viola-<h12>` namespace · occupied-resources.md
- S3 · Every OS-specific branch compiles and is tested on its CI runner (windows-2025, macos-latest, ubuntu-latest) · body §Cross-cutting Patterns
- S4 · A new log line goes through `obs_event!` to the role's home-level file, codes only, content only in instance detail files · body §Cross-cutting Patterns
- S5 · A new error variant is a fixed-message thiserror variant of the owning crate's enum, no path or payload · conventions.md
- S6 · A new snapshot or frame field is additive with `v` kept, readers tolerant of unknown fields · standard-contracts.md

## Not needed
- The `list` payload and the board read: `list` is a caller-side disk read and writes no channel frame
- The ledger stamps envelope, the capability ledger and `viola verify`'s probes: no CLI behaviour is newly relied on
- The budget governor and `budget.json`: the gate's reading is not touched by client verification
- The hook statusline contract and its `/bin/sh -c` shell-out: the statusline arm writes no channel frame
- The PTY seam, the ConPTY sideload and the vendored ConPTY binaries
- The screen model and delivery confirmation: wrapper-side `send` behaviour is unchanged
- The wheel, pause/release semantics and their `ok` payloads beyond the client's pre-check
- Links and the explicit `link` / `unlink` verbs: held, not served
- Event `data` per kind and the normalised event kinds: no event is added or changed
- Pinned binary hashing, plugin rewrite and `marketplace/`: owned by "Home and code-bearing file integrity" and `plugin install`
- The Windows listener DACL, remote rejection and first-instance: landed with 2026-10-25-windows-endpoint-admission
- Unix socket dir and listener hardening: owned by "Unix endpoint and home hardening"
- Config management, `config.json` keys and the terminal colour env vars
- Fixtures, fuzz targets and the repository's `target/` tool dirs other than test homes
