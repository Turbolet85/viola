# arch extract

## Relevance
Relevant. The chunk adds code to `viola-core`, the root bin and `viola-e2e`, adds home and instance filesystem resources, adds a `config.json` key and a `schemas/` file, and uses a Stack row (Logging) that the architecture plan locks.

## Constraints
- **viola-core dependencies.** `viola-core` depends on no viola crate, and its only third-party dependency is nutype (per architecture §Infrastructure Patterns → Crate dependency direction). tracing and tracing-subscriber belong to the root bin only (per architecture §Stack and Technologies, Logging row).
  - So `ObsEvent`, `ObsProcess`, `SERVICE_NAME` and `VERSION` must add no `tracing` or `tracing-subscriber` edge to `viola-core`. `obs_event!` must expand to a `::tracing::` path that resolves in the caller's crate.
  - `viola-core` is a sync crate listed in `scripts/sync-crates.txt`. The sole-root tokio ban still applies to it (per architecture §Established Decisions [Concurrency]).
- **Feature bans and allowed tracing-subscriber features.** `deny.toml` bans `tracing-appender` (telemetry family) and `tracing-subscriber/env-filter` (feature bans) (per architecture §Infrastructure Patterns → Build system). tracing-subscriber is locked to default features off with `fmt,json,registry,std` (per architecture §Stack and Technologies, Logging row).
  - The per-role file writer must be hand-rolled, not tracing-appender.
  - `diagnostics_level` must use `filter::Targets`, never `EnvFilter`.
  - Research must check whether `filter::Targets` and the `MillisUtc` timer compile under exactly those four features, or whether the chunk needs a new feature. A new feature would be a Stack-row amendment.
- **Config, not env vars.** Environment variables are not a configuration channel (per architecture §Cross-cutting Patterns → Config management). `config.json` is parsed tolerantly: it carries `v`, and unknown keys are skipped and counted. Precedence is CLI flag > `config.json` > built-in default.
  - `diagnostics_level` therefore arrives only through `config.json`, with a built-in default of `"info"`.
  - The chunk may add no `VIOLA_*` variable and no `RUST_LOG` (per architecture §Conventions → Naming patterns, Environment variables).
- **Diagnostic output channels.** While the child runs, `run` writes nothing to the terminal except the child's output. `hook` never writes to stderr. `mcp` writes only MCP frames to stdout (per architecture §Cross-cutting Patterns → Diagnostic output channels; per architecture §Established Decisions [Hook Contract]).
  - The panic hook in `src/main.rs` and every sink must stay off stdout and stderr for these roles.
  - Whether the current panic hook or the `run` lines ever reach stderr is a question for research.
- **Line discipline and torn lines.** Each line is one complete JSON object plus `\n`, written with a single `write`. Appends use std `OpenOptions::append`. Readers always tolerate a torn last line (per architecture §Conventions → Data model conventions, ndjson line discipline; §Cross-cutting Patterns → Crash-safe disk writes; §Stack and Technologies, State-file primitives row). This covers the per-role sinks, the detail writer and the harness `logs` merge.
- **Registered locations and file modes.** Home-level `diagnostics/` holds codes-only per-role logs: dir 0700, files 0600 on Unix, line format owned by obs. Content-bearing detail belongs in `instances/<ViolaName>/diagnostics/` (per architecture §Occupied Resources → Filesystem).
  - The home comes from `--home`, else the grandparent of `VIOLA_DIR`, else the default (per architecture §Cross-cutting Patterns → Config management).
  - The instance is found by `ViolaName`, never by the working directory (per architecture §Cross-cutting Patterns → Identity by instance).
