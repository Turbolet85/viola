# test-plan — archived amendment originals

Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-09-27-wrapper-channel wrap — 18 re-worded · 0 pruned

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

## 2026-09-24-observability-gates — internal gate subcommands, exit-aware 10 s readiness, mutants kill below the floor, runner jq and pinned ripgrep, scan-gated uploads
**Section:** §3 `run` step 2 (root `booted_wrapper` readiness) · §3 Internal harness subcommands (new `schema-check`, `secret-scan`; `gate` CI-upload sentence; Closed enums) · §3 Bootstrap phases test-runner-install (`[profile.mutants]`) and ci-tool-install · §9 tool-install paragraph · §9 Test report format
**Change:**
- Root `booted_wrapper` readiness is bounded at 10 s (was 20 s) and exit-aware: it fails at once when `child.try_wait()` reports an exit. Harness `boot` keeps 20 s.
- `schema-check` and `secret-scan` are declared as internal subcommands, with their output shapes, reasons and closed class enum.
- `[profile.mutants]` slow-timeout is 5 s × 2, with a `package(viola-e2e)` override at 15 s × 2. The cargo-mutants auto-timeout floor is recorded as measured.
- jaq leaves CI installation; G2 uses the runner-provided, presence-checked `jq`.
- ripgrep 15.2.0 comes from `scripts/install-ripgrep.sh`.
- The `agent-run-<os>` upload and the "raw junit.xml is never uploaded" clause are retired. In their place are the scan-gated `harness-`, `diag-` and `junit-<os>` uploads and `secret-scan-<os>`.
**Why:** chunk 2026-09-24-observability-gates (report Changes: Symbols / APIs, Schema / config, Dev-tool versions, Harness / gate surface; Spec claims disproved 2). The `2834e4d` mutants red on run `35995290314` was two `wait_ready` consumers spinning to the 20 s bound under cargo-mutants' 20 s floor. Operator decisions at phase P4: runner jq; deadlines fixed by cause and by value.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `bounded at 20 s`: 0 hits after the apply;
  - `period = "15s"`: 1 hit, the amended `:749` override;
  - `jaq 3.1.1`: 2 hits, test `:606` (assertion form, no change) and the amended `:783`;
  - `agent-run-<os>` / `agent-run-${{`: 1 hit, the amended `:657` ("there is no unscanned `agent-run-<os>` upload");
  - `is never uploaded`: 0 hits.
- Other `20 s` sites kept: `:530` (harness boot), the fixture timeout derived from boot, and the "boot's 20 s / 10 s deadlines" line.
- Leaves: `.claude/rules/verification-harness.md` (internal-subcommand list; print-bans-landed wording) re-derived; `.claude/docs/commands.md:10` (jaq install) and `.claude/docs/stack.md:46` (jaq) re-derived; `tests-summary.md` needs no change.

## 2026-09-24-quality-gates — per-job gate, coverage JUnit and regex, two-leg mutation union, seeded fuzz replay, rustup MSRV
**Section:** §2 (Property-based row) · §3 (preamble usage details; `run` body, `--coverage`, `--fuzz-replay`, exit semantics, Output format; `gate`; Bootstrap `ci-tool-install`; Closed enums) · §6 (Property suite) · §9 (Coverage report, Mutation, MSRV, Fuzz replay and Quality gates rows; tool install paragraph; Matrix builds; Test report format) · §10 (Stack adjustments; Mutation gate) · §11 (CI ×2) · §12 (new entry)
**Change:**
- The coverage JUnit source is `target/nextest/ci/junit.xml`, not `target/llvm-cov-target/…`.
- The coverage ignore regex is separator-agnostic (`crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`, harness `COVERAGE_IGNORE`). This is not a widening.
- `run` gains `--leg` (per-leg `mutants-verdict-<leg>.json`, survivors deferred to the gate), a `detail` field, the llvm-cov failure codes, and `--fuzz-replay` on the `fuzz/rust-toolchain.toml` channel (`fuzz-linux-only` off Linux; `tool-missing`, `corpus-empty`).
- `gate` gains `--mutants-legs`, the union rule, fixed `detail` codes and usage `unknown-suite` / `invalid-leg`. The mutants legs defer their gate to `mutants-verdict`.
- §9: mutation is a two-leg matrix plus `mutants-verdict`; MSRV and fuzz use rustup (not dtolnay); `mutants.out/` is not uploaded; the fuzz-replay job and the nightly fuzz job are recorded.
- §6 / §2: `viola_name` is the eighth, pre-parser seed target in the separate `fuzz/` workspace.
- §12: one dated entry (union verdict, seed target, declined `concurrency:`, rustup toolchains, new closed values).
**Why:** chunk 2026-09-24-quality-gates. Research M6 (JUnit path) and M5 (the Windows regex) are measured; the cargo-mutants `#[cfg]` limitation explains CI run 36005608858's `file_mode` misses. Operator P4 decisions: the windows leg with a union verdict, and seeding the fuzz pipeline now. P5-approved leans: rustup, no `concurrency:`, `mutants.out/` removed.
**Sweep** (all 7 masters + CLAUDE.md, `.claude/rules/*`, `.claude/docs/**`, playbook, drift-base; control `grep -c dtolnay` on a planted line → 1):
- `dtolnay` 0 · `cargo +nightly fuzz` 0 · `runs only the weekly` 0 · `the integration pass` 0 · `single jobs by design` 0 · the fixed string `crates/viola-e2e|tests/support` 0 in masters.
- `llvm-cov-target` 1: this entry's own `:647` "never exists".
- `mutants\.out` / `outcomes\.json`: 14 master hits, all the local `run --mutants` verdict source (`:441, :556, :557, :565, :649, :1495, :1608, :1652, :1781, :1788`), all still true, no change; `:1457` and `:1807` are this pass's text.
- Leaves re-derived: `.claude/docs/commands.md` (run/gate/built-today lines, llvm-cov command, fuzz replay command, deny scope), `.claude/docs/tests-summary.md` (Mutation row), `.claude/docs/workflow.md` (PRs line, verdict sources), `.claude/rules/testing.md` (Running tests line), `.claude/rules/verification-harness.md` (`--leg`, runner seam).

