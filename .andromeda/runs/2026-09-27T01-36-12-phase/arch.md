# arch extract

## Relevance
Relevant. This chunk lands the instance state layer that architecture keeps returning to, and it creates the planned `viola-state` crate.

## Constraints
- **Start order (per architecture §Established Decisions [Session Liveness]).** `viola run` must run these steps in this order: collision check → pinned copy + plugin folder → version gate → exclusive endpoint bind → first snapshot (with `endpoint`, `pinned_bin`, `statusline_command`) + heartbeat → start `wheel` + `budget-gate` events → child spawn. The point of the order is that the child's first SessionStart always finds a recorded snapshot.
  - The bind slot belongs to the channel chunk.
  - Whether a version gate already exists in code is research's question.
- **Refusal vs takeover (per architecture §Established Decisions [Session Liveness] and §Standard Contracts → Session liveness).**
  - A name with an answering endpoint or a `stale` heartbeat must refuse to start: exit 1, with a message naming the live instance.
  - Only a gone instance's directory is reused, and its `events.ndjson` is appended to, never truncated.
  - Architecture defines `stale` as a beat older than 5 s whose pid + start-time check still says the process is alive. It defines gone as a beat older than 5 s whose process is dead.
  - Scope item 7 says "a stale instance … is taken over". That contradicts architecture, where stale refuses and only gone is taken over. P3 must settle which one holds.
- **Heartbeat (per architecture §Standard Contracts → Session liveness).** The carrier is a separate file, `<instance dir>/heartbeat`, which the wrapper touches every 1 s. The staleness threshold is 5 s. Readers match pid + `started_at` from the snapshot, never a bare pid. This answers the scope's [inferred] question about carrier and period.
- **On-disk shapes (per architecture §Standard Contracts: ndjson event line, Snapshot envelope, Instance snapshot).**
  - Every event line is `{v, ts, instance, kind, source, data}`. The start events use `source: wrapper`.
  - The snapshot envelope is `{v, written_at, writer, data}`, where `writer` = `CARGO_PKG_VERSION`.
  - The instance snapshot's `data` has these fields: `{endpoint, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid, agent_session_id?}`. Only the wrapper writes it, and it rewrites it on every field change. Because of that rule, `child_pid` lands in a rewrite after the spawn.
  - `wheel{holder, cause:"start"}` and `budget-gate{paused, window?, override_until?}` are appended "once at start" (per architecture §Standard Contracts → Event `data` per kind).
- **Filesystem layout (per architecture §Occupied Resources → Filesystem).**
  - Instance directory: `instances/<ViolaName>/{events.ndjson, events.ndjson.lock, snapshot.json, snapshot.json.lock, heartbeat, settings.json, diagnostics/}`.
  - Pinned copy: `bin/<version>-<hash>/viola(.exe)`.
  - Plugin folder: `plugin/<version>-<hash>/{.claude-plugin/plugin.json, hooks/hooks.json, .mcp.json}`.
  - v1 never deletes pinned copies or plugin folders.
  - Architecture places `settings.json` under `instances/<name>/`, not in the plugin folder as scope item 5 implies (per architecture §Occupied Resources → Claude Code integration names).
- **Pinning and the plugin (per architecture §Established Decisions [Deployment / Distribution] and §Stack and Technologies Content hash row).**
  - The key is `<CARGO_PKG_VERSION>-<first 8 bytes of SHA-256 of the exe as 16 hex>`, hashed with sha2 `=0.11.0` (`default-features = false`). `viola-state` is named as its first product consumer.
  - The exe is copied only if the copy is absent.
  - The plugin files are embedded with `include_str!`, and `plugin.json`'s version comes from `CARGO_PKG_VERSION`.
  - The copy's forward-slash path is substituted into every exec-form `command` in `hooks.json` and `.mcp.json`. The hook shape is `"command": "<pinned copy>", "args": ["hook","<event>"]` (per architecture §Established Decisions [Hook Transport]).
  - The folder is passed to the child with `--plugin-dir`.
- **Child environment (per architecture §Occupied Resources → Environment variables and §Cross-cutting Patterns → Config management).**
  - `VIOLA_NAME` = the `ViolaName`.
  - `VIOLA_DIR` = the absolute instance directory. It is the only way `--home` reaches the child.
  - `VIOLA_BIN` = the pinned copy's path with forward slashes.
  - `PATH` is prefixed with the pinned copy's folder.
  - The R8 `CLAUDE*` strip stays in place.