- **Naming and timestamps.**
  - JSON keys are snake_case (`service_name`, `duration_ms`).
  - Wire enum values are kebab-case (`ObsEvent`'s `Display`).
  - Constants are SCREAMING_SNAKE_CASE (`SERVICE_NAME`, `VERSION`).
  - Timestamps are RFC 3339 UTC with milliseconds and a `Z` suffix, via chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`.
  - Optional fields are omitted on write, never written as `null` (per architecture §Conventions → Naming patterns; §Conventions → Data model conventions, Timestamps and Optional fields; §Established Decisions [Timestamps]).

## Patterns to follow
- **One source for identity.** The service identity already appears as `"name":"viola"` plus `CARGO_PKG_VERSION` in `/health` and `/api/info`, and every channel `params` carries `sender` (per architecture §Standard Contracts; §Conventions → Interfaces and versioning, Sender version).
  - `viola_core::{SERVICE_NAME, VERSION}` should be the one source that later `ui` and channel code reads.
  - Crates share `version.workspace = true` (per architecture §Infrastructure Patterns → Build system), so `env!("CARGO_PKG_VERSION")` in `viola-core` equals the bin's version.
- **Closed vocabularies live in `viola-core`.** `RefusalReason`, its details and the normalised event kinds are closed kebab sets that are extended only in `viola-core` (per architecture §Conventions → Error handling schema; §Conventions → Naming patterns, Normalised event kinds). `ObsEvent` and `ObsProcess` follow the same model.
- **Errors.** Each crate has one thiserror enum named `<Crate>Error`, and anyhow appears only in the root bin (per architecture §Established Decisions [Error Handling]; §Conventions → Error handling schema, Rust error types).
  - A `viola::obs` failure is a root-bin concern.
  - Any `viola-core` failure folds into `CoreError`.
- **Harness boundary.** The harness lives in test-only `viola-e2e`, whose command contract is test-plan §3. No product crate depends on it (per architecture §Established Decisions [Module Boundaries]; §Occupied Resources → Binary, subcommands and exit codes).
  - The `logs` merge changes stay inside `viola-e2e`.
  - `viola-state` holds the planned torn-line healing (per architecture §Infrastructure Patterns → Project directory structure). It is created by its first consumer, so whether it exists yet is a question for research.

## Anti-patterns to avoid
- **Tokio or an async writer on these paths.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O (per architecture §Cross-cutting Patterns → Tokio containment). The sinks must be synchronous std-file writers.
- **Content in home-level role files.** The home-level `diagnostics/` is codes-only (per architecture §Occupied Resources → Filesystem; §Cross-cutting Patterns → Diagnostic output channels). The panic payload and backtrace must go only to the instance's detail file, never to `run-<name>.ndjson`.
- **Env or shell configuration.** No env-var level knob, no `EnvFilter`, no `tracing-appender` (per architecture §Cross-cutting Patterns → Config management; §Infrastructure Patterns → Build system).

## Contract bindings
- **arch ↔ obs.** The architecture plan hands the line format, levels, logger and diagnostics file format to obs (per architecture §Stack and Technologies, Logging row; §Occupied Resources → Filesystem; §Cross-cutting Patterns → Diagnostic output channels). The `event` vocabulary, builder settings and schema content are obs-plan's. The architecture plan owns only the file locations, modes and crate placement.
  - §Conventions → Interfaces and versioning requires `v` on event lines, snapshots and channel params. It does not name diag lines. Whether diag lines carry `v`, or are versioned only by `diag-line.v1.json`, is obs's decision.
- **arch ↔ obs, drift to amend at wrap.** Two passages still route `hook` and `mcp` diagnostics to the instance's `diagnostics/` only, or to stderr for `mcp`: §Established Decisions [Hook Contract] and §Cross-cutting Patterns → Diagnostic output channels. The chunk creates home-level `hook-<name>`, `mcp`, `ui-<port>` and `cli-<name>` role files plus instance `detail-<process>.ndjson`, following obs D-08. The `run` log already moved this way in an earlier amendment. Wrap must amend both passages for hook, mcp, ui and cli.
- **arch ↔ tests.** The chunk adds `schemas/diag-line.v1.json` to the repository registry. Arch describes `schemas/` as "test-side" (per architecture §Occupied Resources → Repository; §Infrastructure Patterns → Project directory structure). The harness `logs` output shape is test-plan §3's.
- **arch ↔ security.** The detail files are 0600 in 0700 directories on Unix (per architecture §Occupied Resources → Filesystem). The NEVER-log floor is security-plan's.
- **Item 8 and item 9 are outside the architecture domain.** Item 8 is the `.claude/settings.json` write guard, which is dev tooling, not the product. The product's no-shell-out rule in §Cross-cutting Patterns → Cross-platform discipline does not govern it. Item 9, the Rust gate, falls under CI target job 5 and test-plan.

## Acceptance criteria contributions
- **(arch) Crate placement.**
  - `ObsEvent`, `ObsProcess`, `obs_event!`, `SERVICE_NAME` and `VERSION` live in `crates/viola-core`.
  - `cargo tree -p viola-core -e normal` shows no `tracing`, `tracing-subscriber` or `tokio`.
  - The sole-root `deny-sync.toml` check for `viola-core` passes.
  - Sinks, the detail writer and `viola_obs_init` live in the root bin's `viola::obs`.
  - (per architecture §Infrastructure Patterns → Crate dependency direction; §Established Decisions [Concurrency])
- **(arch) Dependency policy.**
  - `cargo deny check` passes with no `tracing-appender` and no `tracing-subscriber/env-filter`.
  - tracing-subscriber keeps default features off, with only `fmt,json,registry,std`. Any other feature needs a recorded Stack-row amendment.
  - (per architecture §Infrastructure Patterns → Build system; §Stack and Technologies, Logging row)
- **(arch) Configuration channel.**
  - `diagnostics_level` is read only from `<home>/config.json`, found through the `--home` / `VIOLA_DIR` / default chain.
  - An absent file, absent key or unknown key gives `"info"` with no error.
  - No new `VIOLA_*` or `RUST_LOG` read exists.
  - (per architecture §Cross-cutting Patterns → Config management)
- **(arch) Registry and doc updates at wrap.**
  - §Occupied Resources → Filesystem lists the new role file names and `instances/<name>/diagnostics/detail-<process>.ndjson`.
  - The `config.json` entry names `diagnostics_level`.
  - §Occupied Resources → Repository and the tree list `schemas/diag-line.v1.json`.
  - The two passages from the obs drift binding are amended.
  - (per architecture §Occupied Resources; §Established Decisions [Hook Contract]; §Cross-cutting Patterns → Diagnostic output channels)

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, "run's process log location, logging and serialization rows".**
  - Moved `run`'s codes-only log to the home-level `diagnostics/run-<name>.ndjson`, with content-bearing detail staying in the instance's `diagnostics/`, per obs D-08.
  - Added the tracing 0.1.44 / tracing-subscriber 0.3.23 (root-only) Logging row.
  - Added serde_json `preserve_order`.
  - Why it matters here: this chunk builds out that split. The same home-level/instance split now needs to be amended for hook, mcp, ui and cli.
- **2026-09-24-three-os-ci-headless-harness-skeleton, "test-only crate, bins, env vars, paths".**
  - Registered the home-level `diagnostics/` with only `run-<name>.ndjson` (0700/0600).
  - Registered the `viola-e2e` harness and the `AGENT_RUN_` prefix.
  - Why it matters here: this chunk extends that registry entry, and the harness `logs` command it modifies was registered here.
- **2026-09-24-fake-agent-and-test-data-fixtures, "test homes, test env vars and test-side paths registered".**
  - Registered `schemas/fake-script.v1.json` and the `schemas/` tree entry as test-side.
  - Why it matters here: this is the precedent for registering `schemas/diag-line.v1.json`.
  - It also rejected "registry over-reach", so only resources the product reads or writes are registered.
- **2026-09-24-supply-chain-and-workflow-gates, "sole-root tokio ban, four-family deny policy, nightly.yml".**
  - Put `deny.toml` into four families, including the telemetry ban on `tracing-appender` and the feature ban on `tracing-subscriber/env-filter`.
  - Made the tokio ban sole-root per sync crate, and `viola-core` is one of those crates.
  - Why it matters here: these are the live gates the new logging dependencies must pass.
