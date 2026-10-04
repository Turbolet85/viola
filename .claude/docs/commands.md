# Commands Reference

_Complete command reference extracted from `.andromeda/architecture.md`, `.andromeda/test-plan.md` §3/§9 and `.andromeda/obs-plan.md` §9 by `/andromeda-setup-project`. The 5 most common commands live in CLAUDE.md Workflow section for always-loaded access — this file has everything else._

_The workspace, `rust-toolchain.toml`, `.config/nextest.toml`, `viola-harness` and `.github/workflows/ci.yml` exist since chunk 1; `deny.toml`, `deny-sync.toml`, `scripts/sync-crates.txt`, `scripts/deny-probes.sh` and `.github/workflows/nightly.yml` since chunk 3; `e2e-web/` lands with a later chunk. Commands for surfaces not built yet are the planned contract._

## Installation
- `rustup toolchain install` — installs the toolchain pinned in `rust-toolchain.toml` (1.98.1, components rustfmt + clippy)
- `cargo install --locked cargo-nextest@0.9.146 cargo-llvm-cov@0.9.1 cargo-mutants@27.1.0 cargo-deny@0.20.2` — test / coverage / mutation / policy tools (CI uses taiki-e/install-action, SHA-pinned)
- `cargo install --locked hyperfine@1.20.0 cargo-modules@0.27.0 zizmor@1.30.1` — perf gate, orphans gate + on-demand boundary review, workflow lint (CI installs cargo-modules this way in `lint`: taiki-e has no manifest for it)
- `bash scripts/install-ripgrep.sh`: ripgrep 15.2.0 (PCRE2, sha256-verified) into `target/tools/ripgrep/bin` for G1/G3.
- `bash scripts/install-node.sh linux-x64 ~/.local/viola-node` — the pinned Node `pre-push`'s `tools` stage checks on the Linux dev host (`pre-push` installs nothing; `cc`, the channel and the cargo tools are the host's own, at ci.yml's pins)
- `bash scripts/install-node.sh <linux-x64|darwin-arm64|win-x64> <dest>` / `--probe` — the official Node build at ci.yml's `NODE_PIN_*` (parsed from the file text), sha256-verified; prints `install-node: <bin dir>`
- `jq` for G2, `scripts/release-check.sh`, `scripts/orphans-check.sh` and `scripts/npm-audit.sh`: runner-provided in CI, never installed there (the `test` and `perf` jobs presence-check it; the scripts, `scripts/g2-zero-panics.sh` included, refuse with `tool-missing: jq`). jaq 3.1.1 takes the same filters locally.
- `npm ci --prefix e2e-web` then `npx --no --prefix e2e-web playwright install chromium` (`--with-deps` on ubuntu CI) — browser suite dependencies (all three CI OSes; local optional, on the host's own Node)
- `cargo install --path .` — install `viola` from the repo root
- `pip install -r scripts/requirements.txt` — code-graph Python deps (duckdb + protobuf)
- `rustup component add rust-analyzer` — the Rust SCIP indexer for the code-graph (already on PATH on the founder's host)
- `npm i -g @sourcegraph/scip-typescript` — the TypeScript indexer for the ts plane, which activates with the tracked `e2e-web/tsconfig.json` (e2e-web only; never over `crates/viola-ui/`)

## Development
- `viola run <name> -- claude [args]` — wrap a live session (Windows live target)
- `viola run <name> --home <dir> -- <fake agent> <args>` — wrap the fake agent in an isolated home
- `viola ui [--port <n>]` — serve the view-only page on 127.0.0.1 (prints the launch line once to stderr)
- `viola verify [--record <dir>] [-- <program> [args…]]` — drive one print-mode probe (Haiku) against the CLI (default `claude`), check the six ledger rows, print `[NN/06]` step lines + the `stamped <ver>  <n> pass  <m> fail` summary on stdout and write `ledger/stamps.json`; exit 1 on a failing row. `--record <dir>` also writes the scrubbed `<dir>/<ver>/<Event>.default.json` fixtures when 0 rows fail. Against the real CLI: local only, spends tokens; CI runs it only against the fake agent's `-p/--print` mode
- Stamps file: `ledger/stamps.json` in the Ledger stamps envelope `{"v":1,"written_at","writer":"verify","data":{"versions":{"<ver>":{"verified_at","rows":{…},"measured":{…}}}}}` (arch §Standard Contracts); a version is verified only when every row reads `pass`

## Harness (the agent's 5 commands — `scripts/agent-run.sh` / `scripts/agent-run.ps1`)
- `boot [--session <id>] [--instance <name>[:<args>]]... [--ui] [--unstamped] [--cli-version <ver>] [--agents-mode recorded|oversize|malformed] [--statusline-echo]` — build, create a home, stamp via `viola verify` against the fake agent, start wrappers (+ UI), wait for readiness
- `run [--unit|--integration|--e2e|--browser|--mutants|--coverage|--perf|--fuzz-replay|--all] [--filter <nextest-filterset>] [--file <path>]... [--package <member>] [--local-live]` — invoke suites; one JSON summary. No selector or `--all` runs unit + integration; `--mutants` runs only when named (the epoch-boundary audit's arm)
- `status [--session <id>]` — `viola list --json` + `/ready` + `/api/sessions`, with `api_sessions_equal_list`
- `cleanup [--session <id>|--all]` — graceful stop, endpoint/port/url-file checks, home removal (idempotent)
- `logs [--session <id>] [--instance <name>] [--kind <kind>] [--process run|hook|mcp|ui|cli] [--after <offset>]` — merged ndjson of events + diagnostics
- Internal (forwarded by the shims, not agent-facing): `supervise`, `ui-restart --session <id>`, `gate --require <suites> [--artifacts <dir>]` (the per-job verdict; a missing artifact is a breach), `pre-push` (Linux host only, else exit 2 `pre-push-linux-only`: the operator pass's local Linux gate — natively in the working tree, no clone; every child `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin`; checks the tools at their pins (`node` included), then runs `run --coverage`, `run --browser` and `gate --require coverage,doctest,playwright`; stages `host · tools · linux-tests`, fail-fast, no mutation stage)
- Built today (the grammar grows per chunk; anything else is exit 2 `reason:"usage"`): `boot [--session] [--instance <name>[:<args>]]... [--cli-version]` · `run [--unit|--integration|--mutants|--coverage|--browser|--perf|--fuzz-replay|--all] [--filter] [--file <path>]... [--package <member>]` (`--package` needs `--mutants`: one whole member, `verdict:"package"`, anything but `viola` or a `crates/<member>` declaring `fake-agent` is exit 2 `package-refused`; `--perf` runs only when named, never under `--all`: a missing hyperfine is `reason:"tool-missing"`, `detail:"hyperfine"`; `--browser` runs only when named, never under `--all`, on every OS: a failed `npm ci` or an absent Chromium is `reason:"browser-missing"`; `--fuzz-replay` is Linux-only: exit 2 `fuzz-linux-only` elsewhere; `--mutants` runs only when named, never under `--all`; the run document gains `archived` = `target/run-archive/<n>`, the newest 10 kept) · `status [--session]` · `cleanup [--session|--all]` · `logs [--session] [--instance] [--process]` · internal `supervise`, `schema-check`, `secret-scan`, `gate`, `pre-push`. Interim fields: `status` → `list`/`ui` `null` + `instances[]`; `cleanup` → `processes_gone`, endpoint/port/url `null`.

## Testing
- `cargo nextest run --workspace --features fake-agent --profile ci -E 'kind(lib) | kind(bin)'` — unit
- `cargo nextest run --workspace --features fake-agent --profile ci -E 'kind(test)'` — integration (gains `& !binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)` once an E2E binary exists: nextest rejects an unmatched `binary()` regex). The harness runs these with `CARGO_TARGET_DIR=target/harness` and `--features viola/fake-agent`.
- `cargo nextest run --workspace --features fake-agent --profile ci -E 'binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)'` — fake-agent E2E
- `cargo test --workspace --doc` — doctests (nextest cannot run them)
- `npx --no --prefix e2e-web playwright test --config e2e-web/playwright.config.ts [--grep "<title>"]` — browser suite directly (from the repo root `--config` is required, else Playwright runs on its defaults; the harness form is `run --browser`, all three OSes)
- `cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --lcov --output-path target/lcov.info --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` — coverage gate (`run --coverage`; the separator class makes the regex match Windows paths; JUnit stays at `target/nextest/ci/junit.xml`), then `cargo llvm-cov report --json --summary-only` → `artifacts/llvm-cov-summary.json`
- `cargo build --package viola --features fake-agent` then `NEXTEST_PROFILE=mutants cargo mutants --workspace --features fake-agent --in-diff target/agent-run/chunk.diff --test-tool=nextest --copy-target=true --caught --unviable --build-timeout-multiplier=5` — `run --mutants` (the epoch-boundary audit's arm, never a chunk or CI gate), both steps with the run's own target dir `target/mutants` (`CARGO_TARGET_DIR=<repo>/target/mutants` for the prebuild, the relative `target/mutants` for cargo-mutants, so each copied tree builds its own test binaries) (the prebuild + copied target dir give the scratch tree the root bins the harness tests spawn; cargo-mutants' output streams live to the harness's stderr, one line per mutant outcome, and each mutant's build is bounded) (diff = working tree + untracked files from `merge-base(<base>, HEAD)`; base = `AGENT_RUN_CHUNK_BASE` when set, else derived: the last master flip before the pending chunk's oldest operator pre-CI commit, `HEAD^` on the wrap push — but the flip itself while a promoted chunk is still uncommitted on top of it; the run document names it as `base`). A diff with no `.rs` path skips cargo-mutants and reports `"verdict":"no-rust-delta"`; a Rust delta deletes a stale `mutants.out/outcomes.json` first and reports `"verdict":"counted"`. On a Windows host the command runs with `TMP`/`TEMP` and `--output` set to the host mutation scratch `<repo parent>/viola-mutants-scratch` (guarded: `scratch-refused`; wiped first: `scratch-wipe-failed`), so `mutants.out/` lives there and the document carries `scratch_bytes`
- `bash scripts/agent-run.sh run --mutants --file <path>` (repeatable) — one file's mutants: the scoped inner loop (`verdict:"scoped"`, survivors red)
- `TMPDIR=<repo parent>/viola-mutants-scratch bash scripts/agent-run.sh run --mutants --package <member> [--file <path>]...` — one whole member's mutants, the boundary tier's form (`verdict:"package"`): the same prebuild, then `cargo mutants --package <member> --features fake-agent --test-tool=nextest --copy-target=true …`, no base or diff. On the Linux dev host the `TMPDIR` dir must be NOCOW (`chattr +C`): `/tmp` is a 32 GB quota tmpfs too small for the ~14 GB `target/` copy, and a COW btrfs reflink copy drops the copied `viola-fake-agent`'s exec bit (test-plan §10)
- `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` — the boundary audit's Windows leg (founder ruling C2): the dispatch-only, non-blocking `windows-2025` matrix, one job per package running `scripts/agent-run.ps1 run --mutants --package <member> --file …` over that package's Windows-gated files (`tests/contract_windows_mutation_scope.rs` keeps the lists complete). Dispatched only during the epoch-boundary audit, never a gate. Read it back: the run of the pushed sha (`gh run list --workflow windows-mutants.yml --json databaseId,headSha`), `python -X utf8 <andromeda-tools>/scripts/ci.py conclusion --sha <sha> --name mutants` for the per-job verdicts, then each job's harness document and outcome lines via `gh run view --job <id> --log`. No artifact is uploaded; its survivors become corrective-chunk items like any boundary survivor
- `cargo +<channel> fuzz run --fuzz-dir fuzz <target> fuzz/corpus/<target> -- -runs=0` — corpus replay (`run --fuzz-replay`, Linux; channel from `fuzz/rust-toolchain.toml`; `fuzz/` is its own cargo workspace)

## Linting & Formatting
- `cargo fmt --all` / `cargo fmt --all --check` — format / format gate (edition from `rustfmt.toml`)
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: the lint gate. `print_stdout`, `print_stderr` and `dbg_macro` are denied workspace-wide, and `clippy.toml` `disallowed-macros` bans the tracing level macros.
- `bash scripts/lint-probes.sh`: proves every clippy ban fires and both controls pass, plus the fail-closed raw-`event!` grep (last line `lint-probes: 4 bans fired, 2 controls clean`; CI runs it on the Linux lint leg).
- `cargo check --workspace --all-targets` — type check
- `cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)` — the listed sync crates compile without tokio (CI job `lint`; each sync crate joins `scripts/sync-crates.txt` with its crate)
- `bash scripts/orphans-check.sh [--probe]` — `cargo modules orphans --deny` per lib/bin target (last line `orphans-check: N/N targets clean`; the probe's `orphans-check probes: 1/1 fired, control clean`); CI job `lint`, all 3 OS
- `cargo modules dependencies --package <crate> --lib` — the on-demand boundary review only: `--acyclic` is not a gate (cargo-modules 0.27.0 reads every type ↔ inherent method as a cycle)
- `cargo tree -e features -p viola --edges normal` — the rmcp `server` + `transport-io` assertion; it lands with the "MCP server for drivers" chunk (no rmcp in the graph before `viola-mcp`)
- `cargo deny check` — advisories, licences, sources, bans over `deny.toml`, the root `Cargo.lock` (weekly in `nightly.yml`: `cargo deny check advisories`)
- `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` — the separate `fuzz/Cargo.lock` audit (CI `supply-chain` with `--format json` into `target/supply-chain/deny-fuzz.json`; weekly `check advisories` in `nightly.yml`). Run it from the repo root: under `fuzz/` rustup demands the fuzz nightly. `fuzz/` stays outside the root graph by a ratified test-only exemption
- `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans` — the tokio ban, once per crate in `scripts/sync-crates.txt` as sole root (cargo-deny 0.20: `--config` goes before the subcommand; `check -c` is rejected)
- `bash scripts/npm-audit.sh [--advisories-only|--probe]` — the `e2e-web/package-lock.json` audit: npm advisories at every level + every `resolved` from registry.npmjs.org (last line `npm-audit: advisories 0, sources registry.npmjs.org only`; JSON in `target/npm-audit/`; CI `supply-chain`, weekly `--advisories-only` in `nightly.yml`; the probe's `npm-audit --probe: 1/1 refused, control clean`)
- `bash scripts/deny-probes.sh` — proves every ban fires (last line `deny-probes: 13/13 banned, control clean`; needs the network)
- `bash scripts/conpty-vendor.sh [--verify|--probe]` — the vendored ConPTY binaries: the default re-vendors `vendor/conpty/<version>/x64/` from the pinned nupkg; `--verify` fetches outside the tree and checks the nupkg SHA-256, the byte-identical files and the Authenticode signer (last line `conpty-vendor: verified <version>`); `--probe` shows its four refusals (last line `conpty-vendor probe: 4/4 refused, control clean`). Pins parsed from `src/conpty.rs`; needs the network; CI `windows-2025` step `ConPTY vendor verification`
- `zizmor .github/workflows/` (CI: `--format=json`) — workflow lint
- `npx --prefix e2e-web eslint -c e2e-web/eslint.config.js -f json` · `npx --prefix e2e-web html-validate --config e2e-web/.htmlvalidate.json --formatter json <index.html>` — a11y lint (ubuntu)

## Observability gates (obs-plan §9, `shell: bash`)
- G1 bare `#[instrument]`: `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` must exit 1
- G2 zero panics: `bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh` over `target/e2e-home/**/diagnostics/*.ndjson` (role files only; exempts only a `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>`; last lines `g2-probe: all cases as expected` / `g2: clean`)
- G3 no abort panic strategy: `rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .` must exit 1
- G1 and G3 need `rg` on `PATH`: `PATH="$PWD/target/tools/ripgrep/bin:$PATH"` after `scripts/install-ripgrep.sh`.
- G4 schema conformance against `schemas/diag-line.v1.json` / `schemas/diag-detail.v1.json`: `bash scripts/agent-run.sh schema-check`.
- Secret scan before any diagnostics-bearing upload: `bash scripts/agent-run.sh secret-scan` (`id: secret-scan`). The hit report goes to `target/secret-scan/hits.json`. The unscanned uploads (nightly `fuzz/artifacts/`, the `supply-chain` reports) are admissible only by content, per obs-plan §8 item 6.
- Locally, G2, G4 and the scan need kept homes. Clear `target/e2e-home` first (the scan reads every home left there). A local `run --mutants` `chunk.diff` in `target/agent-run/` needs no clearing: the scan skips exactly that file. Then run `AGENT_RUN_KEEP_HOMES=1 bash scripts/agent-run.sh run --integration`.

## Build & Deploy
- `bash scripts/release-check.sh [--probe]` — release build `cargo build --release --locked --bin viola` (fake agent excluded: feature off), judged from its own artifact records: last line `release-check: viola only`; an artifact built with the test-only `test-support` or `fake-agent` feature fails it (the probe's `release-check probes: 5/5 refused, control clean`). CI job `release`, all 3 OS. Locally, pass `CARGO_TARGET_DIR=target/release-check` to keep it off a shared `target/release/`
- `CARGO_TARGET_DIR=target/harness cargo build --workspace --features viola/fake-agent` — harness build (boot step 1; its own target dir because a running `target/debug/viola-harness.exe` cannot be relinked on Windows)
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
