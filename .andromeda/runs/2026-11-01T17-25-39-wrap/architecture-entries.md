## 2026-11-01-unix-endpoint-and-home-hardening — `socket-dir-refused`, exit 21's sixth cause and `revive --json`'s eleventh code
**Section:** §Conventions (conventions.md: Unreachable cause, CLI exit codes, MCP results; architecture.md: CLI exit codes); §Established Decisions (established-decisions.md: [CLI Conventions]); §Occupied Resources (occupied-resources.md: Binaries, `diagnostics/`); §Standard Contracts (standard-contracts.md: CLI `--json` output; architecture.md: CLI `--json` output); §Infrastructure Patterns, Project directory structure (project-directory-structure.md: the `viola-core` row)
**Change:**
- `viola_core::UnreachableCause` gains `SocketDir`, its word `socket-dir-refused`, with no serde path and no `RefusalReason`. Exit 21's closed causes were five; now six: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, in the CLI document's `detail` and MCP's `instance-unreachable` result.
- `viola revive --json`'s exit-1 codes were ten; now eleven, `socket-dir-refused` after `squatted-name`, `internal-error` staying last. `refused_with` prints `refused_document` for it too.
- `schemas/diag-line.v1.json`: `socket-dir-refused` is a word of `process-exit`'s `detail` enum and of `hook-decision`'s.
- The master's exit-code line names a refused socket directory under exit 1 (`run`, `viola revive`'s start) and exit 21 (a CLI frame verb, `viola link`, `viola unlink`).
**Why:** A refused socket directory carries a new closed word, no path: the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1). The halt's Confirm that these rows lie inside his answers is the overseer's as the founder's delegate, PROVISIONAL, its standing superseded by the founder's own word in the batch at Epoch 7's boundary; the ratification stays his answers.
**Kept:** The exit set: the word adds no exit code.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the start's refused socket directory, exit 1
**Section:** §Conventions (conventions.md: CLI exit codes); §Established Decisions (established-decisions.md: [Session Liveness]); §Occupied Resources (occupied-resources.md: Binaries, `diagnostics/`)
**Change:**
- Was an absence: the socket-directory refusal had no code, `bind_endpoint`'s `create_private_dir` failure was an anyhow error at the catch site, an OPEN owned by the route entry "Unix endpoint and home hardening". Now `viola run` and `viola revive`'s start refuse through `socket_dir_ready` in `src/cmd/run.rs`, before any snapshot, child or `Server::bind`: exit 1, the stderr `unable:` / `hint:` pair or under `viola revive --json` the `socket-dir-refused` document, and one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`; no path, pid or uid.
- `socket_dir_ready` runs `check_endpoint_dir`, creates the parent only when absent and checks again, so a race's loser reads the same.
- A failed create and a refused check read one code; a `Server::bind` failure does not.
- The start order and the as-landed chain open the endpoint bind with `socket_dir_ready`, a refusal making no bind.
**Why:** The owed code is built, its word the founder's own (`inputs#I2` Q1). The one code for a failed create and a refused check is a lean the P5 word took: the overseer's word as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a client's refused socket directory, CLI, hook and MCP
**Section:** §Conventions (conventions.md: CLI exit codes, MCP results); §Established Decisions (established-decisions.md: [MCP], [Hook Transport], [Hook Contract], [Links], [Session Liveness]); §Occupied Resources (occupied-resources.md: IPC endpoints, Binaries, `diagnostics/`, Filesystem); §Standard Contracts (standard-contracts.md: Channel methods, CLI `--json` output, Session liveness); §Infrastructure Patterns, Project directory structure (project-directory-structure.md: the `client.rs` and `hook.rs` rows)
**Change:**
- A frame-writing client's order was the strict-modes check, the liveness pre-check, the server check; now `check_endpoint_dir` on the recorded endpoint follows the pre-check, ahead of the connect. The seams: `src/cmd/client.rs` `connect` with its 8 callers, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`. The registry's `connect` row now names `link` / `unlink` beside the six frame verbs.
- CLI, the eight verbs: `CallError::SocketDirRefused` to `Unreachable::SocketDir`, exit 21, the `instance-unreachable` document with `detail` `socket-dir-refused` or `send_hint`'s cause `socket-dir`, `hint:` last; one `process-exit` at WARN, `during` `connect`, no `channel-request`. `hint_cause` keys `socket-dir`.
- Hook: `NotSent::SocketDir`, `Asked::SocketDir`: exit 0, empty streams, no frame, one `hook-decision` at WARN with `detail` `socket-dir-refused`, on the dialog path after `hook-invoked{corr:null}` with `decision_emitted:false`. `hook.event` needs this check passed.
- MCP: `frame`, after `live_endpoint`, answers `result::unreachable(UnreachableCause::SocketDir)`, no frame, no line by rule, `isError` the record.
- Filesystem: this check is no part of the instance check's no-line rule; it comes after that check, on the recorded `endpoint`.
**Why:** The check owed to the route entry "Unix endpoint and home hardening" is built at every client, its word the founder's own (`inputs#I2` Q1). The hook's one `hook-decision` line is a lean the P5 word took: the overseer's word as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the peer euid on both sides
**Section:** §Conventions (conventions.md: CLI exit codes, MCP results); §Established Decisions (established-decisions.md: [Message Broker / IPC], [MCP], [Hook Transport], [Hook Contract], [Links], [Session Liveness]); §Occupied Resources (occupied-resources.md: IPC endpoints, Binaries, `diagnostics/`; architecture.md: Linux / macOS endpoint); §Stack and Technologies (stack-and-technologies.md: Wrapper IPC); §Standard Contracts (standard-contracts.md: Channel methods, Session liveness); §Infrastructure Patterns, Project directory structure (project-directory-structure.md: the `viola-channel`, `client.rs` and `hook.rs` rows)
**Change:**
- `viola_channel` exports `peer_euid_admitted`, `peer_admitted` and `PEER_EUID_REPORTED` (`cfg!(unix)`). `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where reported. `Client::server_euid` and `TokioClient::server_euid` read `peer_creds().ok()?.euid()` through `peer_euid_of`, none on Windows, writing and logging nothing.
- The wrapper: `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, dropping a refused stream on the accept thread, before its worker thread and any frame, reading end-of-file, with no line, nothing on the terminal and no wheel moved. `Server::serve` injects `peer_admitted` as a `type Admission` through the private `serve_admitting`, with no env, flag or feature seam.
- Each client reads `peer_admitted(client.server_euid())` after the connect, ahead of the pid comparison. `server-verify-failed` was a mismatched or unverifiable pid or start time; on Unix it now also covers another euid or an unreadable one: `CallError::ServerUnverified`, `UnreachableCause::ServerVerify`, `NotSent::ServerUnverified`, no frame.
**Why:** A client meeting a server of another euid reads `server-verify-failed` on Linux and macOS: the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q2). The owed client euid check is built. The wrapper's drop writing no line is a lean the P5 word took: the overseer's word as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Kept:** Windows reads no euid; its DACL and the pid comparison stay the controls.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — macOS's server check, an euid mismatch alone
**Section:** §Conventions (conventions.md: CLI exit codes); §Established Decisions (established-decisions.md: [Message Broker / IPC]); §Occupied Resources (occupied-resources.md: IPC endpoints); §Standard Contracts (standard-contracts.md: Channel methods)
**Change:**
- `server-verify-failed` was claimed off macOS only, the carve-out left with the route entry "Unix endpoint and home hardening"; now claimed on Linux, Windows and macOS, on macOS meaning an euid mismatch alone.
- The macOS client euid and socket-directory checks and the 0755-directory exit-cause case were owed to that route entry; now they stand built.
- The gap: macOS compares no peer pid, `v1-18`'s named carve-out, was owned by that route entry; now owned by no route entry.
**Why:** The euid reading on macOS is the founder's own word, with the pid and start-time comparison staying absent as the named carve-out: live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q2). The owed clauses are discharged by what the chunk built.
**Kept:** `viola_state::liveness::server_verdict` returns `Matches` with no comparison where no peer pid is reported: a true claim, the recorded gap.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket directory's human pair on the sanitised surface
**Section:** §Cross-cutting Patterns (cross-cutting-patterns.md: Sanitised error surfaces); §Conventions (conventions.md: CLI exit codes)
**Change:**
- New: the `unable:` / `hint:` pair for a socket directory names only the parsed `ViolaName`, no path, pid or uid, is plain ASCII with `hint:` last and no SGR, its hint `send_hint`'s cause `socket-dir` in `src/human/hints.rs`.
- `src/human/hints.rs` holds the socket-directory hint beside the unwrapped-name and endpoint-lost hints; conventions points to design-system, Surface: cli for its wording.
**Why:** The refusal's body stays sanitised. The pair's wording lands as a planning form on the overseer's word as the founder's delegate, PROVISIONAL (`inputs#I3`), the founder's wording owed at Epoch 7's boundary batch.
**Kept:** This body writes no status of the wording; the one gap marker stands in design-system.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory check's read and its home in the code
**Section:** §Established Decisions (established-decisions.md: [Message Broker / IPC]); §Occupied Resources (occupied-resources.md: IPC endpoints, Filesystem; architecture.md: Linux / macOS endpoint); §Stack and Technologies (stack-and-technologies.md: Wrapper IPC); §Infrastructure Patterns, Project directory structure (project-directory-structure.md: the `viola-state` row)
**Change:**
- New: `viola_state::strict::check_socket_dir` takes one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, then `socket_dir_verdict` reads a symlink as `Refused::Symlink`, a non-directory as `Refused::NotADirectory` and anything else by the read rule. Off Unix it returns `Ok(())`. `check_endpoint_dir` is that check on a recorded endpoint's parent.
- It refuses a symlink, a non-directory, a foreign owner or any group or other bit, never chmods, and a refused directory stays as found.
- `Refused` held `Readable` and `Symlink` for `check_diagnostics`; now `Symlink` also serves `check_socket_dir`, and the fixed-message `NotADirectory` joins them.
- New: no check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design; the check stands at the channel seam's callers.
- The master's Unix line now reads the directory checked before the bind and before every client's connect, left as found when refused, never narrowed.
**Why:** The socket-directory check owed to the route entry "Unix endpoint and home hardening" is built, with no dependency added. Three client seams and the bind each need the recorded endpoint's parent, so one helper holds it.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a force-killed wrapper's socket file
**Section:** §Occupied Resources (occupied-resources.md: IPC endpoints, Filesystem)
**Change:**
- New: a force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it and no removal is owed, a choice: a harmless file in a 0700 directory.
- New, test-only: harness `cleanup`'s `endpoint_gone` on Unix reads through `unconnectable` (`crates/viola-e2e/src/harness/cleanup.rs`): `UnixStream::connect` refused or `NotFound` reads gone, so `cleanup` reads `ok:true` after a force-killed wrapper, removing no leftover file; `holder_gone` holds the same reading for `Wrapper::kill`.
**Why:** `endpoint_gone`'s earlier reading, the socket file no longer existing, measured false after a force-killed wrapper on Unix: `false` and exit 1. Leaving the file, no owner proposed, and the harness rule removing no leftover stand on the P5 word: the overseer's as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `viola list`'s one home rule
**Section:** §Occupied Resources (occupied-resources.md: Filesystem)
**Change:**
- `board::scan` ran `check_path` on the home and on `instances/`; now `check_read` on the home and `check_path` on `instances/`, on every OS.
- Was two checks that disagreed: `board::scan` held the home to the write rule and `check_instance` to the read rule, so a 0750 home passed a board with no instance and was refused (`ScanRefused::Instance`) once it held one, the reconciliation a CARRY on the route entry "Unix endpoint and home hardening". Now one read rule with or without an instance: a 0750 home, or on Windows one granting read to a SID `trusted_sid` rejects, refuses the whole board (`ScanRefused::Home`).
**Why:** The read rule on every OS is the overseer's technical answer as the founder's delegate, PROVISIONAL (`inputs#I2` Q3), the founder's own word owed in the batch at Epoch 7's boundary. No widening, a refusal tightened, so it stood off the halt's card, carried with one gap marker, in security-plan, none in this body.
**Kept:** `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home, a lean the P5 word took: the overseer's as the founder's delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the test-owned socket roots
**Section:** §Occupied Resources (occupied-resources.md: Filesystem)
**Change:**
- New, test-only and outside the viola home: the test-owned socket roots under `/tmp`. `tests/support/home.rs` `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on` give a test its own socket root, `SOCKET_ROOT_VAR` being `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, used by `tests/security_negatives_socket_dir.rs`.
**Why:** The chunk's new test binary and its helpers occupy these paths, so the register names them.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/
