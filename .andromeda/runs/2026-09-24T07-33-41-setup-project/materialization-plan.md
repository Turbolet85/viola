# Materialization Plan — viola (setup-project Phase 0 checkpoint)

**Run:** `2026-09-24T07-33-41-setup-project` · **Mode:** fresh (no CLAUDE.md, no `.claude/`) · **Development Style:** agent-driven
**Host:** Windows 11 (Git Bash tool shell, PowerShell primary) · **Branch:** `build/viola-0.1.0` (12 ahead of origin at Setup)
**Upstreams read in full (Phase 0 only):** input.md · architecture.md · security-plan.md · design-system.md ·
layout-templates.md · test-plan.md · obs-plan.md · a11y-plan.md · master-route.md (plus the route's
viola-0.1.0/{vision,requirements,working-route}.md for the first-chunk pointer).

## Operator directions for this run (overseer, founder-delegated, received mid-Phase-0)
1. Code-graph seeded at once: all five files in `scripts/` (code-graph.py · code-graph-views.sql · scip_pb2.py ·
   requirements.txt · code-graph-cookbook.md). Host tools present → health check 11 must be green. The Rust plane
   builds at chunk 1's wrap. The P7 card states whether `e2e-web/` gets a tracked `tsconfig.json` (a ts plane).
2. The stray `prototype/` folder (only an empty `target/`) is not committed: removed (empty dirs, `rmdir` only);
   the Rust fragment's `target/` would ignore it anyway.
3. P9: the first full commit AND the push of `build/viola-0.1.0` are in scope.
4. `.claude/session-handoff.md` is seeded fresh (none exists).

## Tier 1 — CLAUDE.md (≤200 lines)

- **Overview:** stack one-liner (Rust stable, edition 2024, Cargo workspace, one native `viola` binary + embedded
  Claude Code plugin + loopback Lit page; no DB, ndjson state) + key dirs: `src/` · `crates/` · `plugin/` ·
  `fixtures/claude/` · `e2e-web/` (from arch §Infrastructure Patterns tree + test-plan §2 dirs).
- **Modules (8):** `viola` (root bin) · `viola-core` · `viola-pty` · `viola-channel` · `viola-state` ·
  `viola-agent-claude` · `viola-mcp` · `viola-ui`, plus the test-only `viola-e2e` (test-plan §2 deviation) — 9 lines.