## 2026-09-24-workspace-tree-and-code-graph-planes — Lint row disposed, release-check job, fuzz lock audit, jq consumers
**Section:** §1 (dependency and boundary policy entity; the traced-to note) · §2 (V9 row) · §3 Bootstrap (`ci-tool-install` jq bullet, `quality-gate-config-emit`) · §9 Pipeline structure (Lint, Supply-chain, Release build rows; tool-install paragraph) · §9 Build failure conditions · §10 Build failure conditions · §12 (new entry; the Initial entry's "No separate test crate" request marked retired)
**Change:**
- Lint row: the orphans gate `scripts/orphans-check.sh` (+ `--probe`) runs per lib/bin target.
- `--acyclic` is an on-demand review with its measured reason; the rmcp `cargo tree` assertion joins with the "MCP server for drivers" chunk.
- Supply-chain row: the `fuzz/Cargo.lock` advisories + sources step (from the repo root, into `deny-fuzz.json`); weekly advisories cover both lockfiles; the artifact is admissible by content.
- Release build row: the `release` job runs `scripts/release-check.sh --probe` then the default mode (`--locked --bin viola`, judged on its own artifact records); a plain exit-code job.
- Build failure conditions: the cycle and rmcp-release-graph conditions are retired; the per-target orphan, fuzz lockfile and release-build conditions are added.
- §3: the gate-as-last-step rule is scoped to suite jobs (`lint`, `supply-chain`, `release` are plain exit-code jobs); the gate list names orphans-check, the fuzz lock audit and release-check; the jq bullet lists the two new consumers with the runner-image versions read.
- §1 and §2 updated to match.
- §12 gains this chunk's entry, and the Initial entry's "needs updating" request is marked retired.
**Why:** chunk 2026-09-24-workspace-tree-and-code-graph-planes (report Changes: Symbols/APIs, Harness/gate surface, Dev-tool versions; Spec claims disproved 1-2). Measured at cargo-modules 0.27.0: `--acyclic` exits 1 on 3 of 4 targets, each a type ↔ its own inherent method, whatever the filters. `cargo tree … | grep -c rmcp` = 0 at HEAD. Operator P4 decisions 1 and 3.
**Sweep** (same pass `sweep.py`): test-plan hits after the apply:
- `acyclic` 4 hits, all this pass's text (:98, :1428, :1815, :1820);
- `cargo tree -e features` 3 hits, all new (:1428, :1816, :1821);
- `last step of every job` 1 hit, amended (:799, now scoped);
- `No separate test crate` 2 hits, amended (:164 note re-derived; :1655 marked retired);
- the jq `G2 uses the runner-provided` hit at :786 amended in place.

0 stale. Fanned 13 proposals (8 `dependent-of`), all applied, re-derived from the report. Leaves: `.claude/docs/commands.md` Lint/release lines (shared with the arch cascade); `.claude/rules/testing.md` and `verification-harness.md` checked, 0 hits on the swept claims (unchanged).

## 2026-09-24-epoch-1-cleanup — mutation leg streams its progress; an unviable swamp is red
**Section:** §2 (Mutation row) · §3 `run` step 4 (Base, Command, Verdict), Exit code semantics, `gate` mutants breach, Bootstrap `quality-gate-config-emit` · §9 (Mutation row, Build failure conditions) · §10 (Mutation gate, Build failure conditions) · §11 (Quality) · §12 (new entry)
**Change:**
- The `run --mutants` invocation gains `--caught --unviable --build-timeout-multiplier=5`. cargo-mutants' stdout streams live to the harness's stderr, so every outcome line reaches the CI step log as it happens.
- The counted verdict requires `unviable <= caught`. A run with more unviable than caught mutants is red with `failures[]` code `unviable-exceeds-caught` and `failed` = survivors + 1. Under `--leg` it is not deferred to the union.
- §10 retires "`unviable` mutants are reported but do not fail the gate": a few do not, and outnumbering the caught ones is red. The measured threshold rows are in the body.
- The `gate` union parenthetical is scoped to the union. The §2/§9/§10 failure lists, `quality-gate-config-emit` and the §11 counting ban name the new condition.
- §3 Base adds the force-push case: a replaced `github.event.before` is reachable from no ref, so the run is `base-missing` (run 36117447745). The remedy is a rewind to the chunk base, then a fast-forward.
- §12 records the decision, including the operator's DECLINE of a reduced partial-verdict upload for cancelled legs (a decision, not a deferral).
**Why:**
- The chunk's report, Changes → Harness/gate surface and Spec claims disproved 1–3.
- CI run 36118112104 windows read `8 caught, 135 unviable` as `ok:true`. A leaked supervisor locked `viola-harness.exe` (relink `os error 5`, reproduced locally).
- CI run 36046091888 windows was silent for 2 h 45 m while the output was captured.
- §10 sits under Founder Direction 1. This tightening was decided by the founder-delegated overseer on 2026-09-25 ("YES to making the leg red when unviable outcomes swamp the verdict … The test-plan §10 wording goes to the wrap reconcile as an amendment"), recorded here as the ratification.
- Green witness: run 36126924953 on d14f234.
**Sweep** (cascade step 2, `sweep.txt` in this wrap's run dir). Pattern `unviable|missed or timed-out|missed == 0|zero missed|only when no leg caught|red only when|missed.{0,20}timeout.{0,40}(fail|red)|copy-target|count \`missed\`` over the seven masters, `playbook.md`, `drift-base.md`, CLAUDE.md, `.claude/rules/*` and `.claude/docs/**`: 25 hits.
- test-plan: 19 hits.
  - 11 amended (:437, :555, :557, :563, :655, :796, :1434, :1468, :1496, :1529, :1609). They were the known-positive control, and the pattern found them.
  - 6 are this pass's new §12 text.
  - :566 (leg-verdict `outcome` shape) and :663 (closed enums): no change, since no new value.
  - :1798: no change. It is chunk 2026-09-24-quality-gates' historical §12 entry.
- architecture.md:515 and obs-plan.md:1253: cross-master restatements of the union as the only red path, amended in this pass (their own sidecars).
- Leaves re-derived: `.claude/docs/commands.md:31/:41`, `.claude/docs/tests-summary.md:42`, `.claude/rules/testing.md:48` (above its Session Additions).
- 0 hits in CLAUDE.md, playbook.md, drift-base.md and the curation homes.
- Fanned 12 proposals (11 `dependent-of`), all applied with their text re-derived from the report. 1 orchestrator-raised (the §3 Base force-push case, from the report's Cross-project CI facts).

## 2026-09-25-security-prerequisites — the `test-only-rust-delta` mutation verdict; the SQOS open and content hash pinned
**Section:** §3 `run` step 4 Classification; §3 `run` Output format (`mutants` object, leg file); §3 Closed enums; §6 Security control negatives → Windows client SQOS; §6 Contract suite; §10 Mutation gate; §12 (new `2026-09-25` entry).
**Change:**
- Every place that listed the verdicts now carries the third closed value `test-only-rust-delta`. It applies to a diff whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`, root or `crates/<member>/`), which never builds or runs cargo-mutants and names the diff, its file count and `rust_files`. A mixed diff stays `counted` and red `outcomes-missing` without a fresh `outcomes.json`.
- §6 records `tests/channel_sqos_open.rs` (recipe + no-SQOS control) ahead of the viola-client negative, and `tests/contract_content_hash.rs` in the Contract suite.
- §12 carries the decision, refining the chunk-2 ruling (:1781/:1790 history left as written).
**Why:**
- The chunk's report: Spec claims disproved #1, Harness / gate surface, Counts moved (the closed set goes from 2 to 3), Symbols.
- Operator ruling "option A" (fold the fix, 4 conditions).
- cargo-mutants 27.1.0 measurements: `No mutants to filter` over a tests-only diff; `--list-files` lists `src/` only.
- Witness: CI run 36138441784, both mutants legs `9 caught`.
**Sweep** (cascade step 2):
- `test files included|naming any \`\.rs\` path` → 0 after the apply. Control: 1 in `git show HEAD:.andromeda/test-plan.md` (:1496), amended.
- Two-value verdict lists (`` `counted`, `no-rust-delta` `` / `"counted"|"no-rust-delta"`):
  - :566 and :663 amended;
  - :1781 no change (the dated 2026-09-24 §12 entry, history, superseded by the new entry).
- 0 hits in the other six masters: obs-plan names no verdict value, as its detector confirmed.
- Leaves re-derived, all above `## Session Additions`:
  - `.claude/rules/testing.md:48`;
  - `.claude/rules/verification-harness.md:43`;
  - `.claude/docs/tests-summary.md:42`.
- 0 hits in CLAUDE.md, the curation homes, playbook and drift-base.
- Fanned 7 proposals (5 D-tests-obs-harness, 4 of them `dependent-of`; 2 D-tests-coverage), all applied with text re-derived from the report.

## 2026-09-25-pty-wrapper-on-windows — 11-name identity floor, no-EOF measured per OS, harness on outer PTYs, fake-agent receipts
**Section:** §1 inherited credentials entity · §3 boot step 5, run step 2 (`booted_wrapper`), cleanup step 1, `supervise` · §4 viola-agent-claude unit oracle · §6 E2 steps + verification · §6 Chaos (no-EOF mode, `.cmd` child) · §7 fake agent receipt kinds + `--exit-no-eof` · §11 Unit anti-pattern
**Change:**
- The 14-name S6 literal is replaced by the 11-name identity floor measured on the Windows host plus the prefix / persistent-set rule (§1, §4, §6 E2 steps, §11).
- E2 verification: persistent-set survival (Unix), `env_stripped_count` ≥ 12 / `env_stripped_known` / `env_kept`, 0 canaries; the `VIOLA_*` presence half re-pinned to "Instance state and start order" and the Unix fds-only half to "Wrapper channel" (their route CARRYs).
- No-EOF mode as measured in CI runs 36165685381 / 36166907442: output outlives the leader natively on ConPTY, on Linux only for a holder in its own process group, never on macOS; the held-output half is asserted on Windows + Linux. `--exit-no-eof` holds the child's console / PTY slave (not viola's stdout), writes `hold`, and uses its own process group on Unix.
- Harness: `supervise` owns an outer PTY per instance (interim stdin pipe retired), cleanup re-presses Ctrl-C every 500 ms, `booted_wrapper` runs over `OuterPty` and waits for the fake agent's `start` receipt.
- Fake-agent receipt kinds gain `size`, `cwd`, `hold`; `start` follows its `HostTerminal` guard. The `.cmd` child case landed in `tests/cli_program_resolution.rs`.
**Why:** chunk 2026-09-25-pty-wrapper-on-windows report Spec claims disproved 1-3, Harness / gate surface, Deviations 5-7, expected amendment 6; overseer direction 4 (macOS measurement wherever a master claims output outlives the session leader).
**Sweep:** patterns as the architecture entry of this chunk plus `grandchild|holds the inherited stdout|holds the PTY slave|exit-no-eof`: test-plan :86, :519, :542, :592, :615, :859, :1191, :1194, :1304, :1313, :1357, :1366, :1554 and :1558 ("the product's own strip list" → `IDENTITY_FLOOR`) amended; :735, :1345 ("statusline stdin piped") unrelated, no change. Expected amendment 7's test-plan half: no site (`zero[- ]viola|viola-originated|own bytes|byte for byte` → only :1139, a statusline marker). Leaves re-derived: `.claude/rules/testing.md:40`, `.claude/rules/verification-harness.md:27, :34`; `.claude/docs/tests-summary.md` recomputed, no change.

## 2026-09-26-ci-chunk-base-and-union-verdict — derived whole-chunk base, compiling-leg union, chunk.diff out of the scan
**Section:** §3 `run` step 4 Base · §3 `run` Output format (`mutants` object) · §3 `secret-scan` Scope · §3 `gate` union bullet and CI paragraph · §6 Error sanitization and secret scan (Canary) · §9 Mutation row · §9 Test report format (`harness-<os>`) · §10 Mutation gate · §12 new 2026-09-26 entry
**Change:**
- Base: the harness derives the whole-chunk base (the last master flip at or before the parent of the oldest `chore({marker}): operator pre-CI commit` of a pending chunk, HEAD when none; `HEAD^` on the wrap push; `merge-base HEAD origin/main` fallback; full sha or `base-missing`). `AGENT_RUN_CHUNK_BASE` is an explicit override for tests; CI passes only `LEG`, no `github.event` value. The unbounded-pickaxe hazard is stated as measured. The `/implement`-sets-it, event-base, `git cat-file -e` and force-push sentences are retired with the event base.
- Every `run --mutants` document form carries `"base":"<sha>"`; the leg file is unchanged.
- Union: each mutant judged only by the legs whose `#[cfg]`s compile its line (gate-side `syn` over the checked-out source; a leg dropped only on a cfg proven false; empty set → every leg); a miss on a compiling leg stays red. §10 carries the run 36165685381 replay witness.
- `secret-scan` Scope and §6 Canary: every file under `target/agent-run/` except exactly `target/agent-run/chunk.diff`; §3 CI paragraph and §9 Test report format: the `harness-<os>` upload excludes the same file.
**Why:** chunk 2026-09-26-ci-chunk-base-and-union-verdict report Symbols/APIs, Harness / gate surface, Spec claims disproved 1-2, Expected amendments (test-plan ×5); overseer wrap direction (the four test-plan amendments); founder ruling 2026-09-25 (V15 instance half).
**Sweep:** `cascade.py sweep` over 12 patterns (`github.event.before` · `pull_request.base.sha` · `sets AGENT_RUN_CHUNK_BASE` · `(e.g. AGENT_RUN_CHUNK_BASE)` · `the mutation gate's diff base, read by` · `no leg (reports caught|caught it)` · `red only (when|if) no leg` · `every file under target/agent-run/` · `the harness capture in target/agent-run/` · `target/agent-run/*` · `path: target/agent-run/` · `drop a push's|push's --in-diff`), every control fired at e4865446. test-plan rows: :553, :628, :655, :1265, :1439, :1455, :1501 amended; :1767 (2026-09-24 skeleton entry), :1803 (2026-09-25 cleanup entry) and :1811 (the declined-concurrency entry) are historical §12 entries, no change. A mechanism grep (`union|compiling leg|compiles it`) over §1–§11 found no other restatement. Leaves re-derived: `docs/tests-summary.md:42`, `docs/commands.md:31/:41/:68`, `docs/workflow.md:10`, `rules/testing.md:48`, `rules/verification-harness.md:43`. Fanned 10 proposals (D-tests-obs-harness, 9 `dependent-of`), all applied with text re-derived from the report.

## 2026-09-26-local-linux-pre-push-gate — `pre-push` local Linux gate, uncommitted-promotion base rule, pump resize baseline
**Section:** §3 `run` step 4 Base · §3 Internal harness subcommands (`pre-push`) · §3 Closed enums · §5 Module ↔ PTY · §9 tool-install paragraph · §10 Mutation gate · §12 Decisions Log `2026-09-27`
**Change:**
- Base: the `HEAD^` step applies only when the working tree's master holds no pending record HEAD's copy lacks; with an uncommitted promotion on top of the flip, the flip itself is the base (as measured: the old rule derived acd08c7 where CI derived a69c5ef).
- New internal subcommand `pre-push` (Windows host, WSL2 `Ubuntu`, `--exec /usr/bin/env -i`): stages `host · tools · sync · cache · linux-tests · linux-leg · windows-leg · union`, the synced history-carrying clone, the document shape and exits; its closed `cmd` / `reason` / `detail` / `stage` values in Closed enums.
- Module ↔ PTY: a resize landing between the spawn sizing and the pump's first look is propagated — the pump's baseline is the spawn size (forced-window test via the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam; 6/6 red before, 6/6 green after).
- §9: the WSL provisioning installs the same runners from ci.yml's `test`-job pins, so each tool keeps exactly one version source.
- §10: before the operator push the local `pre-push` runs the same two legs and union.
- §12: the `2026-09-27` entry (gate, base rule, folded red and its measured cause, witness).
**Why:** chunk 2026-09-26-local-linux-pre-push-gate report Symbols/APIs, Harness / gate surface, Spec claims disproved 1-2, Expected amendments 1-6, 14-15.
**Sweep:** `cascade.py sweep` (baseline a69c5efb) over 23 patterns — the retired base wording (`When that commit is HEAD`, `HEAD\^`, `wrap push`, `last (master )?flip`), the resize claim (`a resize is propagated`, `[Rr]esize`), the install claim (`exactly one version source`, rustup install phrasings, `rustup toolchain install`, `cargo install --locked`), the subcommand list (`[Ii]nternal (harness )?subcommand`, `Closed enums`), the env rule (`VIOLA_\*`, `every variable the product`, `never read by (the )?viola`, `not a configuration channel`, `never reads env`, `AGENT_RUN_`), the mutation union (`Mutation gate`, `--mutants-legs`, `operator (pass|push)`) and the registry/tree anchors; four more (`host_size`, a subcommand count, the `schema-check…secret-scan` list, `399`) never fired over the masters and were controlled by hand over the leaves. test-plan lines :553, :660-664 (under :611 Internal harness subcommands), :671 (under :665 Closed enums), :926, :1452, :1507 amended and :1890-1901 added (§12); :1878 is a dated §12 entry (history, still true for CI) — no change; every other test-plan row is a true statement or a cross-reference. Curation homes 0 · bases 0. Leaves re-derived: `rules/verification-harness.md` (:23 `pre-push`, :43 base clause), `rules/testing.md` (:48), `docs/tests-summary.md` (:42), `docs/commands.md` (:31, :32, :41, WSL provisioning lines), `docs/gotchas.md` (two entries), `docs/workflow.md` (before-the-push bullet). Full row list: `.andromeda/runs/2026-09-27T00-51-08-wrap/sweep-dispositions.md`.

## 2026-09-27-instance-state-and-start-order — staged readiness, liveness by process check, mutation runs keep no home, pre-push scratch
**Section:** §1 (`boot` readiness, critical path 1, PID file location) · §2 pyramid (Performance row) · §3 `boot` Readiness, exit-code grammar, `run` step 2, `run` step 4, `cleanup` step 6, Test data bootstrap (Cleanup), `pre-push` (stages, sync, document), interim list · §4 `viola-state` liveness · §6 Path 1 · §10 heartbeat flip row
**Change:**
- Readiness staged: the two `process-start` lines first, then the snapshot (`pid`, `started_at`, `child_pid`; `endpoint` from "Wrapper channel") and a heartbeat < 5 s (missing `<name>:snapshot` / `<name>:heartbeat`); `events.ndjson` lines 1–3 and the Path 1 third record / M6 witness join with "Hooks to normalised events". The root `booted_wrapper` counts starts newer than the boot, then the snapshot + heartbeat.
- Liveness: `classify(beat_age, same_process)` fed ages (no clock) — pid/start-time mismatch → `gone` at any age; same process 5.0 s live, 5.1 s stale.
- `run --mutants`: `AGENT_RUN_KEEP_HOMES=0` / `AGENT_RUN_KEEP_FAILED=0` on the `cargo mutants` command (CI's workflow-wide `=1` overridden there); fixture owner record + gone-owner sweep (pid + start time only).
- `pre-push`: Linux leg `TMPDIR=<distro home>/viola-pre-push-scratch` (wiped 0700 at `cache`); document `cache{…,scratch_bytes,…,scratch_bytes_after}`. `logs --kind` landed (no longer an unbuilt-selector example).
**Why:** chunk report Harness, Symbols (`classify`), Spec claims 2/5; Expected amendment 8. T17–T18 (crate-level `viola-state/tests/` round-trip suite) and T19–T20 (Path 1 `path_` E2E binary) rejected as Sequencing deferrals — pinned as CARRYs on "Self-healing state" and "The board: viola list" at route-resolve.
**Sweep:** rows :174, :526, :746, :1027 this pass's text; :529, :1775 amended (fold); :1335, :1406 true; :1742 (B1 ruling record) history — no change. Leaves re-derived: `rules/verification-harness.md` (:24, :29, :43, :44), `docs/tests-summary.md` (:20, :25), `docs/services/viola-state.md` (:38). Bind test-plan §3 ↔ obs-plan §3: consistent. Full rows: `runs/2026-09-27T06-12-23-wrap/cascade-sweep.md`.

## Registry migration (U35) — 2026-09-29

<!-- U35 · test-plan.md · ## 3. Test Harness Contract · sha256 a10383b97c932da6b9401eb2dfaad483a6fb3ee8d31ed598440f7cfdcb52aece -->

## 3. Test Harness Contract

Architecture committed to the harness pattern (verbatim from
upstream-context arch Test Harness Commitment): "- Development Style: agent-driven." This section specifies
the concrete contract.

**Harness implementation:** `scripts/agent-run.sh` (POSIX) and `scripts/agent-run.ps1` (PowerShell) are identical thin shims. They run `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forward its exit code and stdout unchanged. All logic lives once, in Rust, so both shells have byte-identical semantics on all 3 OSes.

- **Session record:** each harness session writes `target/agent-run/<session>/session.json` with `{home, instances:[{name, wrapper_pid, started_at, fake_args}], ui:{port, pid}|null, supervisor_pid, cookie_file}`. The default session is `default`, and `--session <id>` allows parallel sessions (one per Playwright test, per §3 `run` step 3).
- **Exit codes:** every harness command exits 0 on success, 1 on failure and 2 on usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}` (key order held by serde_json `preserve_order`). A usage error carries `reason:"usage"` and a `detail` from `invalid-session-id`, `session-exists`, `invalid-instance`, `arguments`, `no-supervise-spec` and `unknown-suite` (a `gate --require` value outside the nine suites). The grammar grows per chunk: a flag or selector whose surface is not built yet (`boot --ui`, `run --e2e`, …) is a usage error until its chunk lands, never a vacuous pass.

### 5-command implementation

**`boot`**: start the product for testing.
- Command body: `scripts/agent-run.sh boot [--session <id>] [--instance <name>[:<fake-agent-args>]]... [--ui] [--unstamped] [--cli-version <ver>] [--agents-mode recorded|oversize|malformed] [--statusline-echo]`. With no `--instance` flag, the instances are `overseer` and `builder` and `--ui` is implied, whatever other flags are given (for example Path 6's `--statusline-echo`). One or more `--instance` flags replace that default list, and the UI is then started only if `--ui` is given (Paths 2 and 7, E4, the Playwright session fixture; `--perf` boots without it). `--agents-mode` defaults to `recorded`. `--statusline-echo` runs before boot step 5. It configures the user statusline command as `<bin dir>/viola-fake-agent[.exe] statusline-echo`, appending to `<home>/fake/statusline.marker`, in the settings source that `run` reads, redirected into the session home. It is used by Path 6 and its `path6` test in `bay-steady-state.spec.ts`. That source is still a §12 open question. Until arch names it, `--statusline-echo` exits 2 with `reason:"statusline-source-unresolved"`, and Path 6's pass-through bullets are blocked. They are never written against a guessed source. `--agents-mode` sets `FAKE_CLAUDE_AGENTS_MODE` (§7) via `Command::env` on every child the supervisor spawns and on the `viola list` that `status` runs. It is recorded in `session.json` and `supervise.json`, so a spec can reproduce the E1 unwrapped row. Steps:
  1. `cargo build --workspace --features viola/fake-agent` with `CARGO_TARGET_DIR=<workspace root>/target/harness`. It never builds into the `target/` that the shim's `cargo run` uses: on Windows a running `target/debug/viola-harness.exe` cannot be relinked, and a workspace-wide build re-fingerprints it (measured 2026-09-24: `os error 5`, `UnitDependencyInfoChanged`). On failure it reports `reason:"build-failed"`, and no timeout applies to this step.
  2. Choose the home under the workspace's `target/e2e-home/` (obs-plan §9: CI's zero-panic G2 gate, the G4 schema gate, the secret scan and the `diag-<os>` upload all read `target/e2e-home/**`). Create a unique parent with `tempfile::Builder::new().prefix("viola-session-").tempdir_in("<workspace root>/target/e2e-home")?.keep()` and pass `<parent>/home`, which does **not** exist yet, as `--home`. viola then creates the home itself: 0700 on Unix, and on Windows the explicit protected user + SYSTEM DACL that security requires for a `--home` outside `%USERPROFILE%` (a hosted runner's workspace is outside it). A home pre-created by the harness would inherit the workspace ACL and fail the Windows strict-modes check.
  3. Copy `<bin dir>/viola-fake-agent[.exe]` (see step 5) to `target/agent-run/<session>/bin/claude[.exe]`, and put that dir first on a harness-scoped `PATH`, passed per child via `Command::env` and never `set_var`. It serves both `viola verify` and the `claude agents --json` stub.
  4. Unless `--unstamped` is given, run `viola verify --home <home>` against the fake agent. This requires exit 0 and a last stdout line matching `^stamped \S+  \d+ pass  0 fail$`. It is the only writer of `ledger/stamps.json`; see Decisions Log.
  5. Spawn the supervisor `viola-harness supervise --session <id>` as an ordinary child process (no detach flags; it outlives `boot` and stops only through `stop.request`). It owns one portable-pty (`=0.8.1`, via the `viola-pty` seam) outer PTY per instance — a master writer, the child and a drain thread — and runs `viola run <name> --home <home> -- <bin dir>/viola-fake-agent[.exe] --cli-version <ver> --fixtures <workspace root>/fixtures/claude --receipt <home>/fake/<name>.receipt.ndjson --control <home>/fake/<name>.control <fake-agent-args>`, where `viola` is also taken from `<bin dir>`. `--fixtures` names the parent dir: the fake agent joins `<cli-version>` itself, so replay follows `--cli-version` even under `--report-version`. `--plugin-dir` is not passed by the harness: `viola run` passes it to the child (architecture §Occupied Resources, Claude Code integration names). It drains every master on its own thread. `<bin dir>` is the directory boot step 3 copies from: `target/harness/debug` (boot step 1's target dir) for the CLI, the test executable's own target dir for `harness_session`, and `target/perf/release` for `--perf`. The harness never reads `CARGO_BIN_EXE_*`, because Cargo sets that only for the root package's own integration tests. `<workspace root>` is resolved from `cargo metadata` or `CARGO_MANIFEST_DIR/../..` and passed as an absolute path, because nextest runs `viola-e2e` tests with `crates/viola-e2e` as the cwd. The control file is created empty before spawn. Both rstest `booted_wrapper` copies pass the same `--control` path.
  6. With `--ui`, the supervisor also starts `viola ui --home <home> --port <p>` in its own outer PTY. `<p>` is taken by binding `127.0.0.1:0` and releasing it.
  7. Exchange the token: read `<home>/ui/<p>.url`, send `GET /?t=<token>` with `redirect(Policy::none())`, require 303 plus `Set-Cookie: viola_<p>=…; HttpOnly; SameSite=Strict`, and store the cookie in a session-scoped 0600 file. The cookie is never printed.
- **Embedded-asset freshness:** the page (Lit 3.3.3, no JS build step) is embedded into `viola` at compile time. Step 1 always runs the incremental `cargo build` before any process starts, so the embed is rebuilt whenever `crates/viola-ui` assets change. If embedding goes through a `build.rs`, it must emit `cargo:rerun-if-changed` for the asset dir. After UI readiness, `boot` also fetches every embedded `/assets/*` path and byte-compares it with the repo file. On any mismatch it exits 1 with `reason:"stale-embedded-assets"`.
- Readiness signal. `boot` polls with a bounded 100 ms interval against a deadline; this is a file-state probe, not a sleep-based synchronisation. Readiness is staged. First the instance's `diagnostics/run-<name>.ndjson` holds a `process-start` line for `subject:"self"` and one for `subject:"claude-child"`, and both pids are alive by pid + start time (missing: `<name>:run-process-start`, `<name>:child-process-start`); a `process-exit` for `subject:"self"` first is `run-exited`. Only then are the checks below read (missing: `<name>:snapshot`, `<name>:endpoint`, `<name>:heartbeat`), and once those hold, the start records (missing: `<name>:events`); each joins as its surface lands. Per instance, all of the following must hold:
  - `instances/<name>/snapshot.json` parses with `pid`, `started_at`, `child_pid` and `endpoint` in its `data` (no `endpoint`: `<name>:endpoint`)
  - `heartbeat` mtime is less than 5 s old
  - `events.ndjson` lines 1–3 are `kind:"wheel"` with `data.cause:"start"`, then `kind:"budget-gate"`, then `kind:"session-start"` with `source:"hook"` (the fake agent fires SessionStart since "Hooks to normalised events", and Path 1 asserts line 3; `boot` checks all three through `start_records`, its fourth readiness stage, because `supervise` hands every booted fake agent `--fixtures <root>/fixtures/claude`, the recorded set)

  For the UI, `GET /health` must return 200 `{"status":"ok"}` and `GET /ready` 200 `{"status":"ready"}`, with the Host header set to `127.0.0.1:<p>`.
- Exit code: 0 on ready within the timeout. Otherwise it exits 1 with `{"v":1,"cmd":"boot","ok":false,"reason":"build-failed"|"verify-failed"|"run-exited"|"readiness-timeout"|"ui-not-ready"|"ui-port-taken"|"stale-embedded-assets"|"token-exchange-failed", "instance":<name|null>, "exit_code":<int|null>, "missing":[<unmet readiness checks>]}`. When the reason is `run-exited`, `exit_code` is the wrapper's exit code verbatim (for example 1 for a `live` or `stale` name, or a tampered pinned copy). `token-exchange-failed` means boot step 7 got a status other than 303, or a `Set-Cookie` without `HttpOnly` or `SameSite=Strict`. `missing` then names the failed attribute, and the cookie value is never printed.
- Timeout: 20 s per instance after the build, and 10 s for the UI.

**`run`**: invoke test suites.
- Command body: `scripts/agent-run.sh run [--unit|--integration|--e2e|--browser|--mutants|--coverage|--perf|--fuzz-replay|--all] [--filter <nextest-filterset>] [--file <path>]... [--local-live]`. `--file` (repeatable) needs `--mutants` (clap `requires`): it scopes the mutation run to those sources, the inner fix-loop (step 4). `--coverage`, `--fuzz-replay`, `--browser` and `--mutants` never join the default selection. The default is `--all`, which runs steps 1 and 2 in order; step 3 runs only under `--browser` and step 4 only under `--mutants`:
  1. Unit + integration: `cargo nextest run --workspace --features viola/fake-agent --profile ci -E '<layer filterset>'`, then `cargo test --workspace --doc`, both with `CARGO_TARGET_DIR=<workspace root>/target/harness` (boot step 1). JUnit is still read from `<workspace root>/target/nextest/ci/junit.xml`: nextest's store is workspace-root-relative whatever the target dir (measured 2026-09-24). The layer filtersets are:
     - unit: `kind(lib) | kind(bin)`
     - integration: `kind(test)` until the first E2E-prefixed binary exists, then `kind(test) & !binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)` — a `binary()` regex that matches no binary is a nextest filterset parse error, not an empty set (cargo-nextest 0.9.133, measured 2026-09-24).
  2. Fake-agent E2E: `cargo nextest run --workspace --features viola/fake-agent --profile ci -E 'binary(/^(path|tui|mcp|http|sse|cross|chaos|contract)_/)'`. Every nextest test uses one boot model:
     - No test shells out to `agent-run` / `viola-harness`, so there is never a nested `cargo run` inside nextest.
     - Root `tests/` suites use the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`.
     - `crates/viola-e2e` scenarios use one of two fixtures, chosen by the scenario's §6 Cleanup line:
       - Cleanup "harness `cleanup`" (Paths 2, 3, 4, 6, 7, E3, E4): the rstest `harness_session` fixture. It calls the same library code as the `boot` / `cleanup` commands (`viola_e2e::harness::{boot, cleanup}`), and runs the supervisor as a fixture-owned thread instead of a spawned `supervise` process (until that fixture exists, the library `boot` spawns the `supervise` process from the given binary dir). `harness_session` always skips boot step 1, so no `cargo build` runs inside nextest. It passes `viola_e2e::harness::boot` the binary dir found from the test executable's own target dir (the assert_cmd `cargo_bin` lookup), never a literal `target/<profile>`, and boot step 3 copies the fake agent from that same dir. This is the same binary-dir parameter `--perf` uses (§10). As a result, the scenarios spawn the instrumented `viola` / `viola-fake-agent` under `--coverage`, and the plain debug binaries otherwise.
       - Cleanup "TempDir drop" or a test-owned outer PTY (Paths 1, 5, E1, E2, E5): the chain `home → fake_agent_path → stamped_home → booted_wrapper` from `viola_e2e::fixtures`. Root `tests/support/` cannot be imported by another package, and the root package must not dev-depend on the tokio-based `viola-e2e`, so the chain exists twice (the sync root copy and the `viola-e2e` copy) against one contract. In both copies `stamped_home` runs `viola verify` against the fake agent. Today only the sync root copy exists (`tests/support/home.rs`; homes under `target/e2e-home/viola-test-*`). The `viola_e2e::fixtures` copy lands with its first consumer among these scenarios. The root `stamped_home` runs `viola --home <h> verify -- <fake agent> --cli-version 2.1.283 --fixtures <root>/fixtures/claude` (the committed recorded set; `2.1.283` is the root chain's one literal, `tests/support/fake.rs` `RECORDED_CLI_VERSION`), and `stamped` is true only on exit 0 and a last line `stamped 2.1.283  <n> pass  0 fail`; anything else panics with the exit and match codes. `StampedHome::unstamped(TestHome)` (`stamped:false`, writes nothing) serves the tests that assert the unverified path. `booted_wrapper` drives the wrapper through a test-owned outer PTY (`tests/support/outer_pty.rs`; `Wrapper::boot/send/stop` in `tests/support/home.rs`, and `tests/run_cli.rs::run_viola`) and waits for `process-start` lines for `self` and `claude-child` and a fake-agent `start` receipt, each newer than the boot (a `gone` name's files are reused) — the receipt means its terminal is raw, so no key sent after it is swallowed — then for the snapshot (`pid`, `started_at`, `child_pid`) and a heartbeat under 5 s, bounded at 7 s — `WITHIN` in `tests/support/watch.rs`, shared by all 9 root waits on a child and strictly below the nextest `mutants` profile's 10 s kill, so a stuck wait fails the test itself. `Wrapper::boot` always passes `--cli-version 2.1.283` (`--fixtures` stays a per-test extra). `Wrapper::stop` / `stop_keep` wait for the wrapper's exit and then, when the snapshot recorded an `endpoint`, for that endpoint to be unconnectable (`wait_endpoint_gone`, the ninth wait; `unconnectable` is the harness `endpoint_gone` rule — on Windows a client connect reads NotFound, on Unix the socket path is gone). An exit code is not the endpoint gone: on Windows the stopped wrapper's own pipe took a hook's connect and write 25 ms after its `process-exit` line, as measured at ci#36532038635; why the pipe outlives the observed exit is recorded, not established. Each wait notes what it polls (codes, counts and booleans, read in the predicate's own short-circuit order) to `<temp dir>/viola-root-watch/<test>.<label>.report`, one line per change, kept on a failure or a runner kill and removed on a pass. The wait is exit-aware: when `child.try_wait()` reports that the wrapper exited first, it fails at once (`wrapper <name> exited before ready`) instead of waiting out the bound. The bound stays below cargo-mutants' 20 s auto-timeout floor, so a mutant that stops the wrapper from getting ready grades as caught, not Timeout. Harness `boot` keeps its 20 s per-instance bound.
     - In §6, a step written `agent-run boot --session <id> <flags>` means `harness_session` with those flags. A Cleanup written `agent-run cleanup --session <id>` means harness `cleanup` as defined below. The `<id>` there (`p2`, `p7`) is only a readable label. The real session id always follows the `<test name>-<label>-<pid>` rule below, so neither parallel tests nor two sessions opened by one test (Path 7) share a session.
     - "Harness `cleanup`" means the test calls `session.cleanup()` at the end and asserts the report: no field `false` (an interim `null` field is allowed), same shape as the `cleanup` command. `Drop` is only a best-effort fallback for panicking tests.
     - The session id is `<test name>-<label>-<pid>`, where `<label>` is the §6 `--session` label (`p2`, `p7`, `p7-stamped`) or `main` when the step gives none. Each session gets its own temp home and its own `target/agent-run/<session>/` dir, because a session record holds exactly one `home`. The endpoint hash includes the absolute home, so the default `overseer` / `builder` names never collide across parallel tests.
  3. Browser (`--browser` only): the harness first deletes `e2e-web/pw.json`, `e2e-web/pw-junit.xml` and `artifacts/junit-playwright.xml`, then spawns through its `Runner` seam, with fixed argv and no shell or npx: `npm ci` in `e2e-web/` (`npm.cmd` on Windows), a Chromium probe (`node -e`, exit 0 iff the locked `playwright-core`'s `chromium.executablePath()` exists), then `node node_modules/@playwright/test/cli.js test`. The suite `playwright` is built from `pw.json` `stats` (passed = `expected`, failed = `unexpected` + `flaky`, skipped = `skipped`), its failure names from `pw-junit.xml`, which is copied to `artifacts/junit-playwright.xml`; a red CLI exit over a clean report is still red (`playwright-exit-<n>`). The pipe stub's config has no `globalSetup`. When specs boot sessions (the Web test toolchain entry), a Playwright `globalSetup` runs once, in the main process, and only performs boot step 1 (`cargo build --workspace --features viola/fake-agent` into `target/harness`). It has no worker index, so it never boots a session.
     - Sessions are test-scoped. A test-scoped fixture runs `scripts/agent-run.sh boot --session pw-<spec>-<test id>-<workerIndex> <that test's --instance / --ui / --agents-mode flags>`, where `<test id>` is the scenario id in the test title (`path2`, `e1`) or the layout-state slug. On teardown it runs `scripts/agent-run.sh cleanup` with the same id. Both must return `ok:true`. No session is shared across tests, because scenario tests in one spec need different instances (Path 2 `mute` / `local`, Path 6's statusline command) and leave destructive state (paused budget, human wheel, 0770 home) behind.
     - Cross-runner rule for §6: every `Playwright:` verification bullet in a Rust `path_` or E-scenario is owned by a test in the spec that the scenario names, and the test title contains the scenario id (for example `path2`) so `--grep path2` selects it. A scenario that names no spec defaults to `bay-steady-state.spec.ts`: Paths 3, 4, 5 and 6, E1 and E3. E1's test boots with `--agents-mode recorded`. A spec releases the fake agent's gated script steps by appending to `<session home>/fake/<name>.control` (§7), the same way the Rust test does. That test boots its own session with the scenario's instance flags. It drives the scenario's CLI steps itself, running `viola <verb> --home <session home> --json` through `child_process` with the home read from `session.json`. In that same session it asserts both the CLI exit code and `--json` result and the DOM state for the same event. The DOM line is found by the unique text the test sent.
     - The Rust scenario owns every non-DOM bullet. No Rust test drives Chromium, and no spec observes a nextest-owned session.
     - The step runs on every OS, and only when `--browser` is selected (`--all` and the default never select it: they would make every local run need Node and Chromium). A failed `npm ci` (`failures:["npm-ci-exit-<n>"]`, or `-signal`) or a missing Chromium (`failures:["chromium-missing"]`) is a failure (`suite:"playwright"`, `failed` 1, `reason:"browser-missing"`, no `detail`), never a skip, and Playwright never runs after it. A `pw.json` absent or without `stats` after the run is `failures:["artifact-missing"]`.
  4. Mutation:
     - Base: the whole chunk, derived by the harness from the repository's own history, so every run of an operator pass, fix pushes included, mutates the whole chunk. `AGENT_RUN_CHUNK_BASE` (trimmed, non-empty) is an explicit override, kept for the harness's own tests; nothing in CI sets it. Otherwise the harness reads `.andromeda/master-route.md` from HEAD's tree, takes the marker of every `{marker} · pending · ` record, and bounds its search at the parent of the OLDEST commit in HEAD's history whose message carries `chore({marker}): operator pre-CI commit`, or at HEAD when there is none (no pending record, or a chunk not yet pushed). The base is the last commit at or before the bound that added or removed a `· complete ·` master line (`git log -1 -G ' · complete · ' <bound> -- .andromeda/master-route.md`), i.e. the last master flip. When that commit is HEAD itself and the working tree's `.andromeda/master-route.md` holds no pending record HEAD's copy lacks (the wrap push, whose own commit is the flip), the base is `HEAD^`, so that push reads its own delta, never an empty diff. When that commit is HEAD but the working tree holds such a record, a promoted chunk is still uncommitted on top of the flip (/implement and the pre-commit operator pass, right after every wrap): the flip itself is the base, the base CI derives once the pre-CI commit lands (as measured at chunk 2026-09-26-local-linux-pre-push-gate: the old rule derived acd08c7 there while CI derived a69c5ef; `chunk_base_of_an_uncommitted_promotion_is_the_flip_at_head`). With no flip in history the base falls back to `git merge-base HEAD origin/main` (which never resolves on this project's single build branch). The bound exists because an unbounded pickaxe moves mid-pass: a fix commit that edits a `complete` record matches `-G` too (measured at chunk 2026-09-26-ci-chunk-base-and-union-verdict: a scratch-repo pass and `chunk_base_holds_the_flip_across_a_pass`). The chosen revision is resolved to its full sha (`git rev-parse --verify --quiet <rev>^{commit}`); when none resolves, `run --mutants` exits 1 with `reason:"base-missing"`. It never writes an empty `chunk.diff`, which cargo-mutants would report as zero mutants, i.e. a false `"mutants":{"tested":0}` pass.
     - Diff: the working tree plus every untracked, non-ignored file, against `merge-base(<base>, HEAD)`, written to `target/agent-run/chunk.diff` (any stale diff is deleted first). /implement never commits, so the committed-only `git diff <base>...HEAD` would mutate nothing; on a committed tree with an ancestor base the two are equal.
     - Command: first `cargo build --package viola --features fake-agent` (the root package only: the harness tests spawn the root's `viola` and `viola-fake-agent`, cargo-mutants builds only the packages the diff touches, and a root-only build never relinks the running `viola-harness`; a failure is `reason:"build-failed"`), then `NEXTEST_PROFILE=mutants cargo mutants --workspace --features fake-agent --in-diff target/agent-run/chunk.diff --test-tool=nextest --copy-target=true --caught --unviable --build-timeout-multiplier=5`, so the scratch copy carries those bins. Both builds use the mutation run's own cargo target dir, `MUTANTS_TARGET` = `target/mutants`: the root pre-build runs with `CARGO_TARGET_DIR=<repo>/target/mutants` and `cargo mutants` with the relative `CARGO_TARGET_DIR=target/mutants` (it was removed from the environment before), so each copied tree builds every test binary inside its own copy. A copied default `target/` keeps the original tree's `CARGO_BIN_EXE_*` paths, so root integration tests drove the unmutated binary (as measured at chunk 2026-09-27-hooks-to-normalised-events `evidence/red-mutants-stale-test-binary.md`: a hand-applied mutant failed 4 tests in the repository and passed in the scratch). The command runs with `AGENT_RUN_KEEP_HOMES=0` and `AGENT_RUN_KEEP_FAILED=0`, overriding `ci.yml`'s workflow-wide `AGENT_RUN_KEEP_HOMES=1`: a mutation run keeps no test home (the kept-home gates belong to the test job), since every home carries a pinned copy of the viola binary. cargo-mutants' stdout goes live to the harness's stderr instead of being captured, and `--caught --unviable` prints every outcome line (repo-relative name plus build and test seconds). A stalled or cancelled `run --mutants` therefore names the mutant it stopped at in its own stderr. `--build-timeout-multiplier=5` bounds each mutant's build, which cargo-mutants leaves unbounded by default. As measured on CI run 36046091888 (windows, 2026-09-24): with the output captured, a stalled run printed nothing for 2 h 45 m. Without them a diff touching only `viola-e2e` fails its baseline (exit 4; measured 2026-09-24 on the dev host with a diff touching only `crates/viola-e2e`). `--copy-target` costs one copy of the target dir per run (2.9 GB of `target/` on the dev host, 2026-09-24). `--file <path>` (each one given) is passed beside `--in-diff`. On a Windows host (`HOST_SCRATCH`) the run first resolves the host mutation scratch `<repo parent>/viola-mutants-scratch` from the repository's absolute path and refuses unless its final component is exactly `viola-mutants-scratch` and the repository is neither it nor inside it (`reason:"scratch-refused"`, exit 1, never a fallback to `%TEMP%` or the repository). It then measures the scratch's bytes and removes and recreates it (`reason:"scratch-wipe-failed"` when the removal fails, a leaked process holding a file), and the command runs with `TMP`/`TEMP` = the scratch and `--output <scratch>`: cargo-mutants' temp copy follows `TMP`/`TEMP` through `std::env::temp_dir()` (as measured at chunk 2026-09-27-epoch-2-cleanup: the copy appeared in the scratch mid-run, none in `%TEMP%`), so no copy lands on C: and no `mutants.out/` in the repository. Off Windows nothing changes: the inherited temp dir and `mutants.out/` at the repository root.
     - Classification: before any build, the harness reads the `diff --git a/… b/…` headers of `chunk.diff`. A diff naming no `.rs` path (an empty diff included) never reaches cargo-mutants: 27.1.0 exits 0 on it and leaves `mutants.out/` untouched (measured 2026-09-24: CI run 35973118026 read `outcomes-missing`, and a dev host read a stale earlier run's counts). It passes with `"mutants":{"tested":0,"verdict":"no-rust-delta","diff":"target/agent-run/chunk.diff","files":N}`. A diff whose `.rs` paths are ALL test targets (`tests/`, `benches/` or `examples/` at the root or under `crates/<member>/`) never reaches cargo-mutants either: 27.1.0 mutates lib and bin targets only (`--list-files` over a package with `src/`, `tests/`, `benches/` and `examples/` files lists `src/lib.rs` alone), and `--in-diff` over such a diff prints `INFO No mutants to filter` and writes no `outcomes.json` (measured at chunk 2026-09-25-security-prerequisites). It passes with `"mutants":{"tested":0,"verdict":"test-only-rust-delta","diff":"target/agent-run/chunk.diff","files":N,"rust_files":[…]}`, building nothing. Any other diff naming a `.rs` path, a mixed one included, first deletes `mutants.out/outcomes.json` in the run's output dir (on a Windows host `<scratch>/mutants.out/`, the whole scratch wiped first), so only this invocation's file can be read, and reports `"verdict":"counted"` — or `"verdict":"scoped"` when `--file` was given. A counted run with no `outcomes.json` is red (`outcomes-missing`).
     - Verdict (the `counted` and `scoped` arms): parse the run's own `mutants.out/outcomes.json` (`<scratch>/mutants.out/` on a Windows host) and require `missed == 0 && timeout == 0 && unviable <= caught`. A run with more unviable than caught mutants tested almost nothing, because its mutant builds failed. It is red with the `failures[]` code `unviable-exceeds-caught`, and `failed` = survivors + 1. As measured on CI run 36118112104 (windows, 2026-09-25): a leaked supervisor locked a relinked `.exe` (`os error 5`), so 135 of 143 mutants were unviable, and the leg read `ok:true` under the earlier rule. The exit code alone is not trusted, because exit 3 masks exit 2. It is still checked: only exit 0, 2 or 3 can pass, and then the counts decide. Exit 1, 4 (baseline failing), 5 (diff does not match the tree), 6 (invalid diff) or 70 fails `run --mutants` with `reason:"mutants-exit-<code>"`. Those runs test no mutants, so their zero counts would otherwise read as a pass.
     - Test scope: cargo-mutants' default is kept (`--test-workspace` is not set), so each mutant is tested only by the tests of the package that owns the mutated file. A mutant in a `crates/viola-<area>` crate must be killed by that crate's own tests (§11 Test Strategy layer rule), and the `viola-e2e` scenarios never run under mutation. `fake-agent` is the root package's feature; `viola-e2e` also declares a no-op `fake-agent` feature because cargo-mutants passes `--features fake-agent` to package-scoped runs (a missing feature would surface as exit 4 and fail the run). The harness's own cargo calls name the root feature as `viola/fake-agent`.
- `--coverage` replaces the nextest invocations of steps 1+2 with `cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --lcov --output-path target/lcov.info --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` (a `--filter` passes as `-E`). The regex is the harness const `COVERAGE_IGNORE`: the same four trees, separator-agnostic, because llvm-cov reports Windows paths with `\`. That run reports as the single suite `coverage` (JUnit copied as `junit-coverage.xml`, see `gate`). Then `cargo llvm-cov report --json --summary-only` writes `target/agent-run/artifacts/llvm-cov-summary.json`. The suite's failure codes are `llvm-cov-exit-N` (a red exit over green tests, e.g. a floor breach) and `llvm-cov-summary-missing`. The `cargo test --workspace --doc` run from step 1 still follows it, as suite `doctest`. llvm-cov builds in its own target dir, so no `CARGO_TARGET_DIR` is set.
- `--perf` (named-only, in neither the default nor `--all`) runs the §10 perf arm — the hyperfine probe (`tool-missing` / `hyperfine` when absent), the `target/perf` release build, one booted perf session, the four timed rows, a zero-panic check and cleanup — as suite `perf` (6 passed on green, `artifact:"target/agent-run/artifacts"`). The deadline verdict belongs to `gate --require perf`, never to `run`.
- `--fuzz-replay` runs `cargo +<channel> fuzz run --fuzz-dir fuzz <target> fuzz/corpus/<target> -- -runs=0` for each `fuzz/fuzz_targets/<target>.rs`, on Linux only. `<channel>` is read from `fuzz/rust-toolchain.toml` (today `nightly-2026-09-20`), the one source of the fuzz toolchain, and `fuzz/` is a separate cargo workspace excluded from the root. Off Linux it exits 2 with `reason:"fuzz-linux-only"`. A failed `cargo fuzz --version` probe or a missing `fuzz/rust-toolchain.toml` exits 1 with `reason:"tool-missing"` and `detail` `cargo-fuzz` or `fuzz/rust-toolchain.toml`. No target at all exits 1 with `reason:"corpus-empty"`. The replay corpus is the committed `fuzz/corpus/<target>/`. Each target's corpus is seeded with the §7 recorded payloads its parser accepts, or with synthetic seeds where no recorded payload applies (`viola_name`: 18 synthetic names; `channel_frame`: 7 synthetic seeds; `hook_stdin`: 10 synthetic seeds), and every reproducer from `fuzz/artifacts/<target>/` is committed there together with its fix. Nothing is cached or carried over between CI runs. A missing or empty corpus dir fails that target with `reason:"corpus-empty"`, never a vacuous pass. Suite `fuzz-replay` has one entry per target: `passed` counts targets that exit 0 over a non-empty corpus, `failed` counts the rest, and `failures[]` names each failing target. The `nightly.yml` time-boxed run uploads `fuzz/artifacts/` as an artifact and fails the job on any crash.
- `--local-live` appends `viola verify` against the real `claude`, on the operator's host only. When the `CI` env var is set (presence only; the harness reads it and passes it to `run_with` as `ci`) it refuses to start: `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before any build, spawn or suite. Otherwise, after the selected suites, it runs `cargo build --workspace --features viola/fake-agent` into `target/harness`, then one `<harness bins>/viola --home target/e2e-home/viola-live-<pid>/home verify` (the program default `claude`, no `--`), both through the `run_with` runner seam, so the harness's own tests use a stand-in runner and never run the real CLI. Suite `local-live` passes 1 when verify exits 0, each of the six literal row ids names exactly one step line ending `  pass`, and the last line reads `stamped <version>  <n> pass  0 fail`; else it fails 1 with one code: `build`, `verify-exit-<n>`, `verify-exit-none` (no exit code) or `row-missing`. It claims no live product measurement beyond that (the H2 key-after-resize question belongs to "First live test and self-drive").
- Exit code semantics: 0 only if every selected suite passes, there are no missed or timed-out mutants, and unviable mutants do not outnumber caught ones. Non-zero otherwise. Survivors, `outcomes-missing` and `unviable-exceeds-caught` each make a `--mutants` run red.
- Output format: `{"v":1,"cmd":"run","ok":<bool>,"suites":[{"suite":"nextest-unit"|"nextest-integration"|"doctest"|"nextest-e2e"|"playwright"|"mutants"|"coverage"|"perf"|"fuzz-replay"|"local-live","passed":n,"failed":n,"skipped":n,"survived":n,"artifact":"<path>"}]}`.
  - Built from `target/nextest/ci/junit.xml`, `e2e-web/pw.json`, `mutants.out/outcomes.json` (the run's own output dir, `<scratch>/mutants.out/` on a Windows host; `survived` = missed + timeout; read only on the `counted` and `scoped` verdicts), `cargo llvm-cov report --json --summary-only`, and the hyperfine `--export-json` files.
  - A `--mutants` run adds a top-level `mutants` object: `{"tested":N,"verdict":"counted","base":"<sha>"}`, or `{"tested":0,"verdict":"no-rust-delta","base":"<sha>","diff":"target/agent-run/chunk.diff","files":N}` when the chunk diff names no `.rs` path, or `{"tested":0,"verdict":"test-only-rust-delta","base":"<sha>","diff":"target/agent-run/chunk.diff","files":N,"rust_files":[<repo-relative .rs paths>]}` when every `.rs` path is a test target (step 4; `rust_files` appears on that verdict only). `base` is the full sha of the base the run read (a commit id, never a path), so every run of a pass can be shown to read the same chunk. With `--file` the object is `{"tested":N,"verdict":"scoped","base":"<sha>","files":[<the given paths>]}`: survivors stay red as in a counted run. On a Windows host a counted or scoped object also carries `"scratch_bytes"`, the bytes the host scratch held before its wipe (a count, never the path).
  - The run document gains a top-level `"archived"` (after `mutants` in key order) whenever the run had a source to archive: the repo-relative `target/run-archive/<n>` holding the JUnit files this run's nextest, coverage or playwright suites wrote (the playwright one, `junit-playwright.xml`, sourced from `e2e-web/pw-junit.xml`, the sibling of the suite's `pw.json` artifact) and the `outcomes.json` its mutation arm read (`<n>` = the highest existing + 1; the newest 10 kept; gitignored, outside `target/agent-run/`, never uploaded). An absent source is skipped and named on stderr (`run-archive: skipped {name} (absent)`).
  - A document refused by a tool arm carries `"reason"` and, where one applies, `"detail"` (e.g. `tool-missing` + `cargo-fuzz`).
  - `skipped` counts only tests that were selected and then not run: JUnit `<skipped/>` entries (`#[ignore]`, Playwright `test.skip` / `test.fixme`). Tests excluded by the layer filterset (for example `kind(test)` binaries during `nextest-unit`) and tests compiled out by `#[cfg(windows)]` / `#[cfg(unix)]` are never counted, so the `gate` `suite-skipped` breach fires only on a real skip. The `doctest` suite has no JUnit. It is counted from the `test result: … N passed; M failed; K ignored` lines of `cargo test --workspace --doc`, with `ignored` as `skipped`, so doc examples that must not run use `no_run` or a `text` fence, never `ignore`.
  - Failing test names are listed in `suites[].failures[]`.
- Test selection:
  - one Rust test: `scripts/agent-run.sh run --e2e --filter 'test(/path2_send_confirms/)'` (a nextest filterset)
  - one browser test: `npx --no --prefix e2e-web playwright test --config e2e-web/playwright.config.ts --grep "cocked strip"` (from the repo root without `--config`, Playwright finds no config and runs on its defaults, as measured at chunk 2026-09-27-browser-verdict-reachability, evidence/operator-pass.md; `--no` never fetches a Playwright the lockfile lacks)
  - one file's mutants: `scripts/agent-run.sh run --mutants --file crates/viola-core/src/paste.rs` (repeatable; `verdict:"scoped"`, the inner fix-loop — the counted `run --mutants` stays the verdict)

**`status`**: query product state.
- Command body: `scripts/agent-run.sh status [--session <id>]`. It runs:
  - `viola list --json --home <home>`, with the harness `PATH` so the fake agent answers `claude agents --json`
  - if the UI is booted, `GET /ready` (ungated) and `GET /api/info` with the stored cookie (security: `/api/info` is behind the `viola_<port>` cookie like every `/api` route; without it the answer is 401)
  - `GET /api/sessions` with the stored cookie
- Polled fields:
  - per item: `name`, `wrapped`, `liveness`, `status`, `wheel`, `budget_paused`, `dialog_pending`, `cli_verified`
  - envelope: `skipped.{unknown_kinds,unknown_fields,torn_lines}` and `budget`
  - UI: `checks.{viola_home,event_tail,claude_agents}`
  - derived: `api_sessions_equal_list`, which is `/api/sessions.items` deep-equal to `list --json` `.ok.items`
- Exit code:
  - 0 when `ok:true` (`state:"ready"`).
  - 1 for `degraded` or `down`. The full Status endpoint shape document is still printed, so an agent waiting for a `stale` or 503 transition reads exit 1 as a state, not a harness fault.
  - 2 on a usage error, or when no `session.json` exists for the id: `{"v":1,"cmd":"status","ok":false,"reason":"unknown-session"}`.

**`cleanup`**: tear down.
- Command body: `scripts/agent-run.sh cleanup [--session <id>|--all]`. Steps:
  1. Create `target/agent-run/<session>/stop.request`. The supervisor writes `\x03` into each outer PTY and presses it again every 500 ms until the child exits, because a Ctrl-C written into a child that has not set raw mode yet is swallowed: the fake agent exits on Ctrl-C, and `viola ui` shuts down gracefully.
  2. The supervisor calls `child.wait()` on every process handle with a 10 s deadline, then `Child::kill()` on survivors. It writes `supervisor-exit.json` with each exit status and a `killed` flag.
  3. Cleanup confirms the supervisor pid is gone via sysinfo (pid + start time) and reads that report.
  4. For each instance, `viola last <name> --json --home <home>` must exit 21. The endpoint recorded in the snapshot must be gone: on Windows a `viola-channel` client connect fails with NotFound, and on Unix the `<per-user 0700 dir>/viola-<h12>.sock` path no longer exists (`$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` as the fallback; security IPC access control).
  5. `127.0.0.1:<port>` must be bindable, and `<home>/ui/<port>.url` must be absent.
  6. It removes the home dir with its `target/e2e-home/viola-session-*` parent, the cookie file and `session.json`. **In CI the home is kept:** `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1` workflow-wide (the `run --mutants` command alone overrides it to `0`), so step 6 removes only the cookie file and `session.json`, and the home stays under `target/e2e-home/` until obs-plan's gate steps have run (G2, G4, the secret scan and the scan-gated uploads, obs-plan §9 step order). The ephemeral runner then discards it with the job. `home_removed` reports `"kept"` in that mode, which is not a failure.
- Idempotency: cleanup MUST be idempotent. With no `session.json` it prints `{"v":1,"cmd":"cleanup","ok":true,"cleaned":[]}` and exits 0.
- Verification: reported in `{"cleaned":[…],"processes_gone":true,"endpoint_gone":true,"port_free":true,"url_file_removed":true,"home_removed":true,"killed":[<names force-killed>]}`. `processes_gone` covers the supervisor, every wrapper and every child, by pid + start time; survivors are force-killed first. `endpoint_gone` is reported from the wrapper channel on; `port_free` and `url_file_removed` are `null` until their surfaces exist. Any `false` field means exit 1. A force-kill is reported but does not by itself fail cleanup. It does fail the `url_file_removed` check for the UI, because that file is removed only on graceful shutdown.

**`logs`**: fetch product logs.
- Command body: `scripts/agent-run.sh logs [--session <id>] [--instance <name>] [--kind <event-kind>] [--process run|hook|mcp|ui|cli] [--after <byte offset>]`. It streams, as ndjson on stdout:
  - every `instances/<name>/events.ndjson` line, wrapped as `{"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}`
  - every per-process log line from `<home>/diagnostics/*.ndjson`, wrapped as `{"src":"diag","file":<basename>,"record":<line>}`
  - every content-bearing detail line from `<home>/instances/*/diagnostics/detail-*.ndjson` (obs-plan D-08), wrapped as `{"src":"diag","file":<basename>,"instance":<name>,"record":<line>}`. The added `instance` disambiguates, because every instance has its own `detail-<process>.ndjson` with the same basename. A panic line appears both in a role file and in its detail file, so panic counts use the role files only (`.file|startswith("detail-")|not`).

  Torn or unparseable lines are emitted as `{"src":…,"torn":true,"offset":n}` and never dropped. Assertions pipe into `jq -e` (jq 1.8.2) or jaq 3.1.1, for example `agent-run logs --instance builder --kind send-issued | jq -e '.record.data.cursor'`.
- Format: see Log format below.
- Retention window: the whole lifetime of the session home. Events are never truncated, and `cleanup` deletes the home, except in CI, where `AGENT_RUN_KEEP_HOMES=1` keeps it until obs-plan's gate steps have run (`cleanup` step 6). On CI failure the home's `diagnostics/` and `events.ndjson` are uploaded as an artifact, after the secret-scan test has passed on them. Every harness and rstest home lives under `target/e2e-home/` (boot step 2), which is the root obs-plan's CI gates and upload read.

**Internal harness subcommands** (implemented in `viola-harness`, forwarded unchanged by the shims, not part of the agent's 5-command surface):

- **`supervise --session <id>`**:
  - Spawned only by `boot` step 5 (the CLI or the library call) as an ordinary child process that outlives `boot`, never by an agent or directly by a test. `boot` hands it `target/agent-run/<session>/supervise.json` (`home`, instances with fake-agent args, `ui` flag and port).
  - It owns the outer PTYs from boot steps 5–6 and drains each master on its own thread, keeping each master writer until its child exits. It watches for `stop.request` with the same bounded 100 ms file-state probe as boot readiness, then runs cleanup steps 1–2.
  - It writes `supervisor-exit.json` as `{"v":1,"cmd":"supervise","ok":<bool>,"exits":[{"name":<instance|"ui">,"exit_code":<int|null>,"killed":<bool>}]}`. Its stdout is discarded; the JSON document is the file.
  - Exit 0 when no child needed `kill()`, 1 otherwise, 2 on usage error.
- **`ui-restart --session <id>`** (overseer fix pass 3, Z4): a real process control, not a fake API.
  - It asks the session's supervisor to stop its `viola ui` gracefully, then starts a new `viola ui` on the same home and port. The new process has a new token. `session.json` is updated with the new `ui.pid` and launch URL.
  - A page opened before the restart keeps its old cookie. Its next `/api/*` request and its SSE reconnect get the real 401 from the new process, which drives the 401 access-strip state (a11y-plan P5).
  - It prints one JSON document, `{"v":1,"cmd":"ui-restart","ok":<bool>,"pid":<new pid>}`. Exit 0, 1, or 2 on usage error.
- **`schema-check`** (the §6 Schema conformance check body behind obs-plan gate G4; CI step `id: schema-conformance`):
  - Walks `target/e2e-home/**`. A `*.ndjson` directly inside a `diagnostics/` dir is validated with jsonschema 0.57.0: home-level role files against `schemas/diag-line.v1.json`, `detail-*` files against `schemas/diag-detail.v1.json`.
  - A non-JSON line is counted as `torn`. A directory at such a path, which some tests create to force an I/O error, is counted as `skipped_non_file` and never read.
  - It prints `{"v":1,"cmd":"schema-check","ok":<bool>,"files":N,"lines":N,"torn":N,"skipped_non_file":N,"failures":[{"file","line","keyword"}]}`. `file` is relative to `target/e2e-home`, and no line content is ever printed.
  - Reasons: `empty-scope` (no diagnostics file) and `schema-unreadable`. Exit 0, 1, or 2 on usage error.
- **`secret-scan`** (the §6 Error sanitization and secret scan body run in CI before any upload; CI step `id: secret-scan`, obs-plan §9 step 3):
  - Scope: `target/e2e-home/**/diagnostics/*.ndjson` (detail files included), every file under `target/agent-run/` except exactly `target/agent-run/chunk.diff`, and `target/nextest/ci/junit.xml`. `run --mutants`' `chunk.diff` is repository source text by construction (it hits `?t=` whenever it touches the scan's own patterns, fresh or stale), and the `harness-<os>` upload excludes the same file, so it is never scanned and never uploaded. A `chunk.diff` anywhere else under the capture is still scanned.
  - Classes:
    - `claude-stripped`: the fixed `CLAUDE*` canaries the root tests plant;
    - `token-query`: `?t=`;
    - `cookie`: `cookie:`, case-insensitive, and `viola_<digits>=`;
    - `gui-token`: the `t` value of each home's `ui/*.url`;
    - `content-canary`: fails only in a home-level role file;
    - `mode`: Unix only, a diagnostics file that is not `0600`;
    - `unreadable`.
  - A contract test (`crates/viola-e2e/tests/scan_patterns.rs`) keeps the canary list single-sourced: every `canary-<kind>-value-<hex4>` literal in the root `tests/` and `src/` must be in the scan's table.
  - It prints `{"v":1,"cmd":"secret-scan","ok":<bool>,"files":N,"skipped_non_file":N,"mode_check":"unix"|"skipped-windows","hits":[{"file","line","offset","class"}]}`, plus `"report":"unwritten"` when the hit file cannot be written. It never prints the matched bytes.
  - It writes the same document to `target/secret-scan/hits.json` only when there are hits, and deletes a stale one on every run.
  - Reason: `empty-scope`. Exit 0, 1 on any hit, or 2 on usage error.
- **Browser-side controls** (the `e2e-web` Playwright fixtures; the same no-fake rule applies: the real `viola ui` answers every request, and no response is fabricated):
  - **Hold the tape:** the fixture holds the page's first `GET /api/events` with `page.route` and releases it with `route.continue()`. The request reaches the real `viola ui` unmodified, only later, so `TAPE connecting` is observable without a timeout. `route.fulfill` and `route.abort` stay banned.
  - **Unknown routes after render:** the fixture makes real requests from the page context through the page's own API client: `GET` on an unknown `/api/` route (404) and a non-GET method on `/api/sessions` (405). The real Problem responses drive the rack strips that appear after first render.
- **`gate --require <suite>[,<suite>…] [--artifacts <dir>]`** (default dir `target/agent-run/artifacts/`):
  - `--require` takes only the nine closed `suite` values other than `local-live`; an empty list or any other value (`a11y` included) is usage `unknown-suite`.
  - Inputs:
    - `junit-<suite>.xml`. `run` copies the invocation's `nextest/ci/junit.xml` here right after each nextest invocation, because nextest overwrites that path on every invocation. For unit, integration, e2e and coverage the source is `target/nextest/ci/junit.xml`: nextest's store stays workspace-root-relative under `cargo llvm-cov nextest` too, and `target/llvm-cov-target/nextest/ci/junit.xml` never exists (as measured at chunk 2026-09-24-quality-gates, research.md M6: llvm-cov 0.9.1, nextest 0.9.133). `run` deletes the source before each invocation. If the file is missing afterwards, that is an `artifact-missing` breach, never a copy of an earlier run's file. `e2e-web/pw-junit.xml` is copied as `junit-playwright.xml`.
    - `run-summary.json`. Every `run` also writes its stdout document here, merged by `suite` across invocations.
    - the `mutants` suite in `run-summary.json` (its `survived` = missed + timeout from the run's own `mutants.out/outcomes.json`, under the host scratch on a Windows host)
    - `llvm-cov-summary.json`, from `cargo llvm-cov report --json --summary-only`
    - `perf-<hook>.json`, from hyperfine `--export-json`
  - A breach is any of:
    - a `--require`d suite is absent from `run-summary.json`, has `failed > 0`, or has `skipped > 0` (skips are never allowed for `playwright`)
    - coverage below §10, each floor its own breach
    - the `mutants` suite's `survived > 0`.
    - a required perf JSON that is missing, unreadable or whose `.results[0].max` is at or over its §10 gate. The four rows `perf-session-start.json`, `perf-user-prompt-submit.json`, `perf-stop.json` and `perf-session-end.json` (the one const `run::PERF_ROWS`) are required by name, each absent one its own `artifact-missing` breach naming that file; every present `perf-*.json` is judged `.results[0].max < 1.0` (`SPINE_DEADLINE_S`). A missing artifact is a breach, never a pass.
  - Output: `{"v":1,"cmd":"gate","ok":<bool>,"breaches":[{"gate":"suite-missing"|"suite-failed"|"suite-skipped"|"coverage"|"mutants"|"perf"|"artifact-missing","suite":<name|null>,"detail":<string>}]}`. `detail` is a fixed code or a count, never file content: `absent`, `no-run-summary`, `failed N`, `skipped N`, `survived N`, `lines 84.99 < 85`, `<metric> unreadable`, `junit-<suite>.xml`, `llvm-cov-summary.json`, `perf-<hook>.json` (the absent row's file) or `<file> max X >= 1`.
  - Exit 0 with no breaches, 1 on any breach, 2 on usage error.
  - CI runs `gate` as the last step of every test-running job, with `--require` set to exactly the suites that job ran (`coverage,doctest,playwright` in the per-OS `test` job). No CI job gates `mutants`. The per-job lists live in `ci.yml`. In the per-OS `test` job, `target/agent-run/` (including `artifacts/`, minus `target/agent-run/chunk.diff`) leaves CI only in obs-plan §9's scan-gated `harness-${{ matrix.os }}` artifact (`path` `target/agent-run/` plus `!target/agent-run/chunk.diff`; `if: failure() && steps.secret-scan.outcome == 'success'`). There is no unscanned `agent-run-<os>` upload.
- **`pre-push`** (the operator pass's local Linux gate; Windows host only, forwarded by both shims, not an agent command — the 5-command surface is unchanged):
  - Runs from the Windows host against WSL2 distro `Ubuntu`, every call `wsl.exe -d Ubuntu [--cd D] --exec /usr/bin/env -i HOME=… PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin …` (every PATH component a constant under the distro home or a system dir, carrying no host value; never the `--` form, which re-parses argv through the distro shell; no inherited environment, so no `CLAUDE*` value and no Windows PATH crosses).
  - Stages, fail-fast in order, the document naming the one it reached: `host` (off Windows → exit 2 `pre-push-windows-only`) · `tools` (the distro, `cc`, `rustc` / cargo-nextest / cargo-mutants / cargo-llvm-cov at the pins read from `rust-toolchain.toml` and the §9 `test`-job `tool:` line, then `node --version` = `v` + ci.yml's workflow `NODE_PIN_VERSION`) · `sync` · `cache` · `linux-tests` (`run --coverage`, then `run --browser` (a red stops before the gate), then `gate --require coverage,doctest,playwright` in the clone) · `vm-release` (once the ubuntu verdict is on the host, `wsl.exe --terminate Ubuntu`, so the idle VM's memory returns to the host before the host builds) · `windows-tests` (`run --coverage` then `gate --require coverage,doctest` on the host, no browser suite), the last stage: `ok:true` when it is green. No stage runs mutation (retired 2026-09-28); `tools` keeps the cargo-mutants pin because the distro's `run --coverage` runs the real-cargo-mutants harness tests. The host stages run with `CARGO_BUILD_JOBS=16`, set by the harness.
  - Sync: a Linux-filesystem clone `~/viola-pre-push` carrying the history `run` step 4 Base reads; per run a binary patch of the working tree (tracked, untracked non-ignored, deletions) built through a temporary index under `target/pre-push/` (the real index untouched), applied after `fetch HEAD` → `reset --hard <sha>` → `clean -fdq`; the clone's `write-tree` must equal the Windows tree id (`sync-mismatch`). The clone's `target/` above 40 GiB is removed before the run (the `cache` stage). Every WSL call is `env -i HOME=… PATH=…` with no further assignment.
  - It prints `{"v":1,"cmd":"pre-push","ok":<bool>,"reason"?,"detail"?,"stage",sync{head,tree,files,ms},cache{bytes,cap,cleaned,bytes_after},linux{run,browser,gate},vm{terminated,free_kib_before,free_kib_after},windows{run,gate}}` — codes, counts and repo-relative names only, never a clone or home path. Exit 0 when every stage is green, 1 on a red stage or a stop reason, 2 on `pre-push-windows-only` or usage.
- **Closed enums:**
  - `run` `suite`: the ten values in `run` Output format (`local-live` only under `--local-live`, by the verify-stamped-test-homes-and-harness chunk). `gate` `--require` takes the other nine. `local-live` failure codes: `build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing`.
  - `boot` readiness missing codes: `<name>:run-process-start`, `<name>:child-process-start`, `<name>:snapshot`, `<name>:endpoint`, `<name>:heartbeat`, `<name>:events` (the start records, by the verify-stamped-test-homes-and-harness chunk).
  - `status.last_error`: `null`, `ready-503`, `list-exit-<code>`, `sessions-<http status>`, `sessions-mismatch`, `ui-unreachable`.
  - `run --mutants` `mutants.verdict`: `counted`, `scoped` (`--file`), `no-rust-delta`, `test-only-rust-delta`.
  - `run` `reason` added by the quality-gates chunk: `fuzz-linux-only` (exit 2), `tool-missing`, `corpus-empty`; by the Epoch 2 cleanup chunk: `scratch-refused`, `scratch-wipe-failed` (the Windows-host mutation scratch); by the browser-verdict-reachability chunk: `browser-missing` (a failed `npm ci` or an absent Chromium; no `detail`); by the verify-stamped-test-homes-and-harness chunk: `live-in-ci` (exit 2). Usage `detail`: `unknown-suite`.
  - `schema-check` `reason`: `empty-scope`, `schema-unreadable`. `secret-scan` `reason`: `empty-scope`. `secret-scan` hit `class`: `claude-stripped`, `token-query`, `cookie`, `gui-token`, `content-canary`, `mode`, `unreadable`.
  - `pre-push`: `cmd` `pre-push`. `reason`: `pre-push-windows-only` (exit 2), `tool-missing` (reused), `tool-pin-mismatch`, `sync-failed`, `sync-mismatch`, `linux-document-unreadable`. `detail`: `wsl-distro-ubuntu`, `cc`, `rustc`, `cargo-nextest`, `cargo-mutants`, `cargo-llvm-cov`, `node` (absent → `tool-missing`, off-pin → `tool-pin-mismatch`), `pins-unreadable`, `source-path`, `patch`, `clone`, `fetch`, `reset`, `clean`, `apply`, `cache`, and `run` / `gate` (the Linux harness command whose document was unreadable; an unreadable `run --browser` document is `run` too). `stage`: `host`, `tools`, `sync`, `cache`, `linux-tests`, `vm-release`, `windows-tests`.
  - A new value needs a Decisions Log entry.

### Status endpoint shape

This is the `agent-run status` document. Its nested `list` and `ui` members are verbatim product shapes from the arch GUI HTTP contract (see Section 1, Status endpoint shape).

```json
{
  "v": 1, "cmd": "status", "ok": true,
  "state": "ready | degraded | down",
  "pid": "<viola ui pid from /api/info, or null without --ui>",
  "uptime_ms": "<now - /api/info.started_at, or null>",
  "last_error": "null | <harness code, e.g. ready-503, list-exit-21, sessions-mismatch>",
  "list": "<viola list --json document>",
  "ui": { "ready": "<GET /ready body>", "sessions": "<GET /api/sessions body>" },
  "api_sessions_equal_list": true,
  "instances": [{ "name": "<ViolaName>", "wrapper_pid": "<pid>", "alive": "<pid + start-time liveness>" }]
}
```

Interim, while `viola list` / `viola ui` are unbuilt: `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`. `state` is `ready` when every booted instance has `liveness:"live"` and `/ready` is 200. It is `degraded` when any item is `stale`, `/ready` is 503, or `api_sessions_equal_list` is false. It is `down` when `list` exits non-zero. `ok` is true only for `ready`. The agent polls this document through the `status` command to verify state transitions during E2E scenarios.

### Log format

- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing codes-only lines to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`. Content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (obs-plan D-08). Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`; null for an id-less `hook.event` notification and on a response to a frame that yielded no id (`error_code` -32700 or -32600)
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null. A dialog `hook-invoked` may still be null (fail-open before `dialog_id` is known, obs-plan D-07), and so may a `hook-decision` that carries `detail`
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`. A send refused before `send-issued` (`human-typing`, `budget-paused`, `input-not-ready`, `turn-running`) has no arch cursor, so its wrapper-side `send-refused` carries `corr` = the target's `events.ndjson` end offset at the moment of refusal, a log join key only, never returned to the caller (obs-plan D-28). Wrapper-side `send-*` lines also carry `conn` and `rpc_id`, the originating `send` request's connection and JSON-RPC `id`, so `(conn, rpc_id)` joins them to that call's `channel-*` lines (obs-plan D-30). The client-side `send-refused{side:"client"}` has `corr` null.
    - `schemas/diag-line.v1.json` requires `corr` exactly where the rules above always define it: `dialog-raised`, `dialog-answered`, `release-from-driver`; `channel-request` unless `method` is `hook.event`; `channel-response` unless `error_code` ∈ {-32700, -32600}; `send-*` when `side` is `wrapper`; `hook-decision` when `hook_event` ∈ {`pre-tool-use`, `permission-request`} and no `detail`. `hook-invoked` never requires it.
    - `release-from-driver`: `corr` = the JSON-RPC request `id` (obs-plan D-01; the wrapper's `-32602` refusal of a `release` that carries `from`)
    - `process-start`, `process-exit`, `http-request`, `panic`: `corr` = null
    - `liveness-changed`, `state-recovered`, `sse-opened`, `sse-closed`, `parse-rejected`: `corr` = null (obs-plan D-02 … D-05)
  - `process` (`run|hook|mcp|ui|cli`; `cli` is a short-lived verb with a resolved instance, writing `cli-<name>.ndjson`, obs-plan D-06) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).
  - **Null encoding:** a null `corr` or `instance` is written as key **absence** (tracing has no null field value; obs-plan D-12). The harness and every `jq` / jaq assertion treat an absent key and `null` as the same value (`.corr == null` holds for both), and obs-plan's `schemas/diag-line.v1.json` rejects a literal `null`.

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Harness-side a11y rows (not part of this `event` enum):** `event:"a11y-violation"` is not a value of the product `event` enum above, has no `ObsEvent` variant, and is never emitted by a product process.
  - The a11y Playwright fixture writes one row per failing check into `e2e-web/test-results/a11y/*.ndjson`.
  - Rows follow a tests-owned schema of their own, `e2e-web/schemas/a11y-row.v1.json`. It reuses the diag-line field names without renaming any: `event` is const `"a11y-violation"`, `process` is const `"ui"`, `corr` and `instance` are absent, and the a11y plan's additive fields (a11y-plan §3 Structured violation JSON schema) are declared there.
  - The same schema-conformance check body that backs obs-plan G4 validates these files against that schema. G2, G4 and `schemas/diag-line.v1.json` never read them.
  - This revises the fix-pass-2 Y3 entry (overseer fix pass 3, Z7).
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain the GUI token, a `Cookie` header, `?t=`, or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.

### PID file

- **Location:** N/A as a product pid file, per test-scope Sec 3. Product pids are in `<home>/instances/<name>/snapshot.json` (`pid`, `started_at`, `child_pid`) with liveness in `<home>/instances/<name>/heartbeat`. The UI pid comes from `GET /api/info` `pid` and `<home>/ui/<port>.url`. The harness's own process record is `target/agent-run/<session>/session.json` (`supervisor_pid`, `wrapper_pid`, `ui.pid`).
- **Lifecycle:**
  - `session.json` is written by `boot` once readiness passes.
  - It is read by `status`, `logs` and `cleanup`.
  - Kill targets are always verified by pid + start time via sysinfo 0.39.6 before signalling, so a reused pid is never killed.
  - `session.json` is removed by `cleanup`.

### Test data bootstrap

- **Strategy:** self-bootstrapping (no developer-seeded data)
- **Mechanism:**
  - a tempfile 3.27.0 home per test or session
  - rstest 0.27.0 `#[fixture]` chain `home → fake_agent_path → stamped_home → booted_wrapper`
  - fake-agent replay of `fixtures/claude/<cli-version>/`
  - `viola verify --home <home>` against the fake agent for stamps
  - generated statusline stdin piped into `viola hook statusline` for `budget.json`
  - proptest 1.11.0 strategies with committed `proptest-regressions/` seeds for randomized inputs
  - Windows x64, root tests only: `tests/support/home.rs` `seed_conpty(home)` places the two ConPTY companions under `<home>/bin/<key>/conpty/` from the per-run `target/conpty-seed/<key>/` copy before the test's `viola run` (the §5 carve-out); the harness `boot`, `run_viola_unseeded` starts and `tests/conpty_sideload.rs` never seed
- **Per-test isolation:**
  - Every test gets a fresh home, so no endpoint, log or `budget.json` is shared: the FNV-1a endpoint hash includes the absolute home path.
  - The few tests touching the default port 47319 run in the nextest test group `fixed-port` (`max-threads = 1`).
- **Cleanup:** `TempDir` drop removes per-test homes. A test the runner kills (nextest `terminate = "immediate"`) never drops its home, so every root `TestHome` records `owner.json` `{pid, started_at}` beside it, and a new `TestHome`, when no `AGENT_RUN_KEEP_*` is set, first removes only the dirs whose recorded owner process is gone (pid dead, or alive with another start time) — never by age or name, never a dir without a record; a kept home drops its record. Harness sessions are cleared by `cleanup`. A failing test calls `TempDir::keep()` only when `AGENT_RUN_KEEP_FAILED=1`, so that `logs` can inspect the home. viola-e2e's booted lifecycle tests (`crates/viola-e2e/tests/harness_lifecycle.rs`) hold a `Booted` guard instead: on drop, even after a failed assertion, it runs `cleanup(.., keep_homes = true)` (every recorded process stopped, the home kept; the tests' own cleanup calls assert `home_removed:"kept"`) and then removes the home unless `AGENT_RUN_KEEP_FAILED=1` and the thread is panicking — `AGENT_RUN_KEEP_HOMES` is not read there, so a passing test's home is always removed. In CI, `AGENT_RUN_KEEP_HOMES=1` (set by `ci.yml`, overridden to `0` under `run --mutants`) makes every rstest home call `TempDir::keep()`, passing or failing, and makes harness `cleanup` keep its home (§3 `cleanup` step 6). The homes then stay under `target/e2e-home/` until obs-plan's G2, G4, secret scan and scan-gated uploads have run.

### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from
the contract above. Listed for explicitness. route may reorder or
combine them, and setup-project may add stack-specific intermediate steps.

- **test-runner-install:**
  - Install cargo-nextest 0.9.146 via taiki-e/install-action (SHA-pinned) in CI, or `cargo install --locked` locally.
  - Add `.config/nextest.toml` with:
    - `[profile.ci]`: `junit.path = "junit.xml"`, `retries = 0`, `slow-timeout = { period = "30s", terminate-after = 4 }`, `fail-fast = false`
    - `[profile.mutants]`: `fail-fast = { max-fail = 1, terminate = "immediate" }`, `slow-timeout = { period = "5s", terminate-after = 2 }`, plus `[[profile.mutants.overrides]] filter = 'package(viola-e2e)'` with `slow-timeout = { period = "15s", terminate-after = 2 }`.
      - A plain `fail-fast = true` waits for the running tests, so a caught mutant that hangs sibling tests outlives cargo-mutants' own timeout and is graded Timeout (measured 2026-09-24, auto timeout 108 s).
      - cargo-mutants 27.1.0 auto-sets its timeout to about `max(20 s, 5 × baseline test time)`. As measured at chunk 2026-09-24-observability-gates, that was 20 s on a 1 s root-package baseline and 110 s on a 21 s baseline with `viola-e2e` in the diff.
      - So every in-test wait a root-package mutant can reach, and the 10 s kill, stay below 20 s. At or above it, a hang grades Timeout instead of caught, as run `35995290314` showed.
      - The `viola-e2e` override keeps a 30 s kill because the harness's own tests wait out its 20 s boot deadline by design.
    - `[test-groups] fixed-port = { max-threads = 1 }` plus `[[profile.default.overrides]] filter = 'test(/default_port/)'`, `test-group = 'fixed-port'` (catalog configuration). Overrides on the default profile are inherited by `ci` and `mutants`. Every test that touches port 47319 has `default_port` in its name. Without the override the group is empty and those tests race.
  - Dev-deps:
    - rstest 0.27, tempfile 3.27, assert_cmd 2.2.2, predicates 3.1.4, trycmd 1.2.1, insta 1.48.0, jsonschema 0.57.0, proptest 1.11.0, mockall 0.15.0, mock_instant 0.6.1
    - windows-sys 0.61.2 with `Win32_Security_Authorization` (cfg windows)
    - in `viola-e2e`: rmcp `=3.4.1` with `client` and `transport-child-process`, reqwest 0.13.5 (no compression features), eventsource-client 0.18.0, tokio 1.53.1 with `test-util`
  - Node side: `e2e-web/package.json` pinning `@playwright/test@1.63.0` exactly (the committed `package-lock.json` resolves `playwright` and `playwright-core` 1.63.0, all from registry.npmjs.org; `@axe-core/playwright@4.13.0` joins with the a11y chunks of Epoch 8), `e2e-web/tsconfig.json` (`noEmit`, `strict`), plus `e2e-web/playwright.config.ts` (catalog configuration):
      - `use: { headless: true }` and a single `chromium` project
      - `retries: 0` and `forbidOnly: true` (a stray `test.only` would silently skip the rest)
      - the test-scoped session fixture (§3 `run` step 3) declares its own fixture `timeout`. It is computed from boot's deadlines (20 s per `--instance` plus 10 s with `--ui`), plus a fixed margin for boot steps 1 and 4 and for teardown `cleanup` (10 s supervisor deadline). Boot time therefore never counts against the test body's timeout. A fixture timeout is a failure, never retried.
      - no `webServer`, and `globalSetup` limited to boot step 1 (§3 `run` step 3)
      - `reporter: [['json',{outputFile:'pw.json'}],['junit',{outputFile:'pw-junit.xml'}]]`, the two files `run` and `gate` read
- **5-command-discipline-wire:** wire `scripts/agent-run.{sh,ps1}` as shims over `viola-harness` (boot / run / status / cleanup / logs), as specified in 5-command implementation above. Binding contract: both shell variants must expose identical semantics.
- **status-endpoint-implement:** the product side (`/health`, `/ready`, `/api/info`, `/api/sessions`, `viola list --json`) is arch-owned. This phase implements the `agent-run status` aggregation per the Status endpoint shape above, and the `api_sessions_equal_list` comparison.
- **log-format-bind-with-obs:** implement §3 Log format above as the single source of truth.
  - The test plan owns the harness-grepped fields (`timestamp`, `level`, `target`, `message`, `event`, `process`, `instance`, `corr`) and the `logs` wrapper shape.
  - obs-plan §3 is a downstream reader. It may add fields, but renaming or removing one needs a Decisions Log entry in this plan first.
  - Do not wait for an obs-plan schema. The phase is done when `agent-run logs` output passes the `jq -e` / jaq assertions in §3 `logs`.
  - Binding contract: the tests harness greps logs for assertions, so a format break is a harness break.
- **pid-file-commitment-wire:** wire the `session.json` write at boot readiness, the pid + start-time verified kill, and the supervisor `stop.request` / `supervisor-exit.json` handshake at cleanup, per PID file above.
- **test-data-bootstrap-wire:**
  - `viola-fake-agent` bin (feature `fake-agent`), its receipt format and its scripted modes (§7)
  - the rstest fixture chain in `tests/support/`
  - the fixture scrub-and-schema walk
  - committed `proptest-regressions/`
  - the `cleanup` home removal
- **coverage-tooling-install:** install cargo-llvm-cov 0.9.1 via taiki-e/install-action. It emits `target/lcov.info` and `cargo llvm-cov report --json --summary-only`. Any test that uses `env_clear()` must re-add `LLVM_PROFILE_FILE`.
- **ci-tool-install:** every other CI-invoked binary, version-pinned:
  - cargo-mutants 27.1.0 and cargo-deny 0.20.2 via taiki-e/install-action v2.87.19 (SHA-pinned), alongside cargo-nextest and cargo-llvm-cov
  - `cargo install --locked` for hyperfine 1.20.0, cargo-modules 0.27.0 and zizmor 1.30.1
  - CI's log assertion G2 is `scripts/g2-zero-panics.sh`, a fail-closed script on the runner-provided `jq` (`tool-missing: jq` without it), run as `--probe` then the check in the `test` and `perf` jobs; each presence-checks jq (`jq --version`), which is never installed. It counts `event:"panic"` role lines under `target/e2e-home` and exempts only a `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>`. `scripts/release-check.sh` (the `release` job), `scripts/orphans-check.sh` (`lint`) and `scripts/npm-audit.sh` (`supply-chain`, nightly `npm-advisories`) use the same runner-provided `jq` and refuse with `tool-missing: jq` without it (runner images read at chunk 2026-09-24-workspace-tree-and-code-graph-planes: jq 1.8.1 on windows-2025, 1.8.2 on macos-15 arm64, 1.7 on ubuntu-24.04). jaq 3.1.1 remains a valid local assertion form (§3 `logs`).
  - ripgrep 15.2.0 (PCRE2), the G1/G3 gate tool, comes from `scripts/install-ripgrep.sh` in the `lint` job. The script downloads the official release asset, checks it against the sha256 its release publishes, installs it idempotently into `target/tools/ripgrep/bin`, and prints `tool-missing: <tool>` when a tool it needs is absent. taiki-e/install-action `7623a79…` has no ripgrep manifest.
  - cargo-fuzz 0.13.2 via `cargo install --locked`, in the ci.yml `fuzz-replay` and `nightly.yml` `fuzz` jobs only, with the nightly toolchain installed by `rustup toolchain install` from `fuzz/rust-toolchain.toml` (no toolchain action)
  - Node v24.21.0 via `scripts/install-node.sh` (the official nodejs.org build, sha256-checked against ci.yml's workflow `NODE_PIN_*` lines it parses from the file text; no setup-node) in the `test` and `supply-chain` jobs and nightly `npm-advisories`; then, in each OS's `test` job, `npm ci --prefix e2e-web` and `npx --no --prefix e2e-web playwright install chromium` (`--with-deps` on ubuntu). The WSL pre-push distro installs the same Node and Chromium through `scripts/wsl-provision.sh`; Chromium's system libraries there are an operator-only root install (`--install-deps`, security-plan §Secret Management).

  `agent-run run --perf` and `--fuzz-replay` check that their tool is on PATH first. If it is missing, they exit 1 with `reason:"tool-missing"` and the tool name, never a silent pass.
- **quality-gate-config-emit:** emit `.github/workflows/ci.yml` enforcing:
  - the coverage thresholds in §10 (lines 85 / functions 95 / regions 80, per OS)
  - `retries = 0`
  - hyperfine `max` gates
  - cargo deny (root graph and `fuzz/Cargo.lock`), zizmor and cargo-modules orphans (`scripts/orphans-check.sh`, per lib/bin target) exit-code gates, and the `release` job's `scripts/release-check.sh`
  - all actions SHA-pinned, with `permissions: {}` at the top level
  - `viola-harness gate --require <that job's suites>` as the last step of every job that runs a harness suite (§3 Internal harness subcommands); `lint`, `supply-chain` and `release` are plain exit-code jobs with no gate step

route uses this list to plan phase ordering (typically:
test-runner-install → 5-command-discipline-wire → status-endpoint-implement
→ log-format-bind-with-obs → pid-file-commitment-wire →
test-data-bootstrap-wire → coverage-tooling-install → ci-tool-install →
quality-gate-config-emit). setup-project uses this list to materialize
each phase's bootstrap script + dependency list + verification
command. Ownership:
- This plan is the binding source for the harness-grepped log fields and the `agent-run status` shape.
- The nested product shapes inside `status` come from the arch GUI HTTP contract.
- obs-plan §3 reads both and may extend them but not redefine them.

---

<!-- U35 · test-plan.md · ## 12. Test Decisions Log · sha256 cf5bbe679c3a68e22e35ffa2a767c35b95ea987afae3898d5c33bd6aac07074f -->

## 12. Test Decisions Log

_Records key decisions during plan generation + manual additions
between phase loops._

**Initial entry:**

`2026-09-24`: Initial test plan generated by `/andromeda-tests`
- **Tier:** Comprehensive (2). Justified by test-scope Sec 6: 9 security vectors needing negative tests, 7 parser surfaces handed to tests for property/fuzz, 8 surface entries across 3 required OSes, 18 entities and 7 cross-surface critical paths, plus the multi-OS, chaos, performance-budget and multi-version triggers and founder-mandated mutation testing. The Compliance Test Coverage subsection is omitted: security tier Minimal, "No compliance triggers".
- **Key decisions:**
  - **Test framework:** cargo test / libtest (Rust 1.95.0) run through cargo-nextest 0.9.146, plus `cargo test --doc`. Chosen because it gives process-per-test isolation for per-test `--home`, JUnit output (stable libtest JSON is still unstable), test groups for the fixed port, and native use by cargo-llvm-cov and cargo-mutants.
  - **E2E driver(s):**
    - cli: assert_cmd 2.2.2 + trycmd 1.2.1
    - hook: assert_cmd
    - tui: portable-pty `=0.8.1` outer PTY. It adds no new dependency and works on ConPTY and openpty; expectrl and rexpect were rejected in research.
    - wrapper channel: viola-channel client + interprocess 2.4.4 + jsonschema 0.57.0
    - MCP: rmcp 3.4.1 client
    - GUI HTTP: reqwest 0.13.5 + curl + eventsource-client 0.18.0
    - web-spa: Playwright 1.63.0 headless Chromium on ubuntu, with @axe-core/playwright 4.13.0

    Chosen because each is the Sec 2 driver the research validated as agent-runnable, with exit-code or structured verdicts.
  - **Coverage tool:** cargo-llvm-cov 0.9.1. Chosen because it works on all 3 OSes (including windows-msvc), uses `--fail-under-*` exit-code gates, and instruments the externally spawned `viola` processes.
  - **Mutation:** cargo-mutants 27.1.0 `--in-diff` with `--test-tool=nextest`. The verdict is read from `outcomes.json`, not the exit code (Founder Direction 1).
  - **Stamps conflict (test-scope Sec 3 open conflict):** resolved by having `boot` and the rstest `stamped_home` fixture run `viola verify --home <home>` against `viola-fake-agent`, which sits on a test-scoped PATH as `claude`. No non-`verify` process ever writes `stamps.json`, which satisfies the security excerpt. This reads the arch CI note "`viola verify` runs only locally" as covering verify against the real CLI (tokens). **Needs arch/security ratification**; if rejected, the fallback is committed fixture stamps produced locally by `viola verify`.
  - **Test crate deviation:** a test-only crate `crates/viola-e2e` (publish = false) is added. It hosts `viola-harness` and the tokio-based test clients (rmcp client, reqwest, eventsource-client) and is never a root of the tokio ban: that ban runs through `deny-sync.toml` once per crate in `scripts/sync-crates.txt`, each as the sole root, so `viola-e2e`'s tokio graph is outside it with no `wrappers` allowlist and no `--exclude` (which false-fails on a feature-unified optional tokio — as measured at chunk 2026-09-24-supply-chain-and-workflow-gates). This keeps the sync root package and sync crates free of tokio dev-deps. The arch statement "No separate test crate is declared" was retired: the arch now registers `crates/viola-e2e` as the test-only member (re-verified absent at chunk 2026-09-24-workspace-tree-and-code-graph-planes). Its tests find `viola` / `viola-fake-agent` through assert_cmd's target-dir lookup after the harness `cargo build --workspace --features fake-agent`.
  - **Fake agent placement:** root-package `[[bin]] viola-fake-agent` with `required-features = ["fake-agent"]`. This makes `CARGO_BIN_EXE_viola-fake-agent` available to root integration tests and keeps it out of `cargo build --release --bin viola`.
  - **Branch coverage:** enforced as LLVM region coverage (`--fail-under-regions 80`). research documents no stable branch-coverage flag for cargo-llvm-cov, and mutation testing covers branch-strength.
  - **Browser caching:** no npm or Playwright browser cache in CI. No caching action beyond rust-cache was researched. Node is the pinned official v24.21.0 build that `scripts/install-node.sh` downloads and sha256-checks (runner-image Node retired 2026-09-27: the images disagreed, 22.23.2 on ubuntu and windows, 24.20.0 on macOS, and gave the WSL leg no pin to copy), still with no setup-node.
  - **criterion 0.8.2:** not used as a gate, because it does not exit non-zero on regressions. hyperfine 1.20.0 is the perf gate.
  - **cargo-fuzz 0.13.2:** ubuntu-only nightly job plus a PR corpus replay (`-runs=0`), because it is Unix-first and needs nightly and sanitizers.
- **Open questions:**
  - The spine hook deadline value is not in the distilled arch contract. The gate provisionally uses `max < 1.0 s` until arch names the constant.
  - The send confirmation window value is not in the distilled contract. Tests drive it through the injected clock and need the constant's location.
  - The source of the user's `statusline_command` that `run` records in the snapshot is not in the distilled contract. Path 6 needs a per-test redirectable settings source.
  - `viola ui --port 0` (OS-assigned port reported via `ui/<port>.url`) is requested from arch to remove the free-port race in `boot`. Until then, `boot` fails with `ui-port-taken` and does not retry.
  - Constant-time token compare: the plan asserts behaviour plus a source-scan test that the token is compared only inside the single compare function. The security plan owns the primitive choice and must confirm this static check is acceptable ("code-path review or a static check").
  - Windows handle non-inheritance of the channel pipe is asserted only at the product unit boundary (non-inheritable creation flag). There is no child-side probe on Windows.
  - obs-plan must adopt the required process-log fields in §3 Log format (`timestamp`, `level`, `target`, `message`, `event`, `process`, `instance`, `corr`), or re-map them in a Decisions Log entry, because the harness greps on them.

`2026-09-24`: User review 1 (overseer, founder-delegated). Tier, thresholds and Node accepted; stamps via verify-vs-fake ratified; arch requests logged; four audit-driven coverage additions.
- **Decision:**
  - Tier stays Comprehensive (2), with the §10 thresholds unchanged.
  - `viola verify` against the FAKE agent runs in CI (harness `boot` and the rstest `stamped_home` fixture). This supersedes the "Needs arch/security ratification" note in the initial entry.
  - Node stays test-side only (Playwright 1.63.0, @axe-core/playwright 4.13.0).
  - The spine-hook gate stays at a provisional `max < 1.0 s`.
  - New coverage:
    - (a) Driver-facing hints for `human-typing` and `budget-paused` never suggest `viola release`: Path 5, Path 6 step 8, and the exit-cause matrix.
    - (b) Exit-cause matrix: one negative test per cause of exit 21 and exit 1, with a cause-specific hint (§6 non-path suites).
    - (c) `budget.paused` with a per-instance override (Path 6 step 7).
    - (d) `list` row integrity for unwrapped names containing `\n` or `\t` (§6 non-path suites).
- **Arch amendment request:** the arch CI note "The real `claude` and `viola verify` run only locally" should read that the real CLI runs only locally, while `viola verify` against the fake agent runs in CI to stamp test homes. The security plan lets only `viola verify` write `stamps.json`, so this is the only way to get stamped homes in CI.
- **Arch requests:**
  - (1) name the send-confirmation window constant and its location, so tests can drive it through the injected clock
  - (2) add `viola ui --port 0` (OS-assigned port reported via `ui/<port>.url`) to remove the free-port race in `boot`
  - (3) name the spine-hook deadline constant, which replaces the provisional 1.0 s
- **Rationale:** the overseer's cross-plan audit of design against arch and security found these gaps:
  - Design hints could point drivers at `release`, which the channel refuses (-32602, exit 20).
  - Exit 21 and exit 1 are overloaded, so a single test per code would miss causes and let hints drift from causes.
  - The per-instance override is a distinct state from the global `budget.paused`.
  - The security plan's C0/C1 escaping "keeping `\n` and `\t`" suits `wait`/`last` content, but inside `list` NAME cells it would break fixed-width rows. That makes it a cross-plan item for security and design to ratify: `list` cells escape `\n` and `\t` too.
- **Impact:** §6 (Paths 5 and 6, plus the two new non-path suites) and §12. It touches arch (the three requests plus the CI-note amendment) and security/design (the `list` cell escaping).
- **By:** `/andromeda-tests` Phase 3.5 review, feedback file `review-feedback-1.md`.

`2026-09-24`: Overseer fix pass 2026-09-24 (cross-plan findings, founder-delegated). Each item was checked against the cited upstream line before the edit.
- **Decision:**
  - T1: `/ready` success is `"status":"ready"` (arch GUI readiness), not `"ok"`. Fixed in §1, the §1 Status endpoint shape and the §3 `boot` readiness check.
  - T6: the Unix endpoint is `<per-user 0700 dir>/viola-<h12>.sock` (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, `/tmp/viola-<uid>/`), not `$TMPDIR/viola-<h12>.sock` (security IPC access control, Occupied Resources amendment). Fixed in §3 `cleanup` step 4 and the §4 endpoint golden vectors.
  - T10: after a per-instance `release --budget`, the envelope `budget.paused` does not change, so the Path 6 ATIS bullet no longer expects a global `gate open` (arch `/api/sessions` budget fields).
  - T11: the Path 2 SSE `id:` is the composite cursor listing every tailed instance, with `builder`'s pair asserted (arch SSE feed).
  - T12: `viola list --json` is `{"v":1,"ok":{items,budget,skipped}}` with no `generated_at`. The parity checks compare `.ok.items` / `.ok.budget` / `.ok.skipped` (arch `list` payload, CLI `--json` output). Fixed in §1, the §3 `status` derived field, Path 6 step 7 and cross-surface parity.
  - T13: `bay-degraded` now makes the home unreadable with `chmod 000`, then restores 0700 before cleanup. A 0770 home stays readable to its owner, and strict-modes runs only at process start (arch `/ready`, security `~/.viola/` access control).
  - T17: `status` sends the cookie on `GET /api/info` (security GUI principal check).
  - Security control negatives: new §6 suite with one negative test for each security control that had none:
    - Linux socket-dir symlink / permissive / foreign-owner (unit);
    - Windows client SQOS impersonation level;
    - raw-channel `link` / `unlink` name validation;
    - Windows `--home` outside `%USERPROFILE%` DACL;
    - umask-independent modes;
    - single writers of `stamps.json` / `snapshot.json`;
    - token absent from error bodies;
    - `/assets` traversal.
  - O1–O3 (obs-plan D-21):
    - the `event` enum gains `release-from-driver` (`corr` = JSON-RPC `id`), `liveness-changed`, `state-recovered`, `sse-opened`, `sse-closed` and `parse-rejected` (`corr` null);
    - `process` and the `logs --process` filter gain `cli` (`cli-<name>.ndjson`);
    - a null `corr` / `instance` is written as key absence, which harness and `jq` treat as null.
  - Obs D-21 items:
    - the `logs` glob covers `instances/*/diagnostics/detail-*.ndjson`, with an added `instance` field;
    - every harness and rstest home sits under `target/e2e-home/`, passed as a not-yet-existing path so viola itself creates it (the Windows protected DACL for a `--home` outside `%USERPROFILE%`);
    - the G4 schema-conformance check body is tests-owned;
    - the concurrent-append check covers every shared diagnostics file, including a >4 KiB detail line;
    - a fixed canary in synthetic inputs must never reach home-level diagnostics;
    - perf hyperfine runs keep `VIOLA_NAME` / `VIOLA_DIR`, live under `target/e2e-home/` and assert zero panic lines before cleanup;
    - a `send-refused` before `send-issued` carries `corr` = the target's end offset (obs D-28), and wrapper `send-*` lines carry `conn` / `rpc_id` (obs D-30).
  - I2 / I3: the MSRV job moves to 1.96. The primary toolchain is the current stable (≥ 1.96), pinned in `rust-toolchain.toml`.
- **Rationale:** the overseer's cross-plan audit (`viola-overseer/cross-plan-findings.md`, tests vs arch/security/design, 2026-09-24 03:20; obs P3.5 amendments, 03:50; intent draft, 04:10).
- **Impact:** §1, §3 (`boot`, `status`, `cleanup`, `logs`, Log format), §4, §5, §6 (Paths 2 and 6, secret scan, schema conformance, the new security-negatives suite, parity, bay-degraded), §9, §10. Arch is unchanged here: the MSRV 1.96 floor and the diagnostics roots are arch amendments routed through route (obs-plan D-22).
- **By:** manual edit, overseer fix pass 2026-09-24 (founder-delegated).

`2026-09-24`: overseer fix pass 2, 2026-09-24 (cross-plan findings "obs vs upstreams" and "a11y P3.5", founder-delegated). Each item was checked against the cited line before the edit.
- **Decision:**
  - B1 (as ruled): in CI, homes are kept until the gate steps have run. `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`. Harness `cleanup` then keeps the home and reports `home_removed:"kept"`, and every rstest home calls `TempDir::keep()`. The homes stay under `target/e2e-home/` for obs-plan's G2, G4, secret scan and scan-gated uploads. The CI data-lifecycle line no longer says homes live in the runner temp dir. Fixed in §3 `cleanup` step 6, `logs` retention, Test data bootstrap → Cleanup, and §7 Test data lifecycle.
  - Y3: the `event` enum gains `a11y-violation` (`corr` null). It is written only by the a11y fixture into `e2e-web/test-results/a11y/*.ndjson`, never into `diagnostics/` (a11y-plan D-A11Y-09, accepted by the D-21 route).
  - Y5: the ubuntu Lint stage runs the a11y lint (eslint-plugin-lit-a11y + html-validate, a11y-plan D-A11Y-14) before `run --browser`, and a finding fails the build. No new harness `suite` value.
- **Rationale:** obs-plan's CI gates read `target/e2e-home/**`, but homes were deleted on green, so G2's non-empty check failed and nothing was scanned (finding B1). The a11y plan promoted the `a11y-violation` rows and the lint stage.
- **Impact:** §3 (`cleanup`, `logs`, Log format, Test data bootstrap), §7, §9 (Lint stage, build failure conditions). The a11y plan is unchanged.
- **By:** manual edit, overseer fix pass 2, 2026-09-24 (founder-delegated).

`2026-09-24`: overseer fix pass 3, 2026-09-24 (cross-plan findings "a11y vs upstreams", founder-delegated). Each item was checked against the cited line before the edit.
- **Decision:**
  - Z4: the harness gains real process controls with no fake API. `viola-harness ui-restart --session <id>` restarts `viola ui` on the same home and port, and the open page then hits the real 401. The `e2e-web` fixtures hold the page's own `/api/events` request (`page.route` + `route.continue()`; `fulfill` / `abort` stay banned), so `TAPE connecting` is observable. The fixtures also make real requests to an unknown route and a wrong method for the post-render 404 / 405 strips. Updated in §3 Internal harness subcommands and §6 Bay layout states (`bay-degraded`).
  - Z6: the a11y lint moves from the per-OS Lint stage to the ubuntu browser job, after `npm ci --prefix e2e-web` and before `run --browser`. It could not run before `npm ci`. This revises the fix-pass-2 Y5 placement.
  - Z7: `a11y-violation` leaves the product `event` enum; it has no `ObsEvent` variant. It gets its own tests-owned harness-side schema, `e2e-web/schemas/a11y-row.v1.json`, checked by the same conformance body as G4. This revises the fix-pass-2 Y3 enum line.
- **Rationale:** the overseer's audit "a11y vs upstreams" (2026-09-24 07:05). The a11y plan's P1 / P5 states needed real controls. The lint needs `node_modules`. A product-enum value would force a product `ObsEvent` variant for a harness-only row.
- **Impact:** §3 (Internal harness subcommands, Log format), §6, §9 (Lint and E2E rows). obs-plan drops the value from its accepted events in the same pass.
- **By:** manual edit, overseer fix pass 3, 2026-09-24 (founder-delegated).

**Subsequent entry format (for manual additions or re-runs):**

`{YYYY-MM-DD}`: {short title of decision}
- **Decision:** {what was decided}
- **Rationale:** {why; reference test-scope / research / org constraint}
- **Impact:** {which sections affected; downstream skills affected}
- **By:** {`/andromeda-tests` re-run / manual edit by {who}}

(Append new entries at the bottom; do not modify historical entries.)

`2026-09-24`: Chunk 1 (3-OS CI + harness skeleton) measured the harness against Windows and the build-branch workflow.
- **Decision:**
  - Harness cargo work goes to `CARGO_TARGET_DIR=target/harness` with `--features viola/fake-agent`, and the CLI's `<bin dir>` is `target/harness/debug`.
  - The integration filterset is `kind(test)` until an E2E binary exists.
  - The mutation diff is the working tree plus untracked files from `merge-base(base, HEAD)`.
  - The `mutants` CI job runs on push and PR, with base = PR base sha or else `github.event.before`.
  - `[profile.mutants]` terminates immediately on the first failure.
  - Interim readiness, status and cleanup fields hold until `viola list` and `viola ui` exist: `instances[]`, `processes_gone`, and `null` product fields. Readiness already reads the snapshot and heartbeat (staged after the two `process-start` lines); its `events.ndjson` check joins with "Hooks to normalised events".
  - New closed values: `reason:"usage"` with `detail` ∈ {`invalid-session-id`, `session-exists`, `invalid-instance`, `arguments`, `no-supervise-spec`}, `run --mutants` `reason:"build-failed"` (its root-package prebuild failed), the cleanup field `processes_gone`, the status member `instances`.
  - Unbuilt selectors are usage errors.
- **Rationale:** measured on the dev host 2026-09-24:
  - `os error 5` relinking the running `viola-harness.exe`;
  - nextest 0.9.133 rejecting an unmatched `binary()` operator;
  - no `origin/main` on the remote;
  - `/implement` never commits;
  - a caught mutant graded Timeout under `fail-fast = true`.

  Operator decisions at phase P4: the walking skeleton, the push + PR trigger, and a grammar that grows per chunk.
- **Impact:** §3 (preamble exit codes, `boot` steps 1/3/5 + readiness, `run` steps 1–4, `cleanup` step 1 + verification, `supervise`, Status shape, test-runner-install), §9 (Mutation row, toolchain source).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-24-three-os-ci-headless-harness-skeleton`.

`2026-09-24`: Chunk 2 (fake agent and test-data fixtures) fixed the mutation verdict for Rust-free diffs and built the consumer-first fake agent.
- **Decision:**
  - New closed value `run --mutants` `mutants.verdict` ∈ {`counted`, `no-rust-delta`}.
  - A diff naming no `.rs` path passes only with the explicit `no-rust-delta` verdict, which names the diff and its file count.
  - A Rust delta first deletes a stale `mutants.out/outcomes.json`, and stays red with no fresh one.
  - The fake agent is consumer-first. It builds the script/control/receipt contract, bracketed paste, hook calls from `<plugin-dir>/hooks/hooks.json` (absolute exec-form only, matchers deferred), `--suppress-prompt-submit`, `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof` and `--report-version`. `--vt100-panic-bytes`, `statusline-echo` + the `settings.json` read, and `agents --json` / `FAKE_CLAUDE_AGENTS_MODE` land with their consumer chunks.
  - Only the sync root fixture chain exists; the `viola_e2e::fixtures` copy lands with its first E2E consumer.
  - The hygiene walk covers `fixtures/fake-scripts/*.json`; the `fixtures/claude` walk joins with the first recorded fixture.
  - `boot` passes the parent `--fixtures <root>/fixtures/claude`.
  - Fixture files are named `<Event>.<variant>.json` (the CLI's PascalCase hook event name).
- **Rationale:**
  - Measured with cargo-mutants 27.1.0: a Rust-free diff exits 0 and leaves `mutants.out/` untouched. CI run 35973118026 (sha `dc01bd9`) read `outcomes-missing`, and the phase P5 baseline on the dev host read a stale `"tested":8`.
  - Operator P1: "a diff with no .rs files passes only with an explicit no-rust-delta verdict … a diff WITH .rs files and no outcomes stays red".
  - Operator P4: "consumer-first, no shapes invented before a recorded fixture".
  - Operator wrap P2: "code wins where the doc invented a shape; no mapping tables".
- **Impact:** §2 (fixture naming), §3 (`boot` step 5, `run` steps 2 and 4, Output format, Closed enums), §7 (seed table, Fake agent, Fixture hygiene), §10 (Mutation gate).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-24-fake-agent-and-test-data-fixtures`.

`2026-09-24`: The Quality gates chunk wired the per-job `gate`, per-OS coverage, MSRV, a two-leg mutation verdict and the seeded fuzz replay.
- **Decision:**
  - Mutation runs as two legs (`ubuntu-latest`, `windows-2025`). Each writes `mutants-verdict-<leg>.json` (`run --mutants --leg`), and the `mutants-verdict` job gates the union: a mutant is red only when no leg caught it and some leg missed it or timed out. `mutants.out/` is no longer uploaded.
  - The fuzz pipeline is seeded with a `viola_name` target (`ViolaName::try_new` against an independent oracle, 18 synthetic seeds) ahead of the seven parser targets. `fuzz/` is its own cargo workspace, excluded from the root, with its own lockfile and `fuzz/rust-toolchain.toml` channel `nightly-2026-09-20`.
  - No `concurrency:` block in `ci.yml` or `nightly.yml`. zizmor's pedantic `concurrency-limits` is declined and stays visible (2 low).
  - The MSRV and fuzz toolchains come from `rustup toolchain install`, never a toolchain action.
  - New closed values: usage `detail` `invalid-leg` and `unknown-suite`; `run` reasons `fuzz-linux-only` (exit 2), `tool-missing` and `corpus-empty`; the `mutants.leg` field; the leg-verdict `outcome` set.
- **Rationale:**
  - cargo-mutants "does not yet understand conditional compilation … will report functions for other platforms as missed" (mutants.rs/limitations.html). CI run `36005608858` on `39c3d2b` MISSED the `#[cfg(not(unix))]` `file_mode` stub (`secret_scan.rs:230:5`), which Linux never compiles. Operator P4: "fix the gate now, not in Epoch 2".
  - Operator P4: "ViolaName is a real consumer, so the fuzz pipeline is infrastructure with a target, not an abstraction over emptiness".
  - A concurrency group cancels pending runs even with `cancel-in-progress: false`, which would drop a push's `--in-diff` mutation diff and its `always()` gate and upload chain.
  - Architecture and security forbid a toolchain action. `RUSTUP_TOOLCHAIN=1.96` outranks `rust-toolchain.toml` (research M4).
  - `mutants.out/outcomes.json` holds absolute argv paths and `log/` holds test output; the mutants job has no homes to scan.
- **Impact:** §2 (Property-based row), §3 (preamble usage details, `run` body / `--coverage` / `--fuzz-replay` / exit semantics / Output format, `gate`, Bootstrap `ci-tool-install`, Closed enums), §6 (Property suite), §9 (Coverage report, Mutation, MSRV, Fuzz replay and Quality gates rows, tool install, Matrix builds, Test report format), §10 (Stack adjustments, Mutation gate), §11 (CI).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-24-quality-gates`.

`2026-09-24`: The Workspace tree and code-graph planes chunk gated the release output and module orphans, audited the fuzz lockfile and disposed of the rest of the Lint row.
- **Decision:**
  - A per-OS `release` job runs `scripts/release-check.sh`, which judges the build's own `compiler-artifact` records and refuses any executable but `viola`. `--probe` proves the refusal (3 refused inputs and a control).
  - `cargo modules orphans --deny` runs per lib/bin target in the per-OS `lint` job through `scripts/orphans-check.sh`, whose `--probe` fires on a planted orphan. `cargo modules dependencies --acyclic` returns to an on-demand boundary review.
  - The rmcp `cargo tree -e features` release-graph assertion moves to the "MCP server for drivers" chunk.
  - `fuzz/Cargo.lock` is audited for advisories and sources in `supply-chain`, and for advisories in the weekly `nightly.yml` run.
  - No new harness subcommand, `suite` value or gate token: `release`, like `lint` and `supply-chain`, is a plain exit-code job.
- **Rationale:**
  - Measured at cargo-modules 0.27.0, the CI pin: `--acyclic` exits 1 on 3 of 4 targets, each a type ↔ its own inherent method, even under `--no-fns --no-types --no-traits --no-owns --no-externs --no-sysroot`. It fails by construction. Operator P4: wire orphans, amend the rest.
  - `cargo tree -e features -p viola --edges normal | grep -c rmcp` is `0` at HEAD, so the assertion would pass vacuously (§3 preamble, "never a vacuous pass"). viola-mcp is its true first consumer.
  - A shared `target/release/` kept a stale `viola-harness.exe` after a `--bin viola` build, so a directory listing cannot judge the release output. A bare `cargo build --release` and `--bin viola` yield `viola` only, while `--workspace` adds `viola-harness`.
  - The advisory DB moves without code changes; operator P4 added the fuzz lockfile to the weekly run.
- **Impact:** §1 (dependency and boundary policy entity), §2 (V9 row), §3 (Bootstrap `ci-tool-install` jq, `quality-gate-config-emit`), §9 (Lint, Supply-chain and Release build rows, tool install, Build failure conditions), §10 (Build failure conditions).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-24-workspace-tree-and-code-graph-planes`.

`2026-09-25`: The Epoch 1 cleanup chunk made the mutation leg diagnosable and closed its false green.
- **Decision:**
  - `run --mutants` passes `--caught --unviable --build-timeout-multiplier=5` and streams cargo-mutants' stdout live to the harness's stderr. Every outcome line reaches the CI step log as it happens.
  - `mutants_suite` is red when `unviable > caught`: `failures[]` code `unviable-exceeds-caught`, and `failed` = survivors + 1. Under `--leg` such a leg is not deferred to the union.
  - No new `mutants.verdict` value, no new leg `outcome` value, no workflow change.
  - The booted-session tests in `crates/viola-e2e/tests/cli.rs` stop their session through the library `cleanup` on drop. A mutated binary `cleanup` then leaks no process.
  - **Declined** (a decision, not a deferral): a reduced partial-verdict upload for cancelled legs. The per-mutant stream already makes a stall diagnosable from the job log, and raw `mutants.out/` stays un-uploaded (security-plan §Bootstrap `secret-scanning-ci-gate`).
- **Rationale:**
  - CI run 36046091888 (windows): 2 h 45 m silent. `Command::output()` waited on a pipe that a leaked supervisor held open. As measured locally, the pipe was still open 61.7 s after the child exited.
  - CI run 36118112104 (windows): `8 caught, 135 unviable`, read `ok:true`. Under the `cleanup_cmd → Default` mutant the booted supervisor leaked and locked `viola-harness.exe` (relink `Access is denied. (os error 5)`, reproduced locally), and every later mutant build failed.
  - The `unviable > caught` threshold was measured across all legs with an uploaded verdict (§10 Mutation gate).
  - Operator (founder-delegated) decisions, 2026-09-25.
  - Green witness: CI run 36126924953 on d14f234. ubuntu `148 … 144 caught, 4 unviable`, windows `148 … 142 caught, 6 unviable`, `mutants-verdict` success.
- **Impact:** §2 (Mutation row), §3 (`run` step 4 Base / Command / Verdict, Exit code semantics, `gate` mutants breach, Bootstrap `quality-gate-config-emit`), §9 (Mutation row, Build failure conditions), §10 (Mutation gate, Build failure conditions), §11 (Quality).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-24-epoch-1-cleanup`.


`2026-09-25`: The Security prerequisites chunk added the `test-only-rust-delta` mutation verdict and pinned the SQOS open and the content hash.
- **Decision:**
  - New closed value: `run --mutants` `mutants.verdict` ∈ {`counted`, `no-rust-delta`, `test-only-rust-delta`}.
    - A diff whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`, at the root or under `crates/<member>/`) never builds or runs cargo-mutants. It passes only with that explicit verdict, which names the diff, its file count and `rust_files`, and the leg file carries it with `"mutants":[]`.
    - A mixed diff (any other `.rs` path) stays `counted`, and red `outcomes-missing` without a fresh `outcomes.json`.
    - This refines the chunk-2 operator ruling ("a diff WITH .rs files and no outcomes stays red"). The ruling still holds for every diff that names a `.rs` path cargo-mutants can mutate.
  - The Windows client SQOS open is pinned ahead of `viola-channel` by `tests/channel_sqos_open.rs`, with a discriminating no-SQOS control.
  - The pinned-exe content hash's library is pinned by `tests/contract_content_hash.rs` (FIPS 180-2 vectors + the 16-hex truncation).
- **Rationale:**
  - cargo-mutants 27.1.0 over this chunk's tests-only Rust diff printed `INFO No mutants to filter` and wrote no `outcomes.json`, so the gate read `outcomes-missing` (chunk implement gate run 1, entry 24).
  - `--list-files` over a package with `src/`, `tests/`, `benches/` and `examples/` files lists `src/lib.rs` alone, so all three target directories are unmutatable (measured on 27.1.0).
  - Operator (founder-delegated) ruling "option A", 2026-09-25: fold the fix. Conditions:
    - only test-target diffs;
    - the mixed red kept, under a literal-oracle test (`run_mutants_mixed_src_and_test_delta_without_outcomes_stays_outcomes_missing`);
    - the new classifier's own mutation reading;
    - this entry.
  - Witness: the local windows leg `counted`, 9 mutants, 9 caught. CI run 36138441784 on `8e25ca7`: ubuntu and windows `9 mutants tested … 9 caught`, `mutants-verdict` success.
- **Impact:** §3 (`run` step 4 Classification, Output format `mutants` object and leg file, Closed enums); §6 (Security control negatives → Windows client SQOS; Contract suite); §10 (Mutation gate).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-25-security-prerequisites`.


`2026-09-26`: The CI chunk base and union verdict chunk made every CI run of an operator pass read the whole chunk, judged the mutation union by compiling legs, and took the mutation diff out of the secret scan.
- **Decision:**
  - The mutation base is derived by the harness: the last master flip at or before the parent of the oldest `chore({marker}): operator pre-CI commit` of a pending chunk (HEAD when there is none), `HEAD^` on the wrap push. `AGENT_RUN_CHUNK_BASE` stays as an explicit override for tests; CI passes no `github.event` value. The run document carries `base`.
  - The `gate --mutants-legs` union judges each mutant only by the legs that compile its line (gate-side, `syn` over the checked-out source's `#[cfg]`s; a leg dropped only on a cfg proven false), every leg when none does. A mutant missed on a compiling leg stays red.
  - `secret-scan` skips exactly `target/agent-run/chunk.diff`, and the `harness-<os>` upload excludes the same file.
- **Rationale:**
  - Founder ruling 2026-09-25 (V15 instance half): CI round-trips cost more than the code, so a fix push must re-read the whole chunk; V17 sanctions fix commits after the pre-CI commit, so the base must not move mid-pass. An unbounded `-G ' · complete · '` pickaxe does move on a fix commit that edits a `complete` record (measured in a scratch-repo pass and by `chunk_base_holds_the_flip_across_a_pass`).
  - Run 36165685381's union breach (`HostTerminal::enter -> Some(Default::default())`, `lib.rs:395:9` / `:420:9`): each mutant was unviable on the leg that compiles its `enter` and missed on the other, where that body is compiled out. Outcomes alone cannot tell "compiled out" from "missed on a compiling leg", so the gate reads the `#[cfg]`s. A leg-side annotation was declined: it would widen the ratified content class of the unscanned leg upload.
  - The pty chunk's secret-scan read `chunk.diff` residue and hit its own `?t=` line: the file is repository source text by construction.
  - Witness: CI run 36270173848 on `acd08c7`, both legs `"base":"fcca1ce…"`, 79 tested / 74 caught / 0 survived each, `mutants-verdict` `ok:true`; the replay of run 36165685381's recorded legs reads `ok:true`.
- **Impact:** §3 (`run` step 4 Base, Output format `mutants` object, `secret-scan` Scope, `gate` union bullet and CI paragraph); §6 (Canary); §9 (Mutation row, Test report format); §10 (Mutation gate).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-26-ci-chunk-base-and-union-verdict`.


`2026-09-27`: The local Linux pre-push gate chunk added `viola-harness pre-push`, conditioned the wrap-push `HEAD^` base step, and folded a Linux resize red the gate found into a `viola-pty` fix.
- **Decision:**
  - `pre-push` is an internal harness subcommand (Windows host; WSL2 `Ubuntu`): the ubuntu test job's suites and the `ubuntu-latest` leg in a synced, history-carrying Linux clone, the `windows-2025` leg on the host, CI's own union; stages and closed values in §3. The operator pass runs it on the uncommitted tree before the pre-CI commit, and a red stops the pass.
  - The `HEAD^` base step applies only when the working tree holds no pending record HEAD's copy lacks; with an uncommitted promotion the flip is the base (§3 `run` step 4 Base).
  - The `viola-pty` pump's resize baseline is the spawn size, never a second host read (§5 Module ↔ PTY); the forced-window test reaches the window through the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam (capped at 5 s, absent from release builds; architecture §Occupied Resources, security-plan carve-out).
- **Rationale:**
  - Founder ruling (relayed 2026-09-26): catch Unix reds before the push; CI round-trips cost more than the code. The two chunks before lost CI round-trips to Unix-only reds this Windows host could not run.
  - Before the pre-CI commit HEAD is the last wrap flip, so the old rule derived acd08c7 where CI derived a69c5ef (measured at P4); a pre-commit local leg could never print CI's base.
  - The gate's first real runs went red 3/3 on `tui_host_resize_reaches_the_child` while 68 isolated reproductions stayed green. The cause was MEASURED, not inferred from a green: a 1 s hold forced the spawn→pump window open, 6/6 red with the exact signature and the probe reading `spawn` 80×24 → resize +23.7 ms → pump baseline 100×30 at +990 ms; control 6/6 green; after the fix 6/6 green.
  - Witness: three consecutive green `pre-push` runs (846 / 850 / 852 s), then the operator pass (entry 17, 851 s) and CI run 36282518379 on `054ebe4`: 15/15, both legs `"base":"a69c5ef…"`, 87 tested, 0 survived, `mutants-verdict` `breaches: []`.
- **Impact:** §3 (Internal harness subcommands `pre-push`, Closed enums, `run` step 4 Base); §5 (Module ↔ PTY); §9 (tool-install paragraph); §10 (Mutation gate).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-26-local-linux-pre-push-gate`.


`2026-09-27`: The Wrapper channel chunk added two `pre-push` stages and the endpoint readiness code, and corrected the E2 fd premise.
- **Decision:**
  - `pre-push` `stage` gains `vm-release` and `windows-tests` (§3 Closed enums). `boot` readiness gains the missing code `<name>:endpoint`; `cleanup` reports `endpoint_gone`.
  - E2's Unix fd witness reads the live `/proc/<pid>/fd` table and asserts that the child holds nothing of viola's (§6 E2).
- **Rationale:**
  - An idle WSL VM held ~21 GB and the host link failed twice until `wsl.exe --terminate` freed it (7.9 → 37.6 GB in ~45 s); `windows-tests` is the Windows coverage `test` stage the route carried to this chunk, so a Windows coverage red stops before the host mutation leg.
  - The fd count was a premise the measurement disproved (`[0,1,2,3,4]`; portable-pty `close_random_fds()`); the overseer's live answer at this wrap ruled the new witness a premise fix, not a widening.
- **Impact:** §3 (`pre-push`, Closed enums, `boot`, `cleanup`); §6 (E2).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-27-wrapper-channel`.

`2026-09-27`: The Epoch 2 cleanup chunk moved the Windows mutation run into a host scratch and added the scoped inner loop and the run archive.
- **Decision:**
  - `run --mutants` on a Windows host uses the host mutation scratch `<repo parent>/viola-mutants-scratch` (guarded, wiped before each counted or scoped run, `TMP`/`TEMP` and `--output` on the `cargo mutants` command). New `run` `reason` values `scratch-refused` and `scratch-wipe-failed`; the counted or scoped `mutants` object gains `scratch_bytes` (§3 `run` step 4, Output format, Closed enums).
  - `run --mutants --file <path>` (repeatable) is the scoped inner loop: `mutants.verdict` `scoped` with `files`, never a leg verdict; with `--leg` it is usage `scoped-leg` (§3 Command body, Exit codes, Closed enums, Test selection).
  - The run document gains `archived` (`target/run-archive/<n>`, newest 10). `pre-push` `cache` gains `windows_scratch_bytes` / `windows_scratch_bytes_after` (§3 Output format, `pre-push`).
  - viola-e2e's booted lifecycle tests keep a failing test's home under `AGENT_RUN_KEEP_FAILED=1` through a `Booted` drop guard (§3 Test data bootstrap). viola-e2e's table-driven cases use a labelled case table, as it carries no dev-dependencies (§4).
- **Rationale:**
  - Killed and green Windows mutation runs left ~30 GB of temp copies on C:, and `mutants.out/` inside the repository fought rust-analyzer. cargo-mutants' temp copy follows `TMP`/`TEMP` (as measured at this chunk: the copy appeared in the scratch mid-run, none in `%TEMP%`). The guard exists because the wipe runs outside the repository (an overseer condition at P4).
  - A scoped verdict must never reach the union, so it writes no leg file and refuses a leg. The archive keeps each run's `outcomes.json` and JUnit before the next run overwrites them, outside every upload path, because `outcomes.json` carries absolute argv paths.
- **Impact:** §2 (machine-parseable output), §3 (Exit codes, `run` Command body / step 4 / Output format / Test selection, `gate` Inputs, `pre-push`, Closed enums, Test data bootstrap), §4 (Test grouping), §9 (Release build row, release-build errors: probe `5/5`), §10 (Mutation gate).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-27-epoch-2-cleanup`.

`2026-09-27`: The Browser verdict reachability chunk proved the browser pipe end to end on all three CI OSes and the WSL pre-push leg (founder ruling W125), and lowered the root test waits below the mutants kill.
- **Decision:**
  - `run --browser` runs on every OS (the `browser-linux-only` arm is retired) and only under `--browser`: `--all` and the default never select it (P4 operator fork 2). The harness spawns `npm ci` (`npm.cmd` on Windows), a Chromium probe and `node node_modules/@playwright/test/cli.js test` directly, with no shell and no npx. New `run` `reason` `browser-missing` (§3 `run` step 3, Closed enums).
  - A `file://` stub page and one spec, `e2e-web/tests/pipe-reachability.spec.ts` (titles `pipe:`), assert the pipe, not a layout; the real specs, axe and the a11y verdict stay in Epoch 8 (§2 naming, §3 Bootstrap `test-runner-install`).
  - Node is the pinned official download (`scripts/install-node.sh`, ci.yml `NODE_PIN_*`) on every CI OS and in the WSL distro. The CI `test` job gains the browser steps and gates `coverage,doctest,playwright`. The `supply-chain` job audits the npm lockfile, and `nightly.yml` gains `npm-advisories` (§9, §12 Browser caching).
  - `pre-push`: `tools` checks `node` (new `detail` `node`), `linux-tests` runs `run --browser` and gates `playwright`, the document's `linux` object gains `browser`, and the `env -i` PATH gains the constant `<home>/.local/viola-node/bin` (§3 `pre-push`, Closed enums). The host `windows-tests` stage runs no browser suite.
  - The 8 root waits on a child are bounded at 7 s (`tests/support/watch.rs` `WITHIN`) and stream a codes-only report to `<temp dir>/viola-root-watch/` (§3 `run` step 2). This is a recorder, not a fix.
- **Rationale:** those gates sat ~44 of 50 chunks after base CI (founder's basis, relayed), so the pipe is proven before any feature needs it. The runner images disagreed on Node. A wait equal to the 10 s mutants kill is a race the test can lose, as measured at this chunk's remove-the-guard pair (`evidence/root-watch-recorder.md`: 7 s `FAIL [7.018s]` with the report in its message, 12 s `TIMEOUT [10.010s]` with the output lost and the file kept).
- **Impact:** §1 (web-spa driver, harness run order, multi-os-compat), §2 (E2E row, V9 row, naming), §3 (`run` Command body / steps 2 and 3 / Output format / Test selection, `gate`, `pre-push`, Closed enums, Bootstrap `test-runner-install` and `ci-tool-install`), §6 (Drivers), §9 (Lint, Supply-chain, E2E and Coverage rows, the tool-install paragraph, Matrix builds, Test report format), §11 (CI), §12 (Browser caching).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-27-browser-verdict-reachability`.

`2026-09-28`: The Mutation testing to the epoch boundary chunk moved mutation testing out of every chunk, the pre-push and CI (the founder's ruling, 2026-09-28 17:59, relay: the Viola overseer).
- **Decision:**
  - CI jobs `mutants (<os>)` and `mutants-verdict` removed (9 → 7 jobs, 18 → 15 check-runs, as measured at CI run 36483042659 on `17b93c7`); the `pre-push` stages `linux-leg`, `windows-leg`, `union`, the reasons `verdict-missing`, `base-mismatch`, the document's `legs`/`gate` and scratch byte fields, and the distro `TMPDIR` scratch removed.
  - `run --leg`, `gate --mutants-legs`, the usage details `invalid-leg` and `scoped-leg`, the leg verdict file and the `#[cfg]` union (`harness::cfg_legs`, with `syn` / `proc-macro2`) removed (the overseer's P4 answer: no code without a consumer). `run --mutants` [`--file`] is kept for `/andromeda-code-audit`; no selector and `--all` select unit + integration only.
  - The two real-cargo-mutants harness tests are compiled out on macOS (§10 Mutation gate), the runner-side arm under the operator's P5 ruling, after the one measurement push (CI run 36481260151).
  - `tests/channel_endpoint.rs` `send_oversize` also accepts ENOTCONN on the oversize write (§5), folded on the operator's word from CI run 36482322449.
- **Rationale:** the epoch-boundary audit is mutation's home (it runs `cargo mutants -p {unit}` itself); the per-chunk, pre-push and CI gates, and the union built only for them, have no consumer left.
- **Impact:** §1 (tier justification, `run` order, required test type), §2 (Unit, Mutation rows), §3 (`run` body, step 4, exit codes, output, test selection, `secret-scan`, `gate`, `pre-push`, Closed enums, `quality-gate-config-emit`), §5 (oversize frame), §9, §10 (Mutation gate, build failure), §11.
- **By:** `/andromeda-wrap-session`, chunk `2026-09-28-mutation-testing-to-the-epoch-boundary`.

`2026-09-29`: The Verify-stamped test homes and harness chunk stamps every test home the one sanctioned way and lands the harness's live mode.
- **Decision:**
  - The root `stamped_home` and harness `boot` step 4 stamp through `viola verify` against the fake agent at the recorded 2.1.283 over `fixtures/claude` (`boot --unstamped` skips step 4; a failed verify is `verify-failed` with verify's `exit_code`, before any supervisor). `StampedHome::unstamped` serves the unverified-path tests. The fake agent's and the harness's `DEFAULT_CLI_VERSION` are 2.1.283.
  - `supervise` passes `--fixtures <root>/fixtures/claude`; `boot` readiness checks `events.ndjson` lines 1-3 (new missing code `<name>:events`).
  - `run --local-live`: new suite `local-live`, `run` reason `live-in-ci` (exit 2, under `CI`), failure codes `build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing`; `gate --require` does not take `local-live`.
  - `Wrapper::stop` / `stop_keep` also wait for the recorded endpoint to be unconnectable (9 root waits share `WITHIN`).
- **Rationale:** stamps come only from `viola verify` (§7 Seed strategies); an exit code is not the endpoint gone, measured at ci#36532038635 (the stopped wrapper's own pipe took a hook 25 ms after its exit line; why is not established; the pipe-name-collision hypothesis was falsified — the endpoint name carries the home).
- **Impact:** §3 (`boot` readiness, `run` step 2, `--local-live`, Output format, `gate --require`, Closed enums), §5 (Module ↔ PTY H2 owner), §7 (Fake agent), §10 (Perf session).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-29-verify-stamped-test-homes-and-harness`.

`2026-09-29`: The Sideloaded ConPTY chunk puts the Windows x64 root tests on the sideloaded ConPTY and carves seeded homes out of the fresh-home convention.
- **Decision:**
  - Seeded homes (the operator's ruling at this chunk's wrap, the overseer agreeing): on Windows x64 `Wrapper::boot` and every piped or outer-PTY `viola run` start seed `<home>/bin/<key>/conpty/` via `seed_conpty` from a per-run `target/conpty-seed/<key>/`, byte-identical to the embedded companions; `tests/conpty_sideload.rs` and `run_viola_unseeded` never seed, and the harness `boot` never seeds. A route CARRY on the Epoch 6 entry "Home and code-bearing file integrity": once viola sets its own home DACL at creation, the seed must let viola create the home first.
  - The piped driver (`tests/support/piped.rs` `Piped`) answers DA1 `ESC[c` with `ESC[?1;0c`; viola stays silent. The forced-window resize test resizes on the wrapper's `process-start{claude-child}` line.
  - The H2 with/without pair (ci#36563868040: sideloaded 0 of 200, inbox 14 of 200) is recorded in §5, no rate; its measurement-only race and CI loop left the tree before the wrap.
- **Rationale:** a first start's two companion writes cost a median 1 193 ms on the dev host under a parallel suite, pushing first-start tests past their 7 s waits (no bound raised); the sideloaded host holds the child's start for a DA1 answer (3.54 s unanswered vs 0.54 s answered, the chunk's `evidence/da1-stall.md`).
- **Impact:** §3 (Test data bootstrap), §5 (Module ↔ PTY, Setup / teardown lifecycle), §7 (Seed strategies), §9 (Coverage report row, Build failure conditions), Critical Path 1.
- **By:** `/andromeda-wrap-session`, chunk `2026-09-29-sideloaded-conpty`.

`2026-09-29`: The Fake-agent drift contract chunk pins the fake agent against the recorded fixtures byte for byte, makes the resize oracle change-driven, and moves matcher evaluation and S8 `annotations` to their consumer
- **Decision:**
  - The contract suite (`contract_fake_agent_drift`) compares each print-mode hook's receipt `stdin_hex` with the recorded `<Event>.default.json` bytes and the hook order with a spine literal — no insta snapshot (P4 fork, the overseer agreeing): the recorded fixture files are the pinned artifact, and a snapshot copy would duplicate them and move with every `--record` refresh. insta stays for decision bodies.
  - The contract witnessed one live drift before its fix: the UserPromptSubmit payload dropped the fixture's trailing newline (now kept).
  - The receipt `size` is written by a watcher at start and on every change (P4 fork, the overseer agreeing), the one size mechanism; `pty_resize_reaches_a_child_that_reads_no_key` was red before it.
  - Hook `matcher` evaluation and the S8 `annotations` assertion move to "Dialog answers by dialog_id" (P4 forks, the overseer agreeing): no recorded fixture carries a `tool_name`, and the question answer path does not exist yet.
- **Rationale:** the fake agent is the one component whose job is not to drift from the measured CLI, so its pin is the measured bytes themselves; a matcher implemented from the docs alone would be unmeasured behaviour inside it.
- **Impact:** §2 (Contract row), §4 (viola-agent-claude M2), §6 (Path 5, Contract suite), §7 (Seed strategies, Fake agent).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-29-fake-agent-drift-contract`.
