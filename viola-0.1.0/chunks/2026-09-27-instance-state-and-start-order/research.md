# Codebase Research — 2026-09-27-instance-state-and-start-order

## Scope
- **Depth:** moderate · **Reads:** 16 (full or ranged) · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full): 5 Session Additions, and the 2026-09-27 `test(=tests::name)` filter form applies to this chunk's `run --filter` lines. `.claude/rules/testing.md` (read in full): 11 Session Additions. These apply here: every function needs an observable effect (2026-09-24), a `#[cfg(unix)]` body is a minimal reader with its logic in a shared function (2026-09-24), each guard test is paired with a remove-the-guard run (2026-09-25), and a test waits on the exact line it asserts (2026-09-24).
- **Platform issues consulted:** searched for a subject and found none to search for. Scope folds no runner-only
  bullet, and Setup 5a read `a892917` at 15/15 check-runs `success`, so there is no failure signature, image or
  runtime to look up. The plan's single CI-reading entry (the operator `check-runs` read) reads this chunk's own
  push. A red it reads carries that run's signature into the operator pass, where the tracker search runs against it.

## Files inspected
- `src/cmd/run.rs` (full, 115 lines) — `run()` is today's whole start path: obs init → `log_self_start` → config rejection → `persistent_names` → `resolve_program` (`BatchScriptChild` refusal at :59) → `plan_strip` → `SpawnSpec{env_set: Vec::new()}` (:74) → `HostTerminal::enter` → `viola_pty::spawn` (:80) → `log_child_start` → `hold_pump_start` (cfg fake-agent, :82-83) → `pump`. The refusal lines are `writeln!(io::stderr().lock(), …)` (:107-115), not `eprintln!`, so clippy `print_stderr` does not fire on them. A second refusal site of the same form needs no new `#[allow]`.
- `src/cmd/mod.rs` (full, 59 lines) — the `Command` enum has only `Run` (:28-31). There is no `hook`, `mcp`, `list` or `verify` verb at HEAD. Home = `--home`, or `home_dir()/.viola` (:40-46). There is no `VIOLA_DIR` grandparent fallback yet (that is "CLI machine contract").
- `src/run/mod.rs` (full, 86 lines) — `log_self_start` / `log_child_start` (names-only strip fields) / `log_child_exit` / `log_self_exit(code, detail)` (:59-78), and `resolve_program`.
- `src/obs.rs` (:39-60) — the private dir helper (`DirBuilder::mode(0o700)` + `set_permissions`, :39-45) and `open_private_append` (`mode(0o600)` + `set_permissions`, :52-60) are `pub(crate)` in the root bin.
- `crates/viola-pty/src/lib.rs` (:40-47, :138-142) — `SpawnSpec.env_set: Vec<(OsString, OsString)>` exists and is applied before `env_remove`. `VIOLA_*` and `PATH` go in `env_set` with no seam change.
- `crates/viola-core/src/lib.rs` (API grep) — exports only `SERVICE_NAME`, `VERSION`, `MAX_FRAME`, `ViolaName` and `obs`. There is no `Clock`, no event-kind enum and no `RefusalReason` yet (grep `^pub ` over the file).
- `Cargo.toml` (:66-135) — `[workspace.dependencies]` has `chrono =0.4.45`, `serde`, `serde_json`, `sysinfo =0.39.6` (`system`), `sha2 =0.11.0` (`default-features = false`, :129), `tempfile =3.27.0`, `thiserror`, `tracing`. It has no `atomic-write-file`, `notify` or `mock_instant`. `sha2` is consumed only as a root `[dev-dependencies]` (:81).
- `scripts/sync-crates.txt` (full) — `viola-core`, `viola-pty`, `viola-agent-claude`.
- `deny.toml` (:15-23) — `allow = ["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]`, plus per-crate 0BSD exceptions for interprocess's two deps only.
- `src/bin/viola-fake-agent.rs` (:190-235, :315-335) — reads `<plugin-dir>/hooks/hooks.json`. If there are no commands it returns `"no-hooks"`. If there is no fixture it returns `"no-fixture"`. A non-absolute command is receipted `command_absolute:false, ran:false`. The `env` receipt lists every env name (:328).
- `tests/tui_env_strip.rs` (:1-90) — E2's strip half: `wrapped_env()` boots `viola --home H run builder -- FAKE --receipt …` in an `OuterPty`, reads the `env` receipt names and the role file's `claude-child` `process-start`. This is the file E2's `VIOLA_*` presence half extends.
- `tests/support/home.rs` (:125-183) — `Wrapper::boot` + interim `wait_ready`: `process-start` self + claude-child, plus the fake `start` receipt, bounded by `READY_WITHIN` and exit-aware.
- `crates/viola-e2e/src/harness/boot.rs` (:189-280) — `role_state` / `readiness` / `wait_ready` read only the `process-start` lines (interim).
- `crates/viola-e2e/src/harness/mod.rs` (:140-180) — `ProcessId{pid, started_at: u64}` from sysinfo `Process::start_time()`, and `alive()` = same pid + same start time + not a zombie. This is the liveness pattern to port.
- `.andromeda/architecture.md` :69, :71, :92, :246-249, :287-290, :309-310 — wheel starts with `driver` · no budget reading → `unknown`, does not block · the start order · the snapshot `data` fields · `wheel`/`budget-gate` "once at start" · the heartbeat file, 1 s, 5 s, pid + start time.
- `schemas/diag-line.v1.json:80` — the `detail` enum already holds `already-live`, `pinned-hash-mismatch`, `squatted-name`, `strict-modes-failed`.
- `viola-0.1.0/working-route.md` :40 (Wrapper channel: its CARRY lands the first spans, `pty.spawn`), :43 (Hooks: exec-form plugin hooks, silent unwrapped no-op, fail-open exit 0), :47 (ledger + verify, transport-only degrade), :67 (Statusline: "settings.json rewritten each start with absolute pinned path"), :89 (Home and code-bearing file integrity: owner-only home, DACL check, pinned-binary re-hash, per-start plugin rewrite, single writers).

