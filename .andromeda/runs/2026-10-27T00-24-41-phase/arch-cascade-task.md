# Task — arch · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The chunk
Closes the pinned-copy race: two first `viola run`s (or a `run` and a `viola verify`) of one version on a fresh home must not both replace `bin/<version>-<hash>/viola(.exe)`, and no agent's hook spawn of the pinned path may fail because another start's write of it is in flight.
The fix acts only on the absent-copy write's concurrency in `viola_state::pin::pin_bytes` / `pin_exe` (through `replace_private_shared` → `replace_private`), resting on a mechanism P3 or a two-sided reproduction establishes (red before, clean after, beside the single-instance control), never on the `ETXTBSY` hypothesis alone.
It reads whether the same race reaches `pin_companions`' exe-side counterpart on Windows and the per-start rewrite of `plugin/<version>-<hash>/` for a concurrent reader of `hooks.json` / `.mcp.json`.
It measures whether two clean release builds (each from an empty target dir) of one committed tree give one file, and where not, which sections and notes differ, through `scripts/release-check.sh`'s build; it records no unmeasured cause, and any build or product change on that basis is the founder's fork.
It re-asserts unchanged the hash-before-reuse rule: a present copy is re-hashed and refused on any difference, never overwritten.

## Reached files
- crates/viola-state/src/pin.rs · edits pin_bytes, pin_exe (the absent-copy arm)
- crates/viola-state/src/fs.rs · edits replace_private_shared, replace_private (what is held open across the rename)
- src/cmd/run.rs · edits run's start pin_bytes call, pin_companions, the per-start plugin rewrite
- src/cmd/verify.rs · edits verify's pin_exe call
- scripts/release-check.sh · re-reads its release build (the profile and the CARGO_TARGET_DIR it uses)

