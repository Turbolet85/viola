group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:5   class: retired
  leaf-text: «; `e2e-web/` (the Playwright browser suite) exists, its axe and a11y lint landing with the a11y chunks; the browser test leg (`e2e-web/`, `scripts/install-node.sh` and the Node pin, `npm ci`, `scripts/npm-audit.sh`, Playwright and Chromium, `run --browser` and the `pre-push` browser stage) stands until the route entry "Remove the browser leg" removes it»
  source: .andromeda/test-plan.md:150 «none in 0.1.0: the browser suite left with the browser leg, and the mutation step keeps its number 4»
  replace-with: «; the browser test leg (`e2e-web/`, the Node pin and `scripts/install-node.sh`, `npm ci`, `scripts/npm-audit.sh`, Playwright and Chromium, `run --browser` and the `pre-push` browser stage) has left 0.1.0»

- leaf: .claude/docs/commands.md:12   class: retired
  leaf-text: «- `bash scripts/install-node.sh linux-x64 ~/.local/viola-node` — the pinned Node `pre-push`'s `tools` stage checks on the Linux dev host (`pre-push` installs nothing; `cc`, the channel and the cargo tools are the host's own, at ci.yml's pins)»
  source: .andromeda/test-plan.md:1031 «The local pre-push gate runs natively on the Linux dev host and installs nothing: its `tools` stage checks the host's `cc`, the `rust-toolchain.toml` channel»
  replace-with: «»

- leaf: .claude/docs/commands.md:13   class: retired
  leaf-text: «- `bash scripts/install-node.sh <linux-x64|darwin-arm64|win-x64> <dest>` / `--probe` — the official Node build at ci.yml's `NODE_PIN_*` (parsed from the file text), sha256-verified; prints `install-node: <bin dir>`»
  source: .andromeda/test-plan.md:150 «none in 0.1.0: the browser suite left with the browser leg, and the mutation step keeps its number 4»
  replace-with: «»

- leaf: .claude/docs/commands.md:14   class: stale
  leaf-text: «`jq` for G2, `scripts/release-check.sh`, `scripts/orphans-check.sh` and `scripts/npm-audit.sh`: runner-provided in CI»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:4 «`scripts/release-check.sh` (`release`) and `scripts/orphans-check.sh` (`lint`) refuse with `tool-missing: jq` without it»
  replace-with: «`jq` for G2, `scripts/release-check.sh` and `scripts/orphans-check.sh`: runner-provided in CI»

