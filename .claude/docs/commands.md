# Commands Reference

_Complete command reference extracted from `.andromeda/architecture.md`, `.andromeda/test-plan.md` §3/§9 and `.andromeda/obs-plan.md` §9 by `/andromeda-setup-project`. The 5 most common commands live in CLAUDE.md Workflow section for always-loaded access — this file has everything else._

_At setup the repo holds no workspace yet: the root `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`, `.config/nextest.toml`, `viola-harness` and `e2e-web/` land with the Epoch-1 Foundation chunks. Commands below are the planned contract._

## Installation
- `rustup show` — installs the toolchain pinned in `rust-toolchain.toml` (one current stable ≥ 1.96, components rustfmt + clippy)
- `cargo install --locked cargo-nextest@0.9.146 cargo-llvm-cov@0.9.1 cargo-mutants@27.1.0 cargo-deny@0.20.2` — test / coverage / mutation / policy tools (CI uses taiki-e/install-action, SHA-pinned)
- `cargo install --locked hyperfine@1.20.0 cargo-modules@0.27.0 zizmor@1.30.1 jaq@3.1.1` — perf gate, boundary review, workflow lint, log assertions
- `npm ci --prefix e2e-web` then `npx --prefix e2e-web playwright install --with-deps chromium` — browser suite (ubuntu CI; local optional)
- `cargo install --path .` — install `viola` from the repo root
- `pip install -r scripts/requirements.txt` — code-graph Python deps (duckdb + protobuf)
- `rustup component add rust-analyzer` — the Rust SCIP indexer for the code-graph (already on PATH on the founder's host)
- `npm i -g @sourcegraph/scip-typescript` — the TypeScript indexer (only if a ts plane is activated)

## Development
- `viola run <name> -- claude [args]` — wrap a live session (Windows live target)
- `viola run <name> --home <dir> -- <fake agent> <args>` — wrap the fake agent in an isolated home
- `viola ui [--port <n>]` — serve the view-only page on 127.0.0.1 (prints the launch line once to stderr)
- `viola verify` — run the live probe suite against the real CLI and stamp the version (local only; spends tokens)

## Harness (the agent's 5 commands — `scripts/agent-run.sh` / `scripts/agent-run.ps1`)
- `boot [--session <id>] [--instance <name>[:<args>]]... [--ui] [--unstamped] [--cli-version <ver>] [--agents-mode recorded|oversize|malformed] [--statusline-echo]` — build, create a home, stamp via `viola verify` against the fake agent, start wrappers (+ UI), wait for readiness
- `run [--unit|--integration|--e2e|--browser|--mutants|--coverage|--perf|--fuzz-replay|--all] [--filter <nextest-filterset>] [--local-live]` — invoke suites; one JSON summary
- `status [--session <id>]` — `viola list --json` + `/ready` + `/api/sessions`, with `api_sessions_equal_list`
- `cleanup [--session <id>|--all]` — graceful stop, endpoint/port/url-file checks, home removal (idempotent)
- `logs [--session <id>] [--instance <name>] [--kind <kind>] [--process run|hook|mcp|ui|cli] [--after <offset>]` — merged ndjson of events + diagnostics
- Internal (forwarded by the shims, not agent-facing): `supervise`, `ui-restart --session <id>`, `gate --require <suites> [--artifacts <dir>]`

## Testing
- `cargo nextest run --workspace --features fake-agent --profile ci -E 'kind(lib) | kind(bin)'` — unit
- `cargo nextest run --workspace --features fake-agent --profile ci -E 'kind(test) & !binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)'` — integration
- `cargo nextest run --workspace --features fake-agent --profile ci -E 'binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)'` — fake-agent E2E
- `cargo test --workspace --doc` — doctests (nextest cannot run them)
- `npx --prefix e2e-web playwright test [--grep "<title>"]` — browser suite (ubuntu)
- `cargo llvm-cov nextest --workspace --features fake-agent --profile ci --lcov --output-path target/lcov.info --ignore-filename-regex '(viola-fake-agent|crates/viola-e2e|tests/support|fuzz/)' --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` — coverage gate
- `NEXTEST_PROFILE=mutants cargo mutants --workspace --features fake-agent --in-diff target/agent-run/chunk.diff --test-tool=nextest` — mutation gate (diff from `git diff <base>...HEAD`)
- `cargo mutants --file <path> --test-tool=nextest` — one file's mutants
- `cargo +nightly fuzz run <target> -- -runs=0` — corpus replay (ubuntu)

## Linting & Formatting
- `cargo fmt --all` / `cargo fmt --all --check` — format / format gate (edition from `rustfmt.toml`)
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — lint gate (`disallowed-macros`, `print_stdout`, `print_stderr`, `dbg_macro` denied workspace-wide)
- `cargo check --workspace --all-targets` — type check
- `cargo check -p viola-core -p viola-pty -p viola-state -p viola-channel -p viola-agent-claude` — the sync crates compile without tokio
- `cargo tree -e features -p viola --edges normal` — assert rmcp shows only `server`, `transport-io`
- `cargo modules dependencies --package <crate> --acyclic` / `cargo modules orphans --package <crate> --deny` — boundary review
- `cargo deny check` (weekly: `cargo deny check advisories`) · `zizmor --format=json .github/workflows/` — supply chain
- `npx --prefix e2e-web eslint -c e2e-web/eslint.config.js -f json` · `npx --prefix e2e-web html-validate --config e2e-web/.htmlvalidate.json --formatter json <index.html>` — a11y lint (ubuntu)

## Observability gates (obs-plan §9, `shell: bash`)
- G1 bare `#[instrument]`: `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` must exit 1
- G2 zero panics over `target/e2e-home/**/diagnostics/*.ndjson` (role files only) via `jq -R -n -e`
- G3 no abort panic strategy: `rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .` must exit 1
- G4 schema conformance against `schemas/diag-line.v1.json` / `schemas/diag-detail.v1.json`; secret scan (`id: secret-scan`) before any upload

## Build & Deploy
- `cargo build --release --bin viola` — release build (fake agent excluded: feature off)
- `cargo build --workspace --features fake-agent` — harness build (boot step 1)
- No deploy stage in v1; v1.x adds a dist 0.33.0 release workflow

## Code-graph
- `python scripts/code-graph.py refresh [plane]` — build every detected plane (wrap backgrounds it)
- `python scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]` — query + trace

## Git & release
- Conventional commits (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:`, `ci:`); work lands on the version's build branch `build/viola-X.Y.Z`
- `gh run view --json jobs` / `gh run download` — read CI results and artifacts

## Troubleshooting
- `cargo clean` — clean build artifacts (also removes `target/e2e-home/` and `target/agent-run/`)
- `scripts/agent-run.sh cleanup --all` — tear down every harness session
- `scripts/agent-run.sh logs --instance <name> | jq -c 'select(.record.level=="ERROR")'` — errors for one instance
