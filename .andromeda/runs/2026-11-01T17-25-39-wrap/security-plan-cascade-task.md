# Task — security-plan · 2026-11-01-unix-endpoint-and-home-hardening

## The session
Built the socket-directory check (`check_socket_dir`, `check_endpoint_dir`) at `viola run`, `revive`'s start, the eight CLI frame verbs, `viola hook` and MCP's `frame`; a refusal reads the new closed word `socket-dir-refused`, the sixth cause of exit 21.
Built the peer euid check on both sides: the wrapper drops a refused peer in `accept_loop` before any frame, and each of the three client seams refuses a server of another euid, which reads `server-verify-failed`, on Linux and macOS.
Moved `viola list`'s home from two rules to the read rule on every OS, standing PROVISIONAL on a delegate's answer.
Retired the dated gaps owed to the route entry "Unix endpoint and home hardening"; macOS's missing peer pid comparison stays named, owned by no route entry.
Measured three standing claims false: the threat model's socket file at the `$TMPDIR` root, a hook's refused directory leaving a note in `diagnostics/`, and a sync plus Tokio server, where the code holds one sync server.
Added the test binary `tests/security_negatives_socket_dir.rs`; CI `ci#41329861618` read `c4bf0a92` green, 15 of 15 checks, no dependency added or bumped.

## Reached files
- crates/viola-core/src/lib.rs
- crates/viola-state/src/strict.rs
- crates/viola-state/src/board.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/server.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/run.rs
- src/cmd/client.rs
- src/cmd/hook.rs
- src/human/hints.rs
- schemas/diag-line.v1.json
- crates/viola-e2e/src/harness/cleanup.rs
- tests/security_negatives_socket_dir.rs
- crates/viola-e2e/tests/harness_lifecycle.rs
- crates/viola-mcp/tests/handlers.rs
- tests/support/home.rs
- tests/cli_list.rs
- tests/cross_exit_causes.rs
- tests/contract_error_bodies.rs
- tests/cli_error_bodies.rs
- tests/cli_controls_not_disableable.rs
- tests/hook_fail_open.rs
- tests/security_negatives_server_identity.rs
- Cargo.toml
- .config/nextest.toml

