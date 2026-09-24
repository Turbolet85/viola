# test-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — harness builds in its own target dir
**Section:** §3 `boot` steps 1, 3, 5 · §3 `run` steps 1–3 (layer commands, globalSetup) · §3 `run` step 4 Test scope
**Change:**
- The harness's cargo work runs with `CARGO_TARGET_DIR=<root>/target/harness` and `--features viola/fake-agent`, and the CLI `<bin dir>` is `target/harness/debug`.
- The fake agent is copied from `<bin dir>`.
- JUnit is still read from `<root>/target/nextest/ci/junit.xml`.
- `viola-e2e` declares a no-op `fake-agent` feature for cargo-mutants' package-scoped runs.

**Why:** measured on Windows 2026-09-24. `os error 5` relinking the running `target/debug/viola-harness.exe`; `UnitDependencyInfoChanged` in cargo's fingerprint log (report Spec claims disproved 2; Deviations 1, 4). Sweep `target/<profile>|--features fake-agent|cargo build --workspace` over test-plan:
- amended: lines 515, 517, 519, 534, 537, 546, 557;
- no change:
  - 541 ("never a literal `target/<profile>`" still holds);
  - 555 (cargo-mutants passes the literal feature);
  - 558 (llvm-cov builds in its own target dir);
  - 1386 (the CI lint runs clippy directly, not through the harness);
  - 1452 (perf builds in `target/perf`).

## 2026-09-24-three-os-ci-headless-harness-skeleton — integration filterset, mutation diff, base and trigger
**Section:** §3 `run` step 1 (integration layer) · §3 `run` step 4 (Base, Diff) · §9 Pipeline structure (Mutation row)
**Change:**
- The integration layer is `kind(test)` until an E2E-prefixed binary exists.
- The mutation diff is the working tree plus untracked files from `merge-base(<base>, HEAD)`.
- The `mutants` job runs on push and pull_request, with base = PR base sha or else `github.event.before`, passed through `env:`.
- The `origin/main` fallback is recorded as never resolving on this project's single build branch.

**Why:** measured:
- nextest 0.9.133 rejects an unmatched `binary()` regex;
- the committed-only diff was 0 lines, against 10 309 working-tree lines;
- `git ls-remote --heads origin` shows only `build/viola-0.1.0`.

(Report Spec claims disproved 1, 3, 4; operator decision at phase P4.) Sweep `binary\(/\^\(path|<base>\.\.\.HEAD|merge-base HEAD origin/main|pull_request\.base\.sha` over test-plan:
- amended: lines 536, 553, 554, 1391;
- no change: 537 (the E2E layer keeps its `binary()` selector, which becomes valid once E2E binaries exist).

## 2026-09-24-three-os-ci-headless-harness-skeleton — interim supervisor, readiness, status and cleanup
**Section:** §3 preamble (Exit codes) · §3 `boot` step 5 + Readiness signal · §3 `run` step 2 (harness_session, harness cleanup meaning) · §3 `cleanup` step 1 + Verification · §3 `supervise` · §3 Status endpoint shape
**Change:**
- The supervisor is an ordinary child process and holds a stdin pipe per wrapper until `viola-pty` exists.
- Interim readiness: the role file holds `process-start` lines for `self` and `claude-child`, and both pids are alive.
- The status shape adds `instances[]`, and `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null` in the interim.
- The cleanup report adds `processes_gone`, and `endpoint_gone`, `port_free` and `url_file_removed` are `null` in the interim.
- The harness cleanup assertion now reads "no field false".
- A usage error carries `reason:"usage"` plus a `detail` code, and an unbuilt selector is a usage error.
- The key order `{"v","cmd","ok",…}` is held by serde_json `preserve_order`.

**Why:** report Harness / gate surface, Reverted facts (detach flags removed), Deviations 3 and 6, expected amendment 5, and the operator decision (the grammar grows per chunk). A §12 Decisions Log entry records the new closed values. Sweep `detached|every field \`true\`|endpoint_gone` over test-plan: lines 519, 541, 544, 589, 596, 611 and 612 amended; 0 remaining hits.

