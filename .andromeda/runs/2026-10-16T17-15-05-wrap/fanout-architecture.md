proposals:
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies'
    change: 'Remove the "Browser e2e (test-side only)" row whole (Node v24.21.0 and its NODE_PIN_* sha256 pin, scripts/install-node.sh, @playwright/test 1.63.0 and its bundled Chromium, e2e-web/ with npm ci and its lockfile audit scripts/npm-audit.sh, the run --browser suite and the native pre-push browser stage); the stack names no test-side Node, npm or browser tool.'
    sidecar: '§Stack and Technologies: the Browser e2e row leaves; chunk 2026-10-16-remove-the-browser-leg removed the browser test leg it named, as the row''s own stand-until clause asked of that entry''s wrap.'
    rationale: 'Report Changes: Files (e2e-web/ whole, scripts/install-node.sh, scripts/npm-audit.sh and harness run/browser.rs deleted), Dependencies (the npm graph with @playwright/test 1.63.0 and the Node v24.21.0 pin removed from the tree), Harness / gate surface (Node (pinned), npm ci, Chromium and Browser suite steps removed from ci.yml); Expected amendments names this site.'
    basis: '.andromeda/architecture.md:35'
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies'
    change: 'CI/CD row, pre-push sentence: every child runs through `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin` (no `<home>/.local/viola-node/bin`), and its `tools` stage checks `cc`, the `rust-toolchain.toml` channel and ci.yml''s `test`-job tool line only (no `NODE_PIN_VERSION`, no node); it installs nothing.'
    sidecar: '§Stack and Technologies CI/CD row: the pre-push PATH loses its viola-node element (5 → 4 elements) and the tools stage no longer checks Node (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Symbols / APIs, harness pre-push bullet (NODE_BIN and node_pin gone; tools checks cc, the channel and ci.yml''s test-job tool line only; the viola-node PATH element left) and Counts (pre-push PATH elements 5 → 4).'
    basis: '.andromeda/architecture.md:34'
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies'
    change: 'Code quality row: jq''s consumers are `scripts/g2-zero-panics.sh`, `scripts/release-check.sh` and `scripts/orphans-check.sh` (drop `scripts/npm-audit.sh`).'
    sidecar: '§Stack and Technologies Code quality row: scripts/npm-audit.sh leaves the jq consumer list (deleted by chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Files: scripts/npm-audit.sh deleted; Expected amendments names the Code quality row (architecture.md:36).'
    basis: '.andromeda/architecture.md:36'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources'
    change: 'Remove the "CI workflow data lines" bullet whole: ci.yml holds no workflow-level `env:` and no NODE_PIN_* line, and neither scripts/install-node.sh nor pre_push::linux::node_pin nor a `node --version` check exists.'
    sidecar: '§Occupied Resources: the NODE_PIN_* workflow data lines and their readers leave the registry (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface (ci.yml''s workflow-level env: with the four NODE_PIN_* lines removed), Symbols / APIs pre-push (NODE_BIN and node_pin gone), Files (scripts/install-node.sh deleted).'
    basis: '.andromeda/architecture.md:332'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources'
    change: 'Remove the "Test-side install sites" bullet whole (the CI $RUNNER_TEMP/node, the dev host ~/.local/viola-node/ PATH component, ~/.cache/ms-playwright/): no workflow, harness verb or pre-push stage installs, reads or adds them to a PATH.'
    sidecar: '§Occupied Resources: the Node and Playwright install sites leave the registry; the dev host''s Node install is the operator''s, outside the project (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface (Node (pinned) removed from the test and supply-chain jobs, nightly npm-advisories removed), Symbols / APIs pre-push (the <home>/.local/viola-node/bin element left), Dev-tool versions (Node and Playwright no longer used by any tree path).'
    basis: '.andromeda/architecture.md:356'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources'
    change: '`target/nextest/ci/junit.xml` bullet: the `junit-<os>` upload (behind the secret scan) carries it alone (drop `target/agent-run/artifacts/junit-playwright.xml`).'
    sidecar: '§Occupied Resources: junit-<os> uploads the nextest JUnit alone; junit-playwright.xml leaves (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Counts / qualifiers moved: `junit-<os>` uploads `target/nextest/ci/junit.xml` alone; Harness / gate surface: Upload JUnit carries one path.'
    basis: '.andromeda/architecture.md:384'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources'
    change: 'Remove the `target/npm-audit/audit.json` bullet whole: scripts/npm-audit.sh and the supply-chain npm step that wrote it are gone.'
    sidecar: '§Occupied Resources: target/npm-audit/ leaves the registry (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Files (scripts/npm-audit.sh deleted) and Harness / gate surface (supply-chain loses the npm lockfile audit step).'
    basis: '.andromeda/architecture.md:385'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources'
    change: 'Remove the `e2e-web/` outputs bullet whole (node_modules/, pw.json, pw-junit.xml, test-results/ with its a11y/ and lint/ subdirectories, the browser arm''s pre-run deletes): e2e-web/ and the browser arm are gone.'
    sidecar: '§Occupied Resources: the e2e-web/ test outputs leave the registry with the directory and its .gitignore block (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Files (e2e-web/ deleted whole; harness run/browser.rs deleted) and Schema / config (.gitignore''s e2e-web block with node_modules/, test-results, pw.json and pw-junit.xml removed).'
    basis: '.andromeda/architecture.md:386'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources'
    change: '`target/run-archive/<n>/` bullet: the archive holds the JUnit files that run''s nextest or coverage suites wrote (`junit-{nextest-unit,nextest-integration,coverage}.xml`) and the `outcomes.json` its mutation arm read; drop the playwright suite and its `e2e-web/pw-junit.xml` source.'
    sidecar: '§Occupied Resources: the run archive loses its playwright arm and pw-junit.xml source (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Symbols / APIs harness run: `archive_sources` archives junit-{nextest-unit,nextest-integration,coverage}.xml and outcomes.json only (no playwright arm, no pw-junit.xml source).'
    basis: '.andromeda/architecture.md:390'
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Build system'
    change: 'Dependency-policy bullet: the feature bans are `veil/toggle`, `rmcp` `auth` / `transport-streamable-http-server` and `tracing-subscriber/env-filter` (drop `axum/http2`, whose ban and probe left).'
    sidecar: 'Build system: the axum/http2 feature ban and its deny-probes row leave deny.toml (P4 fork 2, inputs#I1; chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Schema / config (deny.toml''s [[bans.features]] axum http2 block removed) and Counts (deny ban probes 13 → 12, deny-probes: 12/12 banned, control clean); Expected amendments names build-system.md:8.'
    basis: '.andromeda/registries/contracts/architecture/build-system.md:8'
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Build system'
    change: 'Code-graph planes, the **ts** bullet: the ts plane has no input (the tracked e2e-web/tsconfig.json left with e2e-web/; `git ls-files ''*tsconfig.json''` lists none); the crates/viola-ui/ tsconfig guard sentence leaves.'
    sidecar: 'Build system: the ts code-graph plane loses its only input and the crates/viola-ui tsconfig guard leaves (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Counts / qualifiers moved (tracked tsconfig.json files 1 → 0, so the ts code-graph plane has no input) and Expected amendments (the ts plane ends; the crates/viola-ui/ tsconfig guard leaves).'
    basis: '.andromeda/registries/contracts/architecture/build-system.md'
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'First bullet: nightly.yml runs two ubuntu jobs with no cache, `advisories` and `fuzz`; the `npm-advisories` sentence leaves.'
    sidecar: 'CI/CD approach: nightly.yml''s npm-advisories job leaves (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface: nightly.yml''s npm-advisories job is removed (advisories and fuzz unchanged).'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'Setup steps bullet: drop the Node v24.21.0 / scripts/install-node.sh sentence, the `npm ci --prefix e2e-web` and Playwright Chromium install sentence and "No npm or browser cache."; drop `scripts/npm-audit.sh` (supply-chain) from the jq tool-missing clause.'
    sidecar: 'CI/CD approach: the Node, npm ci and Chromium setup steps and the npm-audit.sh jq consumer leave (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface (test loses Node (pinned), Browser suite dependencies (npm ci), Chromium (with system deps), Chromium; supply-chain loses Node (pinned) and the npm lockfile audit) and Files (install-node.sh, npm-audit.sh deleted).'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'Jobs bullet, `test`: drop "then the browser suite (`Node (pinned)`, `npm ci`, the Chromium install, and `agent-run run --browser` …: suite `playwright`)"; the `Gate verdict` step runs `gate --require coverage,doctest`.'
    sidecar: 'CI/CD approach: the test job loses its browser suite stage and the gate drops playwright (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface (the Browser suite (pwsh shim) / (sh shim) steps removed; Gate verdict runs gate --require coverage,doctest) and Symbols / APIs gate (SUITES the eight, playwright left).'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'Jobs bullet, `test` uploads: `junit-<os>` carries `target/nextest/ci/junit.xml` alone (drop `target/agent-run/artifacts/junit-playwright.xml`).'
    sidecar: 'CI/CD approach: junit-<os> carries one path (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Counts / qualifiers moved (junit-<os> uploads target/nextest/ci/junit.xml alone) and Harness / gate surface (Upload JUnit carries one path).'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md'
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'Jobs bullet, `supply-chain` list: remove the item "`Node (pinned)`, then the `npm lockfile audit (advisories, sources)` …" whole; the list is target job 4, the fuzz lockfile audit, the sole-root tokio ban, deny-probes and zizmor.'
    sidecar: 'CI/CD approach: supply-chain loses its Node and npm lockfile audit steps (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface: supply-chain loses Node (pinned) and npm lockfile audit (advisories, sources) and their comment.'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'Pre-push bullet: every child is `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>`; `tools` checks the C linker, the channel and ci.yml''s cargo tools (no `node --version`, no `NODE_PIN_VERSION`, no `node` detail); `linux-tests` runs `run --coverage` then `gate --require coverage,doctest` (no `run --browser` stage).'
    sidecar: 'CI/CD approach: the pre-push PATH, tools check and linux-tests stage lose Node and the browser suite (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Symbols / APIs harness pre-push bullet (PATH without viola-node; tools checks cc, the channel and the test-job tool line only; linux-tests runs run --coverage then gate --require coverage,doctest; the linux object holds run and gate only).'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'Remove the `e2e-web/` subtree whole (package.json, package-lock.json, playwright.config.ts, tsconfig.json, stub/pipe.html, eslint.config.js, .htmlvalidate.json, tests/, fixtures/, schemas/, a11y/, test-results/).'
    sidecar: 'Project directory structure: e2e-web/ leaves the tree (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Files: e2e-web/ deleted whole (six tracked files).'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'scripts/: remove the `install-node.sh` and `npm-audit.sh` entries.'
    sidecar: 'Project directory structure: scripts/install-node.sh and scripts/npm-audit.sh leave (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Files: scripts/install-node.sh and scripts/npm-audit.sh deleted.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: '.github/workflows comments: ci.yml''s drops "(+ the browser suite)", "npm lockfile audit" and "the workflow env holds the NODE_PIN_* lines"; nightly.yml''s drops "npm-advisories (npm-audit.sh --advisories-only)".'
    sidecar: 'Project directory structure: the workflow comments lose the browser suite, the npm audit, the NODE_PIN_* env and npm-advisories (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Harness / gate surface (ci.yml workflow env, browser steps and supply-chain npm steps removed; nightly.yml npm-advisories job removed).'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: '`.config/nextest.toml` comment reads "nextest profiles `ci` and `mutants`" (drop the `fixed-port` group).'
    sidecar: 'Project directory structure: the nextest fixed-port test group leaves (chunk 2026-10-16-remove-the-browser-leg).'
    rationale: 'Report Changes, Schema / config: .config/nextest.toml''s [test-groups] fixed-port group and its test(/default_port/) override are removed.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
note: 'No new resource landed (the chunk only removes), so every proposal retires a stale registry or stack claim. Swept and left as true: the npm-shim → claude.exe resolution rows (architecture.md:43, :45, :76, :100, :281, per the report''s sweep hazard); the four role files at the diagnostics/ bullet already name no ui-<port>; no architecture site carries ObsEvent / ObsProcess, VIOLA_TARGETS, the status / cleanup / session-record keys or the secret-scan classes.'
