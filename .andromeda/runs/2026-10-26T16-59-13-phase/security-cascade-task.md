# Task — security · 2026-10-26-home-and-code-bearing-file-integrity

## The chunk
Makes the home rules whole at every entry point: an owner-only home (Unix 0700 dirs / 0600 files set explicitly, Windows a protected owner + SYSTEM DACL) and the strict-modes check run BEFORE any read or open of the home, the role-file open included, in `send`, `wait`, `last`, `answer`, `pause`, `release`, `viola hook`'s nine events, `viola mcp`'s frame tools, `verify`'s `update_stamps` and `run`.
Closes the founder's dated strict-modes exceptions held "until Home and code-bearing file integrity": the `hook.event` snapshot read, the stamps read, the Windows ConPTY companions without the owner/DACL check, and the CLI and MCP frame gaps (whose server-verification half already landed).
Owns the code-bearing files: the pinned `bin/<version>-<hash>/viola` re-hashed before reuse and its first-copy race (`pin_exe`), `plugin/` rewritten atomically on every start with absolute pinned exec paths, the `marketplace/` tree's integrity, and the single writers of `snapshot.json` (the wrapper) and `ledger/stamps.json` (`viola verify`).
Re-reads and proves: the Windows DACL leg of `check_instance` on CI's `windows-2025`, the seeded test home `seed_conpty` makes (whether the product must refuse it), and re-measures whether two clean release builds of one tree differ (by file and by section).
The readers' move onto `read_snapshot_or_replay` is a founder P4 fork, neither planned in nor out; any boundary widening met on the way halts for his live word.

## Reached files
- crates/viola-state/src/strict.rs · edits check_instance, check_path (the Windows owner + DACL leg and its proof)
- crates/viola-state/src/fs.rs
- crates/viola-state/src/pin.rs · edits pin_exe (the home copy of the binary, the first-copy race), pin_companions (the `conpty/` folder joining the Windows strict-modes set)
- crates/viola-state/src/stamps.rs · edits update_stamps (the stamps read under strict-modes)
- crates/viola-state/src/liveness.rs · edits live_endpoint (the strict-modes check before its snapshot read)
- crates/viola-state/src/snapshot.rs
- crates/viola-state/src/replay.rs
- src/obs.rs · edits open_role_file (the check before the open)
- src/cmd/client.rs · edits start (the role-file open after the instance check)
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits the snapshot read through `VIOLA_DIR` (strict-modes before it, failing open)
- src/cmd/hook/statusline.rs
- crates/viola-mcp/src/tools.rs · edits the frame tools' `live_endpoint` read
- src/cmd/verify.rs · edits the `update_stamps` call
- src/cmd/run.rs · edits start (the per-entry-point checks of the home, `instances/<name>/`, `bin/`, `plugin/`, `ui/` and the trusted files), pin_and_plugin, the collision check
- src/conpty.rs
- src/cmd/plugin.rs
- tests/cli_controls_not_disableable.rs · edits the 0770 `--home` negative (exit 21), the table's completeness case
- tests/support/home.rs · edits seed_conpty (let viola create the home first)
- tests/hook_statusline.rs

