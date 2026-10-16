group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «The `viola_ui` log target in `src/obs.rs`, the reserved `ui`»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 68 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/stack.md:30   class: stale
  leaf-text: «PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin`; it refuses `pre-push-linux-only` off Linux, and its `tools` stage checks `cc`, the `rust-toolchain.toml` channel, ci.yml's `test`-job pins and `NODE_PIN_VERSION` (it installs nothing)»
  source: .andromeda/architecture.md:34 «PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`; it refuses `pre-push-linux-only` off Linux, and its `tools` stage checks `cc`, the `rust-toolchain.toml` channel and ci.yml's `test`-job pins»
  replace-with: «PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`; it refuses `pre-push-linux-only` off Linux, and its `tools` stage checks `cc`, the `rust-toolchain.toml` channel and ci.yml's `test`-job pins (it installs nothing)»

- leaf: .claude/docs/stack.md:31   class: retired
  leaf-text: «| Browser e2e (test-side only) | Node v24.21.0, the official nodejs.org build, sha256-pinned by ci.yml's workflow `NODE_PIN_*` lines»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «Its one input was the tracked `e2e-web/tsconfig.json`, which left with the browser leg, so the tree tracks no `tsconfig.json`»
  replace-with: «(delete the whole row: architecture.md's Stack table no longer holds a Browser e2e row)»

- leaf: .claude/docs/stack.md:32   class: stale
  leaf-text: «`scripts/release-check.sh`, `scripts/orphans-check.sh` and `scripts/npm-audit.sh`, which refuse with `tool-missing: jq`»
  source: .andromeda/architecture.md:35 «`jq` (runner-provided: G2's `scripts/g2-zero-panics.sh`, `scripts/release-check.sh` and `scripts/orphans-check.sh`, which refuse with `tool-missing: jq`)»
  replace-with: «`scripts/release-check.sh` and `scripts/orphans-check.sh`, which refuse with `tool-missing: jq`»

- leaf: .claude/docs/stack.md:65   class: retired
  leaf-text: «in the root `Cargo.toml` (and `e2e-web/package.json` for Node tools)»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «Its one input was the tracked `e2e-web/tsconfig.json`, which left with the browser leg, so the tree tracks no `tsconfig.json`»
  replace-with: «in the root `Cargo.toml`»

- leaf: .claude/docs/workflow.md:10   class: stale
  leaf-text: «`nightly.yml` runs the weekly advisory checks — `cargo deny` and the npm lockfile's `npm-advisories` — and»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:2 «runs two ubuntu jobs with no cache. `advisories` runs `cargo deny check advisories` over the root lockfile and `cargo deny --manifest-path fuzz/Cargo.toml check advisories` over the fuzz lockfile»
  replace-with: «`nightly.yml` runs the weekly advisory check — `cargo deny` over the root and fuzz lockfiles — and»

- leaf: .claude/docs/workflow.md:10   class: retired
  leaf-text: «Each OS's `test` job also runs the browser suite (`run --browser`) and gates `coverage,doctest,playwright`.»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:5 «last, an `if: always()` `Gate verdict` step runs `gate --require coverage,doctest`)»
  replace-with: «Each OS's `test` job gates `coverage,doctest`.»

- leaf: .claude/docs/workflow.md:11   class: stale
  leaf-text: «(the ubuntu test suites, the browser suite included, natively»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:20 «then runs the ubuntu `test` job's `run --coverage` and `gate --require coverage,doctest`: stages `tools → linux-tests`»
  replace-with: «(the ubuntu `test` job's `run --coverage` and `gate --require coverage,doctest`, natively»

- leaf: .claude/docs/workflow.md:47   class: retired
  leaf-text: «nextest JUnit, Playwright JSON,»
  source: .andromeda/architecture.md:385 «the JUnit files that run's nextest or coverage suites wrote (`junit-{nextest-unit,nextest-integration,coverage}.xml`)»
  replace-with: «nextest JUnit,»

- leaf: .claude/docs/services/viola-ui.md:9   class: stale
  leaf-text: «, and the browser test leg are removed by the route entry "Remove the browser leg".»
  source: .andromeda/registries/contracts/architecture/build-system.md:15 «Its one input was the tracked `e2e-web/tsconfig.json`, which left with the browser leg, so the tree tracks no `tsconfig.json`»
  replace-with: « are removed by the route entry "Remove the browser leg"; the browser test leg has left the tree.»
