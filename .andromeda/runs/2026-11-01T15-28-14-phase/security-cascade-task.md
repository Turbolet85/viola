# Task — security · 2026-11-01-unix-endpoint-and-home-hardening

## The chunk
Verifies the per-user Unix socket directory (owner, mode, not a symlink) before a wrapper binds and before a client connects, refusing a failing directory, never using it or narrowing it in place; the socket stays 0600 by chmod after the bind, a secondary control only.
Checks the peer euid on both sides of the local socket: the wrapper against a connecting client, every Unix client against the server it reached, before its first frame.
Applies Unix strict modes on the paths this entry reaches, makes the home check of `viola list`'s `board::scan` one rule, and gives each new refusal a closed code, a hint and an exit-cause catalogue row.
Records the macOS pid gap as a named carve-out (no peer pid, so the server check there is a match with no comparison) and adds test-plan §6's macOS exit-cause case (the socket directory loosened to 0755, exit 21).
Re-derives at HEAD the `holder_gone` measurement and whether the next bind's takeover of a killed wrapper's leftover socket file holds beside the new checks, and measures the hypothesis that harness `cleanup` reads `endpoint_gone:false` after a force-kill on Unix.

## Reached files
- tests/cross_exit_causes.rs · edits the catalogue's rows for the socket-directory refusal and the peer euid mismatch
- board.rs · edits `board::scan`'s home check
- tests/support/home.rs · re-reads `holder_gone`, the root kill path's connect probe
- crates/viola-e2e/src/harness/cleanup.rs · re-reads harness `cleanup`'s Unix rule, `endpoint_gone` after a force-kill

## Needs
- The per-user socket directory in each of its three roots (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, `/tmp/viola-<uid>/`): its 0700 mode, its owner, its not being a symlink, and the rule that a socket never sits directly in `/tmp` or at the `$TMPDIR` root.
- `bind_endpoint` and `create_private_dir`: a failure there is an anyhow error at the catch site today, and the verified directory's refusal replaces that with a closed code and hint; the anyhow chain, path and uid stay out of what the refusal shows.
- The socket's 0600 by chmod after the bind, as a secondary control only: `ListenerOptionsExt::mode` stays unused, `try_overwrite` stays off, the Linux abstract namespace stays unused.
- The `.lock` start arbiter and the takeover of a killed wrapper's leftover socket file on the next bind: how that takeover sits beside the directory and euid checks, and beside the ban on `try_overwrite`.
- The wrapper's accept path: a peer whose euid differs from the server's is dropped before a frame is read.
- The client's server verification, every Unix client: the euid check runs before the first frame of any method, in order with the instance's strict-modes check and the one snapshot read; a check that cannot be made sends no frame and never falls back to a default connect.
- `interprocess` `Stream::peer_creds()` as the source of the peer's euid on Linux and macOS, and its `pid()` reading none on macOS.
- The macOS carve-out: the server check is a match with no comparison, its negatives compile out on macOS, no weaker comparison stands in for the peer pid and no test reads the gap as a comparison; macOS keeps its euid and verified-directory checks.
- The dated gaps this entry owes: the Authentication & Authorization row on client server verification and the Authentication ban on writing a frame before verifying the server both name the client euid check and macOS as owed to this entry, and no other frame or process may borrow those gaps.
- What a failed directory check does on each surface: it refuses `run`, reports `instance-unreachable` to clients and fails `hook` open; a peer euid mismatch on either side takes its place on the same surfaces, P3 reading which exit each one is.
- `instance-unreachable`'s closed `detail` causes and the shapes that carry them: MCP `isError` `structuredContent`, CLI `--json`, `of_reply`, `reply_of`, `viola_core::is_code_word`, and the `refused: <code>` text; a new cause must be a code word or it reads `unknown` or null.
- The typed exit set stays 0, 1, 2, 10–14, 20, 21: a new refusal maps into it, with no new exit code.
- `hook` keeps exit 0, no body, no stderr: a refusal of this chunk reaches it only as a fail-open, and the ban on a security refusal blocking the human keeps its reach, with the `run` refusal on a failed directory check as the plan's own named case.
- External text of each new refusal: fixed messages in `Display`, path-holding fields excluded, no absolute path or internal type name in `--json`, MCP or channel `error.data`, and a hint that holds no path, pid or uid.
- The home strict-modes check on Unix: the write rule of `check_path` and the read rule of `check_instance` (any group or other bit on a home or instance directory), run before a process's first read or open under the home.
- `viola list`'s `board::scan` and `ScanRefused::Instance`: a 0750 home passes while it holds no instance and is refused naming the instance once it holds one, and that split ends.
- No control is switched off from outside: no `config.json` key, `VIOLA_*` variable or CLI flag reaches the socket-directory check, the peer check or the home check, and no new seam or environment read comes without its own Decisions Log entry.
- Proving a peer euid mismatch from a test: no harness command, gate or pre-push stage launches anything as uid 0, and a seam for it falls under the rule above, so the way this negative is proved stays inside both.
- The 0755 negative loosens a socket directory the test side made under a root it set itself, never the operator's real per-user directory.
- Linux and macOS parity: the same end-to-end suite on both OSes with its live Linux run (`v1-53`), the socket-directory leg of `v1-18` standing on a verified cap, and `v1-44`'s owner-only connect with every client verifying; a leg this chunk cannot prove stays in the suite.
- Shared bind and connect code keeps the Windows arms as they stand: the pipe listener's SDDL, the client's SQOS open and the Windows strict-modes reading do not move.

