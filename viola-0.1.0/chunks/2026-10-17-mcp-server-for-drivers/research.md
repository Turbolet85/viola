# Codebase Research — 2026-10-17-mcp-server-for-drivers

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full).
  - One addition applies: 2026-10-10. On the Linux dev host the bare `claude` resolves to a newer CLI than the stamp, so a live rig puts the stamped version's install directory first on `PATH` and reads `claude --version` by path. Measured here: the bare `claude` is `~/.local/share/mise/installs/claude/latest/claude`, `2.1.296 (Claude Code)`; `2.1.287` is installed beside it at `~/.local/share/mise/installs/claude/2.1.287/`.
  - The Fake agent section states that it reads only `<plugin-dir>/hooks/hooks.json`. It never loads `.mcp.json`.
- **Platform issues consulted:** the installed CLI bundle `~/.local/share/mise/installs/claude/2.1.287/claude` was read as text (`/usr/bin/grep -a -o`). It holds:
  - a per-server `timeout` option for an MCP server, described as "Per-server tool-call timeout in milliseconds. Overrides the MCP_TOOL_TIMEOUT environment variable for this server. Hard wall-clock limit per call; progress notifications do not extend it. Values below 1000ms are ignored";
  - the stdio call-timeout function `$r(e)` = `e.timeout (≥1000) ?? MCP_TOOL_TIMEOUT ?? So`, clamped to `[1000, 2147483647]`;
  - an idle timeout of `CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT ?? (stdio ? vo : Eo)`, with `vo=1800000` and `Eo=300000` at that site;
  - a sibling HTTP-path default `_r=60000`.

  The default `So` is a minified name the scan cannot pin: the constant scan reads `So=1e8` among five other `So=` assignments. **hypothesis:** the stdio tool-call default is 1e8 ms, and a live start would measure it. No issue tracker was consulted: no runner-only bullet stands.
- **External inputs:** `inputs#I1` — the operator's take-up direction. The P4 forks are dialogs, and a boundary widening, a cap on live `claude` starts and the `v1-33` switch question are the founder's own word.

