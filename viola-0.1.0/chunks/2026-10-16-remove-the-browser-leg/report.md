# Report — 2026-10-16-remove-the-browser-leg

**Chunk:** the browser test leg and the GUI harness leftovers leave 0.1.0: e2e-web/, Node, npm and Playwright out of CI,
the pre-push and the harness; every gate green with no Node on the host
**Date:** 2026-10-16
**Commits:** `fac2d9d chore(2026-10-16-remove-the-browser-leg): operator pre-CI commit` (on `b6bbbd6`, the last wrap's
0-pending route adaptation; basis `git log --format='%h %s' b6bbbd6..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (basis: `gate.py scope` — `scope: clean — changed 36 · listed 36 · recorded 0`, base `b6bbbd68`)
  - Deleted: `e2e-web/` whole (six tracked files: `package.json`, `package-lock.json`, `playwright.config.ts`,
    `tsconfig.json`, `stub/pipe.html`, `tests/pipe-reachability.spec.ts`); `scripts/install-node.sh`;
    `scripts/npm-audit.sh`; `crates/viola-e2e/src/harness/run/browser.rs`.
  - Modified: `.gitignore`; `.github/workflows/ci.yml`, `nightly.yml`, `windows-mutants.yml`;
    `crates/viola-e2e/src/harness/{run,gate,pre_push,status,cleanup,mod,boot,secret_scan,logs}.rs`,
    `crates/viola-e2e/src/harness/pre_push/linux.rs`, `crates/viola-e2e/src/bin/viola-harness.rs`;
    `crates/viola-core/src/obs.rs`; `src/obs.rs`; `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`;
    `tests/contract_diag_schema.rs`; `scripts/agent-run.sh`, `scripts/agent-run.ps1`; `.config/nextest.toml`;
    `deny.toml`, `deny-sync.toml`, `Cargo.toml`; `scripts/deny-probes.sh`.
- **Symbols / APIs:**
  - Harness `run`: the `--browser` selector, `Selection::browser`, the `browser` module (`browser`, `playwright_suite`,
    the `NPM` / `CHROMIUM_PROBE` / `JUNIT` consts) and the `browser-missing` refusal are gone. `test_suites` returns the
    suites alone; `run_with` takes its refusal from `tool_arms` alone; `archive_sources` archives
    `junit-{nextest-unit,nextest-integration,coverage}.xml` and `outcomes.json` only (no `playwright` arm, no
    `pw-junit.xml` source). The run document's `reason` values lose `browser-missing`.
  - Harness `gate`: `SUITES` is the eight `nextest-unit · nextest-integration · doctest · nextest-e2e · mutants ·
    coverage · perf · fuzz-replay` (`playwright` left); `JUNIT` is the four `nextest-unit · nextest-integration ·
    nextest-e2e · coverage`. `gate --require playwright` is usage exit 2, `"detail":"unknown-suite"` (measured, gate
    entry 8).
  - Harness `pre-push` (`pre_push/linux.rs`): `NODE_BIN` and `node_pin` are gone; the `tools` stage checks `cc`, the
    `rust-toolchain.toml` channel and ci.yml's `test`-job tool line only (no `node`, no `NODE_PIN_VERSION`); every
    child runs under `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin` (the
    `<home>/.local/viola-node/bin` element left); `linux-tests` runs `run --coverage` then
    `gate --require coverage,doctest`, and the document's `linux` object holds `run` and `gate` only (no `browser`).
    The passwd-home probes and the HOME + PATH-only rule are unchanged.
  - Harness `status`: the document drops `"ui": null` and `"api_sessions_equal_list": null` (keys now `v, cmd, ok,
    state, pid, uptime_ms, last_error, list, instances`; measured by the smoke entry `status --session p-rbl-smoke`
    with `lacks api_sessions_equal_list`).
  - Harness `cleanup`: the report drops `"port_free": null` and `"url_file_removed": null` (keys `v, cmd, ok, cleaned,
    processes_gone, endpoint_gone, home_removed, killed`; measured by the smoke entry with `lacks port_free`).
  - `SessionRecord` (`target/agent-run/<session>/session.json`) drops `ui` and `cookie_file`: it is `{v, session,
    home, instances, supervisor_pid, supervisor_started_at}`. No `deny_unknown_fields` in `viola-e2e`, so an older
    record carrying both keys still parses.
  - Harness `secret-scan`: `CRITICAL` holds the twelve `claude-stripped` canaries (`token-query` / `?t=` left); the
    `gui-token` class (the `t` value of each home's `ui/<port>.url`) and the `viola_<port>=` cookie-pair form are gone
    (`gui_tokens`, `url_token`, `cookie_pairs` removed). The `cookie` class keeps the case-insensitive `cookie:` header
    form; the `content-canary` class and the non-0600 `mode` check are unchanged; the module doc and `CONTENT` are
    unchanged.
  - The shims `scripts/agent-run.{sh,ps1}`: `ui-restart` is no longer forwarded (internal list: `supervise, gate,
    schema-check, secret-scan, pre-push`); the `.sh` header's `run` selector list loses `--browser`, `boot` loses
    `(+ viola ui)`, `status` reads "the session's status document (test-plan §3 Status endpoint shape)", `cleanup`
    reads "graceful stop, endpoint checks, home removal (idempotent)"; both embedded-asset comments are one line ("boot
    builds the workspace itself, so the shim adds no pre-build step"); `ensure_fresh_artifacts` /
    `Invoke-EnsureFreshArtifacts` stay (upgrade U10 reads the first).
  - Product obs (`crates/viola-core/src/obs.rs`): `ObsEvent::{HttpRequest, LivenessChanged, SseOpened, SseClosed}` and
    `ObsProcess::Ui` are removed. `ObsEvent::ALL` is the fifteen `channel-request · channel-response · dialog-raised ·
    dialog-answered · hook-invoked · hook-decision · send-issued · send-confirmed · send-refused · release-from-driver ·
    process-start · process-exit · panic · state-recovered · parse-rejected`; `ObsProcess::ALL` is the four
    `run · hook · mcp · cli`. No product path emitted any removed value (every `viola_obs_init` caller passes `Run` /
    `Hook` / `Cli`, research §Graph impact).
  - Product obs (`src/obs.rs`): `VIOLA_TARGETS` is the seven `viola · viola_core · viola_pty · viola_channel ·
    viola_state · viola_agent_claude · viola_mcp` (`viola_ui` left); the OFF-target comment names `rmcp, notify, …`
    (axum left); `role_file_name(process, instance)` and `open_role_file(home, process, instance)` lose their `port`
    parameter (the role files are `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>`; no `ui-<port>`); the sole caller
    `viola_obs_init` and the file's own tests are the only other users (graph: `src/obs.rs` alone).
- **Crates / modules:** removed module `viola-e2e::harness::run::browser`; no crate added or removed.
- **Dependencies:** none added or bumped. Removed from the tree: the test-side npm graph (`e2e-web/package-lock.json`,
  `@playwright/test` 1.63.0) and the Node v24.21.0 pin; no Cargo dependency changed (`Cargo.lock` untouched).
- **Schema / config:**
  - `schemas/diag-line.v1.json`: the `process` enum is `run · hook · mcp · cli`; the description's role-file list is
    `{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; the four message-const blocks and the field blocks of
    `http-request`, `liveness-changed` and `sse-opened` / `sse-closed` are removed; `process-start` loses `port`;
    `parse-rejected`'s `parser` enum loses `last-event-id`; `$defs.event` is the fifteen; `$defs.liveness` is removed.
  - `schemas/diag-detail.v1.json`: the inlined `event` enum is the fifteen; the `process` enum is the four.
  - Both keep top-level `unevaluatedProperties: false`.
  - `.config/nextest.toml`: the `[test-groups] fixed-port` group and its `test(/default_port/)` override are removed
    (no test held `default_port`).
  - `deny.toml`: the `[[bans.features]] crate = "axum" deny = ["http2"]` block is removed (P4 fork 2, `inputs#I1`);
    its header reads "The tokio ban cannot live here (viola-mcp uses tokio)". `deny-sync.toml`'s tokio reason reads
    "tokio is confined to viola-mcp; the sync crates stay tokio-free". `Cargo.toml`'s interprocess comment reads "the
    `tokio` feature stays confined to viola-mcp".
  - `.gitignore`: the `# e2e-web (Node, test-side only)` block (`node_modules/`, `*.tsbuildinfo`,
    `e2e-web/{test-results,playwright-report}/`, `e2e-web/pw.json`, `e2e-web/pw-junit.xml`) is removed.
- **Spec-master edits:** none this chunk (the wrap amends them, P2).
- **Counts / qualifiers moved:** (each with its rule)
  - closed obs events 19 → 15 (`ObsEvent::ALL`, the list above); processes 5 → 4 (`ObsProcess::ALL`);
  - closed `suite` values 9 → 8 (`gate::SUITES`); suites with a JUnit file 5 → 4 (`gate::JUNIT`);
  - `CRITICAL` secret-scan needles 13 → 12 (the twelve `claude-stripped` canaries);
  - deny ban probes 13 → 12 (`scripts/deny-probes.sh`'s rows; summary `deny-probes: 12/12 banned, control clean`,
    measured, gate entry 14);
  - `VIOLA_TARGETS` 8 → 7 (the list above);
  - the pre-push PATH elements 5 → 4 (`<home>/.cargo/bin`, `/usr/local/bin`, `/usr/bin`, `/bin`);
  - the pre-push `linux` sections 3 → 2 (`run`, `gate`); a green pass makes nine harness-runner calls (two passwd
    probes, `cc`, `rustc`, three cargo tools, `run --coverage`, `gate`), pinned by
    `pre_push_every_native_call_carries_only_home_and_path`;
  - `windows-mutants.yml`'s `viola-e2e` item lists two files (`run/mutants/scratch.rs`, `cleanup.rs`); the `- package:`
    item count stays ten;
  - CI `test` job's gate: `gate --require coverage,doctest` (was `coverage,doctest,playwright`); `junit-<os>` uploads
    `target/nextest/ci/junit.xml` alone.
  - Tracked `tsconfig.json` files 1 → 0 (`git ls-files '*tsconfig.json'` after the commit: none), so the ts code-graph
    plane has no input.
- **Dev-tool versions:** none — Node v24.21.0 and `@playwright/test` 1.63.0 are no longer used by any tree path; the
  dev host's Node install is the operator's (not uninstalled; plan Constraints).
- **Harness / gate surface:**
  - `ci.yml`: the workflow-level `env:` (the four `NODE_PIN_*` lines) is removed; the `test` job loses `Node (pinned)`,
    `Browser suite dependencies (npm ci)`, `Chromium (with system deps)`, `Chromium`, `Browser suite (pwsh shim)`,
    `Browser suite (sh shim)` and their comment; `Upload JUnit` carries one path; `Gate verdict` runs
    `gate --require coverage,doctest`; `supply-chain` loses `Node (pinned)` and `npm lockfile audit (advisories,
    sources)` and their comment. Every other step's order, `if:`, `shell:` and pin is unchanged (G2 → G4 → capture →
    secret scan → the four scan-gated uploads → gate).
  - `nightly.yml`: the `npm-advisories` job is removed (`advisories` and `fuzz` unchanged).
  - `windows-mutants.yml`: `crates/viola-e2e/src/harness/run/browser.rs` leaves the `viola-e2e` item.
  - The harness verbs above (`run`, `gate`, `pre-push`, `status`, `cleanup`, `secret-scan`); `ui-restart` is not
    forwarded by either shim.
- **Cross-project / external claims:**
  - CI run `ci#38937051980` on `fac2d9defd76` (the pre-CI commit): `verdict: green · checks 15/15 · wall 479 s ·
    completed/success`, `attempt` 1 (`chunks/2026-10-16-remove-the-browser-leg/evidence/operator-pass.md`).
  - `inputs.py verify`: `I1 n/a — a message has no live source · copy message · message: the operator, as the overseer
    (technical, founder-delegated), in the P4 dialog, 2026-10-16T15:50Z`; cited `scope.md:12 · plan.md:10 ·
    plan.md:113 · plan.md:175 · plan.md:445 · plan.md:478 · plan.md:529 · plan.md:552`; summary `inputs: 1 entries —
    unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 0 · unparsed 0`.
    I1's two answers: Q1 "Remove all GUI forms (Recommended)", Q2 "Remove ban and probe (Recommended)", each with the
    overseer's note (technical, founder-delegated).
  - No entry names a P5 review card; the phase card `.andromeda/runs/2026-10-16T15-37-36-phase/p5-card-1.md` reads
    `## Beyond the plan` none.
  - The founder's ruling the chunk rests on is the committed file
    `.andromeda/runs/2026-10-16T14-58-10-wrap/inputs/I1-restructure-cli-only.md.txt` (this repository; item 3).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan (step 1, Implementation notes) and research (§Files to modify, §Files inspected) say `e2e-web/` holds
    five tracked files; `git ls-files e2e-web` at `b6bbbd6` listed six (`stub/pipe.html` the sixth). All six left.
    (A chunk-record count, no master claim.)
  - The plan's step 7 literal `["gate", "run"]` for the pre-push document's `linux` keys: the document keeps insertion
    order, measured `["run", "gate"]` (first gate firing, `pre_push.rs:234`). (A plan literal, no master claim.)
  - The plan's sweep expectation `http-request|liveness|sse-` over `tests/` → 0: 2 hits, `tests/cli_list.rs:90,237`,
    the `viola list` document's own `liveness` field (a different contract), no change.
- **Expected amendments (from plan):** (search: `cascade.py sweep --pre-pass` over the seven masters and every
  `.andromeda/registries/**` file, `{run_dir}/report-patterns.toml`, 13 patterns, `total (13 patterns) · 439 rows over
  42 files`, `per class · standing 318/22 · leaf 121/20`; trail `cascade-2026-10-16-remove-the-browser-leg-pre-pass.json`.
  Standing figures per pattern: node-pin 31 · npm 56 · playwright 90 · e2e-web 57 · ts-plane 4 · axum 4 · probes-13 2 ·
  viola-ui 6 · fixed-port 7 · gui-keys 7 · scan-gui 18 · obs-web 27 · dejavu 9. Per master and key file, standing rows
  by pattern: architecture.md e2e-web 4 node-pin 4 npm 9 playwright 5 · reg:architecture/build-system.md axum 1 e2e-web
  1 ts-plane 1 viola-ui 1 · reg:architecture/ci-cd-approach.md e2e-web 3 node-pin 5 npm 4 playwright 3 ·
  reg:architecture/project-directory-structure.md e2e-web 3 fixed-port 1 node-pin 2 npm 5 playwright 3 ts-plane 2
  viola-ui 1 · security-plan.md axum 1 e2e-web 4 node-pin 8 npm 9 playwright 3 scan-gui 4 · test-plan.md e2e-web 8
  fixed-port 3 gui-keys 1 node-pin 6 npm 12 obs-web 1 playwright 22 probes-13 2 scan-gui 3 viola-ui 1 ·
  reg:test-plan/5-command-implementation.md e2e-web 6 gui-keys 3 node-pin 2 npm 4 playwright 17 scan-gui 4 ·
  reg:test-plan/bootstrap-phases-derive-for-route-setup-project.md e2e-web 2 fixed-port 1 node-pin 1 npm 3 playwright 4
  ts-plane 1 · reg:test-plan/log-format.md e2e-web 2 obs-web 3 playwright 1 scan-gui 1 ·
  reg:test-plan/status-endpoint-shape.md gui-keys 2 · reg:test-plan/test-data-bootstrap.md fixed-port 1 ·
  reg:test-plan-contracts.toml gui-keys 1 · obs-plan.md axum 1 e2e-web 3 node-pin 3 npm 3 obs-web 13 playwright 6
  scan-gui 5 viola-ui 1 · reg:obs-plan/log-format-json-schema.md e2e-web 4 obs-web 5 playwright 2 scan-gui 1 ·
  reg:obs-plan/logging-stack.md axum 1 obs-web 1 · reg:obs-plan/heartbeat-ticks.md obs-web 2 ·
  reg:obs-plan/log-file-location.md obs-web 1 · reg:obs-plan/bootstrap-phases-derive-for-route-setup-project.md obs-web
  1 · a11y-plan.md e2e-web 6 npm 6 playwright 16 viola-ui 1 · reg:a11y-plan/ci-integration.md e2e-web 3 playwright 4 ·
  reg:a11y-plan/bootstrap-phases-derive-for-route-setup-project.md e2e-web 8 npm 1 playwright 4 viola-ui 1 ·
  design-system.md dejavu 9 fixed-port 1. The listing reaches its patterns' wording only; classes of hit that state a
  true claim sharing a token: the `npm` pattern's npm-shim → `claude.exe` resolution rows (architecture.md:43, :45, :76,
  :100, :281; security-plan.md:110, :212; test-plan.md:54, :301, :501, :556; obs-plan.md:76 — the product's resolution of
  the CLI's npm shim, unchanged); `dejavu`'s font-stack rows (design-system.md:258, :259, :263, :264, :266, :267 — the
  0.2.0 design inputs the restructure wrap kept); `fixed-port`'s design-system.md:247 (`127.0.0.1:47319` as ATIS anatomy
  Data, a 0.2.0 design input); `obs-plan.md` the `liveness` field of the `viola list` contract is not swept.)
  - architecture §Stack and Technologies — the Browser e2e row leaves; the CI/CD row's pre-push PATH and
    `NODE_PIN_VERSION` check narrow; the Code quality row drops `scripts/npm-audit.sh` → carried: **Harness / gate
    surface**, **Symbols / APIs** (pre-push), **Files** (deleted scripts); sites architecture.md:34, :35, :36.
  - architecture §Occupied Resources — the `NODE_PIN_*` data lines, the Node and Playwright install sites,
    `target/npm-audit/`, the `e2e-web/` outputs and `junit-playwright.xml` (the upload and the run-archive) leave →
    carried: **Harness / gate surface**, **Schema / config** (.gitignore), **Symbols / APIs** (`archive_sources`);
    sites architecture.md:332, :356, :384, :385, :386, :390.
  - architecture §Infrastructure Patterns → CI/CD approach, → Build system (the ts plane ends; the `crates/viola-ui/`
    tsconfig guard, the `axum/http2` ban and its probe leave; probes 13 → 12 with their rule; the tokio-confinement
    wording names `viola-mcp` alone), → Project directory structure (`e2e-web/`, `install-node.sh`, `npm-audit.sh`,
    the nextest `fixed-port` comment), and the 1.96 floor's axum mention → carried: **Harness / gate surface**,
    **Schema / config**, **Counts / qualifiers moved**, **Files**; sites the three key files above (ci-cd-approach 15,
    build-system 4, project-directory-structure 17 rows). The 1.96 floor's axum mention: build-system.md:8's
    `axum/http2` row (the axum pattern's 1 hit there); no other floor site carries axum by the pattern.
  - security-plan §Dependency Security — the npm graph, the Node pin, `npm-audit.sh`, the `supply-chain` npm step, the
    `npm-advisories` job and the Toolchains Node sentence leave; the `axum` `http2` ban leaves with its probe; the
    pre-push `tools` stage checks no Node → carried: **Harness / gate surface**, **Schema / config**, **Symbols / APIs**
    (pre-push); sites security-plan.md:286, :287, :295, :306, :307, :308, :313, :314, :331, :380, :395.
  - security-plan §Secret Management — the pre-push PATH loses `<home>/.local/viola-node/bin`; the scan's `?t=`,
    `viola_<port>=` and `ui/*.url` token forms leave (`inputs#I1`); the `cookie:` header stays → carried: **Symbols /
    APIs** (pre-push, secret-scan); sites security-plan.md:382, :417, :425, :426, :427.
  - test-plan §3 — `run`, `gate`, `pre-push`, Closed enums, Status endpoint shape, `cleanup`, the session record,
    `secret-scan` classes, Log format (kept equal to obs-plan's copy), Bootstrap → carried: **Symbols / APIs**,
    **Schema / config**, **Counts / qualifiers moved**; sites reg:test-plan/5-command-implementation.md (36 rows),
    status-endpoint-shape.md (2), log-format.md (7), bootstrap-phases-derive-for-route-setup-project.md (12),
    test-plan-contracts.toml (1), test-plan.md:449, :555.
  - test-plan §5 (the `fixed-port` group) and §9 (the browser leg, the npm Supply-chain steps, the cache sentence's
    npm/Playwright half, the Lint row's jsx-a11y lint home, `junit-<os>` one path, deny-probes 12) and §10–§11 (the
    Playwright retries and CI ban text) → carried: **Schema / config** (nextest), **Harness / gate surface**,
    **Counts / qualifiers moved**; sites test-plan.md:561, :1015, :1017, :1023, :1024, :1026, :1027, :1028, :1036, :1041,
    :1048, :1052, :1066, :1120, :1188, :1189, :1190; reg:test-plan/test-data-bootstrap.md:16.
  - obs-plan §3 (Log format JSON schema, Log file location `ui-<port>`, Logging stack's `viola_ui` / axum /
    tower_http / hyper rows and the `sse-closed` failure class, Heartbeat ticks' `liveness-changed` line), §5 (the
    liveness-transition metric row), §6 (Required fields, the catalog rows, `port`, `last-event-id`), §8 (the GUI token
    row, the viola-node PATH mention), §9 (Telemetry artifact handling, Step order item 1, the browser marker
    paragraph), §10 (the `ui` role), §11 (the GUI token wording) → carried: **Symbols / APIs** (product obs),
    **Schema / config** (both schemas), **Harness / gate surface**; sites obs-plan.md (35 rows) and its five key files
    (18 rows).
  - a11y-plan §1 → CI integration, §3 (CI integration key, Bootstrap phases), §9, §10, §11 → Strategy and CI — the
    browser a11y leg's text leaves; the tier `Standard (1)` is not touched → carried: **Harness / gate surface**,
    **Files**; sites a11y-plan.md (29 rows), reg:a11y-plan/ci-integration.md (7),
    reg:a11y-plan/bootstrap-phases-derive-for-route-setup-project.md (14).
  - design-system §Typography — the Rationale's headless-GUI-check clause, the "Assertions hold on the Linux fallback"
    CI-render and DejaVu-provisioning text and the stand-until note → carried: **Harness / gate surface** (no CI
    render step exists, before or after); sites design-system.md:243, :269, :271 (the stack rows stay, above).
  - matrix#v1-08 notes — dated 2026-10-16 (the ts plane has no input; the record stands as measured) → ledger-note —
    owner P7.3.
  - matrix#v1-23 notes — dated 2026-10-16 (the axum `http2` ban and its probe left, `12/12`; part (b)'s `13/13` stands
    as measured; part (c)'s `e2e-web/*` rows left the tree) → ledger-note — owner P7.3.
- **Coverage of new surfaces:** none — the chunk adds no surface; it removes the browser leg and the reserved web
  vocabulary.
  - `gate --require playwright` (a refusal reached by a removed value) → validation clap/closed-suite✓ · instrumentation
    n/a · PII n/a · tests unit (`parse_require_refuses_playwright`) + probe (gate entry 8) · a11y n/a · tokens n/a
  - `secret-scan` narrowed classes → validation n/a · instrumentation n/a · PII the twelve CLAUDE canaries + `cookie:`
    header + content canary + mode✓ · tests unit (`secret_scan_ignores_the_removed_gui_forms`, the five stock-needle
    tests on `canary-socket-value-2b9d`) · a11y n/a · tokens n/a

## Deviations from intent
- `e2e-web/` held six tracked files, not five (`stub/pipe.html`); all six left, with the ignored outputs
  (`node_modules/`, `pw.json`, `pw-junit.xml`, `test-results/`), so the removed `.gitignore` block strands nothing.
- `e2e-web/` was removed by the founder (his `! rm -r`, relayed by the operator), not by implement: the host's deny rule
  refuses a recursive `rm` and the auto-mode classifier refused the move into `target/e2e-home.disk/`. On the overseer's
  direction the block ran with it standing (the absence guard red on its six files alone), and the guard was re-fired
  green after the removal (`implement` run dir `journal.md`).
- Step 7: the pre-push `linux` keys are asserted `["run", "gate"]` (the plan wrote `["gate", "run"]`): the document
  keeps insertion order.
- Step 6: `pre_push_every_native_call_carries_only_home_and_path`'s unnamed `fake.calls.len() > 10` bound became
  `assert_eq!(fake.calls.len(), 9, …)`.
- Step 5: `gate_fails_failed_and_skipped_suites` plants a coverage summary at the floors, so the retarget to `coverage`
  adds no `llvm-cov-summary.json` breach.
- Step 9: `url_token_takes_the_t_parameter` left with `url_token`.
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 36 · listed 36 · recorded 0 (companion 0
  · mechanical 0 · in-intent 0 · widening 0) · absorbed 6 · excluded 49`).

## Decisions & corrections
- The operator, as the overseer, 2026-10-16 (relayed in the implement session): "Overseer: deletion is the founder's;
  the overseer has asked him to run it (6 tracked files + 19 MB of generated node_modules/pw output, inventoried). Run
  the block now, then re-fire the absence guard once e2e-web/ is gone; journal it." — followed.
- Guard / permission refusals this chunk: the host deny rule refuses a recursive `rm` (alone and in any compound); the
  auto-mode classifier refused moving a tracked repository directory into `target/e2e-home.disk/` (the handoff's
  standing move-aside rule covers scratch homes, not tracked source) — a tree deletion of a whole tracked directory is
  the founder's at the desk; the Bash guard refused a `cd` into a skill references directory (re-issued with absolute
  paths); the Bash guard (overseer1's G1, 2026-10-16T16:17Z) refuses a pipeline tool's stdout sent to a file — every
  tool ran bare, background outputs read whole with Read.
- Oracle literals written from a plan's text before the first run were wrong twice (a `> 10` call bound the plan did not
  name, and an insertion-ordered key list the plan wrote sorted): both fixed in one iteration after the first block red.
- The I1 forks (P4, the overseer's word, technical, founder-delegated): every GUI secret-scan form leaves (the
  `cookie:` header and CLAUDE canaries stay); the axum `http2` ban leaves with its probe.
- Sweep hazard: the `npm` token reaches the product's npm-shim → `claude.exe` resolution text (twelve master rows), and
  `liveness` reaches the `viola list` contract's own field — a removal sweep for the browser leg's npm or the obs
  vocabulary's `liveness-changed` must key on `npm ci|npm-audit|npm-advisories|package-lock` and
  `liveness-changed|\$defs.liveness`, never the bare word.

## Outcome
- Acceptance (re-asserted against the diff):
  - (arch) `e2e-web/`, `scripts/install-node.sh` and `scripts/npm-audit.sh` are gone; `git ls-files '*tsconfig.json'`
    after the pre-CI commit lists nothing → met.
  - (tests) No workflow, pre-push stage, harness verb, script or config runs, installs or checks `node` / `npm` / `npx`
    or the browser suite: the browser-and-Node absence guard reads `exit 1`, `no output`, after reading 145 hits at the
    base (P5 baseline) → met.
  - (tests) `gate --require playwright` exits 2 with `"detail":"unknown-suite"`; the `test` job's verdict step and the
    pre-push `linux-tests` gate both read `gate --require coverage,doctest` → met.
  - (security) The pre-push launcher carries exactly `HOME` and `PATH`, no `.local/viola-node` element, no `node` in
    `tools`; pinned by `pre_push_every_native_call_carries_only_home_and_path` and
    `pre_push_native_stages_run_tools_then_linux_tests_and_end_the_pass`, with
    `pre_push_native_child_sees_only_home_and_path` green → met.
  - (security) `cargo deny check` and the sole-root tokio loop pass; `deny-probes: 12/12 banned, control clean`; zizmor
    exits 0 → met.
  - (security) The secret scan keeps every `CLAUDE*` canary, the content canary, the `cookie:` header and the non-0600
    check; `?t=`, `viola_<port>=` and the `ui/*.url` token no longer report (`secret_scan_ignores_the_removed_gui_forms`)
    → met.
  - (obs) Both schemas hold no `ui` process value and none of the four events; `diag-line.v1.json` holds no `port`,
    `last-event-id` or `liveness` def; both keep `unevaluatedProperties: false`; `contract_diag_schema` and G4 read zero
    failures → met.
  - (obs) `ObsEvent::ALL` the fifteen and `ObsProcess::ALL` the four, pinned by literal oracles; `Targets` names no
    `viola_ui`; the leftover-vocabulary absence guard reads `exit 1`, `no output` → met.
  - (obs) In `ci.yml`, G2, G4 and `secret-scan` stay `if: always()`; every upload stays gated on
    `steps.secret-scan.outcome == 'success'`; `junit-<os>` carries the nextest JUnit alone → met (diff: only the browser
    pipe, the second path and the gate list changed in the `test` job).
  - (tests) Status has no `ui` / `api_sessions_equal_list`, cleanup no `port_free` / `url_file_removed`, the session
    record no `ui` / `cookie_file`; `harness_lifecycle` and the booted smoke green → met.
  - (tests) `.config/nextest.toml` has no `fixed-port` group; `windows-mutants.yml` keeps ten items with
    `contract_windows_mutation_scope` green → met.
  - (a11y) The CLI output-discipline and TUI boundary cases run unchanged inside the gated `coverage` suite on all three
    OSes, none `#[ignore]`d; the CI read is green → met.
  - (tests) Every gate green, `pre-push` included; CI on the final HEAD `verdict: green` → met.
  - No capability is claimed (`matrix.py show --chunk`: `claimed by 2026-10-16-remove-the-browser-leg: 0`) → met.
- Gates (implement's second whole firing, the outcome basis; trail
  `.andromeda/runs/2026-10-16T16-17-45-implement/gate-2026-10-16-remove-the-browser-leg.json`): `cargo fmt --all
  --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green · the Windows-target
  clippy (`CARGO_TARGET_DIR=target/wincheck …`) green · G1 `rg … instrument` green (exit 1, no output) · `bash
  scripts/lint-probes.sh` green · `bash scripts/agent-run.sh run --unit` green · `run --integration --filter
  'binary(contract_diag_schema) | …'` green · `gate --require playwright` green (exit 2, unknown-suite) · the
  browser-and-Node absence `git grep` green (exit 1, no output) · the leftover-vocabulary `git grep` green · the
  preservation `git diff --quiet b6bbbd68428a …` green · `cargo deny check` green · the sole-root tokio loop green · `bash
  scripts/deny-probes.sh` green (12/12) · `zizmor --format=json .github/workflows/` green · `cleanup --session
  p-rbl-smoke` green · `boot --session p-rbl-smoke --instance builder` green · `status --session p-rbl-smoke` green
  (lacks api_sessions_equal_list) · `bash scripts/g2-zero-panics.sh` green · `schema-check` green · `cleanup` green
  (processes_gone, endpoint_gone, lacks port_free) · `bash scripts/agent-run.sh pre-push` green · the three `leg =
  'operator'` entries (`gate.py hygiene`, the guarded push, `ci.py conclusion`) fired by the operator pass, recorded in
  `evidence/operator-pass.md`: hygiene clean · push green (`b6bbbd68→fac2d9de`) · `verdict: green` (15/15,
  `ci#38937051980`, attempt 1). Summary: `entries 25 · green 22 · red 0 · recorded 0 · timeout 0 · not-run 3`.
  Smoke: the plan's smoke entries ran in the block (boot path of the harness changed).
- watches: none.
- Outcome basis: the operator pass ran (Setup 4: `fac2d9d` on `b6bbbd6`); its final HEAD's CI run is
  `evidence/operator-pass.md`; implement's report `.andromeda/runs/2026-10-16T16-17-45-implement/outcome-2026-10-16-remove-the-browser-leg.md`
  (this window holds that run whole).
- Process hygiene: implement's census — the `p-rbl-smoke` supervisor, wrapper and fake agent (a leg, terminated: entry
  21 `processes_gone:true`, `endpoint_gone:true`) and the `pre-push` children (terminated); no process whose executable
  lies under this repository was running at implement P4 (host process list); the `viola-lab` prototype's `viola`
  processes are another tree's, left running.
## Addendum — resumed 2026-10-16
- inputs#I2 · message: the operator, relaying the founder word, 2026-10-16 (resume) · copy message · n/a — the founder's
  word («ок давай продолжать»): the wrap resumes at P2 Validate in this run dir, reusing this report and
  `fanout-results.md`; Validate also raises the expected amendments the agents missed, design-system §Typography among
  them, and the browser-test and reserved-obs text this chunk owns
  - bears on **Expected amendments (from plan)**: adds to it — every entry there is reconciled at Validate against the
    seven fan-out files, and an entry no proposal covers is raised there, §Typography's sites included
- inputs.py verify, re-fired: `inputs: 2 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 2 · uncited 1 · unparsed 0`; moved since the report: I2 entered (`UNCITED` before this block, cited
  by it); I1 unchanged (`n/a`)
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: b6bbbd68 (the parent of the oldest pre-CI commit fac2d9de) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — nothing added (lines removed only)
### .github/workflows/ci.yml — added 2 line(s) in 2 range(s)
added: 133 · 148
### .github/workflows/nightly.yml — nothing added (lines removed only)
### .github/workflows/windows-mutants.yml — added 1 line(s) in 1 range(s)
added: 53
### .gitignore — nothing added (lines removed only)
### Cargo.toml — added 1 line(s) in 1 range(s)
added: 194
### crates/viola-core/src/obs.rs — added 6 line(s) in 6 range(s)
added: 31 · 86 · 161 · 190 · 192 · 205
### crates/viola-e2e/src/bin/viola-harness.rs — nothing added (lines removed only)
### crates/viola-e2e/src/harness/boot.rs — nothing added (lines removed only)
### crates/viola-e2e/src/harness/cleanup.rs — added 2 line(s) in 1 range(s)
added: 70-71
### crates/viola-e2e/src/harness/gate.rs — added 14 line(s) in 6 range(s)
added: 14 · 26 · 270 · 276 · 280-282 · 415-421
  - 416-421 @418 «fn parse_require_refuses_playwright() {»
### crates/viola-e2e/src/harness/logs.rs — added 2 line(s) in 2 range(s)
added: 262 · 277
### crates/viola-e2e/src/harness/mod.rs — nothing added (lines removed only)
### crates/viola-e2e/src/harness/pre_push.rs — added 18 line(s) in 6 range(s)
added: 103 · 169 · 216-217 · 226 · 229-235 · 242-247
### crates/viola-e2e/src/harness/pre_push/linux.rs — added 20 line(s) in 10 range(s)
added: 42 · 102-103 · 175 · 186 · 193 · 212 · 214 · 218-226 · 366 · 394-395
### crates/viola-e2e/src/harness/run.rs — added 17 line(s) in 10 range(s)
added: 1 · 53-55 · 144-145 · 162-163 · 167 · 169-172 · 235 · 241 · 268 · 748
### crates/viola-e2e/src/harness/run/browser.rs — deleted
### crates/viola-e2e/src/harness/secret_scan.rs — added 23 line(s) in 15 range(s)
added: 16 · 70 · 86 · 114 · 276-277 · 279 · 288-289 · 292-294 · 303 · 306 · 313 · 331 · 338-342 · 418 · 420
### crates/viola-e2e/src/harness/status.rs — added 3 line(s) in 2 range(s)
added: 1-2 · 26
### deny-sync.toml — added 1 line(s) in 1 range(s)
added: 12
### deny.toml — added 1 line(s) in 1 range(s)
added: 1
### e2e-web/package-lock.json — deleted
### e2e-web/package.json — deleted
### e2e-web/playwright.config.ts — deleted
### e2e-web/stub/pipe.html — deleted
### e2e-web/tests/pipe-reachability.spec.ts — deleted
### e2e-web/tsconfig.json — deleted
### schemas/diag-detail.v1.json — added 2 line(s) in 2 range(s)
added: 14 · 16
### schemas/diag-line.v1.json — added 5 line(s) in 5 range(s)
added: 5 · 14 · 68 · 86 · 109
### scripts/agent-run.ps1 — added 2 line(s) in 2 range(s)
added: 10 · 29
### scripts/agent-run.sh — added 7 line(s) in 5 range(s)
added: 13-14 · 16-17 · 19 · 37 · 70
### scripts/deny-probes.sh — nothing added (lines removed only)
### scripts/install-node.sh — deleted
### scripts/npm-audit.sh — deleted
### src/obs.rs — added 16 line(s) in 8 range(s)
added: 21-22 · 53-56 · 74 · 128 · 366-370 · 378 · 387 · 395
### tests/contract_diag_schema.rs — added 1 line(s) in 1 range(s)
added: 20