## Patterns to follow
- **Separate lock files (per architecture §Established Decisions [Database / State Store] and §Cross-cutting Patterns → Crash-safe disk writes).**
  - Append-only logs are written with std `OpenOptions::append`, one `write` per line.
  - Snapshots are replaced atomically with atomic-write-file 0.3.1 (same-directory temp + fsync + rename).
  - Locking uses std `File::lock` on a separate `<name>.lock` sibling, never on the log itself, because append plus an exclusive lock fails on Windows (rust-lang/rust#54118).
- **The new crate (per architecture §Established Decisions [Module Boundaries] and §Infrastructure Patterns → Crate dependency direction).**
  - Create `crates/viola-state` as its first consumer. Planned dependencies: `viola-core`, atomic-write-file, notify, sysinfo, and chrono directly, plus the shared serde / serde_json / thiserror from `[workspace.dependencies]`.
  - Its error type is a `StateError` thiserror enum (per architecture §Conventions → Rust error types).
  - Mirror the landed sync crates: `publish = false`, `license.workspace = true`, `[lints] workspace = true`, and a code-free `fake-agent = []` feature (per architecture §Occupied Resources → Workspace crates).
- **Sync-crate enrolment (per architecture §Infrastructure Patterns → Build system Dependency policy, CI target job 3, and §Cross-cutting Patterns → Tokio containment).** Add `viola-state` to `scripts/sync-crates.txt`. The sole-root `deny-sync.toml` tokio ban and the per-crate `cargo check` then cover it. `run`'s start path stays on std threads and blocking I/O.
- **Formats (per architecture §Conventions → Data model conventions / Naming patterns).**
  - Timestamps are RFC 3339 UTC with milliseconds and `Z` (`to_rfc3339_opts(SecondsFormat::Millis, true)`), in `ts` or `*_at` fields.
  - Optional fields use `Option<T>` with `skip_serializing_if`.
  - JSON field names are snake_case, and wire enum values are kebab-case.
  - Files are lower-case `.ndjson`, `.json` or `.lock`.
- **Test homes (per architecture §Cross-cutting Patterns → Config management and §Established Decisions [CI/CD]).**
  - Home resolution order: `--home`, then the grandparent of `VIOLA_DIR`, then the default.
  - Each test runs in its own home with the fake agent named after `--`.
  - The witness reads disk state from that home.

## Anti-patterns to avoid
- **Never truncate or rotate `events.ndjson`.** `wait`'s `after`, `send`'s `cursor` and SSE ids are byte offsets into it. A reused directory is appended to (per architecture §Occupied Resources → Filesystem and §Established Decisions [Session Liveness]).
- **Plugin and hook commands must not rely on PATH.** Every exec-form command in the plugin files is the absolute forward-slash pinned path. Nothing shells out through `sh`, `bash` or `cmd`. No std `DefaultHasher` for any name that mixed binary versions must agree on (per architecture §Established Decisions [Deployment / Distribution], §Cross-cutting Patterns → Cross-platform discipline, and §Conventions → Endpoint name).
- **No Tokio in `viola-state`, directly or transitively** (per architecture §Established Decisions [Concurrency]).
- **Don't write to the terminal while the child runs.** Codes-only process lines go to `diagnostics/run-<name>.ndjson` and content goes to `instances/<name>/diagnostics/detail-run.ndjson` (per architecture §Cross-cutting Patterns → Diagnostic output channels).

## Contract bindings
- **arch ↔ security**
  - 0700 directories and 0600 files, mode set before the atomic rename, and a pinned-exe re-hash that exits 1 on mismatch. Architecture fixes the paths and the hash, and security owns the enforcement.
  - Whether strict-modes checks on read belong to this chunk is decided by the security plan.
- **arch ↔ obs**
  - The start-sequence process-log events and the exit-1 refusal line follow §Diagnostic output channels.
  - The refusal's "message naming the live instance" is a human stderr site. Today the root bin has exactly one local `print_stderr` allow, for the `.cmd`/`.bat` refusal (per architecture §Infrastructure Patterns → Build system Lint). A second site would need a new allow, which becomes a wrap amendment and a lint-contract binding.
- **arch ↔ tests**
  - Test-plan §6 Path 1, E2 (presence of `VIOLA_NAME` and `VIOLA_DIR`) and the exit-cause matrix (live-name refusal, re-hash mismatch) bind to the §Occupied Resources paths and env names.
  - Fake-agent isolation per test home through `--home`.
- **arch ↔ channel chunk (next)**
  - The snapshot's `endpoint` value and the exclusive-bind slot in the start order. The FNV-1a `viola-<h12>` name is computed in `viola-channel` (per architecture §Conventions → Endpoint name).
  - Whether this chunk writes a placeholder or leaves the field out is a P3 question.
- **arch ↔ wheel / budget chunks**
  - Architecture puts the start `wheel` and `budget-gate` events inside `run`'s start order. Which chunk emits them is scope's open P3 premise.

## Acceptance criteria contributions
- A start witness records the steps this chunk lands in the documented order: collision check → pinned copy + plugin folder → version gate → first snapshot + heartbeat → start events → child spawn, with the spawn last. On a second start of a gone instance, the existing `events.ndjson` is appended to, not truncated (per architecture §Established Decisions [Session Liveness]).
- A second `viola run` of a name whose heartbeat is fresh (or `stale`, pid + start time alive) exits 1 before any spawn. A name whose heartbeat is older than 5 s and whose process is dead is taken over. The heartbeat file is touched about every 1 s while the wrapper runs (per architecture §Standard Contracts → Session liveness).
- `crates/viola-state` exists, is listed in `scripts/sync-crates.txt`, and passes the sole-root `deny-sync.toml` tokio ban. `snapshot.json` parses as the envelope `{v:1, written_at, writer, data}` with the Instance snapshot fields, and every `events.ndjson` line carries `v:1`, `ts`, `instance`, `kind`, `source`, `data`. Each guarded file has its own `.lock` sibling (per architecture §Infrastructure Patterns → Build system Dependency policy and §Standard Contracts).
- The pinned copy lives at `bin/<CARGO_PKG_VERSION>-<16 hex of SHA-256>/viola(.exe)`. Every exec-form `command` in the written `hooks.json` and `.mcp.json` equals that copy's absolute forward-slash path. The child sees `VIOLA_NAME`, `VIOLA_DIR` (absolute instance dir), `VIOLA_BIN` and a `PATH` that starts with the copy's folder (per architecture §Established Decisions [Deployment / Distribution] and §Occupied Resources → Environment variables).

## Relevant amendment history
- **2026-09-25-security-prerequisites.** Added the Content hash row: sha2 `=0.11.0` (`default-features = false`), truncated to 16 hex, "a root dev-dependency until `viola-state`". The pin was placed ahead of time so this chunk could consume it (scope CARRY 1).
  - The §Crate dependency direction entry for `viola-state` does not list sha2 yet, so moving it to a normal dependency is a wrap amendment.
- **2026-09-25-pty-wrapper-on-windows.**
  - Recorded the as-built spawn: viola resolves the program itself, sets the child cwd explicitly, uses `HostTerminal`, and keeps the ConPTY input writer.
  - Recorded R8 as a `CLAUDE*` prefix rule with an 11-name identity floor, and the root bin's windows-sys registry reader in `src/run/env.rs`.
  - Recorded the root bin's single `print_stderr` allow and the exit-1 `batch-script-child` refusal.
  - That chunk set no `VIOLA_*` (scope CARRY 2). This chunk adds them on top of the same spawn env.
- **2026-09-24-supply-chain-and-workflow-gates.** Moved the tokio ban to a sole-root run of `deny-sync.toml` per crate in `scripts/sync-crates.txt`, because `--exclude` false-fails under feature unification. `viola-state` joins through that list.
- **2026-09-24-diagnostics-plane.** Process logs now go to the home-level `diagnostics/run-<name>.ndjson`, and instance `diagnostics/detail-<role>.ndjson` files are owner-only. It also recorded the `v` exception: process-log lines carry no `v`, while events and snapshots still must.
- **2026-09-24-three-os-ci-headless-harness-skeleton.**
  - [Module Boundaries] now says each product crate is created by its first consumer. That is why `viola-state` is created here.
  - At wrap, the new crate is registered in Workspace crates → "Landed so far", the rust code-graph plane member list and the Licence inheritance list.
- **2026-09-26-local-linux-pre-push-gate.** Added the `FAKE_AGENT_PUMP_DELAY_MS` seam: `hold_pump_start` in `src/cmd/run.rs`, behind `cfg(feature = "fake-agent")`, which runs after the child starts. Reordering the start sequence in `src/cmd/run.rs` must keep this seam after the spawn.
