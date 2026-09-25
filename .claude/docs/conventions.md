# Project Conventions

_Extracted from `.andromeda/architecture.md` §Conventions by `/andromeda-setup-project`. Reference for detailed conventions that don't fit in CLAUDE.md's 200-line budget._

## File & directory naming
- Crates: `crates/viola-<area>/`, kebab-case; the root `Cargo.toml` is both the `viola` bin package and `[workspace]`. Rust modules and files are snake_case; prefer `foo.rs` over `mod.rs`.
- `src/cmd/<subcommand>.rs` — one module per subcommand; `src/run/` — PTY pump, wheel, budget governor, readiness-gate wiring.
- Tests: inline `#[cfg(test)] mod tests`; crate `tests/<topic>.rs`; root `tests/<surface>_<topic>.rs` (`cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_`); `crates/viola-e2e/tests/` (`path_`, `mcp_`, `http_`, `sse_`, `cross_`).
- On-disk files (viola home): lower-case; `.ndjson` logs, `.json` snapshots, one `<name>.lock` sibling per guarded file.

## Variable & identifier naming
- Types UpperCamelCase; functions/variables snake_case; constants SCREAMING_SNAKE_CASE (standard Rust).
- Env vars: `VIOLA_` prefix (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`) — wrapper-to-child plumbing only, never configuration. Test-harness-only variables use `AGENT_RUN_` (`AGENT_RUN_CHUNK_BASE`, `AGENT_RUN_KEEP_HOMES`, `AGENT_RUN_KEEP_FAILED` — read by `viola-harness` and/or the root test chain `tests/support/home.rs`) and are never read by `viola`.
- Product/binary/identifiers use `viola`; `BRIDGE_NAME` / `BridgeName` from earlier artifacts are superseded.
- Error enums: one enum per crate named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `UiError`, `CoreError`), each `thiserror` except `PtyError`, whose `Display`/`Error` are hand-written with fixed messages (`viola-pty` takes no thiserror).

## Data model conventions (no database)
- Instance key: `ViolaName` — ASCII `[a-z0-9-]`, 1–32 chars, starts with a letter, enforced by nutype. No UUID/serial keys.
- Endpoint name: `viola-<h12>`, the first 12 hex chars of FNV-1a 64 over `ViolaName` + `\0` + absolute viola home — hand-written in `viola-channel` (never `DefaultHasher`, not stable across Rust releases); recorded in the snapshot.
- Timestamps: `ts` on every event/frame, `*_at` for other instants; RFC 3339 UTC with ms and `Z` (`to_rfc3339_opts(SecondsFormat::Millis, true)`).
- Optional fields: `Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]`; missing and `null` read the same.
- Unparseable external values become `"unknown"`, never an error. Percentages are the `Percent` newtype (0–100, JSON number).
- ndjson: one complete object + `\n` per single `write`; multi-line text as an escaped string.

## Wire formats
- JSON field names snake_case (Claude's own `last_assistant_message`, `resets_at`, `used_percentage` kept); enum values kebab-case (`#[serde(rename_all = "kebab-case")]`).
- Integer `v` in every channel `params`, event line, snapshot and GUI JSON body; `sender` (`CARGO_PKG_VERSION`) in every `params`; snapshot `writer`.
- Channel methods lower-case, dot-namespaced for hooks (`hook.dialog`, `hook.event`); JSON-RPC ids integers, monotonic per connection.
- GUI routes unversioned under `/api/`; `application/json`, `application/problem+json` (with `v`), `text/event-stream`.

## Error handling
- Channel: refusal is a normal `result.refusal` + `detail`; `error` only for protocol faults (`-32700/-32600/-32601/-32602/-32603`).
- CLI exit codes: 0 · 1 · 2 · 10 human-typing · 11 budget-paused · 12 unverified-cli · 13 not-delivered · 14 unknown · 20 wrapper fault · 21 instance unreachable; `viola hook` always 0.
- `anyhow` only in the root bin: `main`, dispatch and the catch-site reporter `viola::obs::report_internal_error`, which records the chain only in the instance detail file; typed thiserror errors elsewhere, with fixed `Display` messages (no paths/payloads).
- GUI: RFC 9457 Problem Details with `urn:viola:problem:*` types (`host-not-allowed` 403, `method-not-allowed` 405, `not-found` 404, `state-unreadable` 503, `unauthorized` 401, `cross-origin-forbidden` 403 v1.x).
- Full detail only to `instances/<name>/diagnostics/detail-<process>.ndjson`.

## CLI
- Flags kebab-case long options (`--json`, `--file`, `--budget`, `--port`, global `--home`); subcommands single lower-case words (`plugin install` the one two-word verb).
- Arguments mirror channel params: `viola send <target>` (text from stdin or `--file`), `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`, `viola last <target>`, `viola answer <target> <dialog_id>` (response JSON from stdin/`--file`), `viola pause|release <target> [--budget]`, `viola link|unlink <driver> <driven>`.
- Human text by default, `--json` for agents; the CLI adds `from` from its own `VIOLA_NAME`.

## Configuration
- Precedence: CLI flags > `<viola home>/config.json` > built-in defaults. Home resolution: `--home` → grandparent of `VIOLA_DIR` → `~/.viola/`.
- `config.json` is parsed tolerantly (unknown keys skipped and counted) and carries `v`; keys: budget thresholds (`Percent`, defaults five_hour 90 / seven_day 85), GUI port (47319), `diagnostics_level` (obs D-15).

## Logging
See `.claude/rules/observability.md` for logging rules that apply when editing product code. This doc contains the *conventions*; the rule file is the *enforcement*.

## Testing
See `.claude/rules/testing.md` for testing rules. This doc describes *conventions* (test naming, structure); the rule file has *enforcement* for path-matched files.

## Imports / module organization
- Crate dependency direction is enforced by manifests: `viola-core` depends on no viola crate; `viola-pty` knows no agent; only `viola-agent-claude` knows Claude payload shapes; only `viola-mcp` / `viola-ui` list tokio; the root bin depends on all members.
- Shared third-party versions come from `[workspace.dependencies]`; every member sets `publish = false`, `version.workspace = true` and `license.workspace = true` (`MIT OR Apache-2.0`; `fuzz/Cargo.toml` carries the literal).

## Cross-references
- For complete architecture, see `.andromeda/architecture.md`
- For directory layout, see `.andromeda/architecture.md` Infrastructure Patterns section
- For per-crate specifics, see `.claude/docs/services/{crate}.md`
- For warnings and gotchas, see `.claude/docs/gotchas.md`