## Touches
- The owner "Unix endpoint and home hardening" (about 9 sites in this plan): every clause owing work to it is discharged and changes; only macOS's pid gap stays named, and it now has no route entry as owner.
- IPC access control (Unix): the per-user directory check is built, before bind and before every client connect; a failed check refuses `run`, reports `instance-unreachable` with the cause `socket-dir-refused`, and fails `hook` open. A row reading this check as owed changes (about 9 `socket-dir` sites: the body 2, `.andromeda/security-plan-detail/authentication-authorization.md` 4, `.andromeda/security-plan-detail/bootstrap-phases.md` 1, `.andromeda/security-plan-detail/security-anti-patterns.md` 2).
- IPC server-side peer check (Unix): the drop on accept is built; a row reading it as owed changes (about 12 `peer-euid` sites across the plan, client and server).
- IPC client-side server verification: the client euid check is built at `src/cmd/client.rs`, `src/cmd/hook.rs` and `crates/viola-mcp/src/tools.rs`; the row's clause owing the client euid check and macOS's euid and verified directory checks changes.
- macOS rows (about 9 `mac-gap` sites): the euid and directory checks now hold there; a row stating macOS compares no peer pid or start time is a true claim and stays, its recorded gap wording kept or read as the named carve-out.
- The ban on writing a channel frame before verifying the server: its exception naming the dated gaps changes, since every client's euid check no longer sits in it and macOS's pid and start-time check alone remains.
- The ban on a socket directly in `/tmp` or at the `$TMPDIR` root: the rule stays; its verified 0700 per-user directory is now verified by code.
- Exit 21 and MCP's `instance-unreachable` closed causes move from five to six (about 5 `causes` sites): a row reading five closed causes changes, a row listing them extends with `socket-dir-refused`, a row naming only the type does not change. §Error Handling's MCP `isError` list is one.
- `server-verify-failed`'s reach moves from off macOS to Linux, Windows and macOS, where on macOS it means an euid mismatch alone; each row stating its reach changes.
- Library row in §Authentication & Authorization: `peer_creds()` now also serves an euid read on Unix (`peer_creds().ok()?.euid()`), none on Windows; its `pid()` staying none on macOS is unmoved.
- `viola list`'s home moves from two rules to the read rule: `.andromeda/security-plan-detail/authentication-authorization.md:36` and `.andromeda/security-plan-detail/input-validation.md:42` change or are re-read; a row saying strict-modes has two rules stays true, since the write rule and the read rule both stand.
- `.andromeda/security-plan-detail/threat-model-summary.md:43`: the socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) is disproved, the endpoint standing in a per-user 0700 directory since chunk 2026-09-27-wrapper-channel; retire that line. Any other row placing the file at that root reads the same claim; rows stating the verified per-user directory or the anti-pattern do not change (about 3 `threat-sock` sites).
- §Authentication & Authorization's claim that a hook meeting a refused directory leaves a note in `diagnostics/` is disproved: the record is one `hook-decision` line at WARN.
- §Authentication & Authorization's sync plus Tokio server is disproved: one sync server is what the code holds. `TokioClient` is a client and stays.
- The `Display` list in §Error Handling: `Refused` gains the fixed-message variant `NotADirectory`, not a derive; the `thiserror` derives stay 9 and the list of types does not move. `TreeRefused` and the exit set are unchanged.
- `revive --json`'s exit-1 codes move from ten to eleven: a row stating ten or listing all ten extends with `socket-dir-refused`; a row saying it prints codes and closed words only stays true.
- `schemas/diag-line.v1.json`'s `process-exit` `detail` moves 19 to 20 and `hook-decision`'s `detail` moves 7 to 8: any site in this plan stating those counts changes.
- `hook` fail-open rows and the bans on stderr and non-zero exit from `viola hook`: the new refusal obeys them, exit 0 and empty streams; a row enumerating the hook's security checks extends with the directory check and the euid check.
- MCP's strict-check rows: `frame` now runs the directory check after `live_endpoint` and the euid check beside `server_verified`, each with no frame and no line, `isError` the record.
- The Universal ban on config, a `VIOLA_*` variable or a flag switching off a control: it names the socket-directory check and the peer verification, both now live and held by the same ban; `setting_does_not_disable_the_socket_dir_check` witnesses the directory check. The wrapper's drop takes its decision by dependency injection, no env, flag or feature seam, so the seam carve-outs do not grow.
- Bootstrap `auth-scaffolding-baseline`: its strict-modes checks in `viola-state` now hold the socket directory, and its client server verification holds the peer euid in `viola-channel`, through what this session built.
- The Data Protection ban on `#[cfg(windows)]` returning `Ok(())` unchecked concerns the home's check: `check_socket_dir`'s off-Unix `Ok(())` is another check, and the board's read rule does run on Windows.
- Test inventory: `tests/security_negatives_socket_dir.rs` joins the security negatives; rows naming its sibling `security_negatives_server_identity` do not change of themselves.
- The wiring of each client seam's euid read is witnessed only by this run's forced refusals, which no committed test repeats; a row claiming a committed test holds that wiring would be false. The remainder's owner is `matrix#v1-44` and test-plan §1, not this plan.