## Files inspected
- `src/cmd/mod.rs` (full) — clap `Command`. `Hook` is `#[command(hide = true)]` with the doc line "run by the plugin, never by a person" (`:80-82`). `dispatch` builds a `DetailSink` per verb. `resolved_home` → `resolve_home` → `canonical_home` (`:225-264`) is the home order `--home`, then `instance_of(VIOLA_NAME, VIOLA_DIR)`, then `<user home>/.viola`.
- `src/main.rs` (`:1-200`) — `role_of` (`:92-115`) files every first word but `hook`, `revive` and `run` (and a leading `-`) as `Role::Cli`. `exit_code` (`:129`) and `prints_internal_error` (`:141-143`, only `Role::Cli` prints) decide the catch-site output. The panic hook writes through `PANIC_SINK`.
- `src/cmd/client.rs` (full) — `live_endpoint` (`:95-102`; snapshot + heartbeat + same-process liveness, no strict-modes), `own_name` (`:105-108`, `VIOLA_NAME` through `ViolaName::try_new`), `reply_of` / `answer_of` (`:45-91`, the three reply shapes), `fault_document` (`:73-75`), the exit-21 `{"v":1,"error":"instance-unreachable","detail":null}` document (`:140-156`).
- `src/cmd/send.rs` (full) — `refusal_of` (`:200-207`: `validate_paste_text`, then `typed_text(..).is_empty()` → `EmptyText`), `deliver` (`:209-247`), the `--json` documents `{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` (`:130-155`), `exit_of` (`:87-95`).
- `src/cmd/wait.rs` (full) — params `after?`, `timeout_ms?`, `from?` (`:46-55`). A refusal reply is an internal error (`:63`).
- `src/cmd/last.rs` (full) — params `from?` only. Same reply handling.
- `src/cmd/answer.rs` (full) — `read_response` → `Response::parse` (`:45-54`), the free-text check before any frame (`:102-110`), params `dialog_id`, `response`, `from?`.
- `src/cmd/list.rs` (full) — `board::scan` before any log (`:42-48`), `read` (`:97-114`: `board::read`, `run::agents::read_here`, `read_budget_stamped`, `read_thresholds`), `document` (`:148-159`, the `{"v":1,"ok":{items,budget,skipped}}` payload), refusal document (`:80`).
- `src/run/agents.rs` (full) — `read_logged` (the `agents-probe` pair) and `read_here` (`:109-116`). Both are root-bin only: they call `super::resolve_program`, `super::persistent_names` and `version_gate::run_capped`.
- `src/run/wait.rs` (`grep timed_out|cursor`) — a timed-out wait answers `{"ok":{"timed_out":true}}` with no cursor (`:161`). A woken one answers `{"ok":{"event":…,"cursor":<end>}}` (`:155`).
- `crates/viola-agent-claude/src/lib.rs` (`:140-230`) — `PLUGIN_FILES` embeds `plugin/.mcp.json`. `render` substitutes `@@VIOLA_BIN@@` / `@@VIOLA_VERSION@@` (`:179-189`). The comment at `:153-154` reads "`mcpServers` stays empty until the `mcp` verb exists: a registered command that exits 2 blocks Claude Code". The test at `:606` pins the empty `.mcp.json`.
- `crates/viola-agent-claude/src/dialog.rs` (`:165-178`) — `Response`, "No permission suggestion is carried: a deliberate v1 limit".
- `crates/viola-core/src/lib.rs` (`:12-22`) — `DIALOG_DEADLINE` = 60 s, "PROVISIONAL, unmeasured".
- `crates/viola-channel/Cargo.toml` (full) — sync only. `interprocess` has no `tokio` feature. Comment `:18`: "the Tokio client lands behind a feature with its first consumer, viola-mcp".
- `Cargo.toml` (`:186-238`) — `[workspace.dependencies]` holds no `rmcp`, `tokio` or `schemars` pin. `interprocess = "=2.4.4"` carries the comment "the `tokio` feature stays confined to viola-mcp".
- `.github/workflows/ci.yml` (`:395-420`) — `Tokio ban per sync crate (sole root)` loops `scripts/sync-crates.txt` (`viola-core viola-pty viola-agent-claude viola-state viola-channel`) through `cargo deny --config deny-sync.toml --manifest-path crates/$crate/Cargo.toml check bans`.
- `deny.toml` (`:44-57`) — `[[bans.features]]` rmcp `transport-streamable-http-server` + `auth`.
- `src/obs.rs` (`grep mcp`) — `ObsProcess::Mcp` → `mcp.ndjson` (`:55`, case `:389`). `viola_mcp` is already in `VIOLA_TARGETS` (`:29`). `rmcp` is OFF (`:417`).
- `schemas/diag-line.v1.json` (`:14`) — `process` enum already holds `mcp`.
- `.andromeda/registries/contracts/architecture/crate-dependency-direction.md` (full) and `build-system.md` (full) — the `viola-mcp` edge set (`:7`), and the Build-system limit sentence on `viola-channel`'s own root.
- `crates/viola-e2e/Cargo.toml` (full) — no tokio and no rmcp. Its tests are `cli.rs`, `harness_lifecycle.rs`, `path6_budget_harness.rs`, `scan_patterns.rs`, `zero_retries.rs`.
- `viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/gap-list.md` (full) — 39 items, 10 stops in five groups.
- `viola-0.1.0/chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`, `I9-relay-4.md.txt` (full) — the `v1-33` row "not priced until he names the gap-list groups", carried here.
- `viola-0.1.0/chunks/2026-09-24-supply-chain-and-workflow-gates/research.md` (`grep`, `:27-29`) — CARRY 1's measured pointer still reads the false fail.
- `.claude/docs/services/viola-mcp.md` (full) — the planned tool inputs, the result mapping, and that "`tools/list` must be exactly the five tools".

