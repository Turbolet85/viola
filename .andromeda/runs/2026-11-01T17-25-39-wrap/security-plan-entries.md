## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory check, built at the bind and the client seams
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC access control (Unix), its stack reference and its new rows the directory verdict, a refused directory, the recorded endpoint; IPC client-side server verification, where the checks sit) · §Bootstrap phases (bootstrap-phases.md: the `viola-channel` and `viola-state` rows) · §Error Handling (error-handling.md: `Refused::NotADirectory`)
**Change:**
- `viola_state::strict::check_socket_dir` makes one `symlink_metadata` read: an absent path passes, an unstattable one is `Refused::Unreadable`; the plain `socket_dir_verdict` then reads a symlink as `Refused::Symlink`, a non-directory as `Refused::NotADirectory`, anything else through `unix_read_verdict`. Off Unix it returns `Ok(())`. It never chmods: a refused directory stays as found.
- `check_endpoint_dir` is the check on a recorded endpoint's parent, the one helper the bind and the three client seams share; the stack reference names its callers, `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs` and `crates/viola-mcp/src/tools.rs`.
- No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, the checks sit at the seams, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design.
- `Refused::NotADirectory` carries the fixed `Display` message `a trusted state path is not a directory`.
- Bootstrap phases: the directory verification was left with its route entry; now it is `check_socket_dir` and `check_endpoint_dir` in `viola-state`, called at the bind and the three client seams. The `viola-state` row gains both, and the `NotADirectory` refusal beside `Readable` and `Symlink`.
**Why:** The directory check the route entry "Unix endpoint and home hardening" owed is built. Three client seams and the bind each need the recorded endpoint's parent, so one helper holds it, a deviation the chunk recorded.
**Kept:** The `thiserror` derives stay nine: `Refused` gained a variant, not a derive.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `socket-dir-refused`, the sixth cause and the eleventh code
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC access control (Unix); the CLI refusal; the MCP refusal) · §Error Handling (error-handling.md: exit 21's causes, `UnreachableCause`, the MCP frame tool, what the refusal names; security-plan.md: MCP `isError` results) · §Input Validation (input-validation.md: Frame verbs, `revive --json` codes) · §Threat Model Summary (threat-model-summary.md: the cause list, the MCP trust boundary)
**Change:**
- Exit 21's closed causes: was five; now six, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
- `viola revive --json`'s closed exit-1 codes: was ten; now eleven, `socket-dir-refused` after `squatted-name`, `internal-error` last.
- CLI frame verbs: past `live_endpoint`, `check_endpoint_dir` runs on that endpoint's parent before the connect. A refusal writes no frame and exits 21: under `--json` `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty; in human mode the `unable` line with the hint of cause `socket-dir` last on stderr. It is `CallError::SocketDirRefused` to `Unreachable::SocketDir`: `during` `connect`, one `process-exit` at WARN, no `channel-request`.
- `viola mcp`'s `frame`: `check_endpoint_dir` after `live_endpoint`; a refusal returns `instance-unreachable` with `detail` `socket-dir-refused`, no frame and no line, `isError` the only record.
- `viola_core::UnreachableCause::SocketDir` has no `RefusalReason`. The refusal names only the parsed `ViolaName`, and its lines hold no path, pid or uid.
**Why:** A refused socket directory carries a new closed word: the founder's own answer `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after this widening was shown to him. The rows reading inside it passed the wrap's halt on the overseer's confirm as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary; the ratification stays his answer.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory at a hook and at a start
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC access control (Unix), its bind clause; the new row at a start; the Collision check's site) · §Error Handling (error-handling.md: the hook's fail-open row, the new hook and start rows)
**Change:**
- A hook on a refused directory: was exit 0, no body, a note in `diagnostics/`; now exit 0, empty streams, no frame, one `hook-decision` of `detail` `socket-dir-refused` at WARN (`NotSent::SocketDir`, `Asked::SocketDir`; on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`). The fail-open row gains the refused socket directory, the word and, on Unix, the socket-directory and peer euid checks.
- A failure at the bind: was an anyhow error at the catch site, its code owed to the route entry "Unix endpoint and home hardening"; now a failed create and a refused check read one code: `run` and the start of `revive` exit 1 with `socket-dir-refused` and one `process-exit` of that `detail`, through `refused_with` and `refuse_socket_dir`, a step with no span. A `Server::bind` failure is outside this code.
- New row, at a start: `socket_dir_ready` runs `check_endpoint_dir`, creates the parent only when absent, then checks again, so a race's loser reads the same; it runs before any snapshot, child or `Server::bind`, and the start of `revive` shares it (`src/cmd/run.rs`, `enum Bound`).
- The Collision check's site: was `src/cmd/run.rs:438`; now `src/cmd/run.rs:448`, where `collision_check` stands; its rule did not move.
**Why:** The word is the founder's own answer `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after this widening was shown to him. A note in `diagnostics/` was disproved: the chunk writes one `hook-decision`, the form the server check took. That form and the one code are leans the overseer's P5 word took as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a killed wrapper's leftover socket file
**Section:** §Authentication & Authorization (authentication-authorization.md: the new row IPC access control (Unix), a leftover socket file)
**Change:** New row: a killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it, by choice, a harmless file in a 0700 directory. Its stack reference is `crates/viola-e2e/src/harness/cleanup.rs` (`unconnectable`, which removes no leftover file).
**Why:** The choice is recorded with no owner proposed, a row the overseer's P5 word took as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the wrapper's drop of a peer of another euid, on the one sync server
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC server-side peer check (Unix), its stack reference and the new row the drop) · §Bootstrap phases (bootstrap-phases.md: the `viola-channel` row) · §Error Handling (error-handling.md: the new row on `accept_loop`)
**Change:**
- The peer check's server: was `viola-channel` sync + Tokio server; now the sync server (`crates/viola-channel/src/server.rs`, `accept_loop`), the one server, `TokioClient` a client.
- New row, the drop: `accept_loop` filters `incoming()` by the peer's euid decision, `admitted(peer_euid_of(stream))`, on the accept thread, before the stream's worker thread and before any frame. The peer reads end-of-file; the wrapper writes no reply, no line and nothing on the terminal, there being no frame to key a `channel-response` on (`serve_admitting`, the decision `peer_admitted`).
- Bootstrap phases: the `peer_creds` decision was left with its route entry; now it is `peer_admitted`, the wrapper dropping a refused peer in `accept_loop` before any frame.
**Why:** The route entry's owed decision is built. The sync + Tokio claim was disproved: one sync server is what the code holds. That reading and the drop's no line are leans the overseer's P5 word took as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the client's peer euid check and the order at a seam
**Section:** §Authentication & Authorization (authentication-authorization.md: IPC client-side server verification, its stack reference and its rows the order at a seam, the observed peer euid, `viola link` / `viola unlink`, the CLI refusal, the MCP refusal; security-plan.md: Library, the client-side clause) · §Bootstrap phases (bootstrap-phases.md: the client row) · §Error Handling (error-handling.md) · §Input Validation (input-validation.md: Frame verbs) · §Security Anti-Patterns (security-anti-patterns.md: the frame ban) · §Threat Model Summary (threat-model-summary.md: the as-built row, the MCP trust boundary)
**Change:**
- The client euid check: was not built, owed to the route entry "Unix endpoint and home hardening"; now checked at `cmd::client::connect`, the `connect` of `cmd::hook` and the `frame` of `viola-mcp`, through `peer_admitted(client.server_euid())`. A peer whose euid cannot be read is refused.
- New row, the read: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`; `PEER_EUID_REPORTED` is `cfg!(unix)`. `Client::server_euid` and `TokioClient::server_euid` read `peer_creds().ok()?.euid()` (`peer_euid_of`), writing and logging nothing. On Windows no euid is read, the DACL and the pid comparison staying the controls.
- New row, the order on every client seam: the directory check, the connect, the peer euid decision, then the server verification against the snapshot. The `link` / `unlink`, Frame verbs, bootstrap client, frame ban and threat-model rows read this order.
- A refused euid has no word of its own: it reads `server-verify-failed` as `CallError::ServerUnverified` in `src/cmd/client.rs`, `NotSent::ServerUnverified` in `src/cmd/hook.rs` and `UnreachableCause::ServerVerify` in `crates/viola-mcp/src/tools.rs`, the same document and hint as a mismatch at the CLI, no frame and no line at MCP.
- security-plan.md's Library names `euid()`, the Unix read of the peer on both sides, none on Windows.
**Why:** A client meeting a server of another euid reads `server-verify-failed`: the founder's own answer `inputs#I2` Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after this widening was shown to him. The rows reading inside it passed the wrap's halt on the overseer's confirm as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary; the ratification stays his answer.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the board's one rule on the home
**Section:** §Authentication & Authorization (authentication-authorization.md: the Strict-modes check's `list` clause; the new row `~/.viola/` access control, the board) · §Input Validation (input-validation.md: Home resolution) · §Threat Model Summary (threat-model-summary.md: the CLI trust boundary)
**Change:**
- `viola list`'s `board::scan`: was `check_path` on the home and on `instances/`; now `check_read` on the home, on every OS, and `check_path` on `instances/`, the first failure refusing the whole board (`ScanRefused::Home` on the home) before anything under the home is opened (`crates/viola-state/src/board.rs`: `scan`, `check_if_present`).
- A 0750 home: was passing a board with no instance and refused as `ScanRefused::Instance` once it held one, a CARRY owed to the route entry "Unix endpoint and home hardening"; now a home another user may read (a 0750 home on Unix) is refused as `ScanRefused::Home` with or without an instance.
- Its proof: was `cfg(unix)` cases only; now all three OSes.
- The new row carries the one gap marker: OPEN, the board's read rule on every OS stands on a delegate's answer, owner the Epoch 7 boundary batch.
**Why:** The overseer's technical answer as the founder's delegate, `inputs#I2` Q3, PROVISIONAL: no widening, a refusal tightened, the founder's own word owed in the batch at Epoch 7's boundary.
**Kept:** `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home, a lean the overseer's P5 word took as delegate, PROVISIONAL (`inputs#I3`). The scan's check order and no-open stand. input-validation.md and threat-model-summary.md carry no status of the rule: the one marker stands in authentication-authorization.md.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — macOS's absent pid comparison, a named carve-out
**Section:** §Authentication & Authorization (authentication-authorization.md: the `viola link` / `viola unlink` row, the macOS cases, the new row the macOS carve-out; security-plan.md: the client-side clause) · §Error Handling (error-handling.md: the MCP frame tool) · §Security Anti-Patterns (security-anti-patterns.md: the frame ban; security-plan.md: the ban's exception) · §Threat Model Summary (threat-model-summary.md: the as-built row, the cause list)
**Change:**
- The ban's exception: was the dated gaps, every client's euid check and macOS's pid and start-time check, owed to "Unix endpoint and home hardening"; now the named carve-out alone, macOS's pid and start-time check, owned by no route entry, its euid and verified socket-directory checks standing; no other frame or process may borrow it. The `viola link` / `viola unlink` row and the threat model's as-built row: was the recorded gap; now that carve-out.
- `server-verify-failed`'s reach: was off macOS; now Linux, Windows and macOS, on macOS an euid mismatch alone, the whole of the check at MCP's frame tool.
- The macOS cases: was a CARRY on the route entry; now on `macos-latest` a socket directory at 0755 or a symlink to a 0700 one gives exit 21 on a frame verb with `detail` `socket-dir-refused`, and the client euid read holds there.
- New row, the carve-out: `server_verdict_rows` keeps its row and the server-identity negatives stay compiled out on macOS (`#![cfg(not(target_os = "macos"))]`).
**Why:** On macOS the mismatch means an euid mismatch alone, the pid and start-time comparison staying absent, the named carve-out: the founder's own answer `inputs#I2` Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after this widening was shown to him. The route entry's owed clauses are discharged, the pid gap staying named but owned by no route entry.
**Kept:** The rows stating macOS compares no peer pid stand as true claims, their recorded-gap wording kept (the identity check's known gap for the public version, the frame ban's recorded known gap, the `viola hook` clauses).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the tests holding the directory and peer checks, and no seam to drive them
**Section:** §Authentication & Authorization (authentication-authorization.md: the new rows the tests holding the directory check, what no test drives; `viola link` / `viola unlink`, the tests holding their controls) · §Security Anti-Patterns (security-anti-patterns.md: Universal)
**Change:**
- `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on `ubuntu-latest` and `macos-latest`, none on Windows. `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` reads the eight frame verbs, `link` and `unlink` among them, refused with no frame.
- A foreign euid and a foreign owner are driven by no test: those decisions are held by injected values (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, `serve_drops_a_refused_peer_before_any_frame`).
- New ban: NEVER add an env, flag or feature seam to force a refusing peer-euid decision. The decisions are proven over injected values, `peer_euid_admitted(observed, own, reported)` and the private `serve_admitting(dispatch, admitted)` over an injected `Admission`; each client seam's `peer_admitted` wiring (`cmd::client::connect`, `cmd::hook::connect`, `viola-mcp`'s `frame`) stays the named weakest proof, read only through a run's forced refusal, which no committed test repeats.
**Why:** One account can make no foreign peer. Each seam's wiring was read once, through a forced refusing decision in this chunk's remove-the-guard runs; a mutant replacing that read with an always-admit survives every committed test by construction.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the Unix socket's place in the threat model
**Section:** §Threat Model Summary (threat-model-summary.md: the entry point, the consequences; security-plan.md: the tier justification, Networking)
**Change:**
- The socket file: was `$TMPDIR/viola-<h12>.sock` (else `/tmp`); now `<dir>/viola-<h12>.sock`, `<dir>` the per-user 0700 directory.
- The consequence: was, with `TMPDIR` unset, a socket in world-writable `/tmp` whose path another local user can create first; now the socket sits in the per-user 0700 directory, and a directory another user owns, a loosened one or a symlink is refused by the socket-directory check, with no bind on `run` and no frame on a client.
- The tier justification: was predictable names in shared namespaces (Windows pipes, `/tmp`); now in the shared Windows pipe namespace, the Unix socket standing in a per-user 0700 directory.
- Networking: was Unix domain sockets in `$TMPDIR` or `/tmp`; now in a verified per-user 0700 directory, never the `/tmp` or `$TMPDIR` root.
**Why:** The placement was disproved: the endpoint has stood in a per-user 0700 directory since chunk 2026-09-27-wrapper-channel.
**Kept:** The section's label as a verbatim copy of threat-assessment.md stands above the edited justification; this diff holds no edit of it.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/
