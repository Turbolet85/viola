# Technology Stack

_Extracted from `.andromeda/architecture.md` §Stack and Technologies by `/andromeda-setup-project`. The arch table is mirrored verbatim below; the specialist-tool rows after it come from the test, obs, security and a11y plans. Primary source: architecture.md._

## Stack (architecture.md §Stack and Technologies — verbatim)

| Layer | Technology | Role |
|---|---|---|
| Language / runtime | Rust 1.98.1, pinned exactly by `rust-toolchain.toml` (rustfmt + clippy); workspace `rust-version = "1.96"`, edition 2024 | One native binary `viola` / `viola.exe`. 1.96 is the declared floor (the security toolchain floor; sysinfo 0.39.6 needs 1.95, std `File::lock` 1.89, rmcp 1.88, axum 1.80). CI checks the floor in the `msrv` job (`rustup toolchain install 1.96` + `RUSTUP_TOOLCHAIN=1.96`; the pin is untouched). The separate `fuzz/` workspace alone builds on `nightly-2026-09-20` (`fuzz/rust-toolchain.toml`) |
| Backend framework (concurrency model) | Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.1 built only inside `mcp`, `ui` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp and axum require it |
| HTTP server (GUI) | axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, with `text/event-stream` excluded) | View-only GET routes and the SSE feed on 127.0.0.1 |
| CLI parser | clap 4.6.7 (derive) | Subcommands `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install` |
| PTY layer | portable-pty `=0.8.1` (pinned) behind viola's own `pty` seam; windows-sys 0.61.2 for the `TerminateProcess` kill fallback and the host console's raw/VT modes (`Win32_System_Console`); libc `=0.2.189` (Unix only) for the host tty's raw mode — both behind `HostTerminal` | Hosts the unmodified `claude` in ConPTY (Windows) or openpty (Unix) |
| Screen model | vt100 0.16.2 | Pre-send readiness and modal detection on `run`'s pump thread. Never used to read content |
| Wrapper IPC | interprocess 2.4.4 `local_socket` (sync API; Tokio flavour only in `viola-mcp`); on Windows the client opens with windows-sys 0.61.2 `CreateFileW(SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION \| FILE_FLAG_OVERLAPPED)` + `Stream::try_from`, never the default connect | One endpoint per `viola run`: a named pipe on Windows, a Unix domain socket elsewhere; the Windows open leaves the server able to identify, never impersonate, the client (measured) |
| Database | None. ndjson append logs + atomically replaced JSON snapshots on the local filesystem | Authoritative audit trail and mutable state (wheel mirror, links, budget readings) |
| ORM / migrations | N/A. Every record carries `v`, readers skip unknown kinds and fields, and a snapshot with an unsupported `v` is rebuilt by replaying the log. Exception: process-log lines carry no `v`; their version is the schema filename (`schemas/diag-line.v1.json`, `diag-detail.v1.json`) | Stands in for schema migration |
| State-file primitives | atomic-write-file 0.3.1 (snapshots); std `File::lock` on separate `.lock` files; std `OpenOptions::append` (one `write` per line) | Crash-safe multi-process writes with no C code |
| Content hash | sha2 `=0.11.0` (`default-features = false`; pure Rust, no `cc`) | SHA-256 of the exe bytes truncated to 16 hex for the `bin/` / `plugin/` `<version>-<hash>` keys; a root dev-dependency (its KAT) until `viola-state` consumes it |
| Serialization | serde 1.0.229, serde_json 1.0.151 (feature `preserve_order`, which pulls indexmap), serde_path_to_error 0.1.20 | Channel frames, log events, snapshots, tolerant parsing of external payloads with path-precise drift reports; `preserve_order` keeps a printed document's declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`) |
| Logging | tracing 0.1.44 (default features off, `std`); tracing-subscriber 0.3.23 (default features off, `fmt,json,registry,std`; root bin only) | One-line JSON process logs into the viola home's `diagnostics/`; the line format and levels are owned by the obs plan |
| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100) |
| Timestamps | chrono 0.4.45 | RFC 3339 UTC, millisecond precision. Already in the tree through rmcp |
| Error types | thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file |
| Message broker | None. Per-wrapper local sockets (JSON-RPC 2.0 over ndjson) for request/response; notify 8.2.0 tailing the ndjson logs for fan-out to `ui` | Local IPC and event fan-out without a broker or daemon |
| Push / real-time | SSE through axum 0.8.9 `Sse::keep_alive`, fed by notify 8.2.0 | Live GUI feed `/api/events` |
| Process liveness | sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file |
| MCP server | rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2 (tool input schemas) | stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list` |
| AI/ML serving | N/A | viola calls no model API. It drives the unmodified `claude` CLI on the user's subscription |
| Mobile framework | N/A | The phone view is a later version: the same web page behind authentication |
| Container runtime / deployment | None. `cargo install --path .`; Claude Code plugin compiled into the binary (`include_str!`) and written out by `viola run` | Local-only v1, no hosting |
| CI/CD | GitHub Actions matrix `windows-2025`, `macos-latest` (macOS 26), `ubuntu-latest`; toolchains installed by `rustup toolchain install` (from `rust-toolchain.toml`, from `fuzz/rust-toolchain.toml` in the fuzz jobs, and `1.96` in `msrv`; no toolchain action); SHA-pinned actions/checkout 7.0.1, Swatinem/rust-cache 2.9.2, taiki-e/install-action 2.87.19, actions/upload-artifact 7.0.1, actions/download-artifact 8.0.1 | Build, lint and test on all three OSes against the fake agent, on native runners |
| Code quality | rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`; ripgrep 15.2.0 (PCRE2; the obs G1/G3 gate tool, installed by `scripts/install-ripgrep.sh`); `jq` (runner-provided: G2, `scripts/release-check.sh` and `scripts/orphans-check.sh`, which refuse with `tool-missing: jq`); jsonschema 0.57.0 (test harness `schema-check`, G4); syn 2.0.119 (`full`, `parsing`, `printing`, `visit`) with proc-macro2 1.0.107 (`span-locations`), test-only in `viola-e2e`: the mutation union's `#[cfg]` reader (`harness::cfg_legs`); cargo-deny 0.20.2 (advisories, licences, sources, bans: C crates, telemetry crates, feature bans; the tokio ban via `deny-sync.toml` per sync crate); zizmor 1.30.1 (GitHub workflow linter); cargo-modules 0.27.0 (`cargo install --locked` in the `lint` job; the orphans gate per lib/bin target in CI, the dependency-graph review on demand); cargo-llvm-cov 0.9.1 with the rustup component `llvm-tools-preview` (the coverage runner behind `run --coverage`; `viola-harness gate` enforces the per-OS floors); cargo-fuzz 0.13.2 (CI-only, `cargo install --locked` in the fuzz jobs), whose `fuzz/` workspace pins libfuzzer-sys `=0.4.13` and arbitrary `=1.4.2` in its own lockfile, outside the root `cargo deny` graph (a test-only exemption, see Build system) | Lint, typecheck, dependency policy, workflow lint, boundary review, coverage, fuzzing |
| Release (v1.x, not v1) | dist (cargo-dist) 0.33.0 + cargo-auditable 0.7.6; later self_update 1.3.0 | Public signed releases and installers once distribution is in scope |

