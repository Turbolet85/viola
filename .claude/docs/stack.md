# Technology Stack

_Extracted from `.andromeda/architecture.md` §Stack and Technologies by `/andromeda-setup-project`. The arch table is mirrored verbatim below; the specialist-tool rows after it come from the test, obs, security and a11y plans. Primary source: architecture.md._

## Stack (architecture.md §Stack and Technologies — verbatim)

| Layer | Technology | Role |
|---|---|---|
| Language / runtime | Rust stable (host 1.95; workspace `rust-version = "1.89"`, edition 2024) | One native binary `viola` / `viola.exe`. 1.89 is the highest floor among the dependencies (std `File::lock`); axum needs 1.80 and rmcp 1.88 |
| Backend framework (concurrency model) | Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.1 built only inside `mcp`, `ui` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp and axum require it |
| HTTP server (GUI) | axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, with `text/event-stream` excluded) | View-only GET routes and the SSE feed on 127.0.0.1 |
| CLI parser | clap 4.6.7 (derive) | Subcommands `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install` |
| PTY layer | portable-pty `=0.8.1` (pinned) behind viola's own `pty` seam; windows-sys 0.61.2 for the `TerminateProcess` kill fallback | Hosts the unmodified `claude` in ConPTY (Windows) or openpty (Unix) |
| Screen model | vt100 0.16.2 | Pre-send readiness and modal detection on `run`'s pump thread. Never used to read content |
| Wrapper IPC | interprocess 2.4.4 `local_socket` (sync API; Tokio flavour only in `viola-mcp`) | One endpoint per `viola run`: a named pipe on Windows, a Unix domain socket elsewhere |
| Database | None. ndjson append logs + atomically replaced JSON snapshots on the local filesystem | Authoritative audit trail and mutable state (wheel mirror, links, budget readings) |
| ORM / migrations | N/A. Every record carries `v`, readers skip unknown kinds and fields, and a snapshot with an unsupported `v` is rebuilt by replaying the log | Stands in for schema migration |
| State-file primitives | atomic-write-file 0.3.1 (snapshots); std `File::lock` on separate `.lock` files; std `OpenOptions::append` (one `write` per line) | Crash-safe multi-process writes with no C code |
| Serialization | serde 1.0.229, serde_json 1.0.151, serde_path_to_error 0.1.20 | Channel frames, log events, snapshots, tolerant parsing of external payloads with path-precise drift reports |
| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100) |
| Timestamps | chrono 0.4.45 | RFC 3339 UTC, millisecond precision. Already in the tree through rmcp |
| Error types | thiserror 2.0.20 (per crate); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains only at dispatch |
| Message broker | None. Per-wrapper local sockets (JSON-RPC 2.0 over ndjson) for request/response; notify 8.2.0 tailing the ndjson logs for fan-out to `ui` | Local IPC and event fan-out without a broker or daemon |
| Push / real-time | SSE through axum 0.8.9 `Sse::keep_alive`, fed by notify 8.2.0 | Live GUI feed `/api/events` |
| Process liveness | sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file |
| MCP server | rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2 (tool input schemas) | stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list` |
| AI/ML serving | N/A | viola calls no model API. It drives the unmodified `claude` CLI on the user's subscription |
| Mobile framework | N/A | The phone view is a later version: the same web page behind authentication |
| Container runtime / deployment | None. `cargo install --path .`; Claude Code plugin compiled into the binary (`include_str!`) and written out by `viola run` | Local-only v1, no hosting |
| CI/CD | GitHub Actions matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`; dtolnay/rust-toolchain; Swatinem/rust-cache 2.9.2 | Build, lint and test on all three OSes against the fake agent, on native runners |
| Code quality | rustfmt, clippy (`-D warnings`), `cargo check`; cargo-deny 0.20.2 (licences, C-dependency bans); cargo-modules 0.27.0 (module graph review) | Lint, typecheck, dependency policy, boundary review |
| Release (v1.x, not v1) | dist (cargo-dist) 0.33.0 + cargo-auditable 0.7.6; later self_update 1.3.0 | Public signed releases and installers once distribution is in scope |

