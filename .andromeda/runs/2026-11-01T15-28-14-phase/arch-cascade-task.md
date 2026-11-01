# Task — arch · 2026-11-01-unix-endpoint-and-home-hardening

## The chunk
Verifies the per-user Unix socket directory (owner, mode, not a symlink) before a wrapper binds and before a client connects, refusing a failing directory and never narrowing it in place, with the socket kept 0600 by chmod after the bind as a secondary control.
Adds the peer euid check on both sides of the local socket: the wrapper against a connecting client, and every Unix client against the server it reached, before its first frame.
Applies Unix strict modes on the paths it reaches, makes the board's home check one rule, and records the macOS peer pid gap as a named carve-out of the server check.
Gives each new refusal (socket directory at a wrapper's start and at a client's connect, euid mismatch on either side) a closed code and a hint in the exit-cause catalogue's form, a catalogue row, and adds test-plan §6's macOS exit-cause case.
Re-measures the takeover of a killed wrapper's leftover socket file beside the new checks, the root kill path's connect probe, and the harness cleanup's reading after a force-kill on Unix.

## Reached files
- tests/cross_exit_causes.rs · edits its rows, one for each refusal this chunk adds
- board.rs · edits `board::scan`'s home check
- tests/support/home.rs · re-reads `holder_gone`, the root kill path's connect probe
- crates/viola-e2e/src/harness/cleanup.rs · re-reads `cleanup`'s Unix rule, the socket path gone, and its `endpoint_gone` reading after a force-kill