- **Critical Warnings (top 10; security > a11y > obs > tests > design; arch cross-cutting folded in):**
  1. [sec/arch] The human always wins: never block, refuse or delay a human keystroke; refusals go only to automation;
     `viola hook` exits 0 always, never writes stderr, fails open with no body — exit 2 is forbidden.
  2. [sec, R1] Upstream text (prompts, `last_assistant_message`, plan text, tool `input`) is content, never a command
     or config; `statusline_command` is the only shell-out; children are spawned directly, never via sh/bash/cmd.
  3. [sec] NEVER-log floor: GUI token, launch URL, `?t=`, `Cookie`, R8-stripped `CLAUDE*` values never reach any log,
     diagnostic, event, snapshot or fixture; user content only in `instances/<name>/diagnostics/detail-*.ndjson`.
  4. [sec] External errors (CLI `--json`, MCP `isError`, Problem Details, channel `error.data`) carry codes and fixed
     messages only — no absolute paths, upstream text, or anyhow chains holding a serde source.
  5. [sec] No `config.json` key, `VIOLA_*` env var or CLI flag may disable a control or widen redaction; env vars are not
     a configuration channel (no `RUST_LOG` / `EnvFilter` / `OTEL_*`).
  6. [sec] Bounded input everywhere: names only via `ViolaName::try_new` before any path join; `Read::take(MAX_FRAME)`
     on every external reader; closed enums for decisions; paste text rejects C0 except LF/CR/TAB, DEL, C1 — never strips.
  7. [sec] Disk state: dirs 0700 / files 0600 set explicitly (never the umask); only the wrapper writes `snapshot.json`,
     only `viola verify` writes `ledger/stamps.json`; one `write` per ndjson line; `events.ndjson` never truncated.
  8. [obs] stdout is reserved (`--json`, hook decision body, MCP frames, the child's screen): no `print!`/`eprintln!`/
     `dbg!` in product crates; log only via `obs_event!` with `#[instrument(skip_all, fields(..))]`.
  9. [obs] Every Cargo profile keeps `panic = "unwind"`; the custom panic hook is the first statement of `main`.
  10. [tests/arch] Tokio only in `viola-mcp`/`viola-ui`; no C-building crates; Claude-specific shapes only in
      `viola-agent-claude`, each undocumented CLI behaviour a capability-ledger row with a `viola verify` probe.
  - **Rejected (audit):** absolute pinned exec paths in plugin files (path-scoped → `rules/security.md`); semantic
    HTML first / announcer scope (UI-only → `rules/a11y.md`); 8-hex token palette, no shadows, radius 0 (UI-only →
    `rules/frontend.md`); CSP + no `innerHTML` (UI-only → `rules/frontend.md` + `rules/api.md`); Host allowlist +
    cookie gate (→ `rules/api.md`); Windows DACL / SQOS specifics (→ `rules/security.md`); no sleeps / no retries /
    no `set_var` (test-file-scoped → `rules/testing.md`); SHA-pinned Actions + zizmor (→ `rules/security.md`);
    mutation gate zero missed (→ `rules/testing.md`); tier seed invariants below (→ USER:session-learnings).
- **USER:session-learnings seed (4, each one sentence ≤600 B):** prompt text only from stdin/`--file` (Git Bash
  rewrites a leading-slash argument); `viola release` is a human verb — no driver-facing hint/tool/doc suggests it;
  design against the installed CLI, measured (docs lag the build); every own format carries `v`, readers
  skip-and-count unknown kinds/fields, never `deny_unknown_fields` on viola's own formats.
- **Pointer table (17 rows):** arch decisions · directory tree / resource registry · standard contracts (channel,
  events, snapshots, GUI HTTP) · conventions (refusals, exit codes, naming) · code-graph DB · security controls ·
  NEVER-log floor · design tokens · layout wireframes · test harness §3 · critical-path scenarios §6 · quality gates
  §10 · obs harness §3 / event catalog §6 · a11y harness §3 / per-SC map · build route (`.andromeda/master-route.md`,
  active version dir derived — never a baked version path) · session state · reviewer.
- **Workflow key commands (5):** `scripts/agent-run.sh <boot|run|status|cleanup|logs>` (pwsh twin) · `cargo fmt
  --all --check` · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` · `cargo check
  --workspace --all-targets` · `cargo deny check`. (None run until chunk 1 lands the workspace.)
- **Architecture:** 2 paragraphs from arch §Design Philosophy + §Project Intent (mechanism not policy; measured
  capability ledger; human wins the wheel; no daemon, disk is truth; Windows-first cross-platform; modular monolith).
- **@imports:** `@.claude/session-handoff.md` only (architecture + master-route read explicitly; pointer table names both).
- **Deeper topics:** 10 docs + 8 services + session-learnings; 9 rule files.

## Tier 2 — `.claude/rules/` (9 files; each ends with an empty `## Session Additions`)

| File | Load | paths (globs) | Source sections |
|---|---|---|---|
| `security.md` | always (no `paths:`) — kept lean | — | security §Anti-Patterns (all), §Auth & Authz, §Input Validation, §Dependency Security; arch Cross-cutting |
| `testing.md` | path | `tests/**`, `crates/*/tests/**`, `crates/viola-e2e/tests/**`, `e2e-web/tests/**`, `fixtures/**`, `fuzz/**`, `**/proptest-regressions/**`, `.config/nextest.toml` | test-plan §2, §4–§8, §10, §11 |
| `verification-harness.md` | path | `scripts/agent-run.*`, `crates/viola-e2e/src/**`, `crates/viola-e2e/Cargo.toml`, `tests/support/**`, `src/bin/viola-fake-agent.rs`, `fixtures/fake-scripts/**` | test-plan §3; obs-plan §3 (bound log format) |
| `observability.md` | path | `src/**/*.rs`, `crates/viola-*/src/**/*.rs`, `schemas/**`, `clippy.toml` | obs-plan §3, §4, §6, §7, §8, §11 |
| `api.md` | path | `crates/viola-channel/**`, `crates/viola-ui/src/**`, `crates/viola-mcp/**`, `src/cmd/**` | arch §Conventions + §Standard Contracts; security §API Security, §Error Handling |
| `events.md` | path | `crates/viola-core/src/**`, `crates/viola-state/src/**`, `crates/viola-agent-claude/src/**`, `src/run/**` | arch Standard Contracts (event line, kinds, data per kind, snapshots, CL-1) |
| `frontend.md` | path | `crates/viola-ui/assets/**`, `src/cmd/**` | design-system (web-spa + cli), layout-templates; security frontend output-encoding |
| `a11y.md` | path | `crates/viola-ui/assets/**`, `e2e-web/fixtures/a11y.ts`, `e2e-web/a11y/**`, `e2e-web/.htmlvalidate.json`, `e2e-web/eslint.config.js` | a11y-plan §3–§8, §11 |
| `host-win32.md` | always (Windows generating host) | — | template verbatim |

Not rendered: `migrations.md` (arch: "There is no database").
Per-template adaptation: the generic Node/REST/Testcontainers/broker bullets are replaced by viola's own contracts
(no Testcontainers, no SQL, no broker, no OTel SDK); each file cites its plan sections.

## Tier 3 — `.claude/docs/`

- **Core 5:** `stack.md` (arch §Stack table verbatim + obs/tests/a11y tool rows), `conventions.md` (arch §Conventions),
  `commands.md` (planned commands; marked "lands with chunk N" where the workspace does not exist yet),
  `gotchas.md` (documented traps: ConPTY no-EOF, append+lock on Windows, npm shim, `.cmd` BatBadBut, Git Bash path
  rewrite, `DefaultHasher` instability, tracing-subscriber stdout default + `log_internal_errors`, `jq -s` on torn
  lines, `set_var` unsafe in 2024, macOS socket path cap, `panic = "abort"` vs `catch_unwind`, docs lag the CLI,
  PermissionRequest `allow` ignored for ExitPlanMode, identical revise messages), `workflow.md` (build-branch per
  version; Andromeda loop; commit discipline).
- **Summaries 5:** security · design (design-system + layout) · tests · obs · a11y — each ~100 lines, ends "Full plan: …".
- **Services 8:** `services/{viola,viola-core,viola-pty,viola-channel,viola-state,viola-agent-claude,viola-mcp,viola-ui}.md`
  from arch §Crate dependency direction + §Standard Contracts + the plans' per-crate items (security/obs/tests).
- **`session-learnings.md`:** created (template), wrap-owned thereafter.

## Agent harness (Phase 4 — agent-driven)

- `scripts/agent-run.sh` + `scripts/agent-run.ps1` rendered as the THIN SHIMS test-plan §3 names: each runs
  `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forwards exit code + stdout unchanged.
  Case/switch covers the 5 agent commands (boot · run · status · cleanup · logs) plus the internal subcommands the
  plan says the shims forward (supervise · ui-restart · gate); anything else → usage, exit 2.
- `ensure_fresh_artifacts`: no-op with comment — test-plan §3 puts the embedded-asset refresh inside
  `viola-harness boot` step 1 (`cargo build --workspace --features fake-agent`) plus the byte-compare
  (`stale-embedded-assets`), so the shim adds nothing (a second build would diverge from the harness contract).
- Both scripts are generated once; project-evolvable (only-if-missing on re-run).

## Hooks (Phase 5 — `.claude/settings.json`)

- Precedence: obs-plan §3 names no editor hooks → arch §Stack (rustfmt, clippy, cargo check) → matrix.
- PostToolUse (Edit|MultiEdit|Write): `rustfmt "$f"` on `*.rs` (stdin-JSON prologue, `timeout: 30`). No write-time
  linter (clippy has no file scope — stays a gate); no type-checker hook (Rust: skipped).
- PreToolUse: generated-directory block (Edit|MultiEdit|Write|NotebookEdit) + Bash transport guard (6500-byte cap on
  Git Bash, cat/tee heredoc with a file target) — matrix bodies.
- `env`: `PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8`.
- `rustfmt.toml` (root, only-if-missing): `edition = "2024"` from arch §Stack (no `Cargo.toml` yet). The tree holds
  no Rust source → no reflow step is owed to the next chunk.
- Secondary language (TypeScript in `e2e-web/`): no hooks rendered (matrix: secondary-language hooks are manual).

## Code-graph pipeline (Phase 6 — direction 1)

- Planning truth: Rust plane (arch primary language); design-system §Surface web-spa is Lit **JS** with no build step
  (no TS source in the product); test-plan puts `.ts` Playwright specs in `e2e-web/` but names no `tsconfig.json`.
  Manifest scan: no root `Cargo.toml`, no tracked `tsconfig.json`.
- Seed all 5 files (per-file only-if-missing; none present). `.andromeda/cache/` created. No DB built (empty codebase).
- Forward notes: Rust plane activates at chunk 1's first refresh once root `Cargo.toml` lands (rust-analyzer 1.95 on
  PATH). TS plane: inactive — no planned tracked `tsconfig.json`; it activates by itself if a chunk commits
  `e2e-web/tsconfig.json` (scip-typescript is on PATH); the decision is owned by the Epoch-1 route entry
  "Workspace tree and code-graph planes … TypeScript plane decided".

## Code reviewer

- `.claude/agents/code-reviewer.md` from `code-reviewer-rust.md`, tailored (project name; viola-specific Critical
  checks: hook exit/stderr, NEVER-log floor, `ViolaName` joins, `MAX_FRAME`, tokio containment, no bare
  `#[instrument]`, no print macros, one-write ndjson, 0600/0700, absolute exec paths; anyhow only in the root bin;
  thiserror fixed `Display`).