- leaf: .claude/docs/commands.md:15   class: retired
  leaf-text: «- `npm ci --prefix e2e-web` then `npx --no --prefix e2e-web playwright install chromium` (`--with-deps` on ubuntu CI) — browser suite dependencies (all three CI OSes; locally the Linux `pre-push` runs it under ci.yml's pinned Node at `~/.local/viola-node`)»
  source: .andromeda/test-plan.md:1022 «| E2E | `agent-run run --e2e` on all 3 OSes | matrix per OS | rust-cache |»
  replace-with: «»

- leaf: .claude/docs/commands.md:19   class: stale
  leaf-text: «- `npm i -g @sourcegraph/scip-typescript` — the TypeScript indexer for the ts plane, which activates with the tracked `e2e-web/tsconfig.json` (`e2e-web/` only; no tsconfig, package manifest or bundler config ever covers `crates/viola-ui/`, and a CI-runnable guard lists any such file)»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «no input in 0.1.0. Its one input was the tracked `e2e-web/tsconfig.json`, which left with the browser leg, so the tree tracks no `tsconfig.json` and `scripts/code-graph.py` detects no ts plane.»
  replace-with: «- no TypeScript indexer in 0.1.0: the ts plane has no input (its one input, the tracked `e2e-web/tsconfig.json`, left with the browser leg), so `scripts/code-graph.py` detects no ts plane»

- leaf: .claude/docs/commands.md:31   class: retired
  leaf-text: «`run [--unit|--integration|--e2e|--browser|--mutants»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:18 «Command body: `scripts/agent-run.sh run [--unit|--integration|--e2e|--mutants|--coverage|--perf|--fuzz-replay|--all] [--filter <nextest-filterset>] [--file <path>]... [--package <member>] [--local-live]»
  replace-with: «`run [--unit|--integration|--e2e|--mutants»

- leaf: .claude/docs/commands.md:32   class: retired
  leaf-text: «`viola list --json`; `ui` and `api_sessions_equal_list` are `null`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:3 «This is the `agent-run status` document. Its nested `list` member is the verbatim `viola list --json` document (see Section 1, Status endpoint shape).»
  replace-with: «`viola list --json`»

- leaf: .claude/docs/commands.md:33   class: retired
  leaf-text: «process and endpoint checks (`port_free` and `url_file_removed` stay `null`)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:76 «Verification: reported in `{"cleaned":[…],"processes_gone":true,"endpoint_gone":true,"home_removed":true,"killed":[<names force-killed>]}`»
  replace-with: «process and endpoint checks»

- leaf: .claude/docs/commands.md:34   class: stale
  leaf-text: «[--process run|hook|mcp|ui|cli]»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:79 «scripts/agent-run.sh logs [--session <id>] [--instance <name>] [--kind <event-kind>] [--process run|hook|mcp|cli] [--after <byte offset>]`»
  replace-with: «[--process run|hook|mcp|cli]»

- leaf: .claude/docs/commands.md:35   class: retired
  leaf-text: «`supervise`, `ui-restart --session <id>` (forwarded unchanged; `viola-harness` implements no such subcommand), `gate»
  source: .andromeda/registries/test-plan-contracts.toml:9 «"Internal harness subcommands", "supervise --session <id>", "schema-check", "secret-scan", "gate --require <suite>[,<suite>…] [--artifacts <dir>]", "pre-push", "Closed enums"]»
  replace-with: «`supervise`, `gate»

- leaf: .claude/docs/commands.md:35   class: stale
  leaf-text: «PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:130 «every child `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root»
  replace-with: «PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin»

- leaf: .claude/docs/commands.md:35   class: stale
  leaf-text: «checks the tools at their pins (`node` included), then runs `run --coverage`, `run --browser` and `gate --require coverage,doctest,playwright`»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:20 «It checks the host's tools against their pins (the C linker, the channel and ci.yml's cargo tools), then runs the ubuntu `test` job's `run --coverage` and `gate --require coverage,doctest`»
  replace-with: «checks the tools at their pins, then runs `run --coverage` and `gate --require coverage,doctest`»

- leaf: .claude/docs/commands.md:36   class: retired
  leaf-text: «`run [--unit|--integration|--mutants|--coverage|--browser|--perf|--fuzz-replay|--all]»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:18 «Command body: `scripts/agent-run.sh run [--unit|--integration|--e2e|--mutants|--coverage|--perf|--fuzz-replay|--all] [--filter <nextest-filterset>] [--file <path>]... [--package <member>] [--local-live]»
  replace-with: «`run [--unit|--integration|--mutants|--coverage|--perf|--fuzz-replay|--all]»

- leaf: .claude/docs/commands.md:36   class: retired
  leaf-text: «; `--browser` runs only when named, never under `--all`, on every OS: a failed `npm ci` or an absent Chromium is `reason:"browser-missing"`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:31 «3. None in 0.1.0: the browser step left with the browser leg, and the mutation step keeps its number 4.»
  replace-with: «»

- leaf: .claude/docs/commands.md:36   class: stale
  leaf-text: «`status` → `list`/`ui`/`pid`/`uptime_ms`/`api_sessions_equal_list` `null`»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17 «`list`, `pid` and `uptime_ms` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`.»
  replace-with: «`status` → `list`/`pid`/`uptime_ms` `null`»

- leaf: .claude/docs/commands.md:36   class: retired
  leaf-text: «`cleanup` → `processes_gone`, `endpoint_gone`; `port_free` and `url_file_removed` stay `null`.»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:76 «`endpoint_gone` is reported from the wrapper channel on. Any `false` field means exit 1.»
  replace-with: «`cleanup` → `processes_gone`, `endpoint_gone`.»

- leaf: .claude/docs/commands.md:43   class: retired
  leaf-text: «- `npx --no --prefix e2e-web playwright test --config e2e-web/playwright.config.ts [--grep "<title>"]` — browser suite directly (from the repo root `--config` is required, else Playwright runs on its defaults; the harness form is `run --browser`, all three OSes)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:31 «3. None in 0.1.0: the browser step left with the browser leg, and the mutation step keeps its number 4.»
  replace-with: «»

- leaf: .claude/docs/commands.md:63   class: retired
  leaf-text: «- `bash scripts/npm-audit.sh [--advisories-only|--probe]` — the `e2e-web/package-lock.json` audit: npm advisories at every level + every `resolved` from registry.npmjs.org (last line `npm-audit: advisories 0, sources registry.npmjs.org only`; JSON in `target/npm-audit/`; CI `supply-chain`, weekly `--advisories-only` in `nightly.yml`; the probe's `npm-audit --probe: 1/1 refused, control clean`)»
  source: .andromeda/test-plan.md:1019 «Weekly: `nightly.yml` job `advisories` runs `cargo deny check advisories` and `cargo deny --manifest-path fuzz/Cargo.toml check advisories`, on `schedule` + `workflow_dispatch`»
  replace-with: «»

- leaf: .claude/docs/commands.md:65   class: stale
  leaf-text: «last line `deny-probes: 13/13 banned, control clean`»
  source: .andromeda/test-plan.md:1019 «`bash scripts/deny-probes.sh` (a permanent negative probe per ban + a clean control, last line `deny-probes: 12/12 banned, control clean`: twelve bans, one probe each)»
  replace-with: «last line `deny-probes: 12/12 banned, control clean`»

- leaf: .claude/docs/commands.md:68   class: retired
  leaf-text: «- `npx --prefix e2e-web eslint -c e2e-web/eslint.config.js -f json` · `npx --prefix e2e-web html-validate --config e2e-web/.htmlvalidate.json --formatter json <index.html>` — a11y lint (ubuntu)»
  source: .andromeda/test-plan.md:1059 «- Lint errors: fmt, clippy `-D warnings`, a sync crate listed in `scripts/sync-crates.txt` failing to build without tokio (or an empty list), a cargo-modules orphan in any lib/bin target»
  replace-with: «»