## New facts
- F1 · authentication-authorization · `viola_state::strict::check_socket_dir` makes one `symlink_metadata` read of the path: absent passes, unstattable is `Refused::Unreadable`, a symlink `Refused::Symlink`, a non-directory `Refused::NotADirectory`, else the Unix read verdict on owner and mode.
- F2 · authentication-authorization · The socket-directory check never chmods: a refused directory stays as found.
- F3 · authentication-authorization · `check_endpoint_dir` is the check on a recorded endpoint's parent, the one helper the bind and the three client seams share.
- F4 · authentication-authorization · At a start, `socket_dir_ready` checks, creates the parent only when absent, then checks again, so a race's loser reads the same; it runs before any snapshot, child or `Server::bind`, `revive`'s start shares it, a failed create and a refused check read one code, and a `Server::bind` failure stays what it was.
- F5 · authentication-authorization · On every client seam the order is the directory check, `Client::connect`, the peer euid decision, then the server verification against the snapshot.
- F6 · authentication-authorization · `peer_euid_admitted(observed, own, reported)` is admit when not reported or observed equals own, with `PEER_EUID_REPORTED` being `cfg!(unix)`, so a peer whose euid cannot be read is refused on Unix; on Windows no euid is read and the DACL and the pid comparison stay the controls.
- F7 · authentication-authorization · The wrapper's drop filters `incoming()` in `accept_loop` on the accept thread, before the worker thread and any frame: the peer reads end-of-file, and no line and nothing on the terminal is written.
- F8 · authentication-authorization · No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, the checks sit at the seams, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design.
- F9 · authentication-authorization · On the board, `scan` holds the home to `check_read` on every OS and `instances/` to `check_path`, with the order, the whole-board `ScanRefused::Home` and no-open unchanged; `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home.
- F10 · authentication-authorization · A killed wrapper's leftover socket file stays in the per-user directory until a next bind of that name takes it over: nothing removes it, no owner is proposed, recorded as a harmless file in a 0700 directory.
- F11 · authentication-authorization · macOS's absent pid and start-time comparison is a named carve-out owned by no route entry: `server_verdict_rows` keeps its row and the server-identity negatives stay compiled out there.
- F12 · authentication-authorization · Provenance: the new word `socket-dir-refused` and `server-verify-failed` on Linux and macOS are the founder's own word (`inputs#I2` Q1, Q2, 2026-11-01); the board's read rule on every OS and the P5 word are the overseer's as delegate, PROVISIONAL, the founder's word owed at Epoch 7's boundary batch.
- F13 · authentication-authorization · `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on `ubuntu-latest` and `macos-latest`, none on Windows.
- F14 · authentication-authorization · A foreign euid and a foreign owner are not driven by any test: one account can make no foreign peer, so those decisions are held by injected values, and no seam may be added to drive them.
- F15 · error-handling · `UnreachableCause::SocketDir`, word `socket-dir-refused`, has no serde path and no `RefusalReason`; on a CLI frame verb it is `CallError::SocketDirRefused` to `Unreachable::SocketDir`, exit 21, `during` `connect`, hint cause `socket-dir`, one `process-exit` at WARN and no `channel-request`.
- F16 · error-handling · `viola run` and `revive`'s start refuse a socket directory with exit 1 and one `process-exit` of `detail` `socket-dir-refused`, a step with no span, through `refused_with` and `refuse_socket_dir`.
- F17 · error-handling · A refused euid has no word of its own: it is `CallError::ServerUnverified`, `NotSent::ServerUnverified` and `UnreachableCause::ServerVerify` on the three seams.
- F18 · error-handling · A hook meeting a refused directory is `NotSent::SocketDir` and `Asked::SocketDir`: exit 0, empty streams, no frame, one `hook-decision` of `detail` `socket-dir-refused` at WARN, on the dialog path `hook-invoked` with a null `corr` first and `decision_emitted` false.
- F19 · error-handling · The new refusal names only the parsed `ViolaName` and its lines hold no path, pid or uid; its wording is a planning form, the founder's wording owed at Epoch 7's boundary batch.

## Untouched
- Dependency Security's `deny.toml` additions, audit tools, vendored ConPTY gate, G2 exemption and workflow rows: no dependency was added or bumped, `cargo deny check` stayed green, and no CI step changed.
- Secret Management's secret classes, canary scan classes, rotation and access-auditing rows: the session adds no secret and moves no scan class.
- Data Protection's repository fixtures, verify's probe captures and its accepted risks and residuals: `viola verify` and `fixtures/claude/` are not reached.
- The DLL search order and sideloaded ConPTY companion rows and bans: no pty or ConPTY file is reached.
- The threat model's security tier, data classes, compliance triggers, hosting and CI/CD rows: no new data class, surface or deploy stage, and the new lines hold no path, pid or uid.
- Bootstrap's `dep-audit-tooling-install`, `secret-scanning-ci-gate`, `dep-security-ci-gate` and `supply-chain-signing-init`: none of their tools, gates or scripts moved.
- The Code Patterns bans on GitHub Action refs, `Swatinem/rust-cache`, `self_update` signatures and the `run --mutants` scratch directory: no workflow, release surface or mutation path is reached.
