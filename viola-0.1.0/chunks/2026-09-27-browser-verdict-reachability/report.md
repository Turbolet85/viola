# Report — 2026-09-27-browser-verdict-reachability

**Chunk:** Browser verdict reachability. An e2e-web/ stub spec runs under pinned Playwright on three CI OSes and the WSL pre-push,
piped through `run --browser` to a gated green playwright suite. The 8 root 10 s waits are lowered to 7 s with streamed reports.
**Date:** 2026-09-27T19:55Z
**Commits:** `5f0a809 chore(2026-09-27-browser-verdict-reachability): operator pre-CI commit, for the run this chunk's verdict reads`
(the only commit since `last_wrap` 2026-09-27T17:41:38Z besides that wrap's own `49f6444`; `git log 49f6444..HEAD`)

## Changes (structured — detectors read this)
- **Files** (`git diff --name-only 49f6444` + the working tree; `.andromeda/runs/` excluded):
  - workflows: `.github/workflows/ci.yml`, `.github/workflows/nightly.yml`;
  - harness: `crates/viola-e2e/src/bin/viola-harness.rs`, `crates/viola-e2e/src/harness/run.rs`,
    `crates/viola-e2e/src/harness/run/browser.rs` (new), `crates/viola-e2e/src/harness/pre_push/linux.rs`,
    `crates/viola-e2e/src/harness/pre_push.rs` (test module only), `crates/viola-e2e/tests/cli.rs`;
  - `e2e-web/` (new): `package.json`, `package-lock.json`, `playwright.config.ts`, `tsconfig.json`, `stub/pipe.html`,
    `tests/pipe-reachability.spec.ts`;
  - scripts: `scripts/install-node.sh` (new), `scripts/npm-audit.sh` (new), `scripts/wsl-provision.sh`;
  - root tests: `tests/support/watch.rs` (new), `tests/support/{mod,fake,home,outer_pty}.rs`, `tests/run_cli.rs`,
    `tests/cli_instance_state.rs`, `tests/contract_diag_schema.rs`;
  - ledgers and route: `viola-0.1.0/verification-matrix.json` (v1-08), `viola-0.1.0/working-route.md` and
    `.andromeda/master-route.md` (the phase's promote), `.andromeda/friction-log.ndjson`;
  - chunk folder: `plan.md` (entry 13, an operator-directed edit), plus `evidence/{root-watch-recorder,code-graph-planes,
    wsl-chromium-deps,operator-pass}.md`.
- **Symbols / APIs:**
  - **`viola-harness run --browser`** (a new `RunArgs.browser` flag; `Selection.browser: bool`). `Selection::from_flags` is
    unchanged: `--all` and the no-selector default do NOT select browser (operator fork 2). The one full `Selection` literal,
    `viola-harness.rs` `run_cmd`, takes `browser: args.browser`. Every other literal spreads a base.
  - **`run/browser.rs` `browser(ws, runner) -> (Suite, Option<Refusal>)`**, which runs each step through the `Runner` seam:
    1. It deletes `e2e-web/pw.json`, `e2e-web/pw-junit.xml` and `artifacts/junit-playwright.xml`.
    2. It spawns `NPM ci` in `e2e-web/`, where `const NPM` is `npm.cmd` on Windows and `npm` elsewhere, with fixed literal args
       through `std::process::Command`.
    3. It runs `node -e <CHROMIUM_PROBE>`: exit 0 iff `require('playwright-core').chromium.executablePath()` exists.
    4. It runs `node node_modules/@playwright/test/cli.js test`. No shell and no npx.
    5. The suite `playwright` is built from `pw.json` `stats`: passed = `expected`, failed = `unexpected + flaky`, skipped =
       `skipped`. Its `failures` names come from `parse_junit(pw-junit.xml)`, which is copied to `artifacts/junit-playwright.xml`.
       Its `artifact` is the absolute `e2e-web/pw.json`. Then `exit_must_agree(code, "playwright")`.

    Failure arms: `npm ci` red → `failed:1, failures:["npm-ci-exit-<n>|signal"]`. Chromium absent →
    `failures:["chromium-missing"]`. Either one gives the refusal `browser-missing` (no `detail`), and Playwright never runs. A
    `pw.json` absent or without `stats` → `failures:["artifact-missing"]`.
  - **`run.rs`:**
    - `test_suites` now returns `(Vec<Suite>, Option<Refusal>)`, and pushes the browser suite after the doctests when
      `sel.browser`. Its caller, `run_with` (the sole caller, per the graph at P3), takes `browser_refusal.or(tool_refusal)`.
    - `archive_sources` adds `playwright`: its archived `junit-playwright.xml` is sourced from the sibling of the suite's
      artifact, `pw-junit.xml`.
    - `run_with`'s other callers (`run()`, `run_cmd`, `pre_push::stages`, `windows_tests`) keep their selections; none sets
      `browser`.
  - **`pre_push/linux.rs`:**
    - A new `pub fn node_pin(ci_yml) -> Option<String>` reads the workflow-level `  NODE_PIN_VERSION:` line.
    - A new `const NODE_BIN = ".local/viola-node/bin"`. `Linux::cmd_env`'s `env -i` PATH is now
      `<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` (a constant, distro-home-derived
      component; no host value).
    - `tools()` checks `node --version` == `v<pin>` after the cargo pins. Absent → `tool-missing` detail **`node`**; off-pin →
      `tool-pin-mismatch` detail **`node`**; the pin line unreadable → `tool-pin-mismatch` `pins-unreadable`.
    - `linux_tests` runs `run --coverage` (doc key `run`), then **`run --browser` (a new doc key `linux.browser`)**, then
      `gate --require coverage,doctest,playwright` (doc key `gate`). A browser red stops before the gate, and the stage stays
      `linux-tests`. An unreadable `run --browser` document stops `linux-document-unreadable` with the existing detail `run`.
    - `linux_tests` has one caller, `pre_push::stages`. The host `windows-tests` stage is unchanged: no browser (fork 2).
  - **The root test support `tests/support/watch.rs`** (a new module):
    - `WITHIN` = 7 s.
    - `Watch::start(label)` → `<std::env::temp_dir()>/viola-root-watch/<thread name, :: → .>.<label>.report`, created and
      truncated.
    - `note(line)` appends only when the line changes; `deadline_check(deadline, what)` panics with `what` + the report so far;
      `Drop` removes the report unless panicking.
    - The 8 sites now wait on `WITHIN` and note codes, counts and booleans. They are `fake.rs` `wait_for`, `home.rs`
      `wait_ready`, `outer_pty.rs` `wait_exit` (`EXIT_WITHIN` = `WITHIN`), `run_cli.rs` `wait_raw` and the `claude-child` wait,
      `cli_instance_state.rs` `-STOP` wait (unix), and `contract_diag_schema.rs` raw and exit waits.
    - `wait_ready` reads `snapshot_ready` / `beat_fresh` in the predicate's short-circuit order (see Insufficient fixes).
  - **Scripts:**
    - `scripts/install-node.sh <linux-x64|darwin-arm64|win-x64> <dest>` parses the pins from ci.yml's text (never the env).
      It downloads with curl `-fsSL --retry 5 --retry-delay 2 --retry-all-errors`, plus `--ssl-revoke-best-effort` under
      Schannel (the `install-ripgrep.sh` precedent). It verifies with `sha256sum` or `shasum -a 256`, and extracts
      (`tar -xJf|-xzf`, `unzip -q`) with the `node-v<ver>-<key>/` top level flattened into `<dest>`. It prints
      `install-node: <bin dir>` and skips when `<bin>/node --version` already equals the pin.
      - Refusals: `install-node: checksum mismatch for <file>` (exit 1, nothing extracted), `install-node: pin-unreadable`
        (exit 1), usage (exit 2).
      - `--probe` → `install-node --probe: 1/1 refused, pins ok v24.21.0`.
    - `scripts/npm-audit.sh` runs `npm audit --prefix e2e-web --package-lock-only --audit-level=low --json` →
      `target/npm-audit/audit.json`, reading `.metadata.vulnerabilities.total`. Its sources check is `jq -e` that every
      `packages[].resolved` starts with `https://registry.npmjs.org/`.
      - Verdicts: `npm-audit: advisories 0, sources registry.npmjs.org only` · `advisories <n>` ·
        `advisories 0, sources outside registry.npmjs.org` · `advisories unreadable`.
      - Modes: `--advisories-only` (the nightly twin), and `--probe` → `npm-audit --probe: 1/1 refused, control clean`.
    - `scripts/wsl-provision.sh`:
      - PATH gains `$HOME/.local/viola-node/bin`.
      - `install_all` adds `install-node.sh linux-x64 "$HOME/.local/viola-node"`, `node --version`, a copy of
        `e2e-web/package.json` + lockfile to `$HOME/.cache/viola-provision/e2e-web/`, and there `npm ci` +
        `npx --no playwright install chromium` (never in a checkout).
      - `--check`'s pins-ok line ends with `node v<ver>`, or prints `pin-mismatch: node`. `--probe` counts 3/3 (the
        install-node checksum refusal added).
      - A new mode, `--install-deps <user home>`, runs as uid 0 only (else exit 2). It refuses (exit 1) until the user provision
        exists. It runs `<home>/.local/viola-node/bin/node <home>/.cache/viola-provision/e2e-web/node_modules/@playwright/test/cli.js
        install-deps --dry-run chromium || true`, then `install-deps chromium`, and its last line is
        `wsl-provision: install-deps ok`. It is launched by the plan entry as `wsl.exe -d Ubuntu -u root … env -i HOME=/root
        PATH=… bash scripts/wsl-provision.sh --install-deps <distro home>` (no sudo).
  - **Ports / sockets:** none. No listener: the stub is `file://`, with no `webServer`.
  - **Env vars:**
    - No new `VIOLA_*` or product-read variable.
    - CI's workflow `env:` gains `NODE_PIN_VERSION`, `NODE_PIN_SHA256_LINUX_X64`, `NODE_PIN_SHA256_DARWIN_ARM64` and
      `NODE_PIN_SHA256_WIN_X64`. These are data lines `install-node.sh` parses from the file text; the scripts never read them
      from the environment, and one CI `test` step compares `node --version` with `v$NODE_PIN_VERSION`.
- **Crates / modules:** changed `viola-e2e` (new private module `harness::run::browser`). The root package's test support gains
  `tests/support/watch.rs`. No workspace member was added or removed. `e2e-web/` is a Node package, not a Cargo member.
- **Dependencies:**
  - npm (test-side only, `e2e-web/package-lock.json`, `npm install` under host Node v24.13.1 / npm 11.8.0, all from
    `https://registry.npmjs.org/`): `@playwright/test` 1.63.0 (exact), which pulls in `playwright` 1.63.0 and
    `playwright-core` 1.63.0. That is 3 packages.
  - No `@axe-core/playwright`, `typescript` or `@types/node`.
  - No Cargo dependency was added or bumped (`Cargo.toml` / `Cargo.lock` untouched: `git diff --name-only 49f6444` lists neither).
- **Schema / config:**
  - `e2e-web/playwright.config.ts`: `testDir 'tests'`, `outputDir 'test-results'`, `retries: 0`, `forbidOnly: true`, one project
    `chromium` `{ headless: true }`, reporters json→`pw.json` + junit→`pw-junit.xml`. No `webServer`, `globalSetup` or
    `bypassCSP`.
  - `e2e-web/tsconfig.json`: `noEmit`, `strict`, `include ["**/*.ts"]`, `target ES2022`, `module commonjs`,
    `skipLibCheck`.
  - The run document's closed `reason` gains `browser-missing`. The pre-push closed `detail` gains `node`, and the pre-push
    document's `linux` object gains the key `browser`.
  - `run-summary.json` may now carry the suite `playwright`. `gate.rs` already listed it in `SUITES` and `JUNIT`, unchanged.
- **Spec-master edits:** none. No `.andromeda/*.md` master was edited by the phase or by implement.
- **Counts / qualifiers moved:**
  - CI's `test` job: steps +6 (`Node (pinned)`, `Browser suite dependencies (npm ci)`, two Chromium steps, and the Browser
    suite pwsh/sh shims). `Gate verdict` now requires `coverage,doctest,playwright`, and `Upload JUnit` now uploads 2 paths.
  - `supply-chain` +2 steps (`Node (pinned)`, `npm lockfile audit`). `nightly.yml` +1 job, `npm-advisories`.
  - Jobs per push: 8, unchanged. Check-runs: 15, unchanged (ci#36345175642: checks 15/15).
  - The pre-push Linux leg's `linux_tests` runs 3 harness commands (was 2). `wsl-provision.sh --probe` counts 3/3 (was 2/2).
  - The root test waits' bound: 10 s → 7 s at all 8 sites (`grep -rnE 'from_secs\(10\)' tests` → no output).
- **Dev-tool versions:**
  - The **runtime (Node) on the CI runners**: runner-image Node (ubuntu and windows 22.23.2, macOS 24.20.0, per research.md)
    → the pinned official build **v24.21.0** from `nodejs.org/dist`, sha256-checked (ci#36345175642, 2026-09-27T19:40–19:49Z).
  - The **runtime (Node) in the WSL distro** (`Ubuntu` 26.04): absent → v24.21.0 at `~/.local/viola-node/bin` (gate entry 41,
    ~19:12Z).
  - The **browser driver (Chromium)**: the Playwright 1.63.0-bundled Chromium, newly installed on this host (`playwright install
    chromium`, gate entry 9), in the distro (`~/.cache/ms-playwright`) and per CI OS.
  - The host's own Node re-read at v24.13.1 (unchanged). `@playwright/test` is lockfile-resolved, NOT this line's subject.
- **Harness / gate surface:**
  - `run --browser` (new selector, suite `playwright`, reason `browser-missing`).
  - `pre-push` `linux.browser` + a Linux gate requiring `playwright`, and the tools stage checks `node`.
  - ci.yml `test` job browser steps between `Harness lifecycle` and `Gate tools present`. The G2 → G4 → capture → scan →
    uploads → gate order is intact, and `junit-playwright.xml` rides the scan-gated `junit-<os>` upload.
  - The `supply-chain` npm audit (JSON under `target/npm-audit/`, not the uploaded `target/supply-chain/`), and the nightly
    `npm-advisories` job.
  - `wsl-provision.sh --install-deps`.
- **Cross-project / external claims:**
  - `https://nodejs.org/dist/v24.21.0/SHASUMS256.txt` was re-read at implement (curl, 2026-09-27 ~18:55Z). All three pin values
    matched the plan's.
  - Playwright 1.63.0 `install-deps --dry-run` exits 1 while packages are missing (measured on WSL Ubuntu 26.04, gate entry 42
    first run).
  - CI **ci#36345175642 on `5f0a8095ec12`: verdict green, checks 15/15, wall 554 s**. Its steps (`gh run view --json jobs`):
    Node (pinned), npm ci, Chromium, Browser suite, Upload JUnit and Gate verdict all succeed on windows-2025, macos-latest and
    ubuntu-latest, and `supply-chain`'s npm lockfile audit succeeds. The run measured the pre-CI commit's tree; this wrap's
    commit adds only bookkeeping and the report.
- **Reverted / negative API facts:**
  - The temporary control test `zz_root_watch_control` in `tests/run_cli.rs` was written for the remove-the-guard pair and
    removed after it (`grep -rl zz_root_watch_control tests` → no output).
  - The `run --browser` case of `viola-e2e::cli unbuilt_selectors_and_unknown_commands_are_usage` was removed: that selector is
    built now. `run --perf` and `boot --ui` stay as the unbuilt witnesses.
- **Insufficient fixes (written, kept, not the remedy):**
  - The defect: the wrapper's state write, `viola_state::fs::replace_private` → `tempfile::persist` (a Windows replace that fails
    while another process holds the target open, as `replace_private_shared`'s own doc says). It made one wrapper exit
    `internal-error` (chain: `state file i/o failed` / `Access is denied. (os error 5)`) and orphan its fake-agent child. That
    reddened `viola::cli_instance_state run_takes_over_a_gone_name_and_appends`, and the orphan's lock on
    `target/harness/debug/viola-fake-agent.exe` (os error 5) reddened the boot smoke's build.
  - The trigger was this chunk's first `wait_ready` recorder, which read `snapshot.json` on every poll from spawn.
  - What the change resolved: `wait_ready` now reads in the predicate's short-circuit order, and the gates went green.
  - What it did not resolve: the wrapper's fragility to any concurrent reader of `snapshot.json` (a test, an indexer, an
    antivirus). That is not reproduced deliberately here.
  - The owner is the NEXT chunk's P1 (overseer direction): a retry on the sharing violation plus a witness test.
- **Spec claims disproved by measurement:**
  - The plan, step 11, says `--install-deps` "first prints the apt command … then runs the same command without
    `--dry-run`". Playwright 1.63.0's dry run exits 1 while any package is missing, so `set -e` stopped the script before the
    install. The evidence is `evidence/wsl-chromium-deps.md`. The plan text is the only home (no master states the dry-run's
    exit); fixed in the script with `|| true`.
  - The plan's entry 13, run from the repo root without `--config`, ran Playwright on its defaults (no `chromium` project, a root
    `test-results/.last-run.json`). The plan text was edited by operator direction; the evidence is `evidence/operator-pass.md`.
  - The plan's HYPOTHESIS "`unzip` exists in the windows-2025 runner's bash" is measured TRUE (ci#36345175642, `Node (pinned)`
    on windows-2025, success). It is a confirmation, not a disproof.
  - The plan's HYPOTHESIS "`Command::new("npm.cmd")` spawns on the Windows host with Rust 1.98.1" is measured TRUE (gate entry
    10, green).