**Pending arch amendments already ratified by the plans** (folded into architecture.md by the wrap reconcile of the implementing chunk): the workspace `rust-version` rises to **1.96** (security toolchain floor; sysinfo 0.39.6 needs 1.95; obs D-22, requirements v1-19), with one current stable pinned in `rust-toolchain.toml`; the test-only crate `crates/viola-e2e` joins the workspace (test-plan §12).

## Security-plan additions
- getrandom 0.4.3 (GUI token) · constant_time_eq 0.6.0 (token compare) · windows-sys 0.61.2 SID / SQOS / DACL APIs · a pure-Rust SHA-256 crate (open question — picked and logged before the chunk that writes `bin/`).
- Audit: cargo-deny `>=0.20.2` (+ `[advisories]`, `[sources]`, `[[bans.features]]`), zizmor `>=1.30.1`; cargo-audit `>=0.22.2` deferred to v1.x binary scans.

## Observability (obs-plan §3)
- tracing 0.1.44 (every instrumentable crate) · tracing-subscriber 0.3.23 (root bin only; `fmt,json,registry,std`, no `ansi`, no `tracing-log`, no `chrono` feature) · veil 0.3.0 `#[derive(Redact)]` (`toggle` feature banned) · tower-http `trace` feature in viola-ui.
- No OTel SDK, no OTLP exporter, no error reporter (sentry banned), no `tracing-appender`, no in-process metrics.

## Testing (test-plan §2, §3, §9)
- cargo-nextest 0.9.146 · cargo-llvm-cov 0.9.1 · cargo-mutants 27.1.0 · hyperfine 1.20.0 · cargo-fuzz 0.13.2 (nightly, ubuntu) · jaq 3.1.1.
- Dev-deps: rstest 0.27 · tempfile 3.27 · assert_cmd 2.2.2 · predicates 3.1.4 · trycmd 1.2.1 · insta 1.48.0 · jsonschema 0.57.0 · proptest 1.11.0 (+ proptest-derive 0.8.0, arbitrary 1.4.2) · mockall 0.15.0 · mock_instant 0.6.1 · axum-test 21.1.0 · tower 0.5.3; in `viola-e2e`: rmcp `=3.4.1` (`client`, `transport-child-process`), reqwest 0.13.5, eventsource-client 0.18.0, tokio 1.53.1 `test-util`.
- Node (test-side only, `e2e-web/`): @playwright/test 1.63.0 · @axe-core/playwright 4.13.0.

## Accessibility (a11y-plan §3)
- colorjs.io 0.7.1 · tabbable 6.5.0 · @guidepup/virtual-screen-reader 0.33.0 · html-validate 11.16.0 · eslint-plugin-lit-a11y 5.1.1 (+ eslint core, lockfile-pinned). Lighthouse / pa11y dropped (D-A11Y-05).

## Frontend (design-system §Surface: web-spa)
- Lit 3.3.3 vendored ESM, light DOM, no build step; hand-written `@layer` CSS; installed fonts only (Bahnschrift / Cascadia Mono stacks with DejaVu as the Linux CI render).

## Rationale

For architectural rationale behind these choices, see `.andromeda/architecture.md` Established Decisions section. Each `[Tag]` in that section explains why a specific technology was chosen over alternatives considered during `/andromeda-arch`.

## Version updates

When updating a dependency version:
1. Update `.andromeda/architecture.md` Stack table first
2. Update `[workspace.dependencies]` in the root `Cargo.toml` (and `e2e-web/package.json` for Node tools)
3. Re-run `/andromeda-setup-project` to refresh this file and any rule files that reference the tool version (name the leaves to regenerate)
4. Test that hooks and tooling still work
5. Commit as `chore: bump {tool} to {version}`
