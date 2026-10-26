# security · Threat Model Summary — category read

## Carried
- (.andromeda/security-plan-detail/threat-model-summary.md:20) The snapshot's `endpoint` is the integrity-sensitive value every frame client connects to, so the verification must bind the connected server to the same snapshot that named the endpoint.
  > `snapshot.json` also stores `endpoint`, the socket path clients connect to, and `pinned_bin`.
- (.andromeda/security-plan-detail/threat-model-summary.md:32) The snapshot's `pid` and `started_at` are the recorded values the connected server's process id and start time must match.
  > Snapshots hold `pid`, `child_pid`, `agent_session_id`, `started_at` and the optional `cwd`
- (.andromeda/security-plan-detail/threat-model-summary.md:43) The check must hold on both endpoint forms, the Windows named pipe and the Unix socket file.
  > One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows, socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) on Unix.
- (.andromeda/security-plan-detail/threat-model-summary.md:44) Every frame-writing method of the channel is in scope for the client-side verification; `link` / `unlink` are not served and stay out (scope boundary).
  > Methods: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, and `hook.event` notifications.
- (.andromeda/security-plan-detail/threat-model-summary.md:45) The impact a wrong server inherits (typed prompts, `allow` decisions, the whole conversation) is why no frame may reach an unverified server.
  > Whoever can connect can type prompts into the driven Claude session (`send`) and allow PermissionRequest dialogs (`answer` with `behavior:"allow"`). That amounts to running arbitrary tools, and so arbitrary code, as the user. They can also read the whole conversation (`wait`, `last`).
- (.andromeda/security-plan-detail/threat-model-summary.md:48) The verification serves the same-OS-user boundary and no other principal.
  > v1 intends to trust every process of the same OS user and no other.
- (.andromeda/security-plan-detail/threat-model-summary.md:49) `from` must not become part of the server or client identity the check relies on.
  > There is no per-connection authentication. The `from` parameter is self-reported by the caller (the CLI and MCP take it from their own `VIOLA_NAME`), so link records and driver identity are not authenticated.
- (.andromeda/security-plan-detail/threat-model-summary.md:50) The endpoint name is not a secret, so the check must rest on the server's process identity, never on knowing the name.
  > The endpoint name is predictable: FNV-1a over `ViolaName` + home path, which is not secret (Conventions: Endpoint name).
- (.andromeda/security-plan-detail/threat-model-summary.md:52) The squatting consequences on each OS are the threat the client check closes.
  > On Unix with `TMPDIR` unset, the socket sits in world-writable `/tmp`, so another local user can create the path first.
- (.andromeda/security-plan-detail/threat-model-summary.md:53) On Windows the shared pipe namespace means the client must verify the serving process, not the name.
  > On Windows, the named-pipe namespace is shared across the machine.
- (.andromeda/security-plan-detail/threat-model-summary.md:54) This is the exact impersonation threat the chunk closes for `hook`, `send` and `mcp`: a client must stop acting on whatever server answers at the recorded endpoint, `hook.dialog` included.
  > Clients (`hook`, `send`, `mcp`) connect to the `endpoint` recorded in the snapshot and act on whatever server answers there. A process holding that name while the real wrapper is gone could receive `hook.dialog` payloads and send back dialog decisions.
- (.andromeda/security-plan-detail/threat-model-summary.md:12) `hook.dialog` carries tool `input` and dialog text to the server, so a mismatched server must receive no `hook.dialog` frame.
  > All of it is logged to `events.ndjson` and sent over the local-socket channel in `hook.dialog` (Standard Contracts: Event `data` per kind, `hook.dialog`).
- (.andromeda/security-plan-detail/threat-model-summary.md:84) A failed or mismatched verification in `viola hook` must fail open to the human.
  > Any failure fails open to the human: exit 0 with no body.
- (.andromeda/security-plan-detail/threat-model-summary.md:92) The MCP frame tools are clients in scope; `list` writes no frame to a wrapper.
  > Tools `send`, `wait`, `last`, `answer` and `list`, called by the driver session's model (Established Decisions: MCP).
- (.andromeda/security-plan-detail/threat-model-summary.md:106) The snapshot (and so the endpoint, pid and start time) the verification reads is the one the resolved home chooses; the check must read it from that home.
  > `--home <dir>` and `VIOLA_DIR` choose which state tree, snapshot, `endpoint`, `statusline_command` and ledger stamps are used (Cross-cutting Patterns: Config management).
- (.andromeda/security-plan-detail/threat-model-summary.md:117) `snapshot.json` and `heartbeat` are cross-process readers the clients' pre-check reads.
  > Readers across processes: `snapshot.json`, `events.ndjson`, `budget.json`, `config.json`, `ledger/stamps.json`, `heartbeat`.
- (.andromeda/security-plan-detail/threat-model-summary.md:122) The log replay has exactly two product readers; moving a frame client onto the replay (CARRY 7 / 8) would add a third and is held for the founder's word.
  > The replay of the log for a snapshot that fails to parse, or carries a newer `v`, has two product readers, `viola revive`'s preflight and `viola list` (`viola_state::board::read`, chunk 2026-10-11-the-board-viola-list), each taking it after the strict-modes check on the instance's files; no other product reader takes it (Standard Contracts: Snapshot envelope).
