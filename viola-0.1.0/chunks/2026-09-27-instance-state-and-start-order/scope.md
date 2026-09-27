# Scope — Instance state and start order

**Marker:** `2026-09-27-instance-state-and-start-order` · viola-0.1.0 · Epoch 2 — Windows slice I: wrapper, events, ledger

**Working entry (verbatim title + hint):** Instance state and start order — append-only event log, atomic snapshot,
heartbeat, pinned bin/ copy, plugin folder, documented start order, live/stale name refusal

## What this chunk builds

The seven deliverables the entry names, each as a bullet the plan must cover:

1. **Append-only event log.** Each instance has `instances/<name>/events.ndjson`: one `write` per ndjson line, every
   line carries `v`, the file is never truncated, and it is created 0600 in a 0700 dir. The file is written by the
   wrapper (`viola run`).
2. **Atomic snapshot.** `instances/<name>/snapshot.json` is written by temp file + mode set before commit + rename.
   The wrapper is its only writer, and a `.lock` sibling guards it. It carries at least `endpoint`, `pinned_bin`,
   `pid`, `started_at` and `child_pid` (test-plan Critical Path 1).
3. **Heartbeat.** The wrapper touches `instances/<name>/heartbeat` every 1 s. A beat older than 5 s is `stale` when
   the snapshot's pid + start time still match a live process, and `gone` otherwise (arch §Standard Contracts →
   Session liveness, `architecture.md:310`; verified at P3).
4. **Pinned `bin/` copy.** At start, the running `viola` exe is copied to `bin/<version>-<hash>/viola` (0700). The
   hash is a truncated SHA-256 content hash. Before reuse, the pinned copy is re-hashed, and a mismatch exits 1
   (test-plan §6 exit-cause matrix; security-plan Vector 7).
5. **Plugin folder.** Each start atomically rewrites the embedded plugin files at
   `plugin/<version>-<hash>/{.claude-plugin/plugin.json, hooks/hooks.json, .mcp.json}`, then passes the folder to the
   child with `--plugin-dir`. Every exec-form `command` in them is the absolute forward-slash pinned path. This chunk
   owns the folder, the per-start rewrite and the path substitution.
   [premise-corrected: `settings.json` lives under `instances/<name>/` (arch §Occupied Resources), and working-route:67
   "Statusline pass-through" names "settings.json rewritten each start with absolute pinned path", so settings.json is
   that entry's, not this chunk's.]
   [premise-corrected: HEAD has no `hook` / `mcp` verb (`src/cmd/mod.rs:28-31`). A hook registered now would run a
   verb clap rejects with exit 2. What `hooks.json` / `.mcp.json` carry in this chunk is a P4 fork (research Open
   question 2).]
6. **Documented start order.** `viola run` executes the start sequence in its documented order: collision check →
   pinned copy + plugin folder → version gate → (exclusive endpoint bind) → first snapshot + heartbeat → start
   events → child spawn, and only then the spawn. The endpoint bind is "Wrapper channel" (working-route:40,
   verified). This chunk leaves the bind's slot empty, and its witness asserts the order of the steps it lands.
   The version gate needs `ledger/stamps.json`, which is "Capability ledger and viola verify" (working-route:47,
   verified). Its slot here records `cli_verified:false` and no `cli_version` (both are optional per arch
   `architecture.md:246,249`).
   [premise-corrected: this chunk emits the start events. The wheel starts with `driver` (`architecture.md:69`) →
   `wheel{holder:"driver", cause:"start"}`. With no budget reading, viola records `unknown` and does not block
   (`architecture.md:71`), and `budget.json` has no writer at HEAD → `budget-gate{paused:false}`. Both are appended
   "once at start" (`architecture.md:290`).]
7. **Live/stale name refusal.** A second `viola run` of a name whose heartbeat is fresh (≤ 5 s) exits 1 with
   `unable: <name> is already live` / `hint: viola list`. One whose beat is older than 5 s while its pid + start
   time are still alive (`stale`) also exits 1, with design's "still running but not answering" pair; both log
   `detail:"already-live"`, and the schema enum has no stale-specific code. A `gone` instance (beat older than 5 s,
   process dead) is taken over: its directory is reused and `events.ndjson` is appended to, never truncated.
   [premise-corrected: stale refuses and only `gone` is taken over, per arch `architecture.md:92,310`, test-plan §6
   exit-cause matrix, and design cli pattern 2.] Squatted-endpoint refusal needs the endpoint, so it belongs to the
   channel chunk (verified).

## Folded CARRYs (from the working entry, re-verified at P3)