## Standing conventions
- S1 · Each new test uses its own `--home`, and the pre-push gate's children carry only `HOME` and `PATH`, so no host environment value crosses into one · Secret Management, Development; `.andromeda/security-plan-detail/secret-management.md`
- S2 · Each new log line stays under the NEVER-log floor · Bootstrap phases, `logging-redaction-wire`; `.andromeda/security-plan-detail/bootstrap-phases.md`
- S3 · Each new test-side removal under `target/e2e-home` takes only a directory the test side itself created · Security Anti-Patterns, Code Patterns; `.andromeda/security-plan-detail/security-anti-patterns.md`
- S4 · Each new test's role files pass the G2 zero-panics gate, which takes no further exemption without its own Decisions Log entry · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
- S5 · What a new test leaves under `target/agent-run/` passes the artifact canary scan, a non-0600 diagnostics file included · Secret Management; `.andromeda/security-plan-detail/secret-management.md`
- S6 · Test-only helpers stay behind the test-only features and out of the release graph `scripts/release-check.sh` judges · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
- S7 · A dependency a change adds keeps the workspace pin, the licence list and the bans, `tokio` in the sync crates included · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
- S8 · A CI step a change adds keeps SHA-pinned actions, `permissions: {}` with job-level `contents: read`, and no event value in `run:` · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
- S9 · A file or directory a change creates under the home gets explicit 0600 or 0700, never the umask · Security Anti-Patterns, Data Protection; `.andromeda/security-plan.md`

## Not needed
- `validate_paste_text`, the `send` and `answer` free-text set and the `control-character` and `empty-text` refusals: the chunk changes no typed text.
- Channel frame contents: the `MAX_FRAME` cap, the `v` check, method params and the `release-from-driver` guard; the new checks run before any frame and change no frame's shape.
- MCP tool input checks, the MCP stdin line cap and the rmcp feature set: the chunk changes no tool or transport.
- Hook stdin parsing and the closed `HookEvent` mapping: only the fail-open outcome reaches this chunk.
- The `statusline_command` source read and its shell-out: the chunk changes neither.
- The pinned binary's re-hash and hash derivation, and the exec-form pinned path in `hooks.json` and `.mcp.json`: no code-bearing artefact is rewritten here.
- Windows DLL search order, the ConPTY sideload and its vendoring, and the persistent-environment registry read: no Windows behaviour moves.
- `viola verify`'s probe runs, fixture recording and scrubbing, and their accepted risks and residuals: the chunk records no fixture and starts no live `claude` by its scope.
- The `.cmd` and `.bat` refusal, `claude` PATH resolution and the session id as a `--resume` argument: the chunk changes no child spawn.
- The R8 strip of inherited `CLAUDE*` names from the child's environment: the chunk changes no child environment.
- The vt100 feed under `catch_unwind`, the stdin typing classifier and the terminal colour facts: no PTY or terminal path moves.
- `events.ndjson` retention, handle inheritance and any portable-pty swap: no log format or PTY dependency moves.
- The `fuzz/` workspace and its lockfile exemption, the nightly and `windows-mutants.yml` workflows, and the v1.x release prerequisites: the chunk adds no parser surface and no release.
- Compliance triggers and the N/A rows for public API, upload, OAuth, rotation and account flows: no such surface exists here.
- The frontend: it stays deferred to 0.2.0.