## Needs
- The Unix socket directory: per-user 0700 at `$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/` or `/tmp/viola-<uid>/`, never the `/tmp` or `$TMPDIR` root and never an abstract-namespace name; the verification of owner, mode and not-a-symlink runs before bind and before connect.
- The bind path in the `channel` seam: `bind_endpoint` and its `create_private_dir`, whose failure is an anyhow error at the catch site today and holds no closed refusal.
- The socket file `<socket dir>/viola-<h12>.sock`: 0600 by chmod after the bind, a secondary control, never through `ListenerOptionsExt::mode`, which is `Unsupported` on macOS and fails the bind.
- The `.lock` start arbiter and the takeover: a killed wrapper removes nothing, its Unix socket file stays, and the next bind takes over the leftover file, which must be weighed beside the directory and euid checks.
- The wrapper-side peer euid check on the server's accept path, against each connecting client.
- The client-side peer euid check before the first frame, on every Unix client: the CLI frame verbs, `viola link` and `viola unlink`, `viola mcp`'s frame tools through `viola-channel`'s Tokio client, and `viola hook`'s connect through the sync client.
- How a peer's euid is read inside the `channel` seam over interprocess 2.4.4 local sockets: new code on the `run`, `hook`, `send` and channel-server paths stays std threads and blocking I/O, and any dependency it adds stays inside the stack's pins, the `deny.toml` allow list, the sole-root `tokio` ban and `scripts/channel-tokio-free.sh`.
- The existing server check beside the euid check: the connected peer's pid and OS start time against the snapshot's `pid` and `started_at` on Linux and Windows, a mismatch writing no frame, reported as `server-verify-failed`.
- The macOS carve-out through the instance snapshot's contract and the server check: macOS reports no peer pid, so the verdict is a match with no comparison, recorded by name, with no substitute comparison and no test reading it as one.
- The recorded `endpoint` of the instance snapshot, which every other process connects to: the client's directory verification acts on that recorded path, read through `viola_state::liveness::live_endpoint` and the hook's connect, and a replayed snapshot carries no `endpoint`, so the instance reads unreachable.
- The CLI exit codes: the typed set stays 0, 1, 2, 10–14, 20, 21; exit 1 holds the start, revive-preflight and verify refusals, exit 21 holds an unreachable instance and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` and the CLI frame verbs.
- The `--json` documents a new refusal must fit: `instance-unreachable` with the closed `viola_core::UnreachableCause` set through `client::unreachable_document`, `viola revive`'s closed code list through `client::refused_document`, and MCP's `isError` result, the same document less `v`.
- Sanitised error surfaces over each new refusal: fixed codes and hints in the `unable:` / `hint:` form, with no path, pid or id in a role line, stdout or stderr.
- Error types for the new refusals: one `thiserror` enum per crate, closed values extended in `viola-core`, `anyhow` only in the root bin, and the census of `thiserror` derives named in Rust error types.
- `v1-18`'s socket-directory leg: the code lands here, and its catalogue row in `tests/cross_exit_causes.rs` carries the closed code and hint.
- Test-plan §6's macOS exit-cause case: the per-user socket directory loosened to 0755 reaches exit 21.
- The hook contract: `viola hook` keeps exit 0, no body and no stderr, exit 2 forbidden, and a refusal of this chunk reaches it only as a fail-open.
- The terminal rule for `run`: while the child runs, `run` writes nothing to the terminal but the child's output, so a wrapper-side euid refusal of a connecting client never reaches that terminal; a socket directory refusal at the start comes before any child is spawned.
- Unix strict modes: the read rule (no group or other bit) and the write rule through `viola_state::strict`, `check_home_tree` and the per-instance check; each check runs ahead of any log, because opening a log narrows an existing home, and a refused check writes no line.
- The board's home check: `board::scan` runs `check_path` (the write rule) on the home, then `check_instance` per instance (the read rule), so a 0750 home passes while empty and is refused as `ScanRefused::Instance` once it holds an instance; `viola list`'s refusal surface carries that outcome.
- The killed wrapper's two recorded markers: the measurement that the root kill path waits on a connect probe because a wait on the file being absent never ended, and the unmeasured hypothesis that `cleanup` reads `endpoint_gone:false` and exits 1 after a force-kill.
- The local endpoint trust boundary: v1 trusts every process of the same OS user and no other, and each endpoint must not be connectable by another user; the directory, mode and euid checks enforce that on Unix.
- Linux and macOS parity, `v1-53`: the same end-to-end suite on both OSes against the fake agent and the live Linux run; a leg this chunk cannot prove stays there.

## Standing conventions
- S1 · Each new test runs in its own viola home through `--home`, under `target/e2e-home`, so no two tests share an endpoint or log and the obs gates and secret scan cover it · Established Decisions, [CI/CD], and occupied-resources.md
- S2 · Each new test runs against the fake agent; the real `claude` runs only in local live tests, never in CI · Established Decisions, [CI/CD]
- S3 · Every OS-specific branch compiles and is tested on its CI runner · Cross-cutting Patterns, Cross-platform discipline
- S4 · Each new log line is codes only in its role file under `diagnostics/`, content-bearing detail only in the instance's detail file, one line per `write`, inside the default-deny `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json` · Occupied Resources, Filesystem and Repository
- S5 · A new test-only endpoint takes a name outside the `viola-<h12>` namespace, unique per test, in the `viola-test-chan-<pid>-<label>` form, and a new test-only file or directory holds its row in Occupied Resources · Occupied Resources, IPC endpoints and Filesystem

## Not needed
- Windows endpoint security: the pipe listener's SDDL, the client's SQOS open and the DACL reading stay as they stand, since no Windows behaviour moves.
- The frontend and GUI contracts: deferred to 0.2.0.
- The PTY seam, the ConPTY sideload, the vt100 readiness gate and delivery confirmation: the chunk's checks act at the socket, ahead of any typed byte.
- The wheel, the budget governor, `budget.json` and the statusline wrapper: no rule of theirs changes with who may connect.
- The capability ledger's rows, stamps and `viola verify`'s probe design: peer credentials and directory modes are OS facts, no `claude` behaviour.
- Channel method payloads, event kinds and their `data`, and the `list` payload's fields: a refusal here comes before the first frame and adds no method or kind.
- Pinned copy placement and hashing, the plugin's file contents, the marketplace's contents and the registered hook events: none of them changes with the endpoint's checks.
- The R8 environment strip and the `claude agents --json` join: neither touches the socket directory or a peer's identity.
- `fuzz/` targets and corpus: the chunk adds no parser of external bytes.
- Distribution beyond `cargo install --path .`: signing, self-update and package managers stay deferred to a later version.
