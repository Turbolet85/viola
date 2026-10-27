# Task — security · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The chunk
Closes the race in which two first `viola run`s of one version on a fresh home both find `bin/<key>/viola(.exe)` absent and both replace it, so that one agent's hook spawn of the pinned path fails (`ran:false`, no `session-start`).
The fix lands on the absent-copy write of `pin_bytes` / `pin_exe` (through `replace_private_shared` → `replace_private`) and rests on a mechanism P3 or a two-sided reproduction establishes (ETXTBSY is only a hypothesis). It ships with a red-before / clean-after witness and the single-instance control beside it.
P3 also reads whether `plugin/<key>/`'s whole rewrite at every start can fail a concurrent reader of `hooks.json` / `.mcp.json`, and whether the Windows exe arm needs the `ERROR_SHARING_VIOLATION` wait that `pin_companions` already has.
Separately, it measures whether two clean release builds of one committed tree give one file, comparing them by file and by section through `scripts/release-check.sh`'s build. It records no unmeasured cause and makes no build change, which is the founder's to act on.
It re-asserts, unchanged, the hash-before-reuse rule, the per-start plugin rewrite and the home-tree strict-modes check.

## Reached files
- crates/viola-state/src/pin.rs · edits pin_bytes, pin_exe (the absent-copy arm)
- crates/viola-state/src/fs.rs · edits replace_private_shared, replace_private (what is held open across the rename)
- src/cmd/run.rs · edits the start's pin_bytes call, pin_companions (Windows x64 exe-arm comparison)
- src/cmd/verify.rs · edits the pin_exe call
- scripts/release-check.sh · re-reads its release build (profile and CARGO_TARGET_DIR) as the measurement's build