## Graph impact (from the code-graph query)
The plane is `rust`, the trace is `.andromeda/runs/2026-10-17T09-28-44-phase/tree-query-2026-10-17-mcp-server-for-drivers.json`, and all three queries returned rows.
- **`role_of`** — 1 product caller, `main` @ `src/main.rs:53`, plus the rstest table `role_of_skips_the_home_flag_and_its_value` @ `src/main.rs:354`. A new `Role::Mcp` touches `exit_code` / `prints_internal_error` and their tables in the same file, nothing else.
- **crate edges of `viola-channel`** — inbound only from `viola` and `viola-e2e`, outbound to `viola-core`. No crate in `scripts/sync-crates.txt` depends on `viola-channel`.
- **`live_endpoint` / `own_name` / `answer_of` / `reply_of`** — callers only in `src/cmd/{send,wait,last,answer,pause,list,revive}.rs` and `src/cmd/mod.rs:111,148`, all root bin. **`read_here`** — `cmd/list/read()` @ `src/cmd/list.rs:104`, `cmd/revive/live_elsewhere()` @ `src/cmd/revive.rs:166`. **`plugin_files`** — `cmd/run/pin_and_plugin()` @ `src/cmd/run.rs:405` and two tests in `crates/viola-agent-claude/src/lib.rs` (`:589`, `:614`).

## Patterns detected
- **The hidden plugin-invoked verb** (`src/cmd/mod.rs:80-82`): `Hook` is `#[command(hide = true)]` because it is "run by the plugin, never by a person". `viola mcp` is the same class: Claude Code spawns it through `.mcp.json`.
- **The plugin template substitution** (`crates/viola-agent-claude/src/lib.rs:179-189`): `@@VIOLA_BIN@@` → the pinned copy's forward-slash path. `hooks.json`'s exec-form commands use it, so `.mcp.json`'s `command` takes the same token.
- **The liveness-only client pre-check** (`src/cmd/client.rs:95-102`): one snapshot read + beat age + same-process check, with no strict-modes and no server identity. This is the shape of the seven dated gaps.
- **The CLI `--json` documents as the parity oracle**: `{"v":1,"ok":<payload>}`, `{"v":1,"refusal":r,"detail":d}`, `{"v":1,"error":"instance-unreachable","detail":null}` and `{"v":1,"error":"wrapper-fault","detail":{code,message,data}}` (`src/cmd/send.rs:130-155`, `src/cmd/client.rs:73-75,140-156`).
- **The caller-side board** (`src/cmd/list.rs:38-75`): scan before any log, then the read, then one document. Its pieces live in the root bin (`run::agents`, `run::budget::read_thresholds`) and in `viola-state` (`board`, `budget`).

## Conventions to follow
- **The obs role is already admitted.** `ObsProcess::Mcp`, its `mcp.ndjson` file name, its `VIOLA_TARGETS` entry and the schema's `process` enum all stand (`src/obs.rs:29,55`; `schemas/diag-line.v1.json:14`). The chunk adds lines, not a role value.
- **One `write_all` per stdout document** (`src/cmd/client.rs:116-118`). The MCP transport's frames go through rmcp's stdio writer, never a print macro (workspace `print_stdout` deny).
- **The `typed_text` rule** (`viola_agent_claude::hook::typed_text`) decides `empty-text` after `validate_paste_text` (`src/cmd/send.rs:200-207`). The MCP `send` handler reruns both in that order.