## Needs
- The pinned copy `bin/<version>-<hash>/viola(.exe)`: created by `run` only when absent, 0700 on Unix, `<hash>` the first 16 hex of the SHA-256 of the exe's content; an existing copy re-hashed on every start, a mismatch refusing the start (exit 1, `pinned-hash-mismatch`) and leaving the file as found; v1 never deletes pinned copies or plugin folders.
- [Deployment / Distribution]: the content hash keys the copy because the version alone collides across rebuilds; the plugin is rewritten for that copy at every start and loaded with `--plugin-dir`; hooks and MCP never search PATH — the rationale the reproducible-build measurement tests.
- [Snapshot writer]: every replaced file goes through the one helper `viola_state::fs::replace_private` (mode set before any byte, write, `sync_all`, `persist`, a failure leaving the target as it was); a fix for the race must stay inside that helper, never a call site's own temp + rename code.
- tempfile 3.27.0 `persist` semantics as the helper uses them (the rename over an existing target) — the surface whose open handles P3 reads.
- [Database / State Store]: any guard the fix adds is a std `File::lock` on a separate `.lock` sibling, never an exclusive lock on the guarded file itself (append + exclusive lock fails on Windows); one lock file per guarded file, named `<name>.lock`.
- The home tree check before any log (`viola_state::strict::check_home_tree` over `bin/`, `bin/<version>-<hash>/`, the pinned copy, `plugin/<version>-<hash>/` with `hooks/` and `.claude-plugin/`): `run` pins the very exe bytes it checked, which the fix must keep; a new `.lock` or temp entry under `bin/<key>/` or `plugin/<key>/` must pass that check's write rule.
- The ConPTY companions `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (Windows x64): written by `pin_companions` through `replace_private_shared` only when absent, never overwritten, waiting out `ERROR_SHARING_VIOLATION`; the strict-modes check, the `FILE_SHARE_READ`-only held open and the full SHA-256 re-hash before the spawn; a failure degrades silently to the inbox ConPTY (`sideload_fallback`).
- The plugin tree `plugin/<version>-<hash>/.claude-plugin/plugin.json`, `hooks/hooks.json`, `.mcp.json`, rewritten atomically at every start, each exec-form `command` the absolute pinned path.
- [Hook Transport] and [Hook Contract]: the agent spawns the pinned path as `"command": "<pinned viola copy>", "args": ["hook", "<event>"]`; a hook spawn that fails loses the event (the `session-start` that never lands), and `hook` itself always exits 0 — the race's observed symptom.
- The child's view of the pinned copy: `--plugin-dir <viola home>/plugin/<version>-<hash>` first on every start, `VIOLA_BIN` the pinned path with forward slashes, `PATH` prefixed with the pinned copy's folder.
- Instance snapshot `pinned_bin`: each instance records the pinned path it uses; with the stamps envelope (per CLI version, written only by `viola verify`) the readers that may or may not key on the hash, which P3 reads for the reproducible-build impact.
- [CLI Version Compatibility]: `viola verify` pins the exe through `pin_exe` before its probes, the second writer racing a `run`'s start on a fresh home.
- [Concurrency / Backend Framework] and Tokio containment: `viola-state` is a sync crate (`scripts/sync-crates.txt`), and the fix on the `run` and `verify` paths uses std threads and blocking I/O only.
- [Error Handling]: a new failure mode of the absent-copy write is a variant of `viola-state`'s thiserror enum with a fixed message (no path, no payload); anyhow only in the root bin.
- Start-refusal exit codes: a refused start exits 1 and starts no child; the race fix adds no new exit code outside the Conventions table.
- Disk-state modes: 0700 dirs and 0600 files (the pinned exe 0700) set explicitly, never the umask.
- Repository resource `target/release-check/`: the cargo target dir the local release-check gate passes as `CARGO_TARGET_DIR` (the script honours an inherited one and never sets it) — where the clean-build measurement is made.
- Every Cargo profile keeps `panic = "unwind"`: a profile key, linker flag or path remap proposed on the measurement's basis is held for the founder at P4.
- Test-home carve-outs that touch `bin/<key>/`: `seed_conpty` hard-links the companions into a test home's `bin/<key>/conpty/` before `Wrapper::boot`, except in `run_viola_unseeded` starts and `tests/conpty_sideload.rs`, which keep the product's write path — the path a Windows race witness exercises.
- Windows behaviour is read on the CI runner only (windows-2025); no Windows-host live measurement.

## Standing conventions
- S1 · each new test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json` · Established Decisions [CI/CD] (established-decisions.md)
- S2 · a new test or harness home lives under `target/e2e-home/viola-test-*/home` or `viola-session-*/home`, removed only when its recorded owner process is gone · occupied-resources.md (Repository)
- S3 · a new test-only or measurement scratch path, target dir or evidence file is registered in Occupied Resources and gitignored, and `viola` never reads it · occupied-resources.md (Repository, Filesystem)
- S4 · a new log line goes only through `obs_event!` to the role's codes-only process log under `diagnostics/`, content only in the instance detail file; `run` writes nothing to the terminal while the child runs · Cross-cutting Patterns (Diagnostic output channels)
- S5 · every OS-specific branch compiles and is tested on its CI runner; paths written into plugin files use forward slashes; children are spawned directly, never through a shell · Cross-cutting Patterns (Cross-platform discipline)
- S6 · a test-only binary or feature never reaches a release build, and `scripts/release-check.sh` refuses an artifact built with `test-support` or `fake-agent` · occupied-resources.md (Binary; Workspace crates)
- S7 · a new error type follows the per-crate `<Crate>Error` thiserror enum with fixed messages · Conventions (Rust error types)

## Not needed
- The wrapper channel: JSON-RPC frames, channel methods, `MAX_FRAME` framing and the server identity check — the race fix sends no frame and changes no method.
- MCP tools and MCP result shapes — nothing the driver calls changes.
- The `list` payload, the board and the `claude agents --json` join — no board field changes.
- The ndjson event line, the normalised event kinds and event `data` per kind — the fix appends no event kind.
- The budget governor, `budget.json` and the `hook statusline` contract — untouched.
- The human-takeover wheel, links, delivery confirmation and the vt100 screen model — untouched.
- Session liveness (heartbeat, pid + start time) and the replay readers — untouched.
- `viola revive`'s preflight codes and the CLI `--json` error documents — no refusal code changes (a start-refusal document is owed elsewhere).
- `viola plugin install` and the `marketplace/` tree — it writes no pinned copy.
- Unix socket and socket-directory hardening, the client euid check and `board::scan`'s home rule — «Unix endpoint and home hardening»'s.
- The vendored ConPTY binaries' pins and `scripts/conpty-vendor.sh` — the companions' bytes and pins are unchanged.
- The `fuzz/` workspace and its lockfile — it depends on no crate this chunk edits.
- The capability ledger rows and `viola verify`'s probe runs A–E — no new CLI behaviour is relied on.
