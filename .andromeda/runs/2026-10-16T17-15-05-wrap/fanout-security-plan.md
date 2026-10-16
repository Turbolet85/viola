proposals:
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → Audit tool"
    change: "Remove the test-side npm graph bullet whole (`e2e-web/package-lock.json`, `scripts/npm-audit.sh`, its `--probe` / `--advisories-only`, the `target/npm-audit/` output, the browser-leg stand-until sentence): no npm graph, no Node pin and no npm audit remain in the tree; the Audit tool list is cargo-deny, zizmor, the deferred cargo-audit and the ConPTY vendor audit."
    sidecar: "§Dependency Security Audit tool: the npm-graph audit bullet leaves with the browser leg (chunk 2026-10-16-remove-the-browser-leg)."
    rationale: "Report Changes → Dependencies: the test-side npm graph (`e2e-web/package-lock.json`, `@playwright/test` 1.63.0) and the Node v24.21.0 pin are removed from the tree; Files: `e2e-web/` whole, `scripts/npm-audit.sh` and `scripts/install-node.sh` deleted. The bullet itself names this entry's wrap as the one that amends it; the report's Expected amendments list it."
    basis: "security-plan.md:286"
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → Audit tool"
    change: "The ConPTY vendor bullet reads that the vendored binaries sit outside every cargo graph (drop 'and npm'), so no lockfile audit reaches them; the rest unchanged."
    sidecar: "§Dependency Security Audit tool: the ConPTY bullet no longer names an npm graph."
    rationale: "Report Changes → Dependencies: the npm graph left the tree, so 'every cargo and npm graph' names a graph that no longer exists."
    basis: "security-plan.md:287"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → `deny.toml` additions"
    change: "`[[bans.features]]` holds the rmcp ban alone (`transport-streamable-http-server`, `auth`); the `axum` denies `http2` sub-bullet leaves with its `deny-probes.sh` row (the probes read `deny-probes: 12/12 banned, control clean`)."
    sidecar: "§Dependency Security deny.toml additions: the axum http2 feature ban and its probe leave (P4 fork 2, inputs#I1 of chunk 2026-10-16-remove-the-browser-leg)."
    rationale: "Report Changes → Schema / config: the `deny.toml` `[[bans.features]] crate = \"axum\" deny = [\"http2\"]` block is removed (P4 fork 2, `inputs#I1`); Counts: deny ban probes 13 → 12 (`scripts/deny-probes.sh`'s rows), measured at gate entry 14."
    basis: "security-plan.md:295"
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → Pinning"
    change: "Remove the `e2e-web/package-lock.json` / `@playwright/test` 1.63.0 / Chromium pinning bullet whole."
    sidecar: "§Dependency Security Pinning: the Playwright lockfile pin leaves with `e2e-web/`."
    rationale: "Report Changes → Files: `e2e-web/` deleted whole (package-lock.json included); Dependencies: `@playwright/test` 1.63.0 removed from the tree."
    basis: "security-plan.md:306"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → Pinning"
    change: "Remove the Node runtime pin bullet whole (ci.yml's workflow `env:` `NODE_PIN_VERSION` / `NODE_PIN_SHA256_<OS>` lines and `scripts/install-node.sh`)."
    sidecar: "§Dependency Security Pinning: the Node runtime pin and install-node.sh leave."
    rationale: "Report Changes → Harness / gate surface: ci.yml's workflow-level `env:` (the four `NODE_PIN_*` lines) is removed; Files: `scripts/install-node.sh` deleted; Dependencies: the Node v24.21.0 pin removed."
    basis: "security-plan.md:307"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → Pinning"
    change: "The `pre-push` bullet's `tools` stage checks `cc`, the `rust-toolchain.toml` channel and the cargo-nextest, cargo-mutants and cargo-llvm-cov versions parsed from ci.yml's `test`-job `tool:` line only; drop the `NODE_PIN_VERSION` / `~/.local/viola-node` / `install-node.sh` clause and the `tool-missing` / `tool-pin-mismatch` with detail `node` refusal; the rest unchanged."
    sidecar: "§Dependency Security Pinning: the pre-push tools stage checks no Node."
    rationale: "Report Changes → Symbols / APIs (pre-push, `pre_push/linux.rs`): `NODE_BIN` and `node_pin` are gone; the `tools` stage checks `cc`, the channel and ci.yml's `test`-job tool line only (no `node`, no `NODE_PIN_VERSION`)."
    basis: "security-plan.md:308"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → CI integration"
    change: "Job 4 (ubuntu) audits the root `Cargo.lock` and, in its own step, `fuzz/Cargo.lock`; drop the third audit (`Node (pinned)`, `npm lockfile audit (advisories, sources)`, `scripts/npm-audit.sh`, its verdict and `target/npm-audit/`)."
    sidecar: "§Dependency Security CI integration: the supply-chain job's npm steps leave."
    rationale: "Report Changes → Harness / gate surface: `supply-chain` loses `Node (pinned)` and `npm lockfile audit (advisories, sources)` and their comment."
    basis: "security-plan.md:313"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → CI integration"
    change: "`nightly.yml` runs the `advisories` job (root and fuzz `cargo deny ... check advisories`) and the `fuzz` job; drop the `npm-advisories` twin sentence."
    sidecar: "§Dependency Security CI integration: nightly.yml's npm-advisories job leaves."
    rationale: "Report Changes → Harness / gate surface: `nightly.yml`'s `npm-advisories` job is removed (`advisories` and `fuzz` unchanged)."
    basis: "security-plan.md:314"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Dependency Security → CI integration"
    change: "Toolchains come from `rustup toolchain install` steps alone (`rust-toolchain.toml`, `fuzz/rust-toolchain.toml`, the `msrv` job's `1.96`), and no toolchain action is used; drop the Node (`Node (pinned)`, `install-node.sh`) and Chromium (Playwright `install chromium`) sentence and the browser-suite `uses:` clause."
    sidecar: "§Dependency Security CI integration: the Toolchains bullet names no Node or Chromium install."
    rationale: "Report Changes → Harness / gate surface: the `test` job loses `Node (pinned)`, `Browser suite dependencies (npm ci)`, `Chromium (with system deps)`, `Chromium` and both browser-suite steps; `supply-chain` loses `Node (pinned)`."
    basis: "security-plan.md:331"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Bootstrap phases (derive for route / setup-project)"
    change: "dep-audit-tooling-install: cargo-deny with the `deny.toml` additions, zizmor, cargo-audit deferred, and the ConPTY vendor script; drop the test-side npm graph sentence (`scripts/npm-audit.sh`, `scripts/install-node.sh`)."
    sidecar: "§Bootstrap phases dep-audit-tooling-install: the npm-audit and install-node scripts leave."
    rationale: "Report Changes → Files: `scripts/install-node.sh` and `scripts/npm-audit.sh` deleted."
    basis: "security-plan.md:380"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: "§Bootstrap phases (derive for route / setup-project)"
    change: "dep-security-ci-gate: the weekly `cargo deny check advisories`, zizmor, SHA-pinned actions, `permissions: {}` / `contents: read` and the `windows-2025` `ConPTY vendor verification` step; drop the `supply-chain` `npm lockfile audit` step and the `nightly.yml` `npm-advisories` twin."
    sidecar: "§Bootstrap phases dep-security-ci-gate: the npm audit steps leave."
    rationale: "Report Changes → Harness / gate surface: the `supply-chain` npm audit step and `nightly.yml`'s `npm-advisories` job are removed."
    basis: "security-plan.md:395"
    dependent-of: D-security-deps
  - detector: D-security-auth
    severity: warning
    section: "§Secret Management → Storage (Development)"
    change: "The pre-push launcher runs every child as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` (four PATH elements; `<home>/.local/viola-node/bin` removed); the HOME + PATH-only rule, the passwd-home probes and the canary pair unchanged."
    sidecar: "§Secret Management Development: the pre-push PATH loses the viola-node element (4 elements)."
    rationale: "Report Changes → Symbols / APIs (pre-push): every child runs under `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin` (the `<home>/.local/viola-node/bin` element left); Counts: pre-push PATH elements 5 → 4; the passwd-home probes and the HOME + PATH-only rule unchanged."
    basis: "security-plan.md:417"
  - detector: D-security-auth
    severity: warning
    section: "§Secret Management → Secret scanning in CI"
    change: "The canary scan's classes are: the stripped `CLAUDE*` canaries (twelve); the `cookie:` header (case-insensitive); the content canary, which fails only in home-level role files; a non-`0600` diagnostics file, Unix only. Drop `?t=`, `viola_<port>=` and the GUI token read from each home's `ui/*.url`."
    sidecar: "§Secret Management Secret scanning in CI: the GUI forms (?t=, viola_<port>=, ui/*.url token) leave the scan classes (inputs#I1 Q1 of chunk 2026-10-16-remove-the-browser-leg)."
    rationale: "Report Changes → Symbols / APIs (secret-scan): `CRITICAL` holds the twelve `claude-stripped` canaries (`token-query` / `?t=` left); the `gui-token` class and the `viola_<port>=` cookie-pair form are gone; the `cookie` class keeps the case-insensitive `cookie:` header; the content-canary class and the non-0600 `mode` check unchanged. Pinned by `secret_scan_ignores_the_removed_gui_forms`."
    basis: "security-plan.md:425-427"
  - detector: D-security-auth
    severity: warning
    section: "§Bootstrap phases (derive for route / setup-project)"
    change: "secret-scanning-ci-gate: the `chunk.diff` skip's parenthetical no longer names `?t=` (that needle left the scan); it reads that the diff is repository source text by construction and trips the scan's own needles whenever it touches the scan's source; the skip, the upload exclusion and the rest unchanged."
    sidecar: "§Bootstrap phases secret-scanning-ci-gate: the chunk.diff skip reason names no removed ?t= needle."
    rationale: "Report Changes → Symbols / APIs (secret-scan): `token-query` / `?t=` left `CRITICAL`; the report lists security-plan.md:382 among the §Secret Management sites. The chunk.diff skip itself is not reported changed."
    basis: "security-plan.md:382"
    dependent-of: D-security-auth
note: "D-security-input: no drift — the chunk adds no external-input surface (report Coverage of new surfaces: none; `gate --require playwright` is refused by clap's closed suite set, exit 2 unknown-suite). D-security-deps fires on removals, not additions: no dependency was added or bumped (Cargo.lock untouched); the proposals retire §Dependency Security text the report's Dependencies / Harness bullets and its own Expected amendments name. security-plan.md:296 ('Every ban is proven live by deny-probes.sh') carries no count and stays true."