## Mechanism re-derivations (the closure's equalities)
- **The CARRY 1 measured mechanism** (`measured at` chunk 2026-09-24-supply-chain-and-workflow-gates, its `research.md:29`): its pointer still reads the false fail. Re-derived here as the equality the new form needs, measured on a scratch workspace (`$TMPDIR/{scratchpad}/feat-probe`: member `a` with an optional `serde` behind feature `x`, member `b` enabling `a/x`; cargo, resolver 3, `--offline`).
  - `cargo tree -p a -e normal --target all` prints `a` alone.
  - `cargo tree -p a -e normal --target all -i serde` exits 101, `package ID specification `serde` did not match any packages`.
  - `cargo tree -p b` shows `serde` under `a`.

  So `cargo tree -p <crate>` resolves the named package's features alone, where `cargo deny` (through `cargo metadata`) reports the unified set. At HEAD, `cargo tree --offline -p viola-channel -e normal --target all -i tokio` exits 101 (`tokio` did not match). `Cargo.lock` holds no `tokio` at all (`/usr/bin/grep -n tokio Cargo.lock`: 0 lines).
- **[premise-corrected] The arch claim on `viola-channel`'s cover.** Build system states that `viola-channel` "is covered through the sync roots that depend on it". It is false at HEAD: the crate-edges query returns no sync crate with an edge into `viola-channel`. Covering through another sync root is impossible, so CARRY 1's "another sync root or form" has only the form arm. This is a wrap amendment to Build system, not a plan step.
- **The `wait` bound decides "below the client's timeout" for these inputs.** The wrapper's `wait` ends at its own `timeout_ms` deadline with `{"ok":{"timed_out":true}}` (`src/run/wait.rs:161`). An MCP `wait` that forwards `timeout_ms = min(given, CAP)`, defaulting to `CAP`, therefore returns at the latest at `CAP` plus a frame round trip, with no adapter-owned timer. A timed-out result carries no cursor (`:161`), so a driver resumes with the `after` it passed. That is the same as the CLI `wait --timeout-ms`.

## New files to create
- `crates/viola-mcp/Cargo.toml` — the sixth product crate. Edges `viola-core`, `viola-channel` (`features = ["tokio"]`), `viola-state`, `viola-agent-claude`, `rmcp` (`server`, `transport-io`), `schemars`, `tokio`, `serde`, `serde_json`, `thiserror`, `tracing`; `[lints] workspace = true`.
- `crates/viola-mcp/src/lib.rs` — the server entry: build the Tokio runtime, the rmcp stdio service, the tool router, `McpError`.
- `crates/viola-mcp/src/tools.rs` — the five handlers and their advisory schemars inputs, re-validated server-side.
- `crates/viola-mcp/src/result.rs` — the mapping from a channel reply to `CallToolResult` (`structuredContent`, `isError`, `refused: <reason>` text).
- `crates/viola-mcp/tests/handlers.rs` — the adapter-level handler tests against a test wrapper.
- `crates/viola-channel/src/tokio_client.rs` — the Tokio client behind the `tokio` feature, with the same frame rules and the same Windows SQOS open.
- `src/cmd/mcp.rs` — the hidden `viola mcp` subcommand: the home, the strict-modes check, obs init as `mcp`, the `list` provider, the call into `viola_mcp`.
- `crates/viola-e2e/tests/mcp_tools.rs` — the stdio E2E: an rmcp client spawning `viola mcp --home <home>` over booted fake-agent instances, covering the tool list, the parity legs and the schema violations.
- `scripts/channel-tokio-free.sh` — the sole-root form for `viola-channel`: `cargo tree -p viola-channel -e normal --target all -i tokio` must not match, and its control (`--features tokio`) must match.
- `scripts/rmcp-features.sh` — CARRY 2's lint step: rmcp is present in `cargo tree -e features -p viola --edges normal`, and its feature set is exactly `server` + `transport-io`.
- `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/` — the live leg's rig copies (`live-pty.py`, `live-ledger.py`, `live-drive.py`, byte for byte from chunk 2026-10-16-unwrapped-sessions-on-the-board), its ledger and `live-mcp.md`.