## Needs
- The pinned binary `bin/<version>-<hash>/viola(.exe)`: its absent-copy write under concurrency. The copy is 0700, never group- or world-writable, and its mode is set before any byte is written.
- The hash-before-reuse rule over the pinned binary: a present copy is re-hashed (truncated SHA-256) against `<hash>` and refused on any difference, never overwritten. No same-bytes acceptance or wait the fix adds may weaken it or use a non-cryptographic hash.
- `viola_state::fs::replace_private` as the one helper for every replaced file under the home (tempfile, mode set first, `sync_all`, `persist`). Any concurrency control the fix adds (a lock, a create-new, a wait) goes through or beside it, never around it.
- The `.lock` sibling convention for cross-process state (std `File::lock`). A lock the pin takes is a 0600 sibling under the owner-only home.
- The pinned binary and plugin folders are never deleted in v1. The fix may not remove, rename away or truncate a copy another start or an agent may be running.
- The `plugin/<key>/` folder is rewritten whole at every start and never reused without rewriting. Each exec-form `command` in `hooks.json` / `.mcp.json` stays the absolute pinned path, never a PATH lookup. This holds under a concurrent start's rewrite.
- The home-tree strict-modes check (`check_home_tree`: the home, `bin/`, `plugin/`, the pinned copy's `bin/<key>/` and `plugin/<key>/`) runs before the pin and before any log, for `run` and `revive`'s start and for `verify`. A directory or file a concurrent start has just created must still pass the owner and mode (Windows owner + DACL) check. No rule of the check changes.
- `viola hook` as the agent's spawn of the pinned path. It fails open (exit 0, no body, no stderr). The race fault is a failed spawn of the pinned exe, not a hook refusal, so no fix adds a hook-side refusal.
- On Windows x64 the ConPTY companions under `bin/<key>/conpty/` follow these rules. They get the strict-modes check (`check_present`) before the held open, then a full SHA-256 re-hash under a `FILE_SHARE_READ`-only handle held until the spawn. A failing companion is never overwritten or deleted, and the run degrades silently to the inbox ConPTY. The existing `ERROR_SHARING_VIOLATION` wait is the model the exe arm is compared against.
- `snapshot.json`'s `pinned_bin` is written only by the instance's own wrapper. P3 reads which readers key on the build hash (`bin/<key>/`, `plugin/<key>/`, snapshot, stamps) for the reproducible-build measurement.
- `ledger/stamps.json` is written only by `viola verify` and read strictly. `verify`'s `pin_exe` runs after its home-tree check and before its stamps write, so a rebuild read as another binary must not let a stamp or pin cross keys.
- The release build route: `cargo build --release --locked --bin viola` through `scripts/release-check.sh`, judged by its own `compiler-artifact` records, refusing test-only features (`test-support`, `fake-agent`), under the exact `rust-toolchain.toml` channel and the committed `Cargo.lock`. The two clean builds use this route from an empty target dir.
- A build change on the measurement's basis (a linker flag, a path remap, a profile key) is the founder's. Every profile keeps `panic = "unwind"`, and no release-workflow `rust-cache` is added.
- A fault on the pin path that reaches a start's refusal or an external error carries a fixed message and code only: no absolute path, hash path or pid.
- The measurement's evidence records (file hashes, section names, sizes) hold no absolute host path or OS username. The CARRY's note that the first build stood only on a tmpfs means the builds are fresh.
- The two-instance witness on a fresh scratch home: unstamped harness boots, or a narrower concurrent-pin control. It runs under the test's own `--home`, and its removal under `target/e2e-home` takes only dirs the test side created.

## Standing conventions
- S1 · each new test or witness uses its own owner-only `--home` and removes only what it created · Secret Management (Development) / security-anti-patterns.md
- S2 · each new log line (a pin wait or race outcome) goes through `obs_event!` and carries no path, pid, hash path or upstream text · Bootstrap phases (`logging-redaction-wire`) / bootstrap-phases.md
- S3 · each new error variant has a fixed `Display`, with path-bearing fields excluded · Error Handling / error-handling.md
- S4 · each new file or directory created under the home (a lock sibling, a temp file) has an explicit 0700 / 0600 mode, never the umask · Data Protection / data-protection.md
- S5 · each new CI artifact or upload runs only after the artifact canary `secret-scan` passes · Secret Management / secret-management.md
- S6 · each new dependency (a file-lock or comparison crate) is workspace-pinned, passes `cargo deny check` and builds no C, and none enters a sync crate's graph as tokio · Dependency Security / dependency-security.md
- S7 · each new or changed CI job or step pins actions by SHA, keeps `permissions: {}` / `contents: read` and passes no event value through `${{ }}` in `run:` · Dependency Security (CI integration) / dependency-security.md

## Not needed
- IPC listener hardening (Windows SDDL, the Unix socket directory, the peer euid check) and the client server verification: «Unix endpoint and home hardening»'s, and the chunk opens no endpoint.
- Paste-text, dialog free-text, channel-frame and MCP tool-input validation: no `send`, `answer` or frame path changes.
- The statusline source read, its `/bin/sh -c` shell-out and the `settings.json` override: untouched.
- The budget gate, `config.json` thresholds and the dialog hold: untouched.
- `viola revive`'s session-id argv, its preflight and the `claude agents --json` parse: untouched.
- `viola verify`'s interactive PTY Runs A–E, its dialog answers and the `--record` fixture scrub: only verify's `pin_exe` call is reached.
- Windows DLL search restriction (`restrict_dll_search`) and the `.cmd` / `.bat` child refusal: no spawn or load order changes.
- Terminal colour facts, `wait` / `last` / `list` human-mode escaping: no output surface changes.
- R8 strip and the inherited `CLAUDE*` credentials: no environment handling changes.
- rmcp features, the MCP transport and the `viola mcp` tools: untouched.
- The `fuzz/` workspace exemption and its lockfile audit: not reached.
- v1.x release prerequisites (dist, signing, cargo-auditable, `self_update`): release packaging is «Version done-check»'s.
- The vendored ConPTY package's vendor audit (`conpty-vendor.sh`, its four pins): the companions' bytes and pins do not change.
- `windows-mutants.yml` and the mutation scratch directory: not reached.