## Needs
- The home strict-modes check (Unix mode and owner; Windows owner + DACL with inherit-only and generic bits, NULL DACL and a non-persistent-ACL volume all refused) run before every read or open of the home at each entry point the chunk names, never after; a `#[cfg(windows)]` arm must never return success unchecked.
- The check-before-open order: a role-file open (`open_role_file` → `create_private_dir`) narrows an existing home to 0700, so the instance check must precede `client::start`'s open on every verb, as `revive` and the statusline arm already do.
- Owner-only creation: every dir 0700 and file 0600 set explicitly (never the umask), the pinned `viola` 0700 and never group- or world-writable, every replaced file through `replace_private` with its mode set before any byte; on Windows a home outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at creation.
- The DACL check before any `instances/<name>/diagnostics/detail-*.ndjson` or role file is opened; diagnostics files stay 0600 and never readable by other users.
- Refusal surfaces of a failed check: CLI frame verbs exit 21 `instance-unreachable`; MCP `instance-unreachable` with `detail: null` and no line; `viola hook` fails open (exit 0, no body, no stderr), never blocking the human; no line or output holds a pid, start time, path or endpoint.
- The client server verification before the first frame (peer pid and start time against the snapshot) stays, now against a snapshot that passed strict-modes in the same process; the anti-pattern's dated-gap list for `hook.event`, `hook.dialog`, CLI `send` / `wait` / `last` / `answer` / `pause` / `release` and `viola mcp`'s frames closes.
- The founder's dated exceptions this entry ends (Decisions Log `2026-09-28` hook and stamps read, `2026-09-29` ConPTY companions, the 2026-10-04 CLI gaps, the 2026-10-17 MCP gap) and the CARRYs that track them: each closing is a security-plan amendment the chunk owes.
- `ledger/stamps.json`: read by `update_stamps` and by `run`'s version gate (`read_stamps_strict`) only after strict-modes; only `viola verify` writes it; a refusal reads `cli_verified:false`; a non-`null` dialog decision still needs a stamp read so.
- `snapshot.json`: only the instance's own wrapper writes it; `endpoint`, `pinned_bin` and `statusline_command` in it decide where clients connect and what runs; every reader keeps `read_snapshot` (unreadable or newer reads as absent) until the founder rules the replay fork.
- `statusline_command` is never run from a home or instance directory that fails strict-modes; `hook statusline`'s instance check before anything under the home stays.
- Pinned binary: `bin/<version>-<hash>/viola` re-hashed (truncated SHA-256, never FNV or `DefaultHasher`) against `<hash>` before reuse; two concurrent first `run`s on a fresh home must not leave a hook spawn failing on a replaced copy.
- ConPTY companions (Windows x64): `bin/<version>-<hash>/conpty/`, `OpenConsole.exe` and `conpty.dll` join the Windows owner + DACL strict-modes set, beside the full SHA-256 re-hash under a `FILE_SHARE_READ`-only handle held until the spawn, loaded by absolute path after `restrict_dll_search`; a failing one is never overwritten or deleted and the run degrades to the inbox ConPTY silently (no refusal, exit change or terminal byte).
- `plugin/` rewritten atomically on every start, never reused unrewritten; every exec-form `command` in `hooks.json`, `.mcp.json` and the Unix `instances/<name>/settings.json` statusline override is the absolute pinned path, never a PATH lookup; the override only for a pinned path of plain characters, none on Windows.
- `viola plugin install`'s `marketplace/` tree: `check_path` on the home and its four tree dirs before any write (exit 21); its `.mcp.json` `command` is the canonical exe that ran the install; an unknown entry left as found and the missing re-hash are the integrity owed here.
- Home resolution feeding the check: `--home` as given, else a valid `VIOLA_NAME` / absolute `VIOLA_DIR` pair, else `~/.viola`; `canonical_home` canonicalises and creates nothing; names only through `ViolaName::try_new` before any `instances/<name>` join.
- No `config.json` key, `VIOLA_*` var or CLI flag may switch off the strict-modes check, the DACL check or the server verification; the controls-not-disableable table proves it, the 0770 `--home` negative included.
- Windows proof of `check_instance` on `windows-2025` (no case today widens an instance tree's DACL) and the seeded-home finding: a home `seed_conpty` made first is refused, so the test carve-out must let viola create the home before seeding.
- Same-user threat to the home (Threat Model filesystem boundary): integrity rests on OS permissions plus the content hash in the folder name; `budget.json`, `statusline-source.json` and `marketplace/` are not trusted files, and adding a reader of a home file on a new basis is a boundary widening.
- The release build measurement (two clean `cargo build --release --locked --bin viola` builds of one tree, compared by file and section) builds `viola` only, through `scripts/release-check.sh`'s rules (no test-only binary or feature).
- `viola_state` stays a sync crate: any new Windows ACL call goes through `windows-sys` features already pinned or a new feature recorded, no C-building crate, no tokio in its graph.

## Standing conventions
- S1 · each new test or harness case uses its own `--home`, never a committed one, and a test-side removal takes only a dir the test itself created · Secret Management (Development) / .andromeda/security-plan-detail/security-anti-patterns.md
- S2 · each new refusal or error a verb, the hook or MCP emits carries codes and fixed messages only: no absolute path, pid, start time, endpoint or anyhow chain with a serde source · Error Handling / .andromeda/security-plan-detail/error-handling.md
- S3 · each new log or diagnostics line keeps the NEVER-log floor: paths and pids only in `instances/<name>/diagnostics/` (0600), never an R8-stripped `CLAUDE*` value · Bootstrap phases (`logging-redaction-wire`) / .andromeda/security-plan-detail/bootstrap-phases.md
- S4 · each new file or directory the product creates under the home is 0600 / 0700 set explicitly, replaced only through `replace_private` · Data Protection / .andromeda/security-plan-detail/data-protection.md
- S5 · each new or changed CI step stays SHA-pinned, `permissions: {}` with job `contents: read`, event values only through `env:`, uploads behind the artifact canary scan · Dependency Security (CI integration) / .andromeda/security-plan-detail/dependency-security.md
- S6 · each new dependency or feature passes `cargo deny check`, the sync-crate tokio ban and the C-build ban, with no new ignore, skip or licence exception · Dependency Security / .andromeda/security-plan-detail/dependency-security.md
- S7 · each boundary widening, new environment read, new G2 exemption or new dated exception needs its own Decisions Log entry and the founder's own word · Security Anti-Patterns (Universal) / .andromeda/security-plan-detail/security-anti-patterns.md

## Not needed
- Paste-text validation, `validate_paste_text`, the typed-text CR/LF rewrite and `empty-text`: the chunk changes nothing typed into the PTY.
- The human terminal's stdin classifier and the wheel's typing rules: no keystroke path changes.
- The vt100 feed under `catch_unwind` and PTY output passthrough.
- Unix socket and socket-directory hardening, the server-side peer euid check, the client euid check and macOS server verification: "Unix endpoint and home hardening"'s freight.
- The Windows pipe listener SDDL, `accept_remote(false)`, SQOS client opens and handle inheritance: the IPC listener and pipe opens are not changed.
- R8 stripping of inherited `CLAUDE*` values and the Windows persistent-environment name read.
- `viola verify`'s interactive PTY runs B–E, their pastes, probe dirs and `--record` fixture scrub: the chunk reaches verify only at its stamps read.
- `revive`'s session-id argv, `--resume` shape and `claude agents --json` preflight; `list`'s `escape_row` and terminal colour facts.
- The statusline shell-out's own mechanics (`/bin/sh -c`, 5 s bound, stdout pass-through) and the statusline source read.
- Dialog decision enums, answer free text and the driver prompt-injection accepted risk.
- The `fuzz/` workspace exemption, nightly advisory runs, the ConPTY vendoring script and the v1.x signing and release prerequisites.
- `events.ndjson` retention and the local-availability accepted risk.