## Files to modify
- `Cargo.toml` — `[workspace.dependencies]` pins for rmcp (`>=3.4.1, <3.5`, default-features off, `server` + `transport-io`), tokio `=1.53.1`, schemars `=1.2.2`; the root's dependency on `viola-mcp`.
- `Cargo.lock` — the new resolution.
- `crates/viola-channel/Cargo.toml` — the `tokio` feature: `interprocess/tokio` and `dep:tokio`.
- `crates/viola-channel/src/lib.rs` — `#[cfg(feature = "tokio")] pub mod tokio_client`.
- `src/main.rs` — `Role::Mcp` in `role_of` (`--home <dir> mcp` included), `exit_code`, `prints_internal_error`, and their tables.
- `src/cmd/mod.rs` — the hidden `Mcp` subcommand and its dispatch arm with a `DetailSink` of process `Mcp`.
- `src/cmd/list.rs` — the board read and its document exposed to the `mcp` dispatch, with no behaviour change.
- `src/cmd/client.rs` — `live_endpoint` moves to `viola-state`, and its callers call it there.
- `crates/viola-state/src/liveness.rs` — `live_endpoint(instance_dir)` (snapshot, beat age, `classify`, `same_process`) with its unit, now shared by the CLI verbs and `viola-mcp`.
- `plugin/.mcp.json` — `mcpServers.viola` = `{"command": "@@VIOLA_BIN@@", "args": ["mcp"]}`.
- `crates/viola-agent-claude/src/lib.rs` — the `.mcp.json` pin test (`:606`) and the comment (`:153-154`).
- `crates/viola-e2e/Cargo.toml` — the rmcp client (`client`, `transport-child-process`), tokio and jsonschema for the MCP tests.
- `.github/workflows/ci.yml` — the Lint rmcp feature assertion (non-vacuous), and `viola-channel`'s tokio check by the tree form.
- `scripts/sync-crates.txt` — `viola-channel` leaves the cargo-deny loop for the tree form.
- `scripts/deny-probes.sh` — the tree form proven live (its control and its fire).
- `tests/contract_lints.rs` — the member set gains `viola-mcp`, which inherits the lints.
- `tests/cli_output_plain.rs` — `help_lists_only_served_verbs` holds `mcp` out (hidden).
- `.config/nextest.toml` — the stamped-home kill line, if `mcp_tools` starts from a stamped home.

## Sweeps
- `mcpServers` over every `*.rs`, `*.json`, `*.sh`, `*.yml` outside `target/` and `.andromeda/`: 3 hits · 3 changed (`plugin/.mcp.json:2`, `crates/viola-agent-claude/src/lib.rs:153`, `:606`) · 0 no-change.
- `sync-crates` over `*.rs`, `*.sh`, `*.yml`, `*.toml` outside `target/`: 4 hits.
  - 1 changed: `.github/workflows/ci.yml`.
  - 3 no-change: `deny-sync.toml` is a comment, and the two `.andromeda/runs/` files are history.
- `viola-mcp` over `*.rs`, `*.toml`, `*.yml`, `*.sh` outside `target/`: 5 hits.
  - 2 changed (comments that say "lands with viola-mcp"): `crates/viola-channel/Cargo.toml:18`, `crates/viola-channel/src/lib.rs:3`.
  - 3 no-change: `deny-sync.toml:12`, `deny.toml:1` and `Cargo.toml:199` stay true.

## Open questions
- **The boundary crossing.** Before Epoch 6, does the MCP server write `send` / `wait` / `last` / `answer` frames after a liveness-only pre-check (an eighth dated gap), or after full server verification built here? And does its `list` spawn the `claude agents --json` read? → blocks: plan-decision (a P4 fork, the founder's own word, `inputs#I1`).
- **Live `claude` starts.** How many may this chunk make? → blocks: plan-decision (a P4 fork, the founder's own word, `inputs#I1`).
- **`v1-33`.** Which gap-list groups (2 the dialog hold, 3 the screen read, 4 `allow <n>`) the switch needs, and which chunk claims `v1-33` → blocks: plan-decision (a P4 fork, the founder's own word, `inputs#I1`).