## Graph impact (rust plane, `db_state: fresh`, trace `tree-query-2026-09-27-instance-state-and-start-order.json`, 59 rows)
- **`run` (cmd/run)** — one product caller, `cmd/dispatch()` @ `src/cmd/mod.rs:52`. Two integration tests call the binary: `tests/cli_program_resolution.rs:56,83` (they drive the exe and don't link the fn). The signature stays; the body grows.
- **`resolve_program`** — its product caller is `src/cmd/run.rs:56`. Unchanged.
- **`spawn` (viola-pty)** — its product caller is `src/cmd/run.rs:79`. `SpawnSpec` gains values, not fields. The other callers (harness `supervise.rs:39`, test helpers) are untouched.
- **`persistent_names` / `log_child_start` / `viola_obs_init` / `read_diagnostics_level`** — each has one product caller in `run()`. They are unchanged, and only their position in the new sequence matters.
- **`hold_pump_start`** — not in the graph (cfg `fake-agent` body). Grep: a single call site at `src/cmd/run.rs:83`, after the spawn. It stays after the spawn.
- **`viola-state`** — a new crate with no inbound edges (additive). The root bin gains the edge `viola → viola-state`.

## Patterns detected
- **Fixed two-line start refusal** (`src/cmd/run.rs:106-115`, `:59-62`): `unable:`/`hint:` on stderr with no path or pid, then `log_self_exit(1, Some(detail))` and `ExitCode::from(1)`. Its witness is `tests/cli_program_resolution.rs` (exit 1, no child, two stderr lines, `process-exit` detail).
- **Owner-only creation** (`src/obs.rs:39-60`): the mode is set at create AND by an explicit `set_permissions`, never through the umask. The Unix half is under `#[cfg(unix)]`.
- **Pid + start-time identity** (`crates/viola-e2e/src/harness/mod.rs:147-163`): sysinfo `refresh_processes(Some(&[pid]), true)` then `start_time()` equality and a not-zombie check.
- **Names-only env logging** (`src/run/mod.rs:29-42`): stripped/kept names only, never a value.

## Conventions to follow
- **Refusal output**: use `writeln!` on a locked stderr, never `eprintln!` (clippy `print_stderr`), per `src/cmd/run.rs:109`.
- **Diagnostics**: every line goes through `obs_event!`, and `detail` comes from the schema enum (`schemas/diag-line.v1.json:80`).
- **Inline tests**: keep `mod tests { … }` inline (testing.md 2026-09-25, the orphans gate).
- **Per-OS bodies**: put a minimal `#[cfg(unix)]` reader or writer beside shared decision logic tested on every OS (testing.md 2026-09-24).

## Load-bearing equalities (verified at HEAD)
- **Liveness identity**: the wrapper records its own sysinfo `start_time()` (u64 seconds) as the snapshot's `started_at`, rendered RFC 3339 `…T..:..:..(.000)Z`. A reader parses it back to seconds and compares it with sysinfo's `start_time()` for the snapshot `pid`. Both values come from the same OS field read by the same library, so equal pid + equal seconds ⇔ the same process. Using `chrono::Utc::now()` at write time would NOT equal it, because it is later than process start. Source: `harness/mod.rs:157,172`.
- **Pinned hash key**: the `<hash>` in `bin/<VERSION>-<hash>/` is `hex(sha256(current_exe bytes))[..16]`. The re-hash of an existing copy is equal to it only for byte-identical content (the FIPS 180-2 vectors + truncation are pinned in `tests/contract_content_hash.rs`). A one-byte tamper → unequal → exit 1.
- **`VIOLA_DIR` home derivation**: `VIOLA_DIR` = `<home>/instances/<name>`, so its grandparent = `<home>`. This is the resolution obs-plan D-09 gives `hook`/`mcp`.

## New files to create
- `crates/viola-state/Cargo.toml` — `publish = false`, `license.workspace = true`, `[lints] workspace = true`, `fake-agent = []` feature, and deps `viola-core`, `serde`, `serde_json`, `chrono`, `sysinfo`, `sha2`, `thiserror`, `tracing`, plus the atomic-write primitive (a P4 fork; see Open questions).
- `crates/viola-state/src/lib.rs` (+ inline modules) — `StateError`, private dir/file creation, the event-line writer (`{v, ts, instance, kind, source, data}`, one `write` per line, `events.ndjson.lock` sibling), the snapshot envelope + instance snapshot writer (atomic, `snapshot.json.lock`), the heartbeat touch, the liveness classifier (live/stale/gone, pure over `(beat_age, same_process)`), and the pinned copy + content hash + re-hash.
- `plugin/.claude-plugin/plugin.json`, `plugin/hooks/hooks.json`, `plugin/.mcp.json` — the embedded plugin (`include_str!`). Contents depend on a P4 fork (see Open questions).

## Files to modify
- `Cargo.toml` — add `crates/viola-state` as a root `[dependencies]` path dep (`sha2` becomes a normal dep of `viola-state`, and the root dev-dep stays for `tests/contract_content_hash.rs`). `members = ["crates/*"]` already picks the crate up.
- `Cargo.lock` — the new member plus any new transitive dependencies.
- `scripts/sync-crates.txt` — add `viola-state`.
- `src/cmd/run.rs` — the start sequence in documented order around today's steps: collision check → pinned copy + plugin folder → (version-gate slot) → (bind slot: channel) → first snapshot + heartbeat thread → start events → spawn with `VIOLA_*` / `PATH` / `--plugin-dir` → snapshot rewrite with `child_pid` → the fake-agent hold → pump. Add the new refusal lines. `run()` must split into step functions, because it already spans 64 lines (:40-104).
- `src/run/mod.rs` — step helpers (the spawn env builder, the pinned paths), as needed.
- `src/obs.rs` — optionally call `viola-state`'s private dir/file helpers instead of its own (:39-60), to keep one implementation. The implement-time judgement is whether the move is worth the diff.
- `tests/tui_env_strip.rs` — the E2 `VIOLA_NAME`/`VIOLA_DIR` (+ `VIOLA_BIN`) presence assertion on the same `env` receipt.
- `tests/cli_program_resolution.rs` — possibly unchanged. The `.cmd` refusal must still precede every state write (resolve before the collision check is not the documented order, so see Open question 3).
- `tests/support/home.rs`, `crates/viola-e2e/src/harness/boot.rs` — readiness grows from interim to the landed surfaces (snapshot fields, heartbeat < 5 s, `events.ndjson` lines 1–2) per test-plan §3 "replaced check by check once its surface lands".
- New root tests `tests/cli_instance_state.rs` (name per testing.md `cli_<topic>`): Path 1 subset, duplicate / stale refusal, tamper refusal, plugin rewrite, modes (unix), gone takeover appends.
- Added at /implement (overseer ruling 2026-09-27, option 1: the pinned exe copy in every test home exhausted
  the mutation leg's disk): `crates/viola-e2e/src/harness/run/mutants.rs` — the `cargo mutants` command sets
  `AGENT_RUN_KEEP_HOMES=0` / `AGENT_RUN_KEEP_FAILED=0` (the one runner CI's `ci.yml:173` and `pre-push` both
  reach); `tests/support/home.rs` — a sweep of leftover homes whose owning test process is verifiably gone;
  `Cargo.toml` — `sysinfo` as a root dev-dependency for that owner check; `tests/cli_fake_agent.rs` — it pins
  the fake agent's `start` receipt that step 13 grows. Second fold (overseer, 2026-09-27):
  `crates/viola-e2e/src/harness/pre_push.rs` — the Linux mutation leg's `TMPDIR` points cargo-mutants' scratch
  copies at `$HOME/viola-pre-push-scratch` on the clone's own disk (cargo-mutants 27.1.0 `copy_tree.rs:81-84`
  builds each with `tempfile::Builder::tempdir()`, i.e. `std::env::temp_dir()`, and has no location option),
  wiped at the start of each run and counted in the cache report.
- Companion sweep: `SpawnSpec {` literal sites — grep over the tree gives `src/cmd/run.rs:70`, `crates/viola-pty/src/lib.rs:1057` (test), `crates/viola-e2e/src/harness/supervise.rs` (harness) · 1 changed · 2 no-change (they build their own spec).

## Measured facts (research-time)
- **atomic-write-file 0.3.1 is `BSD-3-Clause`** (crates.io API `GET /api/v1/crates/atomic-write-file/0.3.1` → `license: "BSD-3-Clause"`, `rust_version: 1.85`). It depends on `rand ^0.10.2` (normal) and `nix ^0.31.3` (cfg unix). BSD-3-Clause is not in `deny.toml`'s allow list, so adding it fails `cargo deny check licenses` unless a new `[[licenses.exceptions]]` is added (security.md: "a new exception needs a Decisions Log entry"). `tempfile =3.27.0` (MIT OR Apache-2.0) is already locked and is the drop-in alternative arch names at `architecture.md:50`.
- **No `viola hook` / `viola mcp` verb at HEAD** (`src/cmd/mod.rs:28-31`). A `hooks.json` that registers `<pinned viola> hook <event>` would run a verb clap rejects with exit 2, and exit 2 on a Claude Code hook is a blocking error. That violates the invariant "`viola hook` always exits 0 … exit 2 is forbidden".

## Open questions
1. Atomic-write primitive: tempfile `persist` (already locked, allowed licence, arch's named alternative → wrap amendment to [Snapshot writer]) vs atomic-write-file + a BSD-3-Clause exception (Decisions Log entry, +rand/nix) → blocks: plan-decision.
2. Embedded plugin content before the hook/mcp verbs exist: an empty `hooks`/`mcpServers` (the mechanism lands, and M6 command-absoluteness waits for "Hooks to normalised events") vs registering hooks now with a minimal fail-open `viola hook` no-op (pre-empts part of the hooks entry) → blocks: plan-decision.
3. Order of the `.cmd`/`.bat` refusal: today it is decided at program resolution before anything else. In the documented order the collision check is first. Resolving the program first means no state is written for a refused child (obs-plan §4: "decided earlier, by program resolution, before … pty.spawn") → blocks: plan-decision (lean: resolution stays first, because a program refusal must not leave a pinned copy/snapshot behind; obs-plan already places it outside `run.pin_copy`).
