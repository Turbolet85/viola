# Task — security · 2026-10-26-server-verification-before-any-frame

## The chunk
Builds client-side server verification before the first frame in every client that writes a frame to a wrapper's endpoint: CLI `viola send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s `hook.event` (every event) and `hook.dialog` (`pre-tool-use` · `permission-request`); `viola mcp`'s `send` / `wait` / `last` / `answer` tools through the Tokio client.
The check: the connected server's process id and start time (Unix `peer_creds` on the socket, Windows the pipe's server process id, start time from sysinfo; macOS euid + verified socket dir only, a recorded gap) match the `pid` and `started_at` the instance's `snapshot.json` records.
On a mismatch or a check that cannot be made: CLI verbs report the instance unreachable (exit 21), MCP tools return `instance-unreachable`, `viola hook` fails open (exit 0, nothing on stdout or stderr, no frame written).
It closes the server-identity half of the dated exceptions one, four, five, six, seven and eight (removed from the list at the wrap, a master amendment); whether the strict-modes half closes here or stays with "Home and code-bearing file integrity" is a P4 question.
It keeps today's outcome for an unreadable or newer `snapshot.json` (read as absent, exit 21 / fail open) behind a stop rule for the founder's live word, and re-reads the existing `viola_state::strict` checks and the Windows exiting-wrapper pipe window (a `hook.event` delivered to an exiting wrapper, skipping `SessionEnd`'s direct-append fallback).

## Reached files
- src/cmd/client.rs · edits the liveness-only pre-check before the first frame (its snapshot read)
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits the `hook.event` and `hook.dialog` sends (its two snapshot reads)
- src/cmd/hook/statusline.rs
- src/cmd/run.rs · edits run's collision check (its snapshot read)
- crates/viola-mcp/src/tools.rs · edits the `send` / `wait` / `last` / `answer` tools
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/endpoint.rs
- crates/viola-state/src/liveness.rs · edits `live_endpoint`
- crates/viola-state/src/snapshot.rs
- crates/viola-state/src/strict.rs · re-reads the existing `viola_state::strict` checks

## Needs
- The IPC client-side server verification (anti-impersonation) row: before the first frame of any method the client verifies the server against the snapshot (Windows the SQOS identification-level open then the serving pid and start time; Unix euid, and pid with start time where available; macOS euid and the verified socket directory only, a known gap), never falling back to a default connect.
- The Authentication anti-pattern banning any channel frame before server verification against a snapshot that passed strict-modes in the same process, the euid check never skipped, and its list of dated gaps (`hook.event`; CLI `send`, `wait` / `last`, `answer`, `pause` / `release`; `hook.dialog`; `viola mcp`'s tool frames) that this chunk's entry closes together with "Home and code-bearing file integrity".
- The strict-modes half of that clause: the snapshot the verification reads must have passed strict-modes in the same process; whether this chunk takes the `viola_state::strict` check with it or stands on a snapshot read without one is the boundary question for P4.
- The interim-gap wording in the auth body (the dated interim gaps stand until their two route entries land) and in the `~/.viola/` access-control row (strict-modes in every process that reads a trusted file, with its dated interim gaps), which this chunk's closure changes.
- The Input Validation rows that carry the same carve-out: CLI arguments (home canonicalised and strict-modes-checked before any snapshot read, except the ratified dated gaps) and own state files on read (`snapshot.json` read only after the home strict-modes check, except the ratified dated gaps; a reader never writes).
- The local IPC threat (whoever connects types prompts and allows dialogs) and its server-impersonation elevation, the threat model's boundary-ipc group, which this check answers.
- The hook boundary: `viola hook` never writes stderr and never exits non-zero, a security check failing in `hook` still ending in exit 0 with no body (Error Handling, the Logging anti-patterns, the Universal ban on a refusal blocking the human), now applied to a server mismatch for both `hook.event` and `hook.dialog`.
- The CLI verbs' unreachable report on a mismatch or an unmakeable check: exit 21 as the liveness pre-check uses today, and a `--json` error body of codes and fixed messages only, no absolute path, pid or `started_at`.
- The MCP `isError` shape for the four frame tools: `error: instance-unreachable` with `detail: null`, text `refused: instance-unreachable`; any other handler fault `-32603` `internal error` with `data: null`.
- The operational-metadata floor reached through the mismatch report: pids, `agent_session_id` and `started_at` may go to `diagnostics/` only, never to CLI `--json` error output, MCP `isError` results or channel `error.data`.
- The libraries the verification uses as the auth body names them: interprocess 2.4.4 `Stream::peer_creds()` and `PipeStream::server_process_id()`, sysinfo 0.39.6 for pid + start time; no new dependency is named.
- The Windows client open: never a pipe opened without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`, the SQOS open that already stands and that the verification sits behind.
- The `snapshot.json` single-writer rule (only the instance's own wrapper writes it; its `endpoint` decides where clients connect): the verification trusts the recorded `pid` and `started_at` on that basis, and no client gains a write.
- The hold on unreadable or newer snapshots: today `read_snapshot` reads either as absent, `live_endpoint` finds no endpoint and the verb exits 21 or the hook fails open; moving a client to the classified read or the log replay (the five snapshot readers named in the scope) would connect on a new basis, a boundary widening held for the founder's live word.
- The no-widening rule: a client that would write a frame on a new basis, or a reader added off the trusted-file list, halts for the founder's word.
- The Universal ban on `config.json`, a `VIOLA_*` variable or a CLI flag switching off the peer or server verification or the strict-modes check: the new check takes no switch.
- The Tokio boundary reached through `viola-channel`'s Tokio client: Tokio only behind `viola-channel`'s `tokio` feature and in `viola-mcp`, the sync crates' `deny-sync.toml` ban and `scripts/channel-tokio-free.sh` staying green.
- The fixed-message `Display` rule for `ChannelError` / `StateError` / `McpError`: any variant this chunk adds for a mismatch or an unmakeable check interpolates no path, pid or payload.
- The hook-stdin boundary's fail-open floor (every failure exit 0, no body) and the `hook.dialog` payload shape check that the frame now also waits behind the server check for.
- The Windows exiting-wrapper window (CARRY 2): a `hook.event` reaching an exiting wrapper's pipe reads as delivered so `SessionEnd`'s direct-append fallback to `events.ndjson` is skipped; whether a pid + start-time check changes it is P3's to read and P4's to plan or name as unclosed.
- The bootstrap phase auth-scaffolding-baseline, which places client server verification in `viola-channel` and the strict-modes checks in `viola-state`.
- The `from` param staying self-reported and never an identity: the server check is the identity, the `release` affordance guard unchanged.

## Standing conventions
- S1 · every new test runs under its own `--home`, and no local `--home` test directory is committed · Secret Management (Development) in the body; security-anti-patterns.md (Secrets)
- S2 · every new log line keeps the NEVER-log floor: no R8-stripped `CLAUDE*` value, no user content, no absolute path outside `diagnostics/`, pids and `started_at` only in `diagnostics/` · bootstrap-phases.md (`logging-redaction-wire`)
- S3 · every new error variant has a fixed `Display` message and every external error carries codes only · error-handling.md
- S4 · a test-side removal under `target/e2e-home` takes only a dir the test side itself created · security-anti-patterns.md (Code Patterns)
- S5 · no new environment read or test seam (a forced-mismatch seam included) without its own Decisions Log entry; the two existing `fake-agent` seams stay compiled out of release · input-validation.md; security-anti-patterns.md (Universal)
- S6 · every CI upload a new test adds waits on the artifact canary scan · Secret Management (Secret scanning in CI) in the body; bootstrap-phases.md (`secret-scanning-ci-gate`)

## Not needed
- Paste-text and dialog free-text validation (`validate_paste_text`, `control-character`, `empty-text`): no text path changes.
- Windows listener hardening (the protected DACL, remote rejection, first-instance, non-inheritable handle): landed with 2026-10-25-windows-endpoint-admission.
- Unix socket hardening (the 0700 per-user directory check before bind and connect, the post-bind chmod 0600, `try_overwrite` off, no abstract namespace) and the server-side peer euid check on accept: owned by "Unix endpoint and home hardening".
- `link` / `unlink`: "Explicit session links" stays held; nothing here serves them.
- Pinned-binary re-hash, plugin rewrite, `marketplace/` integrity and the exec-form `command` paths: owned by "Home and code-bearing file integrity".
- The statusline shell-out (`/bin/sh -c`), the statusline source read and the `budget.json` write: unchanged.
- The budget gate, `config.json` thresholds and the dialog hold: unchanged.
- The `unverified-cli` gate, `ledger/stamps.json` and its writer `viola verify`: unchanged.
- `viola verify`'s PTY children, Runs A–E, the `--record` scrub and the repository fixtures.
- `viola revive`'s preflight, the `claude agents --json` read and `viola list`'s board rendering and row escaping.
- The Windows DLL search restriction and the ConPTY companion sideload.
- The R8 strip and secret storage: no new secret and no environment change.
- The supply-chain gates (cargo-deny advisories and licences, zizmor, SHA-pinned actions, `release-check.sh`, `conpty-vendor.sh`, the `fuzz/` exemption) and rmcp's feature set: no dependency or workflow is named.
- vt100 feed, PTY output and the human's stdin classifier.
- `events.ndjson` retention and the data lifecycle.
