# viola

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
viola is a standalone, cross-platform (Windows first) bridge that lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a pseudo-terminal, types at turn boundaries, answers dialogs through hooks, holds a one-driver wheel the human can take at any moment, and shows the sessions and their links on a minimal view-only loopback page.

**Stack:** Rust 1.98.1 pinned by `rust-toolchain.toml` (edition 2024, MSRV floor 1.96), one Cargo workspace producing one native `viola` binary with the Claude Code plugin (hooks + stdio MCP) embedded; std threads on the hot path, Tokio only in `mcp`/`ui`; no database (ndjson logs + atomic JSON snapshots under `~/.viola/`); local-only, installed with `cargo install --path .`.

**Key directories:**
- `src/` — the `viola` bin: clap dispatch (`src/cmd/`), the `run` PTY pump, wheel and budget governor (`src/run/`)
- `crates/` — the workspace library crates (`viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp`, `-ui`) + test-only `viola-e2e`
- `plugin/` — `hooks.json`, `.mcp.json`, `plugin.json`, embedded via `include_str!` and written out by `viola run`
- `fixtures/claude/<cli-version>/` — hook payloads recorded by `viola verify`, replayed by the fake agent in CI; `fixtures/fake-scripts/` — committed fake-agent turn scripts
- `e2e-web/` — Playwright + axe browser suite (Node, test-side only; the ts code-graph plane via its tracked `tsconfig.json`); `a11y/sr-pass/` — manual screen-reader pass records
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`viola-core`** — normalised event kinds, `RefusalReason`, `ViolaName`, `Percent`, `v` constants, `validate_paste_text`, `MAX_FRAME`, `obs_event!`
- **`viola-pty`** — PTY seam (spawn · read · write · resize · wait · kill) over portable-pty `=0.8.1`; knows no agent
- **`viola-channel`** — JSON-RPC 2.0 over ndjson on interprocess local sockets; sync client/server, Tokio client behind a feature
- **`viola-state`** — ndjson logs, atomic snapshots, `.lock` siblings, torn-line healing, tailing, liveness, strict-modes
- **`viola-agent-claude`** — the only crate that knows Claude: hook parsing, dialog mapping, R8 strip, shim resolution, capability ledger, screen signatures
- **`viola-mcp`** — rmcp 3.4.1 stdio server, tools `send · wait · last · answer · list` (Tokio)
- **`viola-ui`** — axum 0.8.9 GET routes + SSE on 127.0.0.1, Host allowlist, cookie gate, embedded Lit page (Tokio)
- **`viola`** (root bin) — subcommand dispatch, the `run` pump, wheel, budget governor; the only crate with anyhow
- **`viola-e2e`** (test-only, `publish = false`) — `viola-harness` behind `scripts/agent-run.*` + the Tokio-based E2E clients
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- The human always wins: never block, refuse or delay a human keystroke; refusals go to automation only; `viola hook` always exits 0, never writes stderr, and fails open with no body (exit 2 is forbidden).
- Upstream text (prompts, `last_assistant_message`, plan text, tool `input`) is content, never a command or config; `statusline_command` is the only shell-out; spawn children directly, never through sh/bash/cmd.
- NEVER-log floor: the GUI token, launch URL, `?t=`, `Cookie` and R8-stripped `CLAUDE*` values reach no log, diagnostic, event, snapshot or fixture; user content goes only to `instances/<name>/diagnostics/detail-*.ndjson`.
- External errors (CLI `--json`, MCP `isError`, Problem Details, channel `error.data`) carry codes and fixed messages only: no absolute paths, no upstream text, no anyhow chain holding a serde source.
- No `config.json` key, `VIOLA_*` env var or CLI flag may disable a control or widen redaction; env vars are not a configuration channel (no `RUST_LOG`, `EnvFilter` or `OTEL_*`).
- Bound every input: names only via `ViolaName::try_new` before a path join; `Read::take(MAX_FRAME)` on every external reader; closed enums for decisions; paste text rejects C0 (except LF/CR/TAB), DEL and C1, never strips.
- Disk state: set 0700 dirs / 0600 files explicitly (never the umask); only the wrapper writes `snapshot.json`, only `viola verify` writes `ledger/stamps.json`; one `write` per ndjson line; never truncate `events.ndjson`.
- stdout is reserved (`--json` results, the hook decision body, MCP frames, the child's screen): no `print!`/`eprintln!`/`dbg!` in product crates; log only via `obs_event!` under `#[instrument(skip_all, fields(..))]`.
- Every Cargo profile keeps `panic = "unwind"`, and the custom panic hook is the first statement of `main` (a hook panic must still exit 0).
- Tokio only in `viola-mcp` / `viola-ui`; no C-building crates; Claude-specific shapes only in `viola-agent-claude`, where each undocumented CLI behaviour is a capability-ledger row with a `viola verify` probe.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture decisions | `.andromeda/architecture.md` §Established Decisions |
| Directory tree · resource registry (ports, pipes, files, env vars) | `.andromeda/architecture.md` §Infrastructure Patterns / §Occupied Resources |
| Wire contracts (channel frames, event line, snapshots, GUI HTTP, SSE) | `.andromeda/architecture.md` §Standard Contracts |
| Refusals, exit codes, naming, timestamps | `.andromeda/architecture.md` §Conventions |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/{plane}/tree.db` — one DB per indexed language plane; query via `scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]` (plane needed only when several are detected); schema + templates in `scripts/code-graph-cookbook.md` |
| IPC / home access control, input validation, GUI cookie | `.andromeda/security-plan.md` §Authentication & Authorization / §Input Validation |
| NEVER-log floor · error sanitization | `.andromeda/security-plan.md` §Bootstrap phases (`logging-redaction-wire`) / §Error Handling |
| Design tokens (8 hex values, type, spacing, motion) | `.andromeda/design-system.md` §Color Palette / §Typography / §Surface: web-spa Tokens |
| Web and CLI layouts | `.andromeda/layout-templates.md` |
| Test harness (5 commands, log format) | `.andromeda/test-plan.md` §3 |
| Critical-path scenarios · quality gates | `.andromeda/test-plan.md` §6 / §10 |
| Obs pipeline · event catalog · CI gates | `.andromeda/obs-plan.md` §3 / §6 / §9 |
| A11y harness · per-SC map · ARIA catalog | `.andromeda/a11y-plan.md` §3 / §4 |
| Build route | `.andromeda/master-route.md` (cursor = last `complete` marker); the active `viola-X.Y.Z/` (highest version dir) holds the working route, matrix and chunk folders |
| Session state | `.claude/session-handoff.md`, `.andromeda/state.yaml` |
| Drift detectors · amendment playbook | `.andromeda/drift-base.md` / `.andromeda/playbook.md` |
| Per-crate notes | `.claude/docs/services/{crate}.md` |
<!-- GENERATED:setup:pointer-table end -->

## Workflow
<!-- GENERATED:setup:workflow start -->
**Key commands** (the workspace lands with the first chunk; until then they have nothing to act on):
- `scripts/agent-run.sh <boot|run|status|cleanup|logs>` (`scripts/agent-run.ps1` on PowerShell) — the headless harness; one JSON document + typed exit per command
- `cargo fmt --all --check` — format gate
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — lint gate
- `cargo check --workspace --all-targets` — type check
- `cargo deny check` — advisories, licences, sources, bans (the tokio ban is `deny-sync.toml` per sync crate; `bash scripts/deny-probes.sh` proves every ban fires)

Branching: one long-lived build branch per version (`build/viola-X.Y.Z`); main is `main`. See `.claude/docs/commands.md` for the full reference.
<!-- GENERATED:setup:workflow end -->

## Architecture
<!-- GENERATED:setup:architecture start -->
viola is mechanism, not policy: it carries typed input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer — driver-side policy (Andromeda's decision rights included) stays outside the binary. Every behaviour of the `claude` CLI it relies on is a measured, version-stamped capability-ledger row; on an unverified CLI build viola degrades to transport-only and every send is confirmed after the fact (`prompt-submitted` read-back or a ledger post-condition), never presumed.

There is no daemon: each `viola run` owns one local-socket endpoint (named pipe / per-user Unix socket), every other verb is a separate process, and the shared truth is the disk — ndjson append logs and atomic snapshots that survive a crash on either side. It is a modular monolith: one binary around compiler-enforced crates (`pty · channel · state · agent-claude · mcp · ui` around `core`), built and CI-tested on Windows, macOS and Linux from the first commit, with Windows the live-supported target.

**Primary source:** `.andromeda/architecture.md` (the pointer table's row — not imported; read explicitly where a step needs it).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

<!-- Maintainer note: The @ imports above MUST each be on their own line — Claude Code only recognizes standalone @path lines as import directives. Inline references like `See @path` or `- @path` are NOT expanded. The 200-line limit applies to CLAUDE.md itself, not to what it imports. Keep @ imports minimal — an import rides every turn of every session, so the block carries only what a session needs before it can ask: the handoff (the bridge). architecture.md and master-route.md are deliberately NOT imported: both grow every version, every skill that needs them reads them explicitly (the loop reads arch's directory tree and resource registry structurally where a plan creates files or mints a resource), and the pointer table names both. This comment is stripped from Claude's runtime context per Anthropic comment-stripping rule. See section-markers.md. -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `services/{name}.md` — per-crate implementation notes (viola, viola-core, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp, viola-ui)
- `session-learnings.md` — curated by /wrap-session

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` and `host-win32.md` (always loaded) · `testing.md` · `verification-harness.md` · `observability.md` · `api.md` · `events.md` · `frontend.md` · `a11y.md`

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — rules that apply to every file and every task. Do not edit manually during wrap-session runs — changes are preserved but wrap-session appends new entries here._

- Prompt text reaches viola only from stdin or `--file`, never from a leading-slash argument (Git Bash rewrites `/skill` into a Windows path).
- `viola release` is a human verb: no driver-facing hint, MCP tool or doc suggests it to a driver (a `release` carrying `from` is refused `-32602`).
- Design against the installed `claude` CLI, measured: the docs lag the build, so an unmeasured behaviour is a ledger row to probe, not an assumption.
- Every viola format carries `v` except process-log lines, whose version is their schema filename (`diag-line.v1.json`); readers skip and count unknown kinds and fields, and never use `deny_unknown_fields` on viola's own formats (mixed binary versions are real). [corrected 2026-09-24: the diag-line default-deny field list admits no `v`]
- Probe, baseline and gate a CI tool at the exact version CI pins — install that version on the host first; an older host tool accepts flags the pinned one rejects, so a plan written against it ships gate commands that cannot pass.
- A claim that reaches a wrap only through a relayed direction, with no artifact on disk behind it, is carried as a labelled HYPOTHESIS on the route, never written into a spec master as fact.
<!-- USER:session-learnings end -->