- **Expected amendments (from plan).** Sites were located by `.andromeda/runs/2026-09-27T19-50-23-wrap/sites.py`, a regex sweep over the 7 masters. Hit counts
  are per master, `doc:N@lines`:
  - test-plan §3 `run` step 3 (browser on every OS; the `browser-linux-only` arm retired; npm and the Playwright CLI spawned
    directly): **carried** (Symbols: `run/browser.rs`; Harness). `browser-linux-only`: test-plan:1@551 · a11y-plan:3@275,671,1126.
  - test-plan §3 `run` step 2 (the `booted_wrapper` bound 10 s → 7 s): **carried** (Symbols: `tests/support/watch.rs`, 8 sites).
    `booted_wrapper`: test-plan:5@519,539,542,740,1365. `10 s`: test-plan:8@530,542,594,762,763,773,1378,1532 (a11y-plan:5 hits
    are other bounds).
  - test-plan §3 Closed enums (`browser-missing`; pre-push detail `node`): **carried** (Schema / config). `browser-missing`:
    test-plan:1@550 · a11y-plan:4@275,671,1126,1265. `tool-pin-mismatch`: security-plan:1@326 · test-plan:2@672,1454.
  - test-plan §3 Bootstrap phases `test-runner-install`, Node side (axe deferred to Epoch 8): **carried** (Dependencies: no
    axe). `test-runner-install`: test-plan:3@756,810,1788. `axe-core/playwright`: architecture:1@512 ·
    test-plan:4@770,996,1662,1687 · a11y-plan:11 hits.
  - test-plan §2 File naming (the `pipe-reachability.spec.ts` stub): **carried** (Files). `bay-layout-type|<bay-`:
    test-plan:1@468 · a11y-plan:2@457,1383.
  - test-plan §6 Drivers table (headless on 3 OSes): **carried** (Cross-project: ci#36345175642). `Drivers`: test-plan:2@986,1128 ·
    layout-templates:1@607 (a different table).
  - test-plan §9 E2E row, Matrix builds and the tool-install paragraph (the pinned Node download, the per-OS Chromium install):
    **carried** (Counts; Dev-tool versions). The browser-ubuntu-only sweep: test-plan:3@551,996,1445 · a11y-plan:7 hits.
  - test-plan §9 Supply-chain row (the npm audit + sources step and its nightly twin): **carried** (Harness). `npm audit|
    package-lock|npm lockfile`: 0 hits in all 7 masters. `nightly.yml`: test-plan:10 hits · security-plan:5 · architecture:2 ·
    obs-plan:1 · a11y-plan:1.
  - test-plan §12 Browser caching (runner-image Node → pinned download): **carried** (Dev-tool versions). `Browser caching`:
    test-plan:2@1609,1671. `runner-image Node`: test-plan:1@1671.
  - test-plan §12 Decisions Log W125: **carried** (the founder ruling is this entry's CARRY). `W125`: 0 hits in all 7 masters.
  - a11y-plan §3 CI integration, §9 Pipeline integration, §11 CI, D-A11Y-12 (the pipe on 3 OSes; the a11y verdict stays
    ubuntu-bound) and §3 a11y-tooling-install (the axe pin deferred): **carried** (Cross-project; Dependencies). `D-A11Y-12`:
    a11y-plan:1@1332.
  - design-system §Typography / §Surface: web-spa → Platform-Specific Notes (Fonts) (render and contrast assertions stay judged
    on the ubuntu leg): **carried**. The stub asserts only a heading's role and name, with no render or contrast. `CI render`:
    design-system:5@227,245,253,414,670.
  - security-plan §Dependency Security, CI integration (the Node source: a verified download, no action): **carried**
    (Dev-tool versions; Symbols `install-node.sh`).
  - security-plan §Dependency Security, the npm lockfile audit and its nightly twin: **carried** (Harness). 0 `npm audit` hits.
  - security-plan §Dependency Security, Pinning (WSL2) (Node and Chromium join the third install site; the PATH component is
    distro-derived; Chromium's system libraries installed by a plan entry as uid 0 through `wsl.exe -d Ubuntu -u root`, never
    sudo): **carried** (Symbols `wsl-provision.sh`, `linux.rs`). `wsl-provision`: security-plan:1@326 · architecture:2@36,501 ·
    test-plan:1@1454. The wrap must judge whether the root install is a boundary widening.
  - architecture §Stack and Technologies CI/CD row, §Infrastructure Patterns → CI/CD approach: **carried** (Harness; Counts).
    `nightly.yml`: architecture:2@528,534.
  - architecture §Occupied Resources (`<temp dir>/viola-root-watch/`, `target/npm-audit/`, the CI `$RUNNER_TEMP/node`, the
    distro's `~/.local/viola-node/` and `~/.cache/ms-playwright/`, plus `~/.cache/viola-provision/e2e-web/`): **carried**
    (Symbols). `viola-root-watch|viola-pty-watch`: architecture:1@383.
  - architecture §Infrastructure Patterns → Project directory structure (the `e2e-web/` entries this chunk creates): **carried**
    (Files). `axe-core/playwright`: architecture:1@512 (the tree line).
  - obs-plan §9 Step order and Test report format (the browser step in the `test` job; `junit-playwright.xml` in
    `junit-<os>`): **carried** (Harness). `junit-<os>|junit-playwright`: obs-plan:2@1288,1750 · architecture:2@402,537 ·
    test-plan:1@648.
- **Coverage of new surfaces:**
  - `run --browser` → validation n/a (fixed argv, no external input) · instrumentation n/a (harness, test-side; one JSON
    document) · PII n/a · tests unit (8 runner-seam tests in `run/browser.rs`) + e2e (gate entries 10–13) · a11y n/a · tokens
    n/a.
  - `pre-push` node check and `linux.browser` → validation closed codes · instrumentation n/a · PII n/a (the document carries
    codes and counts; `summary()` drops paths) · tests unit (`pre_push_node_*`, `pre_push_linux_browser_red_stops_before_the_gate`,
    `pre_push_linux_tests_run_coverage_browser_then_the_playwright_gate`) · a11y n/a · tokens n/a.
  - `install-node.sh` / `npm-audit.sh` / `wsl-provision.sh --install-deps` → validation sha256 / registry-prefix / uid 0 ·
    tests `--probe` controls (gates 6, 22, 23) · others n/a.
  - `e2e-web/stub/pipe.html` (test fixture, not a UI surface) → a11y n/a (one `<h1>`, `lang="en"`) · tokens n/a (no style;
    gate 19: 0 banned tokens).
  - `tests/support/watch.rs` → PII n/a (codes, counts and booleans; the dir is outside home `diagnostics/`, G4 green) · tests
    the remove-the-guard pair.

## Deviations from intent
- **Edits outside the plan's Files-to-modify:**
  - `crates/viola-e2e/src/harness/pre_push.rs`, test module only: `CI_LINE` gains the `NODE_PIN_VERSION` line, and `Fake::green`
    answers `node --version` and `run --browser`. Justification: the shared fake drives every `linux.rs` test, and the new
    `tools()` / `linux_tests` calls need those answers.
  - `crates/viola-e2e/tests/cli.rs`: the `run --browser` usage case was removed. Justification: it pinned the selector this
    chunk builds.
  - Both are research misses (evolve records).
- **Archive source:** the plan says the browser suite's `artifact` is `pw.json`, and also that `archive_sources` archives
  `junit-playwright.xml`. `archive_sources` sources the playwright JUnit from the artifact's sibling `pw-junit.xml`, which is
  test-pinned.
- **`install-node.sh` flattens `node-v<ver>-<key>/` into `<dest>` itself**, so its printed bin dir is right on every leg. The plan
  had `wsl-provision.sh` doing the move. The script also takes install-ripgrep's Schannel `--ssl-revoke-best-effort` flag (the
  measured Windows-runner revocation failure) and skips the download when the pinned Node is already there.
- **`npm-audit.sh --advisories-only`:** a mode step 5 did not name; the nightly twin (step 6, "advisories only") needs it.
- **The browser arm also deletes a stale `artifacts/junit-playwright.xml`** before the run (the fresh-file rule extended to the
  copy the gate reads).
- **`--install-deps`'s dry run carries `|| true`** (Spec claims disproved).
- **The operator pass was run by this session** on the overseer's founder-delegated direction: `plan.md` entry 13 edited, the
  pre-CI commit, the push and the CI read. /implement's own rules forbid a plan edit and a history move; the operator overrode
  them explicitly.

## Decisions & corrections
- Overseer (founder-delegated), after implement:
  - "fix plan entry 13 to run Playwright with the e2e-web config … and confirm no root test-results/ is left";
  - "run the operator pass yourself … a red stops the pass … If CI goes red, record the red-to-act time";
  - "The snapshot.json replace fragility goes to the NEXT chunk P1 as a fold (a retry on the sharing violation plus a witness
    test)".
- Wrap direction:
  - retire both recurrence watches by their expiry (3 consecutive green: ci#36333711860, ci#36340086338, ci#36345175642);
  - the pty recorder STAYS in the tests;
  - H2 (rstudio#18884) stays OPEN as the founder's product question;
  - the root install through `wsl -u root`: if a detector flags it as a boundary widening, halt for the operator's live answer.
- Learned this chunk:
  - A test-side recorder must not change what its loop reads or when: an extra early read of a file the wrapper replaces
    reddened a Windows run (os error 5 on the wrapper's own write). Note the value in the predicate's short-circuit order.
  - `npx --prefix <dir> playwright test`, run from another cwd, does not find `<dir>/playwright.config.ts`: pass `--config`.
  - Playwright `install-deps --dry-run` exits 1 while packages are missing.
  - A root test's thread name under nextest is the bare test name, so `Watch` reports are `<test>.<label>.report`.
- Sweep hazard: none new. The `sites.py` sweep's `10 s` pattern also matches unrelated bounds (a11y-plan:5 hits), read per hit
  as other bounds.

## Outcome
- **Acceptance criteria, against the diff:**
  - `run --browser` exit 0 with `playwright` 1/0/0, `gate --require playwright` exit 0 `breaches []`, and `junit-playwright.xml`
    present: **met** (gates 10–12).
  - Runner-seam negatives (npm ci red, Chromium absent, artifact-missing, stale deletion, red exit with a clean report) and
    `gate_fails_failed_and_skipped_suites_playwright_included`: **met** (gates 26–27, 31).
  - Config/spec bans: **met** (gates 15–19).
  - CI 3 OSes run `run --browser`, `Gate verdict` requires `coverage,doctest,playwright`, and `ci.py conclusion` is green:
    **met** (ci#36345175642 on `5f0a809`, 15/15). G2/G4/secret-scan still run `if: always()` after the browser steps, and the
    JUnit upload stays behind `steps.secret-scan.outcome == 'success'`.
  - `zizmor` passes with no new `uses:`, and the Node pin lines exist once: **met** (gates 5, 7). `install-node.sh --probe`:
    **met** (gate 6).
  - The npm lockfile committed and audited (0 advisories, registry-only), `--probe` refuses a foreign source, the nightly twin
    exists, and `target/supply-chain/` gains no member: **met** (gates 22–23; CI `supply-chain`). The nightly job itself has
    not run yet (schedule / dispatch only).
  - The WSL leg installs only CI's pins, `tools()` refuses `node` off-pin, the only new `env -i` content is the constant PATH
    component, and no gate runs through `wsl-exec.sh`: **met** (gate 21; unit tests; gate 41).
  - The pre-push document's `linux.browser` passes playwright ≥ 1 and the gate requires `coverage,doctest,playwright` at
    `union` ok: **met** (entries 43 and 44).
  - `e2e-web/tsconfig.json` has `noEmit` + `strict`, there is no tsconfig or package.json under `crates/`, no Cargo member was
    added, and `release-check: viola only`: **met** (gates 14, 15, 39).
  - `verification-matrix.json#v1-08` → `implemented`, ref `evidence/code-graph-planes.md`: **met** (gates 32–34).
  - The stub design bans: **met** (gate 19).
  - The overseer direction: `from_secs(10)` census empty, 8 sites on `WITHIN`, the pair recorded (7 s `FAIL [7.018s]` report
    kept; 12 s `TIMEOUT [10.010s]` output lost, file kept), and G4 `schema-check` green: **met** (gates 24, 25, 29;
    `evidence/root-watch-recorder.md`).
  - The mutation verdict green on the CI union: **met** (ci#36345175642 mutants jobs green; local pre-push legs `counted` 39,
    union 0 breaches).
- **Gates (implement run `2026-09-27T18-52-22-implement`, by `run` text):**
  - Green on the first run: `cargo fmt --all --check` · clippy · `lint-probes.sh` · `orphans-check.sh` ·
    `zizmor .github/workflows/` · `install-node.sh --probe` · the NODE_PIN census (4) · `npm ci --prefix e2e-web` ·
    `playwright install chromium` · `agent-run.sh run --browser` · `gate --require playwright` · `test -f junit-playwright.xml`
    · the crates manifest census (exit 1, 0) · the tsconfig census (2) · the config census (5) · the config bans (exit 1, 0) ·
    the spec bans (exit 1, 0) · the stub bans (exit 1, 0) · the ci.yml gate census (1) · the wsl-exec census (exit 1, no
    output) · `npm-audit.sh` · `npm-audit.sh --probe` · the `from_secs(10)` census (exit 1) · the control census (exit 1) ·
    `run --unit` · `run --unit --filter 'package(viola-e2e)'` · `schema-check` · `secret-scan` · `code-graph.py refresh` ·
    both tree.db present · `health.py check` (check 11 ✓ `planes rust, ts`) · `release-check.sh` · the MSRV check ·
    `wsl-provision.sh` (as the distro user).
  - **Red on the first run, green on re-run:**
    - `AGENT_RUN_KEEP_HOMES=1 … run --integration` (two failures: the takeover test and the cli unbuilt-selector case);
    - `run --mutants --file …/run/browser.rs` (the baseline's cli case) → green `scoped`, 17 tested, 15 caught, 0 survived;
    - the smoke `boot`/`status`/`cleanup` (the orphan's exe lock) → green;
    - `wsl-provision.sh --install-deps` (the dry-run exit) → green `wsl-provision: install-deps ok`.
  - `playwright test --config e2e-web/playwright.config.ts --grep 'pipe:'` (entry 13, after the operator edit) → green.
  - `agent-run.sh pre-push` (non-leg) → green, `union`.
  - The `leg = 'operator'` entries (recorded in `evidence/operator-pass.md`):
    - `pre-push` → green, 19:31:14Z → 19:39:04Z, `union` ok;
    - the guarded push → exit 0, ff `49f6444..5f0a809`;
    - `ci.py conclusion --sha HEAD --wait 5400` → exit 0, `5f0a8095ec12 verdict: green · checks 15/15 · wall 554 s ·
      runs ci#36345175642 completed/success`.
  - Smoke: harness lifecycle green (status `ready`; cleanup `processes_gone` / `endpoint_gone` true, `killed []`).
- **Outcome basis:** the operator pass ran (`5f0a809`, the only commit of the pass; no fix commits). The CI verdict is
  ci#36345175642 on `5f0a809`. Implement's P4 report and this session's conversation are the basis for the deviations and the
  gate history.
- **CARRY 2:** ci#36345175642 is the 3rd consecutive green CI run with no recurrence of either watched name (after
  ci#36333711860 on `f0e6dbc` and ci#36340086338 on `49f6444`), and no local run of this chunk recurred. The expiry is
  reached, and by the wrap direction both watches retire. The viola-pty recorder stays in the tests. H2 stays OPEN (the
  founder's product question).
- **Process hygiene:**

  | process | started by | final state |
  |---|---|---|
  | `viola-fake-agent.exe` pid 13856 (orphan of the red integration test) | this run | terminated (stopped by exact ExecutablePath + pid) |
  | the smoke session's supervisor, wrapper and agent | this run | terminated (cleanup `processes_gone:true`) |
  | the WSL `Ubuntu` VM | this run (gates 41–44, the dpkg read) | not running (`wsl.exe -l --running`: none) |

  Re-measured at this wrap: 0 processes under `D:\dev\projects\viola\`. The four `viola.exe` of `viola-lab/prototype` were not
  this run's.
