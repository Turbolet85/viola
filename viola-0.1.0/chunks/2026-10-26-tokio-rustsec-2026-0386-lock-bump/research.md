# Codebase Research — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## Scope
- **Depth:** moderate · **Reads:** 12 · **Globs/Greps:** 10 · plus one scratch-worktree measurement (below)
- **Harness rules consulted:** none — no live leg in this chunk (no scenario or harness session is added; the gates are
  cargo, cargo-deny and the existing scripts, and the operator pass's `pre-push` is the seeded entry, its firing form
  `.claude/rules/ci.md` §The operator pass)
- **Platform issues consulted:** the runner-only bullet's signature (`unsound` RUSTSEC-2026-0386 on `tokio` 1.53.1, read
  from job 121361224603's log) traced to its source's own tracker and release notes, fetched: `gh api
  repos/tokio-rs/tokio/releases/tags/tokio-1.53.3` → "Tokio v1.53.3", published 2026-10-13T20:49:48Z, "io: add
  `AsyncFd::register` and `AsyncFd::register_with_interest` ([#8587])" and "io: deprecate `AsyncFd::{new, with_interest,
  try_new, try_with_interest}` ([#8587])"; `gh api repos/tokio-rs/tokio/pulls/8587` → "io: add `AsyncFd::register` and
  deprecate `AsyncFd::new`", merged 2026-10-10T12:46:17Z into `tokio-1.51.x` (the same text ships in the crate's own
  `CHANGELOG.md`, `inputs#I4`). No runner or GitHub Actions issue is involved: the job failed on the advisory database,
  as designed
- **External inputs:** `inputs#I1` — the operator's phase directive relaying overseer1's call (ordinary chunk, no forced
  sample; the stub run dir left; the three standing words) · `inputs#I2` — RUSTSEC-2026-0386 (patched `>= 1.51.6, <
  1.52.0` and `>= 1.53.3`; the four safe `AsyncFd` constructors deprecated from 1.53.3) · `inputs#I3` — the published
  tokio 1.53.3 manifest (rust-version 1.71, dependencies identical to 1.53.1's) · `inputs#I4` — tokio's changelog for
  1.53.2 and 1.53.3, the whole behaviour move from 1.53.1

## Files inspected
- `Cargo.toml` (195–251) — `[workspace.dependencies]`: `tokio = { version = "=1.53.1", default-features = false }` at
  line 242 under the comment "viola-mcp's runtime and viola-channel's `tokio` client only … Each user names its
  features" (240–241); every third-party pin in the `=` form but rmcp's `>=3.4.1, <3.5` (225).
- `Cargo.lock` (1630–1671) — `tokio` 1.53.1 at 1632–1633; `tokio-macros` 2.7.2, `tokio-stream` 0.1.19, `tokio-util`
  0.7.20 beside it; `interprocess` 2.4.4 at 555–556.
- `crates/viola-channel/Cargo.toml`, `crates/viola-mcp/Cargo.toml`, `crates/viola-e2e/Cargo.toml` (tokio lines) — the
  three consumers, each `workspace = true` with its own features: channel `io-util` (optional, behind `tokio =
  ["dep:tokio", "interprocess/tokio"]`) and dev `rt`; mcp `rt`; e2e `rt` + `process`. No per-crate version: the pin
  move is one line.
- `.github/workflows/ci.yml` (372–433) — job `supply-chain`: `Dependency policy (cargo deny, all four families)` (395–399),
  `Fuzz lockfile audit` (402–406), `Tokio ban per sync crate (sole root)` with `channel-tokio-free.sh` (409–416),
  `Ban probes` (417–419), `Workflow lint (zizmor)` (420–424), upload `if: always()` (426–433); tools cargo-deny@0.20.2,
  zizmor@1.30.1.
- ci#40257114709 job 121361224603 log (`gh api --allow-escape-sequences …/jobs/121361224603/logs`, 354 lines) — one
  `"severity":"error"` (`"code":"unsound"`, `"id":"RUSTSEC-2026-0386"`, `tokio`), four `"severity":"warning"` (`duplicate`:
  bitflags, syn, thiserror, thiserror-impl); `##[error]Process completed with exit code 1.` at log line 312; the
  step groups end at `Run mkdir -p target/supply-chain` (the deny step) then the upload — the four later steps did not run.
- `crates/viola-e2e/tests/mcp_tools.rs` (64, 87) — the MCP E2E owns its `viola mcp` child as a `tokio::process::Child`
  built by `tokio::process::Command::new(viola_exe(s))`: the user of the `process` feature 1.53.2's Windows fix touches.
- `~/.cargo/advisory-dbs/advisory-db-3157b0e258782691/crates/tokio/RUSTSEC-2026-0386.md` (`inputs#I2`, DB commit
  `21d909e4`).
- `~/.cargo/registry/…/tokio-1.53.3/Cargo.toml` (`inputs#I3`) against `tokio-1.53.1/Cargo.toml` — `diff`: the `version`
  line, one test target removed (`io_async_fd_memory_leak`), one added (`task_trace_wake`); no dependency line differs.
- `~/.cargo/registry/…/tokio-1.53.3/src/io/async_fd.rs` (908, 920, 932, 944) — the four `#[deprecated(` attributes.
- `~/.cargo/registry/…/interprocess-2.4.4/src/os/unix/unnamed_pipe/tokio.rs` (15–19) — the graph's one `AsyncFd` user
  (`type RecverImpl = AsyncFd<FdOps>`): a dependency, outside viola's lint gate (deprecations in a dependency are
  capped by cargo).

## Graph impact
- **crate_edges** around the tokio consumers (trace `.andromeda/runs/2026-10-26T05-14-53-phase/tree-query-2026-10-26-tokio-rustsec-2026-0386-lock-bump.json`,
  11 rows): `viola → viola-mcp`, `viola → viola-channel`, `viola-mcp → {viola-agent-claude, viola-channel, viola-core,
  viola-state}`, `viola-e2e → {viola-agent-claude, viola-channel, viola-core, viola-pty, viola-state}`. No symbol is
  changed by this chunk; the blast radius is the dependency version alone, reached through `viola-mcp`, `viola-channel`'s
  `tokio` feature and `viola-e2e`.
- viola's tokio API use (`grep -rn 'tokio::' crates src tests`, `viola-e2e` excluded): `runtime::Builder::new_current_thread`,
  `spawn`, `task::spawn_blocking`, `io::{split, duplex, BufReader, AsyncBufReadExt, AsyncWriteExt}` in
  `crates/viola-mcp/src/{lib,tools}.rs` and `crates/viola-channel/src/tokio_client.rs` — none of the deprecated
  constructors.
- tokio's enabled features in the graph (`cargo tree -e features -i tokio --target all --prefix none`): `bytes`, `default`,
  `fs`, `io-std`, `io-util`, `libc`, `macros`, `mio`, `net`, `rt`, `socket2`, `sync`, `time`, `tokio-macros`,
  `windows-sys` — `net` is on (through interprocess's `tokio` flavour), so the advisory's `AsyncFd` is compiled in on
  Unix: the security plan's enabled-feature SLA arm (a bump) is the one that applies.

## Patterns detected
- **The equality the plan rests on** (measured in a scratch `git worktree` of HEAD `b1249fb1`, removed after; the real
  tree untouched): the pin line edited to `=1.53.3`, then `cargo update -p tokio --precise 1.53.3` → exit 0, `Updating
  tokio v1.53.1 -> v1.53.3`; `git diff --stat` → `Cargo.lock | 4 ++--`, `Cargo.toml | 2 +-`; the lock hunk is the `tokio`
  entry's `version` and `checksum` lines alone. On that tree, at the CI pins:
  - `cargo deny --format json check` → exit 0; 4 `warning` (`duplicate`), 0 `error`, no `RUSTSEC-` id;
  - `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` → exit 0; 1 `warning`
    (`advisory was not encountered`, the RUSTSEC-2017-0008 ignore the fuzz graph does not hold);
  - the four `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans` over
    `scripts/sync-crates.txt` → exit 0 each; `bash scripts/channel-tokio-free.sh` → `channel-tokio-free: ok`;
  - `bash scripts/deny-probes.sh` → `deny-probes: 13/13 banned, control clean`;
  - `zizmor --format=json .github/workflows/` → exit 0, `[]`;
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` → exit 0, no `deprecated` line.

- **What else the move carries** (`inputs#I4`, 1.53.2 and 1.53.3 sections, set against viola's tokio use above): no
  API viola calls is removed or changed in signature; four fixes touch what viola runs — 1.53.3 "io: wake and drop
  `DuplexStream` wakers outside the lock" (`tokio::io::duplex` in `crates/viola-mcp/src/tools.rs`), 1.53.3 "runtime: fix
  paused time auto-advancing when I/O wakes a task on `current_thread`" (every viola runtime is `new_current_thread`; no
  viola test pauses time — no `start_paused`, no `test-util`, test-plan §8), 1.53.2 "process: unregister Windows wait
  before closing child handle" (`viola-e2e`'s `process` feature, the `viola mcp` child its MCP tests own) and 1.53.2
  "rt: drop blocking pool mutex before shutting down rejected task" (`spawn_blocking` in `tools.rs`). Their witnesses are
  the Tokio-bearing suites: `crates/viola-mcp/tests/handlers.rs`, `crates/viola-e2e/tests/mcp_tools.rs`, the
  `viola-channel` `tokio_client` units — in `pre-push`'s coverage run on Linux, and on every OS in CI's `test` row.

## Conventions to follow
- **Exact pins in one place**: every third-party version is an `=` pin in `[workspace.dependencies]`, crates take it with
  `workspace = true` (`Cargo.toml:196-251`); the tokio move keeps `=` and `default-features = false`.
- **The lock moves through cargo, never by hand**: `cargo update -p tokio --precise <v>` after the pin edit, so the
  checksum is the registry's.
- **A gate's verdict is its printed line and the bare command's exit** (`.claude/rules/host-linux.md` §Exit codes): the
  deny JSON is read for `error` diagnostics, the scripts by their last line.
- **No widening**: `deny.toml`, `deny-sync.toml` and the lint tables untouched (security-plan §Dependency Security,
  `deny.toml` additions).

## New files to create
- none

## Files to modify
- `Cargo.toml` — the `[workspace.dependencies]` tokio pin `=1.53.1` → the patched version (P4's lean)
- `Cargo.lock` — the `tokio` entry, moved by `cargo update -p tokio --precise <v>`

## Sweeps
- `1\.53\.1` over the tracked tree (`git grep -n '1\.53\.1' -- . ':!.andromeda/runs' ':!.andromeda/friction-log.ndjson'
  ':!viola-0.1.0/chunks'`): 20 hits · 2 changed (`Cargo.toml:242`, `Cargo.lock:1633`) · 18 no-change (the spec masters,
  their sidecars, archive and registry key file — `.andromeda/architecture-detail/stack-and-technologies.md:11`,
  `.andromeda/architecture.md:442`, `.andromeda/security-plan-detail/dependency-security.md:14,58`,
  `.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8,17`, `.andromeda/test-plan-detail/test-data-fixtures.md:21`,
  the key file `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:24` — the
  wrap's amendments; the sidecar and archive lines are history; the Tier-3 docs `.claude/docs/stack.md:10,47` and
  `.claude/docs/services/viola-mcp.md:11` — written at the wrap commit, as `git log -- .claude/docs/stack.md` shows every
  edit in a `feat(<marker>)` wrap commit; `viola-0.1.0/working-route.md:171` — the entry itself).
- `AsyncFd` over `crates src tests` (`grep -rn AsyncFd --include=*.rs`): 0 hits.

## Open questions
- none