## 2026-09-24-three-os-ci-headless-harness-skeleton — nextest mutants profile and toolchain source
**Section:** §3 Bootstrap test-runner-install · §9 CI Integration (tool paragraph; Matrix builds → Language version)
**Change:**
- `[profile.mutants]` is `fail-fast = { max-fail = 1, terminate = "immediate" }` with a 15s×2 slow-timeout.
- rustfmt and clippy come from the `rust-toolchain.toml` components, installed by `rustup toolchain install`.
- The language version is the exact 1.98.1 pin.

**Why:** a caught mutant was graded Timeout under `fail-fast = true` (auto timeout 108 s, measured 2026-09-24); the chunk shipped the rustup step (report Deviation 5, Harness / gate surface; expected amendment 6). Sweep `dtolnay` over test-plan: lines 1398 and 1403 amended; 756 and 1393 no change (the nightly fuzz toolchain and the MSRV 1.96 job are separate toolchains).

## 2026-09-24-three-os-ci-headless-harness-skeleton — mutation gate prebuilds the root bins (post-commit CI fix)
**Section:** §3 `run` step 4 (Command)
**Change:** `run --mutants` first runs `cargo build --package viola --features fake-agent`, which fails with `reason:"build-failed"`, and then passes `--copy-target=true` to cargo-mutants, so the scratch tree carries the root `viola` / `viola-fake-agent` bins that the harness tests spawn.
**Why:** cargo-mutants 27.1 scopes the baseline to the packages the diff touches. A diff touching only `crates/viola-e2e` failed its baseline with `fake agent: NotFound`, exit 4, measured on the dev host 2026-09-24. `test_workspace` / `test_package` in `.cargo/mutants.toml` and `--test-workspace=true` did not widen that scope (measured). The operator chose prebuild + copy-target over moving tests or `--in-place` (founder-delegated, 2026-09-24). Cost: one `target/` copy per run, 2.9 GB on the dev host. Sweep `cargo mutants --workspace` over test-plan: the line-555 command was amended; 0 other hits. Leaf `.claude/docs/commands.md` re-derived.

## 2026-09-24-fake-agent-and-test-data-fixtures — mutation verdict for Rust-free diffs
**Section:** §3 `run` step 4 (Classification bullet added; Verdict scoped to the `counted` arm) · §3 `run` Output format (`mutants` object) · §3 Closed enums (`mutants.verdict`) · §10 Mutation gate · §12 Decisions Log (new entry)
**Change:**
- The harness classifies `chunk.diff` by its `diff --git` headers.
- A diff with no `.rs` path (an empty one included) never reaches cargo-mutants. It passes as `{"tested":0,"verdict":"no-rust-delta","diff":"target/agent-run/chunk.diff","files":N}`.
- A Rust delta deletes a stale `mutants.out/outcomes.json` first and reports `"verdict":"counted"`. With no fresh `outcomes.json` it is red.
- New closed value `counted` | `no-rust-delta`.
- §10's "tests-only diff = `{"tested":0}`" is retired.

**Why:** report Spec claims disproved 3: cargo-mutants 27.1.0 exits 0 on a Rust-free diff and leaves `mutants.out/` untouched. CI run 35973118026 (sha dc01bd9) read `outcomes-missing`, and the phase P5 baseline on the dev host read a stale `"tested":8`. Operator P1 constraint. Sweep over all seven masters (`"tested":0}`, `touches only tests`, `parse \`mutants.out/outcomes.json\``): 2 test-plan sites amended (:557 Verdict, :1445 §10); test-plan :553 (base-missing rationale) and obs-plan :1237 (artifact list) no change.

