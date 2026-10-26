# Task — arch · 2026-10-26-home-and-code-bearing-file-integrity

## The chunk
Makes the home's owner-only rules whole at every entry point: the strict-modes check (Unix mode/owner, Windows owner + DACL) runs before any read or open of the home by `send`, `wait`, `last`, `answer`, `pause`, `release`, `viola hook`'s nine events (failing open), `viola mcp`'s frame tools, `verify`'s stamps read and `run`, closing the dated strict-modes exceptions.
Puts that check before `client::start`'s role-file open (an open narrows an existing home to 0700) and reads the Windows owner + DACL before any diagnostics or role file, with a Windows proof of `check_instance` and a reading of a home a test seeded first.
Re-hashes the pinned binary before reuse and owns its home copy (the `pin_exe` race of two first starts on a fresh home), adds the Windows `conpty/` companions to the strict-modes set, and keeps the per-start atomic plugin rewrite with absolute pinned `command`s.
Re-reads that only the wrapper writes `snapshot.json` and only `viola verify` writes `ledger/stamps.json`, and the `marketplace/` tree's integrity; re-measures whether two clean release builds of one tree hash as one binary.
The snapshot readers' move onto the replayed read is a founder fork at P4, neither planned in nor out until then.

## Reached files
- crates/viola-state/src/strict.rs · edits check_instance (its Windows proof), check_path
- crates/viola-state/src/liveness.rs · edits live_endpoint (the snapshot read behind the check)
- crates/viola-state/src/stamps.rs · edits update_stamps (the stamps read)
- crates/viola-state/src/pin.rs · edits pin_exe (the re-hash and the race), pin_companions (the conpty set)
- crates/viola-state/src/fs.rs
- crates/viola-state/src/snapshot.rs
- src/cmd/client.rs · edits start (the role-file open)
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits the snapshot read through VIOLA_DIR
- src/cmd/hook/statusline.rs
- src/cmd/verify.rs · edits the update_stamps call
- src/cmd/run.rs · edits pin_and_plugin, the home and instance checks, the collision check
- src/cmd/plugin.rs
- src/obs.rs
- crates/viola-mcp/src/tools.rs · edits the frame tools' live_endpoint read
- tests/cli_controls_not_disableable.rs · edits the 0770 --home negative, the completeness case
- tests/support/home.rs · edits seed_conpty
- .claude/rules/security.md · edits §Local trust boundary's dated strict-modes exceptions