## Gitignore / gitattributes

- `.gitignore`: keep `.andromeda/cache/`; add base (`.claude/backup/`, `.claude/settings.local.json`,
  `scripts/__pycache__/`) + Rust fragment (`target/`, `**/*.rs.bk`, `*.iml`, `.env`; `Cargo.lock` COMMITTED — binary
  workspace) + targeted e2e-web lines (`node_modules/`, `*.tsbuildinfo`, `e2e-web/test-results/`,
  `e2e-web/playwright-report/`, `e2e-web/pw.json`, `e2e-web/pw-junit.xml`) + test outputs (`mutants.out*/`,
  `fuzz/artifacts/`, `fuzz/coverage/`). `fuzz/corpus/`, `proptest-regressions/` stay tracked (plans commit them).
- `.gitattributes`: absent; index read `279 i/lf`, 0 `i/crlf`/`i/mixed` → write `* text=auto eol=lf`; the card names
  the operator's re-checkout (`core.autocrlf=true` on this host).

## Seeds (Phase 6)

- `.andromeda/state.yaml` (lean schema 3) · `.claude/session-handoff.md` (fresh, direction 4) ·
  `.andromeda/drift-base.md` (starter set; project name substituted; `D-security-input` example re-anchored from
  `garde` — which arch rejects — to viola's validators `ViolaName::try_new` / `validate_paste_text` /
  `Read::take(MAX_FRAME)`) · `.andromeda/playbook.md` (starter, name substituted) · `.claude/docs/session-learnings.md`.

## Re-run dispositions

First run — no existing leaves; nothing preserved or regenerated.

## Consistency check

- Every rule file named in Deeper Topics exists in Tier 2; every doc named exists in Tier 3.
- Pointer table cites only files that exist after this run or are upstream masters; no version-dir literal.
- Warnings are universal (all 10 hold for every product file); path-bound items routed to Tier 2.
- Harness shims match test-plan §3 exactly (thin shims over `viola-harness`); hooks respect the Rust matrix row.