## 2026-09-24-fake-agent-and-test-data-fixtures — fake-agent contract as built, consumer-first
**Section:** §7 Fake agent · §7 Fixture hygiene · §7 seed table (recorded-payload row) · §3 `run` step 2 · §12
**Change:**
- Hooks are read from `<plugin-dir>/hooks/hooks.json`, not `plugin/` + `settings.json`. Only an absolute exec-form command runs, and matchers are not yet evaluated.
- Payload `<fixtures>/<cli-version>/<Event>.<variant>.json`, with only `prompt` set for UserPromptSubmit.
- Receipt kinds and fields listed; script schema `schemas/fake-script.v1.json`.
- Modes built vs deferred: `--vt100-panic-bytes`, `statusline-echo`, `agents --json` land with their consumers.
- The hygiene walk covers `fixtures/fake-scripts/*.json`, with the class-only checker. The `fixtures/claude` walk joins with the first recorded fixture.
- Only the sync root chain exists (`viola-test-*` homes); the `viola_e2e::fixtures` copy lands with its first consumer, and the root `stamped_home` is an interim no-stamp seam.

**Why:** report Symbols / APIs, Schema / config, Crates / modules; operator P4 "consumer-first, no shapes invented before a recorded fixture"; plan expected amendments. Sweep `plugin/\` and \`settings.json\``, `exists twice`, `In both copies`, `FAKE_CLAUDE_AGENTS_MODE`, `statusline-echo`, `vt100-panic-bytes` over all seven masters:
- :1324, :542 amended.
- :229, :349 no change (`viola run` rewriting its own plugin folder).
- :514, :1101, :1149, :1315 no change (sequencing: target state; owners pinned as CARRYs at this wrap's route-resolve).

## 2026-09-24-fake-agent-and-test-data-fixtures — fixture naming and `--fixtures` root (operator-resolved escalations)
**Section:** §2 File naming · §7 seed table · §3 `boot` step 5 · §6 Path 4 step 1 and scenario step 3
**Change:**
- Fixtures are named `<Event>.<variant>.json` with the CLI's PascalCase hook event name (`PreToolUse.ask-question.json`), with no mapping table.
- `boot` passes the parent `--fixtures <root>/fixtures/claude`, and the fake agent joins `<cli-version>`.
- `--plugin-dir` comes from `viola run` (architecture §Occupied Resources).

**Why:** the fan-out escalated two doc-vs-code shape conflicts. The operator (wrap P2) resolved them with "code wins where the doc invented a shape; no mapping tables". Sweep `hook-event>`, `pre-tool-use\.ask`, `fixtures/claude/<ver>` over all seven masters:
- 4 sites amended (:467, :1048, :1133, :1315 row).
- :519 amended (`--fixtures`).
- :828, :930, :954, :1296 no change (directory references to a version dir, not the argument).

## 2026-09-24-supply-chain-and-workflow-gates — Lint row from sync-crates.txt, Supply-chain stage, wrappers sentence retired
**Section:** §2 trigger map (Supply chain V9 row) · §9 Pipeline structure (Lint row + new Supply-chain row, paragraph after the table) · §9 Build failure conditions · §12 Test crate deviation
**Change:**
- The Lint row's `cargo check` reads one `-p` per crate in `scripts/sync-crates.txt`; an empty list fails.
- A new Supply-chain row covers the ubuntu job `supply-chain`: cargo deny, the sole-root `deny-sync.toml` tokio ban, `scripts/deny-probes.sh`, zizmor, and the JSON artifact `supply-chain`. It also covers the weekly `nightly.yml` advisories job.
- The §2 V9 location now names both stages plus `nightly.yml`.
- The least-privilege sentence now covers both workflows. The cargo-deny 0.20 CLI form (global `--config`, `check -c` rejected) is recorded as measured on 0.20.2.
- Failure conditions: the deny and zizmor findings move under a new Supply-chain bullet, which also adds the sole-root and probe failures and the nightly run.
- §12: `viola-e2e` is no longer "added to the cargo-deny tokio wrappers list". It is never a sole root of the tokio ban.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. The report's "Spec claims disproved" #2 falsified the wrappers mechanism. Its Harness/gate surface gives the new jobs. The fan-out's 5 D-tests-framework proposals were all applied as re-derived. Sweep: see architecture-amendments.md, same entry heading, where one sweep served every master. For this master, 5 sites were amended (:486, :1399, :1411, :1434, :1617). Hits at :98, :164, :363 and :396 were left unchanged, because they are still true or unrelated.