- (.andromeda/security-plan-detail/threat-model-summary.md:150) Any verification code added to the sync clients must keep the sync crates' tokio ban and `viola-channel`'s tokio-free normal graph green.
  > `cargo deny check` over `deny.toml` (advisories, licences, sources, C-crate / telemetry-crate / feature bans), the sole-root `deny-sync.toml` tokio ban per sync crate and `viola-channel`'s `cargo tree` form (`scripts/channel-tokio-free.sh`; its Tokio client behind the `tokio` feature), `Cargo.lock`, and `publish = false`.
- (.andromeda/security-plan.md:23) The tier's local-boundary elevation names server impersonation as an item the chunk serves.
  > On top of it sit specific local-boundary items: IPC endpoint access control and server impersonation, bracketed-paste breakout in `send.text`, and integrity of `~/.viola/`.
- (.andromeda/security-plan.md:100) The libraries the check uses are the stack's own; no new dependency is needed.
  > IPC: interprocess 2.4.4 (`ListenerOptionsExt::security_descriptor`, `Stream::peer_creds()`, `PipeStream::server_process_id()`), windows-sys 0.61.2 (`GetTokenInformation(TokenUser)`, `ConvertSidToStringSidW`, the SDDL-to-descriptor conversion under feature `Win32_Security_Authorization`, `CreateFileW` with SQOS flags), std chmod on the bound Unix socket, and sysinfo 0.39.6 (pid + start time).
- (.andromeda/security-plan.md:110) On a failed check, clients report `instance-unreachable` and `hook` fails open.
  > a failed check refuses `run`, reports `instance-unreachable` to clients and fails `hook` open
- (.andromeda/security-plan.md:114) The per-OS shape of the verification the chunk builds, with no fallback to a default connect.
  > IPC client-side server verification (anti-impersonation) — before the first frame of any method, the client verifies the server against the snapshot (Windows: an SQOS identification-level open, then the serving pid and start time; Unix: euid, and pid with start time where available; macOS: euid and the verified socket directory only, a known gap), never falling back to a default connect; the dated interim gaps stand until their two route entries land.
- (.andromeda/security-plan.md:154) Own-state reads stand behind the home strict-modes check except the ratified dated gaps; the strict-modes half belongs to "Home and code-bearing file integrity".
  > read only after the home strict-modes check, except the ratified dated gaps
- (.andromeda/security-plan.md:410) The MCP tools' mismatch result is `instance-unreachable` with `detail: null`.
  > `instance-unreachable` with `detail: null`
- (.andromeda/security-plan.md:415) A verification failure in `hook` keeps exit 0, no body and empty stderr.
  > `hook` never writes to stderr and never exits non-zero: a security check failing in `hook` still ends in exit 0 with no body, so the dialog renders for the human
- (.andromeda/security-plan.md:439) The ban the chunk discharges for its half: no frame before verification, the dated gaps lasting until both Epoch 6 entries land.
  > NEVER write any channel frame before verifying the server against a `snapshot.json` that passed the strict-modes check in the same process (the euid check never skipped), except the dated gaps (`hook.event`; the CLI `send`, `wait` / `last`, `answer`, `pause` / `release` frames; `hook.dialog`; `viola mcp`'s tool frames) until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" land; no other frame or process may borrow them.
- (.andromeda/security-plan.md:441) The Windows client open keeps its SQOS flags under the new check.
  > NEVER open a Windows client pipe without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`
- (.andromeda/security-plan.md:507) A verification refusal goes only to automation; the human is never blocked.
  > NEVER let a security refusal block the human. Refusals go only to automation (`send`/`answer`), and hooks fail open.
- (.andromeda/security-plan.md:516) No config key, env var or flag may switch the server verification off.
  > NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan (the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check, `validate_paste_text`): environment variables are not a configuration channel.

## Bindings
- **Impersonation threat ↔ Authentication & Authorization (body + its detail file)**: the threat named at (.andromeda/security-plan-detail/threat-model-summary.md:54) is answered by the client-side server verification row (.andromeda/security-plan.md:114) and its Unix failure outcome (.andromeda/security-plan.md:110); the dated gaps' closure is bounded by (.andromeda/security-plan.md:439).
- **Recorded `pid` / `started_at` ↔ architecture Standard Contracts (Instance snapshot)**: the values matched are the snapshot's own fields (.andromeda/security-plan-detail/threat-model-summary.md:32); the snapshot is written only by the instance's wrapper (.andromeda/security-plan.md:514).
- **Replay readers ↔ scope CARRY 7 / 8 hold**: the two-reader rule (.andromeda/security-plan-detail/threat-model-summary.md:122) is what a client moved onto the replay would widen; it is held for the founder's live word.
- **Strict-modes half ↔ "Home and code-bearing file integrity"**: the snapshot read the check stands on is gated by strict-modes only after that entry, except the dated gaps (.andromeda/security-plan.md:154), (.andromeda/security-plan.md:439).
- **Hook fail-open ↔ Error Handling / Universal bans**: (.andromeda/security-plan-detail/threat-model-summary.md:84), (.andromeda/security-plan.md:415), (.andromeda/security-plan.md:507).
- **MCP result ↔ Error Handling (MCP `isError`)**: `instance-unreachable` with `detail: null` (.andromeda/security-plan.md:410).
- **Sync-crate tokio ban ↔ Dependency Security / CI supply-chain job**: (.andromeda/security-plan-detail/threat-model-summary.md:150).