- **CARRY 1 (sha2).** Chunk 2026-09-25-security-prerequisites pinned `sha2 =0.11.0` (`default-features = false`) in
  `[workspace.dependencies]`. Today it is consumed only as a root `[dev-dependencies]` by
  `tests/contract_content_hash.rs` (FIPS 180-2 vectors + the 16-hex truncation). `viola-state` takes it as a normal
  dependency for the pinned-exe re-hash, and this chunk lands the re-hash refusal test (test-plan §6 exit-cause
  matrix: a pinned exe failing its SHA-256 re-hash → exit 1). Measured at take-up: `Cargo.toml:129` holds the pin,
  `Cargo.toml:81` the root consumer.
- **CARRY 2 (`VIOLA_*` at spawn).** Chunk 2026-09-25-pty-wrapper-on-windows set no `VIOLA_*` (operator ruling 3).
  This chunk sets `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` and the `PATH` prefix at child spawn. The spawn is
  `src/run/mod.rs` → `viola-pty` `SpawnSpec`, and its env today is inherited minus the R8 strip. This chunk lands
  test-plan §6 E2's `VIOLA_NAME`/`VIOLA_DIR` presence assertion (the strip half is `tests/tui_env_strip.rs`), and
  builds the `viola_e2e::fixtures` chain copy if its E2 completion is that copy's first Tokio-side consumer.
  Closed at P3: `SpawnSpec.env_set` exists (`crates/viola-pty/src/lib.rs:44`, empty at `src/cmd/run.rs:74`). E2's
  presence half extends the root sync `tests/tui_env_strip.rs` (`wrapped_env`), which is not a Tokio-side consumer,
  so the CARRY's condition is false and the `viola_e2e::fixtures` copy is not built here.

## Harness surfaces that grow with this producer (added at P5 validation-1, intent-incomplete)

- Test-plan §3 grows the harness "check by check once its surface lands", and this chunk lands `snapshot.json`,
  `heartbeat` and `events.ndjson`. So boot readiness (`harness/boot.rs`, root `tests/support/home.rs`) adds the
  snapshot and heartbeat checks, and `logs` gains the events source and `--kind`, which `harness/logs.rs:4` defers
  to "its producer".
- The fake agent's `start` receipt gains `started_at` (Path 1's spawn-last proof) and `plugin_dir` (the
  `--plugin-dir` witness).

## Boundaries (not this chunk)

- No `viola-channel` crate and no endpoint bind or SQOS client ("Wrapper channel", working-route:40; verified). The
  snapshot's `endpoint` stays unset until that chunk.
- No hook runtime or event normalisation ("Hooks to normalised events", working-route:43; verified). This chunk's
  event-line writer carries only the two start kinds.
- No torn-line healing, snapshot rebuild by replay or unknown-kind counting ("Self-healing state", working-route:65;
  verified).
- [premise-corrected: strict-modes on read and the Windows DACL belong to "Home and code-bearing file integrity"
  (working-route:89) per the security-plan sidecar's recorded deferral (three-os-ci-harness-skeleton). Only the
  0700/0600 creation half lands here.]
- No `run.*` / `state.*` spans: working-route:40's CARRY lands the workspace's first spans with the channel entry
  (verified; the Scenario 1 span set is the wrap's route freight to that CARRY).
- `viola list --json` / `viola ui` (Path 1 step 2) are other verbs, and this chunk's witness reads the disk state
  directly (verified: `src/cmd/mod.rs` has `Run` only).

## Surfaces and contracts touched

- New crate `crates/viola-state` (no Tokio; `deny-sync.toml` sole-root run applies). It is absent at HEAD (verified:
  `ls crates` → `viola-agent-claude`, `viola-core`, `viola-e2e`, `viola-pty`).
- `src/run/` + `src/cmd/run.rs` (the start sequence and the spawn env), `viola-pty` `SpawnSpec` (env).
- `plugin/` (embedded via `include_str!`). It is absent at HEAD (verified: `ls plugin` → no such directory).
- Arch §Standard Contracts (event line, snapshots), §Occupied Resources (home tree, files, env vars),
  §Infrastructure Patterns (directory tree). Security-plan (home modes, pinned exe, plugin absolute paths). Obs-plan
  (start-sequence spans, process-log events). Test-plan §6 Path 1, E2 and the exit-cause matrix.

## CI verdict read at Setup (5a)

- `a892917` (the last wrap's flip = HEAD; the only sha in range): 15/15 check-runs `completed · success`, wall-clock
  01:32:17Z → 01:35:35Z (3 m 18 s). The overseer relayed run id 36285830440 for this sha. No red, nothing folded.