The workspace `rust-version` 1.96 floor, the exact `rust-toolchain.toml` pin and the test-only crate `crates/viola-e2e` are folded into architecture.md (chunk `2026-09-24-three-os-ci-headless-harness-skeleton`).

## Security-plan additions
- getrandom 0.4.3 (GUI token) · constant_time_eq 0.6.0 (token compare) · windows-sys 0.61.2 SID / SQOS / DACL APIs · sha2 `=0.11.0` for the SHA-256 `<hash>` (Decisions Log `2026-09-25`).
- Licence: the project is `MIT OR Apache-2.0` (`[workspace.package]`, `license.workspace = true`; `LICENSE-MIT` + `LICENSE-APACHE`). `deny.toml` allows `MIT`, `Apache-2.0`, `Zlib`, `Unicode-3.0`, plus `0BSD` only per crate for interprocess's `doctest-file` and `recvmsg`.
- Audit: cargo-deny `>=0.20.2` (+ `[advisories]`, `[sources]`, `[[bans.features]]`), zizmor `>=1.30.1`; cargo-audit `>=0.22.2` deferred to v1.x binary scans.

## Observability (obs-plan §3)
- tracing 0.1.44 (every instrumentable crate) · tracing-subscriber 0.3.23 (root bin only; `fmt,json,registry,std`, no `ansi`, no `tracing-log`, no `chrono` feature) · veil 0.3.0 `#[derive(Redact)]` (`toggle` feature banned) · tower-http `trace` feature in viola-ui.
- No OTel SDK, no OTLP exporter, no error reporter (sentry banned), no `tracing-appender`, no in-process metrics.

## Testing (test-plan §2, §3, §9)
- cargo-nextest 0.9.146 · cargo-llvm-cov 0.9.1 · cargo-mutants 27.1.0 · hyperfine 1.20.0 · cargo-fuzz 0.13.2 (nightly, ubuntu) · ripgrep 15.2.0 (G1/G3, `scripts/install-ripgrep.sh`) · `jq` (runner-provided: G2, `scripts/release-check.sh`, `scripts/orphans-check.sh`; jaq 3.1.1 an equivalent local form) · cargo-modules 0.27.0 (`scripts/orphans-check.sh`, the CI orphans gate).
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