## Needs
- The instance strict-modes check (`check_instance` / `check_path`) as an entry-point rule: run before the first read or open under the home, never after a role-file or log open, at every verb named in The chunk
- The `instances/<ViolaName>/` filesystem entry: its files (`snapshot.json`, `events.ndjson`, their `.lock` siblings, `heartbeat`, `settings.json`, `diagnostics/`) and the rule that a verb checks them before use, ahead of any log, now extended past `list`, `revive`, `hook statusline`, `link` and `unlink`
- The home-level `diagnostics/` role files (`run-`, `hook-`, `mcp`, `cli-`) and the instance `diagnostics/detail-*.ndjson`: opened only after the check, the Windows owner + DACL read first
- The pinned copy `bin/<version>-<hash>/viola`: content-hash key, created 0700 if absent, re-hashed on every start, `pinned-hash-mismatch` refusing the start (exit 1) and leaving the file as found, never deleted; two first starts on a fresh home racing to create it
- The Windows x64 companions `bin/<version>-<hash>/conpty/`: written if absent and never overwritten, held `FILE_SHARE_READ` and fully re-hashed until the spawn, a mismatch failing open to the inbox ConPTY with no refusal; the folder and both files joining the Windows strict-modes set
- The Deployment / Distribution decision: `run` pins its own exe, refuses a copy failing its re-hash and rewrites the plugin for that copy, loaded by `--plugin-dir`; hooks and MCP never search PATH
- The `plugin/<version>-<hash>/` files (`plugin.json`, `hooks.json`, `.mcp.json`): rewritten atomically on every start, each exec-form `command` the absolute pinned path with forward slashes
- The Snapshot writer decision: one helper `replace_private`, mode set before any byte; the instance snapshot written only by its own wrapper
- The Ledger stamps envelope and `ledger/stamps.json` entry: one writer `update_stamps`, called only by `viola verify`; `run`'s gate reads behind the strict check, a refusal recording `cli_verified:false`; verify's own read joining the checked set
- The Hook contract's fail-open rule: a refused check in `viola hook` is exit 0, no body, nothing on stderr, never exit 2
- The CLI exit codes and `--json` error document: a refused check of a frame verb is exit 21 with the `instance-unreachable` document (codes only)
- The MCP result shape: a refused check in a frame tool is the `isError` `instance-unreachable` result with `detail: null`, no frame written
- The Instance snapshot's `pid` and `started_at`: the server check every frame-writing client already makes stays, the strict-modes check added before the same snapshot read
- The Snapshot envelope's replay rule (an unreadable or newer snapshot rebuilt from `events.ndjson`, without `endpoint`, `pid` or `started_at`): the subject of the P4 fork over `read_snapshot_or_replay`
- Session liveness: `live_endpoint` reading endpoint, pid and start time from one snapshot read, and the heartbeat
- Home resolution (Config management): `--home`, else the grandparent of `VIOLA_DIR`, else `<user home>/.viola/`; the hook's snapshot read through `VIOLA_DIR` with no canonicalisation today
- The `marketplace/` tree: written only by `viola plugin install` after its check, an unknown entry left as found, never read back, its `.mcp.json` `command` the canonical installing exe with no re-hash
- The Windows protected owner + SYSTEM DACL set at creation for a home outside `%USERPROFILE%`, and a home `seed_conpty` created first being refused by `check_instance`
- The `target/conpty-seed/<key>/` test resource and `seed_conpty`'s carve-out: the seed must let viola create the home first once viola sets its own DACL
- The Config management rule that no `config.json` key, `VIOLA_*` env var or flag disables a control: the controls table's completeness case over every `VIOLA_*` var (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`), global flag and `config.json` key
- The Diagnostic output channels rule: refusals carry no path, pid, start time or endpoint on any line or output; `mcp` writes stderr only when its role file cannot be opened
- The release build's reproducibility against the content-hash key: two clean release builds of one tree compared by file and by section
- The Windows `RUNNER_TEMP` negative case for a home with a widened DACL, proven on CI's `windows-2025` leg

## Standing conventions
- S1 · every new test runs in its own viola home through `--home`, under `target/e2e-home` · Established Decisions ([CI/CD]) / .andromeda/architecture-detail/occupied-resources.md
- S2 · every OS-specific branch compiles and is tested on its own CI runner · Cross-cutting Patterns (Cross-platform discipline)
- S3 · new code on the `run`, `hook`, `send` or channel paths uses std threads and blocking I/O; only `viola-mcp` builds a runtime · Cross-cutting Patterns (Tokio containment)
- S4 · a new error type is one thiserror enum per crate with fixed messages, anyhow only in the root bin · Conventions (Rust error types) / .andromeda/architecture-detail/conventions.md
- S5 · a new test-only path or resource is recorded in the Occupied Resources filesystem or repository list · .andromeda/architecture-detail/occupied-resources.md

## Not needed
- Unix socket directory hardening, the client euid check and macOS server verification (IPC endpoints): owned by "Unix endpoint and home hardening"
- Bounded readers and the `MAX_FRAME` cap on readers: owned by "Bounded inputs at every boundary"
- A new exit-cause code or `run`'s start-refusal document: owned by "Exit-cause code catalogue"
- The wrapper channel methods' `params` and `ok` payloads and the frame envelope: no method or frame shape changes
- Event data per kind and the normalised event kinds: no event kind is added
- The `list` payload and the `claude agents --json` join: `list` already checks its home and its board is unchanged
- The Human Takeover / Wheel and Links decisions: no wheel or link behaviour changes
- The Screen Model and Delivery Confirmation decisions: no readiness or confirmation change
- The capability-ledger rows and their `viola verify` probes: no new CLI behaviour is relied on
- The Hook Transport tiers and the Hook → kind map: unchanged
- The MCP tool set and rmcp: no tool is added or removed
- The deferred frontend and GUI contracts: not in 0.1.0
- The fuzz workspace and the recorded fixtures: untouched
