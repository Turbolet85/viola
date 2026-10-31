# test-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — harness builds in its own target dir
**Section:** §3 `boot` steps 1, 3, 5 · §3 `run` steps 1–3 (layer commands, globalSetup) · §3 `run` step 4 Test scope
**Change:**
- The harness's cargo work runs with `CARGO_TARGET_DIR=<root>/target/harness` and `--features viola/fake-agent`, and the CLI `<bin dir>` is `target/harness/debug`.
- The fake agent is copied from `<bin dir>`.
- JUnit is still read from `<root>/target/nextest/ci/junit.xml`.
- `viola-e2e` declares a no-op `fake-agent` feature for cargo-mutants' package-scoped runs.
**Why:** measured on Windows: relinking the running `target/debug/viola-harness.exe` fails with `os error 5`, and a workspace build re-fingerprints the bins (`UnitDependencyInfoChanged`).
**Kept:** "never a literal `target/<profile>`" still holds; cargo-mutants passes the literal feature; llvm-cov builds in its own target dir; the CI lint runs clippy directly, not through the harness; perf builds in `target/perf`.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — integration filterset, mutation diff, base and trigger
**Section:** §3 `run` step 1 (integration layer) · §3 `run` step 4 (Base, Diff) · §9 Pipeline structure (Mutation row)
**Change:**
- The integration layer is `kind(test)` until an E2E-prefixed binary exists.
- The mutation diff is the working tree plus untracked files from `merge-base(<base>, HEAD)`.
- The `mutants` job runs on push and pull_request, with base = PR base sha or else `github.event.before`, passed through `env:`.
- The `origin/main` fallback is recorded as never resolving on this project's single build branch.
**Why:** measured: nextest 0.9.133 rejects an unmatched `binary()` regex; the committed-only diff was 0 lines against 10 309 working-tree lines; the remote holds only `build/viola-0.1.0`. Operator decision at phase P4.
**Kept:** the E2E layer keeps its `binary()` selector, which becomes valid once E2E binaries exist.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — interim supervisor, readiness, status and cleanup
**Section:** §3 preamble (Exit codes) · §3 `boot` step 5 + Readiness signal · §3 `run` step 2 (harness_session, harness cleanup meaning) · §3 `cleanup` step 1 + Verification · §3 `supervise` · §3 Status endpoint shape
**Change:**
- The supervisor is an ordinary child process (detach flags removed) and holds a stdin pipe per wrapper until `viola-pty` exists.
- Interim readiness: the role file holds `process-start` lines for `self` and `claude-child`, and both pids are alive.
- The status shape adds `instances[]`, and `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null` in the interim.
- The cleanup report adds `processes_gone`, and `endpoint_gone`, `port_free` and `url_file_removed` are `null` in the interim.
- The harness cleanup assertion now reads "no field false" (was "every field `true`").
- A usage error carries `reason:"usage"` plus a `detail` code, and an unbuilt selector is a usage error.
- The key order `{"v","cmd","ok",…}` is held by serde_json `preserve_order`.
**Why:** the chunk's harness as built; operator decision that the grammar grows per chunk. A §12 Decisions Log entry records the new closed values.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — nextest mutants profile and toolchain source
**Section:** §3 Bootstrap test-runner-install · §9 CI Integration (tool paragraph; Matrix builds → Language version)
**Change:**
- `[profile.mutants]` is `fail-fast = { max-fail = 1, terminate = "immediate" }` with a 15s×2 slow-timeout.
- rustfmt and clippy come from the `rust-toolchain.toml` components, installed by `rustup toolchain install` (was `dtolnay`).
- The language version is the exact 1.98.1 pin.
**Why:** a caught mutant was graded Timeout under `fail-fast = true` (auto timeout 108 s, measured); the chunk shipped the rustup step.
**Kept:** the nightly fuzz toolchain and the MSRV 1.96 job are separate toolchains, not changed here.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — mutation gate prebuilds the root bins (post-commit CI fix)
**Section:** §3 `run` step 4 (Command)
**Change:** `run --mutants` first runs `cargo build --package viola --features fake-agent`, which fails with `reason:"build-failed"`, and then passes `--copy-target=true` to cargo-mutants, so the scratch tree carries the root `viola` / `viola-fake-agent` bins that the harness tests spawn.
**Why:** cargo-mutants 27.1 scopes the baseline to the packages the diff touches: a diff touching only `crates/viola-e2e` failed its baseline with `fake agent: NotFound`, exit 4 (measured). `test_workspace` / `test_package` in `.cargo/mutants.toml` and `--test-workspace=true` did not widen that scope (measured). The operator chose prebuild + copy-target (founder-delegated). Cost: one `target/` copy per run, 2.9 GB on the dev host.
**Kept:** moving the tests and `--in-place` were weighed and declined.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-fake-agent-and-test-data-fixtures — mutation verdict for Rust-free diffs
**Section:** §3 `run` step 4 (Classification bullet added; Verdict scoped to the `counted` arm) · §3 `run` Output format (`mutants` object) · §3 Closed enums (`mutants.verdict`) · §10 Mutation gate · §12 Decisions Log (new entry)
**Change:**
- The harness classifies `chunk.diff` by its `diff --git` headers.
- A diff with no `.rs` path (an empty one included) never reaches cargo-mutants. It passes as `{"tested":0,"verdict":"no-rust-delta","diff":"target/agent-run/chunk.diff","files":N}`.
- A Rust delta deletes a stale `mutants.out/outcomes.json` first and reports `"verdict":"counted"`. With no fresh `outcomes.json` it is red.
- New closed value `counted` | `no-rust-delta`.
- §10's "tests-only diff = `{"tested":0}`" is retired.
**Why:** cargo-mutants 27.1.0 exits 0 on a Rust-free diff and leaves `mutants.out/` untouched, so CI read `outcomes-missing` and a dev-host baseline read a stale `"tested":8`. Operator P1 constraint.
**Kept:** the test-plan base-missing rationale and the obs-plan artifact list stand unchanged.
**Ref:** .andromeda/runs/2026-09-24T08-45-42-wrap/

## 2026-09-24-fake-agent-and-test-data-fixtures — fake-agent contract as built, consumer-first
**Section:** §7 Fake agent · §7 Fixture hygiene · §7 seed table (recorded-payload row) · §3 `run` step 2 · §12
**Change:**
- Hooks are read from `<plugin-dir>/hooks/hooks.json` (was `plugin/` + `settings.json`). Only an absolute exec-form command runs, and matchers are not yet evaluated.
- Payload `<fixtures>/<cli-version>/<Event>.<variant>.json`, with only `prompt` set for UserPromptSubmit.
- Receipt kinds and fields listed; script schema `schemas/fake-script.v1.json`.
- Modes built vs deferred: `--vt100-panic-bytes`, `statusline-echo`, `agents --json` land with their consumers.
- The hygiene walk covers `fixtures/fake-scripts/*.json`, with the class-only checker. The `fixtures/claude` walk joins with the first recorded fixture.
- Only the sync root chain exists (`viola-test-*` homes); the `viola_e2e::fixtures` copy lands with its first consumer, and the root `stamped_home` is an interim no-stamp seam.
**Why:** the contract as built; operator P4 direction: consumer-first, no shapes invented before a recorded fixture.
**Kept:** `viola run` rewriting its own plugin folder; the sequencing lines that describe the target state, their owners pinned as CARRYs on the route.
**Ref:** .andromeda/runs/2026-09-24T08-45-42-wrap/

## 2026-09-24-fake-agent-and-test-data-fixtures — fixture naming and `--fixtures` root (operator-resolved escalations)
**Section:** §2 File naming · §7 seed table · §3 `boot` step 5 · §6 Path 4 step 1 and scenario step 3
**Change:**
- Fixtures are named `<Event>.<variant>.json` with the CLI's PascalCase hook event name (`PreToolUse.ask-question.json`), with no mapping table.
- `boot` passes the parent `--fixtures <root>/fixtures/claude`, and the fake agent joins `<cli-version>`.
- `--plugin-dir` comes from `viola run` (architecture §Occupied Resources).
**Why:** two doc-vs-code shape conflicts were escalated; the operator (wrap P2) resolved them: code wins where the doc invented a shape, and no mapping tables.
**Kept:** directory references to a version dir (not the argument) stand unchanged.
**Ref:** .andromeda/runs/2026-09-24T08-45-42-wrap/

## 2026-09-24-supply-chain-and-workflow-gates — Lint row from sync-crates.txt, Supply-chain stage, wrappers sentence retired
**Section:** §2 trigger map (Supply chain V9 row) · §9 Pipeline structure (Lint row + new Supply-chain row, paragraph after the table) · §9 Build failure conditions · §12 Test crate deviation
**Change:**
- The Lint row's `cargo check` reads one `-p` per crate in `scripts/sync-crates.txt`; an empty list fails.
- A new Supply-chain row covers the ubuntu job `supply-chain`: cargo deny, the sole-root `deny-sync.toml` tokio ban, `scripts/deny-probes.sh`, zizmor, and the JSON artifact `supply-chain`. It also covers the weekly `nightly.yml` advisories job.
- The §2 V9 location now names both stages plus `nightly.yml`.
- The least-privilege sentence now covers both workflows. The cargo-deny 0.20 CLI form (global `--config`, `check -c` rejected) is recorded as measured on 0.20.2.
- Failure conditions: the deny and zizmor findings move under a new Supply-chain bullet, which also adds the sole-root and probe failures and the nightly run.
- §12: `viola-e2e` is no longer "added to the cargo-deny tokio wrappers list". It is never a sole root of the tokio ban.
**Why:** the chunk falsified the wrappers mechanism, and its harness/gate surface gives the new jobs.
**Ref:** .andromeda/runs/2026-09-24T09-41-13-wrap/

## 2026-09-24-observability-gates — internal gate subcommands, exit-aware 10 s readiness, mutants kill below the floor, runner jq and pinned ripgrep, scan-gated uploads
**Section:** §3 `run` step 2 (root `booted_wrapper` readiness) · §3 Internal harness subcommands (new `schema-check`, `secret-scan`; `gate` CI-upload sentence; Closed enums) · §3 Bootstrap phases test-runner-install (`[profile.mutants]`) and ci-tool-install · §9 tool-install paragraph · §9 Test report format
**Change:**
- Root `booted_wrapper` readiness is bounded at 10 s (was 20 s) and exit-aware: it fails at once when `child.try_wait()` reports an exit. Harness `boot` keeps 20 s.
- `schema-check` and `secret-scan` are declared as internal subcommands, with their output shapes, reasons and closed class enum.
- `[profile.mutants]` slow-timeout is 5 s × 2, with a `package(viola-e2e)` override at 15 s × 2. The cargo-mutants auto-timeout floor is recorded as measured.
- jaq leaves CI installation; G2 uses the runner-provided, presence-checked `jq`.
- ripgrep 15.2.0 comes from `scripts/install-ripgrep.sh`.
- The `agent-run-<os>` upload and the "raw junit.xml is never uploaded" clause are retired. In their place are the scan-gated `harness-`, `diag-` and `junit-<os>` uploads and `secret-scan-<os>`; there is no unscanned `agent-run-<os>` upload.
**Why:** a CI mutants red was two `wait_ready` consumers spinning to the 20 s bound under cargo-mutants' 20 s floor. Operator decisions at phase P4: runner jq; deadlines fixed by cause and by value.
**Kept:** the other `20 s` sites: harness boot, the fixture timeout derived from boot, and the "boot's 20 s / 10 s deadlines" line.
**Ref:** .andromeda/runs/2026-09-24T13-07-17-wrap/

## 2026-09-24-quality-gates — per-job gate, coverage JUnit and regex, two-leg mutation union, seeded fuzz replay, rustup MSRV
**Section:** §2 (Property-based row) · §3 (preamble usage details; `run` body, `--coverage`, `--fuzz-replay`, exit semantics, Output format; `gate`; Bootstrap `ci-tool-install`; Closed enums) · §6 (Property suite) · §9 (Coverage report, Mutation, MSRV, Fuzz replay and Quality gates rows; tool install paragraph; Matrix builds; Test report format) · §10 (Stack adjustments; Mutation gate) · §11 (CI ×2) · §12 (new entry)
**Change:**
- The coverage JUnit source is `target/nextest/ci/junit.xml` (was `target/llvm-cov-target/…`, which never exists).
- The coverage ignore regex is separator-agnostic (`crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`, harness `COVERAGE_IGNORE`). This is not a widening.
- `run` gains `--leg` (per-leg `mutants-verdict-<leg>.json`, survivors deferred to the gate), a `detail` field, the llvm-cov failure codes, and `--fuzz-replay` on the `fuzz/rust-toolchain.toml` channel (`fuzz-linux-only` off Linux; `tool-missing`, `corpus-empty`).
- `gate` gains `--mutants-legs`, the union rule, fixed `detail` codes and usage `unknown-suite` / `invalid-leg`. The mutants legs defer their gate to `mutants-verdict`.
- §9: mutation is a two-leg matrix plus `mutants-verdict`; MSRV and fuzz use rustup (was dtolnay); `mutants.out/` is not uploaded; the fuzz-replay job and the nightly fuzz job are recorded.
- §6 / §2: `viola_name` is the eighth, pre-parser seed target in the separate `fuzz/` workspace.
- §12: one dated entry (union verdict, seed target, declined `concurrency:`, rustup toolchains, new closed values).
**Why:** the JUnit path and the Windows regex are measured; the cargo-mutants `#[cfg]` limitation explains a CI run's `file_mode` misses. Operator P4 decisions: the windows leg with a union verdict, and seeding the fuzz pipeline now. P5-approved leans: rustup, no `concurrency:`, `mutants.out/` removed.
**Kept:** the `mutants.out` / `outcomes.json` references that name the local `run --mutants` verdict source are still true.
**Ref:** .andromeda/runs/2026-09-24T14-48-15-wrap/

## 2026-09-24-workspace-tree-and-code-graph-planes — Lint row disposed, release-check job, fuzz lock audit, jq consumers
**Section:** §1 (dependency and boundary policy entity; the traced-to note) · §2 (V9 row) · §3 Bootstrap (`ci-tool-install` jq bullet, `quality-gate-config-emit`) · §9 Pipeline structure (Lint, Supply-chain, Release build rows; tool-install paragraph) · §9 Build failure conditions · §10 Build failure conditions · §12 (new entry; the Initial entry's "No separate test crate" request marked retired)
**Change:**
- Lint row: the orphans gate `scripts/orphans-check.sh` (+ `--probe`) runs per lib/bin target.
- `--acyclic` is an on-demand review with its measured reason; the rmcp `cargo tree` assertion joins with the "MCP server for drivers" chunk.
- Supply-chain row: the `fuzz/Cargo.lock` advisories + sources step (from the repo root, into `deny-fuzz.json`); weekly advisories cover both lockfiles; the artifact is admissible by content.
- Release build row: the `release` job runs `scripts/release-check.sh --probe` then the default mode (`--locked --bin viola`, judged on its own artifact records); a plain exit-code job.
- Build failure conditions: the cycle and rmcp-release-graph conditions are retired; the per-target orphan, fuzz lockfile and release-build conditions are added.
- §3: the gate-as-last-step rule (was "last step of every job") is scoped to suite jobs (`lint`, `supply-chain`, `release` are plain exit-code jobs); the gate list names orphans-check, the fuzz lock audit and release-check; the jq bullet lists the two new consumers with the runner-image versions read.
- §1 and §2 updated to match; the §1 "No separate test crate" note re-derived.
- §12 gains this chunk's entry, and the Initial entry's "needs updating" request is marked retired.
**Why:** measured at cargo-modules 0.27.0: `--acyclic` exits 1 on 3 of 4 targets, each a type ↔ its own inherent method, whatever the filters; `cargo tree … | grep -c rmcp` = 0 at HEAD. Operator P4 decisions 1 and 3.
**Ref:** .andromeda/runs/2026-09-24T16-23-20-wrap/

## 2026-09-24-epoch-1-cleanup — mutation leg streams its progress; an unviable swamp is red
**Section:** §2 (Mutation row) · §3 `run` step 4 (Base, Command, Verdict), Exit code semantics, `gate` mutants breach, Bootstrap `quality-gate-config-emit` · §9 (Mutation row, Build failure conditions) · §10 (Mutation gate, Build failure conditions) · §11 (Quality) · §12 (new entry)
**Change:**
- The `run --mutants` invocation gains `--caught --unviable --build-timeout-multiplier=5`. cargo-mutants' stdout streams live to the harness's stderr, so every outcome line reaches the CI step log as it happens.
- The counted verdict requires `unviable <= caught`. A run with more unviable than caught mutants is red with `failures[]` code `unviable-exceeds-caught` and `failed` = survivors + 1. Under `--leg` it is not deferred to the union.
- §10 retires "`unviable` mutants are reported but do not fail the gate": a few do not, and outnumbering the caught ones is red. The measured threshold rows are in the body.
- The `gate` union parenthetical is scoped to the union. The §2/§9/§10 failure lists, `quality-gate-config-emit` and the §11 counting ban name the new condition.
- §3 Base adds the force-push case: a replaced `github.event.before` is reachable from no ref, so the run is `base-missing`. The remedy is a rewind to the chunk base, then a fast-forward.
- §12 records the decision, including the operator's DECLINE of a reduced partial-verdict upload for cancelled legs (a decision, not a deferral).
**Why:** a CI windows leg read `8 caught, 135 unviable` as `ok:true` because a leaked supervisor locked `viola-harness.exe` (relink `os error 5`, reproduced locally); another windows leg was silent for 2 h 45 m while the output was captured. §10 sits under Founder Direction 1; the tightening was ratified by the founder-delegated overseer.
**Kept:** the leg-verdict `outcome` shape and the closed enums (no new value); the quality-gates chunk's historical §12 entry.
**Ref:** .andromeda/runs/2026-09-25T11-29-18-wrap/

## 2026-09-25-security-prerequisites — the `test-only-rust-delta` mutation verdict; the SQOS open and content hash pinned
**Section:** §3 `run` step 4 Classification; §3 `run` Output format (`mutants` object, leg file); §3 Closed enums; §6 Security control negatives → Windows client SQOS; §6 Contract suite; §10 Mutation gate; §12 (new `2026-09-25` entry).
**Change:**
- Every place that listed the verdicts now carries the third closed value `test-only-rust-delta` (the closed set goes from 2 to 3; was `counted`, `no-rust-delta`). It applies to a diff whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`, root or `crates/<member>/`), which never builds or runs cargo-mutants and names the diff, its file count and `rust_files`. A mixed diff stays `counted` and red `outcomes-missing` without a fresh `outcomes.json`.
- §6 records `tests/channel_sqos_open.rs` (recipe + no-SQOS control) ahead of the viola-client negative, and `tests/contract_content_hash.rs` in the Contract suite.
- §12 carries the decision, refining the chunk-2 ruling (its dated 2026-09-24 §12 history left as written).
**Why:** cargo-mutants 27.1.0 measured: `No mutants to filter` over a tests-only diff; `--list-files` lists `src/` only. Operator ruling "option A" (fold the fix, 4 conditions).
**Kept:** obs-plan names no verdict value, so it needs no change.
**Ref:** .andromeda/runs/2026-09-25T13-11-43-wrap/

## 2026-09-25-pty-wrapper-on-windows — 11-name identity floor, no-EOF measured per OS, harness on outer PTYs, fake-agent receipts
**Section:** §1 inherited credentials entity · §3 boot step 5, run step 2 (`booted_wrapper`), cleanup step 1, `supervise` · §4 viola-agent-claude unit oracle · §6 E2 steps + verification · §6 Chaos (no-EOF mode, `.cmd` child) · §7 fake agent receipt kinds + `--exit-no-eof` · §11 Unit anti-pattern
**Change:**
- The 14-name S6 literal is replaced by the 11-name identity floor measured on the Windows host plus the prefix / persistent-set rule (§1, §4, §6 E2 steps, §11); "the product's own strip list" is now `IDENTITY_FLOOR`.
- E2 verification: persistent-set survival (Unix), `env_stripped_count` ≥ 12 / `env_stripped_known` / `env_kept`, 0 canaries; the `VIOLA_*` presence half re-pinned to "Instance state and start order" and the Unix fds-only half to "Wrapper channel" (their route CARRYs).
- No-EOF mode as measured in CI: output outlives the leader natively on ConPTY, on Linux only for a holder in its own process group, never on macOS; the held-output half is asserted on Windows + Linux. `--exit-no-eof` holds the child's console / PTY slave (not viola's stdout), writes `hold`, and uses its own process group on Unix.
- Harness: `supervise` owns an outer PTY per instance (interim stdin pipe retired), cleanup re-presses Ctrl-C every 500 ms, `booted_wrapper` runs over `OuterPty` and waits for the fake agent's `start` receipt.
- Fake-agent receipt kinds gain `size`, `cwd`, `hold`; `start` follows its `HostTerminal` guard. The `.cmd` child case landed in `tests/cli_program_resolution.rs`.
**Why:** the chunk disproved the earlier spec claims; overseer direction 4 binds later chunks: macOS is measured wherever a master claims output outlives the session leader.
**Kept:** "statusline stdin piped" is unrelated; the test-plan half of the "viola's own bytes" amendment found no site.
**Ref:** .andromeda/runs/2026-09-25T17-43-18-wrap/

## 2026-09-26-ci-chunk-base-and-union-verdict — derived whole-chunk base, compiling-leg union, chunk.diff out of the scan
**Section:** §3 `run` step 4 Base · §3 `run` Output format (`mutants` object) · §3 `secret-scan` Scope · §3 `gate` union bullet and CI paragraph · §6 Error sanitization and secret scan (Canary) · §9 Mutation row · §9 Test report format (`harness-<os>`) · §10 Mutation gate · §12 new 2026-09-26 entry
**Change:**
- Base: the harness derives the whole-chunk base (the last master flip at or before the parent of the oldest `chore({marker}): operator pre-CI commit` of a pending chunk, HEAD when none; `HEAD^` on the wrap push; `merge-base HEAD origin/main` fallback; full sha or `base-missing`). `AGENT_RUN_CHUNK_BASE` is an explicit override for tests; CI passes only `LEG`, no `github.event` value. The unbounded-pickaxe hazard is stated as measured. The `/implement`-sets-it, event-base (`github.event.before` / `pull_request.base.sha`), `git cat-file -e` and force-push sentences are retired with the event base.
- Every `run --mutants` document form carries `"base":"<sha>"`; the leg file is unchanged.
- Union: each mutant judged only by the legs whose `#[cfg]`s compile its line (gate-side `syn` over the checked-out source; a leg dropped only on a cfg proven false; empty set → every leg); a miss on a compiling leg stays red (was: red only when no leg caught it). §10 carries a CI-run replay witness.
- `secret-scan` Scope and §6 Canary: every file under `target/agent-run/` except exactly `target/agent-run/chunk.diff`; §3 CI paragraph and §9 Test report format: the `harness-<os>` upload excludes the same file.
**Why:** the chunk disproved the event-base and any-leg-union claims; overseer wrap direction (the four test-plan amendments); founder ruling 2026-09-25 (V15 instance half).
**Kept:** the historical §12 entries (2026-09-24 skeleton, 2026-09-25 cleanup, declined `concurrency:`) stand as written; no other §1–§11 restatement of the union mechanism exists.
**Ref:** .andromeda/runs/2026-09-26T20-59-23-wrap/

## 2026-09-26-local-linux-pre-push-gate — `pre-push` local Linux gate, uncommitted-promotion base rule, pump resize baseline
**Section:** §3 `run` step 4 Base · §3 Internal harness subcommands (`pre-push`) · §3 Closed enums · §5 Module ↔ PTY · §9 tool-install paragraph · §10 Mutation gate · §12 Decisions Log `2026-09-27`
**Change:**
- Base: the `HEAD^` step applies only when the working tree's master holds no pending record HEAD's copy lacks; with an uncommitted promotion on top of the flip, the flip itself is the base (as measured: the old rule derived acd08c7 where CI derived a69c5ef).
- New internal subcommand `pre-push` (Windows host, WSL2 `Ubuntu`, `--exec /usr/bin/env -i`): stages `host · tools · sync · cache · linux-tests · linux-leg · windows-leg · union`, the synced history-carrying clone, the document shape and exits; its closed `cmd` / `reason` / `detail` / `stage` values in Closed enums.
- Module ↔ PTY: a resize landing between the spawn sizing and the pump's first look is propagated — the pump's baseline is the spawn size (forced-window test via the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam; 6/6 red before, 6/6 green after).
- §9: the WSL provisioning installs the same runners from ci.yml's `test`-job pins, so each tool keeps exactly one version source.
- §10: before the operator push the local `pre-push` runs the same two legs and union.
- §12: the `2026-09-27` entry (gate, base rule, folded red and its measured cause, witness).
**Why:** the chunk disproved the earlier base rule and resize claim, and built the local gate.
**Kept:** the dated §12 entry for the earlier base rule stands (history, still true for CI).
**Ref:** .andromeda/runs/2026-09-27T00-51-08-wrap/

## 2026-09-27-instance-state-and-start-order — staged readiness, liveness by process check, mutation runs keep no home, pre-push scratch
**Section:** §1 (`boot` readiness, critical path 1, PID file location) · §2 pyramid (Performance row) · §3 `boot` Readiness, exit-code grammar, `run` step 2, `run` step 4, `cleanup` step 6, Test data bootstrap (Cleanup), `pre-push` (stages, sync, document), interim list · §4 `viola-state` liveness · §6 Path 1 · §10 heartbeat flip row
**Change:**
- Readiness staged: the two `process-start` lines first, then the snapshot (`pid`, `started_at`, `child_pid`; `endpoint` from "Wrapper channel") and a heartbeat < 5 s (missing `<name>:snapshot` / `<name>:heartbeat`); `events.ndjson` lines 1–3 and the Path 1 third record / M6 witness join with "Hooks to normalised events". The root `booted_wrapper` counts starts newer than the boot, then the snapshot + heartbeat.
- Liveness: `classify(beat_age, same_process)` fed ages (no clock) — pid/start-time mismatch → `gone` at any age; same process 5.0 s live, 5.1 s stale.
- `run --mutants`: `AGENT_RUN_KEEP_HOMES=0` / `AGENT_RUN_KEEP_FAILED=0` on the `cargo mutants` command (CI's workflow-wide `=1` overridden there); fixture owner record + gone-owner sweep (pid + start time only).
- `pre-push`: Linux leg `TMPDIR=<distro home>/viola-pre-push-scratch` (wiped 0700 at `cache`); document `cache{…,scratch_bytes,…,scratch_bytes_after}`. `logs --kind` landed (no longer an unbuilt-selector example).
**Why:** the chunk as built. T17–T18 (crate-level `viola-state/tests/` round-trip suite) and T19–T20 (Path 1 `path_` E2E binary) were rejected as Sequencing deferrals and are pinned as CARRYs on "Self-healing state" and "The board: viola list".
**Kept:** the B1 ruling record stands as history; test-plan §3 and obs-plan §3 are consistent.
**Ref:** .andromeda/runs/2026-09-27T06-12-23-wrap/

## 2026-09-27-wrapper-channel — pre-push VM release and Windows tests, endpoint readiness, channel corr rules, E2 fd premise fix
**Section:** §1 (boot readiness; viola-channel coverage scope) · §2 (Property-based row; test directory conventions) · §3 (`boot`, `run --fuzz-replay`, `cleanup`, `pre-push` stages and document, Closed enums, Log format) · §5 (Module ↔ IPC; Wrapper channel oversize) · §6 E2 · §12 (new `2026-09-27` entry)
**Change:**
- `pre-push` stages add `vm-release` (`wsl.exe --terminate Ubuntu` once the ubuntu verdict is back) and `windows-tests` (`run --coverage` + `gate --require coverage,doctest` on the host; a red stops before `windows-leg`); host stages at `CARGO_BUILD_JOBS=16`; the document gains `vm{terminated,free_kib_before,free_kib_after}` and `windows{run,gate}`.
- `boot` requires snapshot `endpoint` (missing `<name>:endpoint`); `cleanup` reports `endpoint_gone` (Windows: client connect NotFound; Unix: socket path absent) — was null.
- Log format: `channel-*` corr null for an id-less `hook.event` and on a -32700/-32600 response; a dialog `hook-invoked` and a `hook-decision` with `detail` may be null; `diag-line.v1.json` requires `corr` exactly on the listed lines.
- E2: was "fds only 0/1/2 plus the PTY slave"; now the live `/proc/<pid>/fd` table shows nothing of viola's (no socket, no path under the home, 0–2 the PTY; fixture files exempt by exact path) — measured `[0,1,2,3,4]`, and portable-pty `close_random_fds()` closes fds above 2.
- §5: DACL set from the GA SDDL and read back canonical FA, SID possibly an alias; Unix 0600 by chmod after the bind; an oversize close reads 0 (Windows), may reset (Linux) or EPIPE the tail write (macOS).
- Fuzz: `channel_frame` (7 seeds) beside `viola_name`; `tests/support/ndjson.rs` the one complete-lines reader.
**Why:** the wrapper channel chunk's measured facts. The E2 change is a premise fix, not a widening — the overseer's live answer at this wrap.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/

## 2026-09-27-epoch-2-cleanup — Windows host mutation scratch, scoped inner loop, run archive, keep-failed lifecycle guard, probe 5/5
**Section:** §2 (machine-parseable output) · §3 (Exit codes; `run` Command body, step 4 Command / Classification / Verdict, Output format, Test selection; `gate` Inputs; `pre-push` stages and document; Closed enums; Test data bootstrap Cleanup) · §4 (Test grouping) · §9 (Release build row; release-build errors) · §10 (Mutation gate) · §12 (new `2026-09-27` entry)
**Change:**
- `run --mutants` on a Windows host: the host mutation scratch `<repo parent>/viola-mutants-scratch` (guard → `scratch-refused`; wipe → `scratch-wipe-failed`; `TMP`/`TEMP` + `--output`); `mutants.out/outcomes.json` read from the run's own output dir; the counted/scoped object gains `scratch_bytes`.
- `--file <path>` (repeatable): `verdict:"scoped"` with `files`, never a leg verdict; with `--leg` usage `scoped-leg`. The one-file selector was raw `cargo mutants --file`; now `run --mutants --file`.
- The run document gains `archived` (`target/run-archive/<n>`, newest 10). `pre-push` `cache` gains `windows_scratch_bytes` / `windows_scratch_bytes_after`; `windows-leg` runs in the host scratch.
- viola-e2e's booted lifecycle tests: a `Booted` drop guard keeps a failing test's home under `AGENT_RUN_KEEP_FAILED=1`. §4: viola-e2e (no dev-dependencies) uses a labelled case table instead of rstest `#[case]`.
- §9: the release-check probe reads `5/5 refused, control clean` (was `3/3`), and the check also fails an artifact built with a test-only feature.
**Why:** the Epoch 2 cleanup chunk's measured facts (the temp copy follows `TMP`/`TEMP`: seen in the scratch mid-run, none in `%TEMP%`).
**Kept:** the §12 history entries that list the earlier verdict set stand (true when written).
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/

## 2026-09-27-browser-verdict-reachability — `run --browser` on every OS, pinned Node, npm audit, root waits at 7 s
**Section:** §1 (web-spa driver; harness run order; multi-os-compat) · §2 (E2E row; V9 row; Playwright naming) · §3 (`run` Command body, steps 2 and 3, Output `archived`, Test selection; `gate`; `pre-push` PATH, stages, document; Closed enums; Bootstrap `test-runner-install`, `ci-tool-install`) · §6 (Drivers web-spa) · §9 (Lint, Supply-chain, E2E, Coverage rows; tool-install paragraph; Matrix builds; Test report format) · §11 (CI ban) · §12 (Browser caching; new `2026-09-27` entry)
**Change:**
- `run --browser` runs on every OS and only when named (was: Linux under `--browser` or `--all`, `browser-linux-only` elsewhere); `--all` runs steps 1, 2, 4. The harness deletes stale reports, spawns `npm ci` (`npm.cmd`), a Chromium probe and `node …/cli.js test` (was `npx --prefix e2e-web playwright test`); `browser-missing` for a failed `npm ci` or absent Chromium. New `run` reason `browser-missing`; pre-push detail `node`.
- The root `booted_wrapper` bound is 7 s (was 10 s), `tests/support/watch.rs` `WITHIN`, with a streamed `viola-root-watch` report.
- CI: the per-OS `test` job runs the browser suite and gates `coverage,doctest,playwright` (was an ubuntu E2E job gating `playwright`); `junit-<os>` carries 2 paths; `supply-chain` + `npm-advisories` audit the npm lockfile; Node from `install-node.sh` (runner-image Node retired). Test selection passes `--config`.
- pre-push: `tools` checks `node`; `linux-tests` runs `run --coverage`, `run --browser`, `gate --require coverage,doctest,playwright`; `linux{run,browser,gate}`; the PATH gains `<home>/.local/viola-node/bin`. §6: headless on all three OSes; axe joins with Epoch 8. §2: `pipe-reachability.spec.ts` is the one non-layout spec.
**Why:** founder ruling W125; P4 operator forks 1–3; the 7 s bound below the 10 s mutants kill (a remove-the-guard pair: 7 s FAIL with the report, 12 s TIMEOUT with the output lost).
**Kept:** the §12 initial entry's "Playwright … on ubuntu, with @axe-core/playwright 4.13.0" stands as history; the new entry records the supersession.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/

## 2026-09-27-hooks-to-normalised-events — mutation target dir, readiness line 3 to :51, forced panic and perf to the "Hook perf gate" tail
**Section:** §1 (cli `boot` readiness; cli events order; hook Required test type) · §2 (Property and Performance rows) · §3 (`boot` readiness; `run` step 4 Mutation; `--fuzz-replay`) · §5 Module ↔ DB · §6 (Path 1; budget path step 2; Security sweep; Property suite) · §7 Fake agent · §9 Perf row · §10 Performance budgets
**Change:**
- `run --mutants` builds in its own target dir `target/mutants` (`CARGO_TARGET_DIR=<repo>/target/mutants` for the root pre-build, relative for cargo-mutants; was removed from the environment): a copied default `target/` kept the original `CARGO_BIN_EXE_*` paths, so root integration tests drove the unmutated binary.
- Harness `boot` readiness line 3 (`session-start{source:"hook"}`) joins with "Capability ledger and viola verify" (was "Hooks to normalised events"); Path 1 asserts records 1–3 + M6 through the sibling test; the fake agent fires SessionStart/default once after `start_receipts`.
- The forced-panic fail-open case moves to the "Hook perf gate" chunk with its trigger `FAKE_AGENT_HOOK_PANIC` (fake-agent only); the landed matrix is 11 cases, the panic path covered in-process by three unit tests.
- The hook perf rows, `--perf`, hyperfine and the CI perf job land with "Hook perf gate"; the gate stays provisionally 1.0 s, the hook's own `SPINE_DEADLINE` is 750 ms; interim bound = the matrix's `< 1.0 s`.
- Concurrent-append landed for the hook files (8 processes, 16 + 8 lines, 3 OSes) without a >4 KiB line; that half moves to the tail. Properties: hook stdin + prompt round-trip landed at 512 cases; `resets_at` → "Statusline pass-through". Fuzz `hook_stdin` (10 seeds) joined.
**Why:** the hooks chunk as built plus the P4 split and the P5 review. The `FAKE_AGENT_HOOK_PANIC` seam was ratified by the founder, live, on 2026-09-28 (relay: the Viola overseer); it needs its security-plan Decisions Log entry when it lands.
**Ref:** .andromeda/runs/2026-09-27T23-42-19-wrap/

## 2026-09-28-hook-perf-gate — `run --perf` built, `gate` requires four rows, the per-OS `perf` job
**Section:** §2 pyramid (Performance); §3 `run` (`--perf`), `gate` (perf breach, detail codes), Bootstrap `ci-tool-install` (G2 / jq); §9 Pipeline Perf row, tools paragraph, Test report format; §10 Perf run rules (Binary under test, Perf session, Status) and the perf table (`session-end`, `pre-tool-use` rows)
**Change:**
- `--perf` is named-only: probe → `target/perf` release build (`--features viola/fake-agent`) → ONE session `perf-<harness pid>` through the `PerfSession` seam (was two sessions, stamped + unstamped) → four rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`) with synthetic `--input` payloads (was fixture payloads) → zero-panic check → cleanup; suite `perf`, 6 passed on green. The verdict stays with `gate`.
- `gate --require perf` requires each row by name (`artifact-missing` per absent `perf-<hook>.json`; was the single `perf-*.json` breach).
- `pre-tool-use` untimed until the dialog-tier chunk; no async-tier row. Status: built (was "not built yet").
- CI: its own `perf` job (was "not yet in `ci.yml`"); G2 is `scripts/g2-zero-panics.sh` in the `test` and `perf` jobs; the `perf` job's scan-gated uploads join the report inventory, and no perf export lands in `harness-<os>`.
**Why:** the chunk built the arm and the job (operator P4 fork 1); measured: a kept-home run took the channel path for all 132 samples; host max 72.8–73.7 ms.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/

## 2026-09-28-hook-perf-gate — forced panic on the real binary, the over-4 KiB concurrent half, the controls table's interim shape
**Section:** §1 Test Scope Summary (hook stdin); §5 Module ↔ DB concurrent-append check, CLI `cli_controls_not_disableable.rs`; §6 Security sweep (the fail-open matrix and its summary)
**Change:**
- `hook_fail_open.rs` gains `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` and `hook_panics_append_whole_lines_over_4_kib_side_by_side` (was "11 cases and no forced panic", the forced panic pending); the seam `src/cmd/hook/seam.rs` carries its Decisions Log entry and is a row of the controls table.
- The concurrent-append check's over-4 KiB half landed: 8 forced panics, 8 whole role lines and 8 whole detail lines over 4 096 B, 3 OSes.
- `cli_controls_not_disableable.rs` as landed: the `FAKE_AGENT_HOOK_PANIC` rows `0` · `false` · `off` · empty × the oversize-stdin and malformed-json refusals, no panic line; the verb negatives and the completeness case join with `send` / `answer`.
**Why:** measured green at implement and in CI `test` on three OSes (ci#36390764600); the forced-panic case took 0.414 s on the Windows debug build; operator P4 fork 3 set the interim shape.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/

## 2026-09-28-cli-output-tokens — the plain-output witness for clap's own output
**Section:** §5 Integration Test Strategy → CLI
**Change:**
- `tests/cli_output_plain.rs` is listed: `--help` (exit 0) and a clap usage error (exit 2) under `CLICOLOR_FORCE=1` / `CLICOLOR=1` into a pipe carry no ESC byte; `--help` under the outer PTY carries no SGR with bold `1` or underline `4` (SGR parameters parsed, never the screen text); one case self-checks the parser; 3 OSes.
**Why:** the chunk landed the witness; red with clap `color` restored and green without it (archives 161/162), PASS on ubuntu/windows/macos in ci#36404931982.
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — verify's lines pinned by literal asserts, the fake agent's print mode, the hygiene walk over the first recorded set
**Section:** §1 Test Scope Summary (the arch CI/CD quotation; harness readiness); §3 `boot` readiness; §5 Integration CLI; §7 Fake agent (modes, `start` receipt) and Fixture hygiene
**Change:**
- §5 CLI: `tests/cli_verify.rs` (12 cases) pins `viola verify`'s six `[NN/06]` step lines, the `stamped` summary, the `unable:`/`hint:` pairs and `error: internal error` by literal asserts; the trycmd verify case arrives with a redaction-heavy consumer (the "`viola-verify` step counter" left the trycmd bullet).
- §7 Fake agent: print mode `-p/--print <prompt>` — start receipts, the four spine `default` fixtures (prompt set), `ok` on stdout, exit 0, never raw, no stdin; `DEFAULT_CLI_VERSION` stays 2.1.0. The `start` receipt follows the raw-mode guard only in interactive mode.
- §7 Fixture hygiene: a run-time directory walk (`claude_fixtures`, with its own walker test), not rstest `#[files]`; it also covers `fixtures/claude/*/*.json` (the 2.1.283 set) against `schemas/claude-fixture.v1.json` (was "join with the first recorded fixture").
- §1/§3: the `boot` line-3 readiness check joins with "Verify-stamped test homes and harness" (was "Capability ledger and viola verify", which landed with the harness unchanged); §1's arch quotation reads the real CLI only locally, `verify` in CI against the fake agent.
**Why:** rstest 0.27 refuses an empty glob at compile time (`rstest_macros-0.27.0/src/parse/rstest/files.rs:635`, measured at implement); the rest is the chunk's landed surface (report Changes). `contract_ledger_probes.rs` and the per-set insta pin stay in §5/§6 unchanged: the "Verify-stamped test homes and harness" and "Fake-agent drift contract" entries own them.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-mutation-testing-to-the-epoch-boundary — mutation moves to the epoch boundary; legs, union and pre-push mutation stages retire; the macOS exclusion
**Section:** §1 (tier justification, `run` order, required test type) · §2 (Unit, Mutation rows) · §3 (Exit codes; `run` body, step 4 Base and Command, exit semantics, output, test selection; `secret-scan`; `gate` signature, inputs, breaches, output, CI; `pre-push` stages, sync, document; Closed enums; `quality-gate-config-emit`) · §5 (oversize frame) · §6 (canary scope) · §9 (Coverage report, Mutation, Quality gates rows; matrix; report format; build failure) · §10 (Mutation gate; build failure) · §11 (CI) · §12 (new `2026-09-28` entry)
**Change:**
- No per-chunk, pre-push or CI mutation gate: `run --mutants` [`--file`] is named-only (no selector and `--all` run steps 1 and 2), and `/andromeda-code-audit` runs mutation at the epoch boundary; the verdict rule (`missed == 0`, `timeout == 0`, `unviable <= caught`) is unchanged.
- Retired: `run --leg`, `gate --mutants-legs`, usage `invalid-leg` / `scoped-leg`, the leg verdict file and its outcome enum, the `#[cfg]` union; `pre-push` stages `linux-leg` / `windows-leg` / `union`, reasons `verdict-missing` / `base-mismatch`, the document's `legs` / `gate` / scratch byte fields, the distro scratch dir. The §9 Mutation row reads "none in CI".
- §10: the two real-cargo-mutants harness self-tests are compiled out on macOS — cargo-mutants' baseline build in its copied tree holds ~77 s of an 80.9 s phase against 0.41 s outside it, as measured at CI run 36481260151; the cause stays unnamed, runner-side under the operator's ruling; kill 120 s and `retries = 0` unchanged.
- §5: the oversize-frame test also accepts ENOTCONN on the write (macOS, CI run 36482322449).
**Why:** the founder's 2026-09-28 17:59 ruling; the overseer's P4 answer removes the union with its last caller; the macOS arm and the ENOTCONN fold on the operator's words.
**Ref:** .andromeda/runs/2026-09-28T21-04-49-wrap/

## 2026-09-29-h2-conpty-resize-probe — Module ↔ PTY: a key after a resize waits for the observed size, and the H2 limit the tests do not cover
**Section:** §5 Integration Test Strategy → `Module ↔ PTY` row
**Change:** the row adds the key-free resize witness `spawn_reports_a_resize_to_a_child_that_reads_no_key`; the rule that a test writing a key after a resize sends it only once the child reports the new size (the red test `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` waits for its `size 120x40` line: 0 of 200 lost on the windows-2025 runner, ci#36529038462); and the measured limit these tests do not cover — a key written right after the resize returned was lost in 13 of 200 runner iterations (ci#36527891850), resize seen, key flushed, `dsr-cpr 0` — lost between the ConPTY input-pipe write and the Rust test child's console read, the cause within that span not established; whether the real `claude` (Node/libuv, `ReadConsoleInputW`) loses a key typed right after a resize is unmeasured and owned by the real-CLI verify entry.
**Why:** the H2 probe's document branch: no loss localised to a cause viola controls, so the red test is reshaped to wait on an observed event (never a timer, a retry or a skip — §10 Zero-flakiness budget) and says plainly that it does not cover a key racing a resize (the operator's ruling at the P5 review); the overseer's correction at the wrap keeps the cause unestablished and the product impact unmeasured.
**Ref:** .andromeda/runs/2026-09-29T06-18-44-wrap/

## 2026-09-29-h2-conpty-resize-probe — coverage: the harness self-tests' throwaway crate writes no profile into the outer run
**Section:** §10 Coverage thresholds → Stack adjustments (the `LLVM_PROFILE_FILE` propagation bullet)
**Change:** the bullet keeps its rule — child `viola` processes of the product stay in coverage because `LLVM_PROFILE_FILE` propagates, and `env_clear()` re-adds it — and names its one deliberate exception, which is not the product: the harness self-tests' nested cargo over the throwaway `mini` crate (package `viola`) removes `CARGO_LLVM_COV`, `LLVM_PROFILE_FILE`, `RUSTC_WRAPPER` and `__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS` through `test_support::uninstrumented`, as measured on the dev host (new `.profraw` per run of one such self-test: 17 inherited, 13 removed, the throwaway's two binary signatures gone). That the corrupt-header profiles failing the ubuntu merge (ci#36529038462, ci#36481260151) came through this channel is recorded, not established.
**Why:** a red met in this chunk's operator pass (919/919 tests passed, the merge failed), folded on the operator's word; the channel was carried from chunk 2026-09-28-mutation-testing-to-the-epoch-boundary, and closing it takes non-product binaries out of the product's coverage set.
**Ref:** .andromeda/runs/2026-09-29T06-18-44-wrap/

## 2026-09-29-verify-stamped-test-homes-and-harness — stamped root seam, stop waits for the endpoint, boot readiness line 3
**Section:** §3 `boot` Readiness signal (staged checks; `events.ndjson` bullet); §3 `run` step 2 (the root fixture chain; `WITHIN`)
**Change:**
- Root `stamped_home` runs `viola --home <h> verify -- <fake agent> --cli-version 2.1.283 --fixtures <root>/fixtures/claude`; `stamped` is true only on exit 0 and a last line `stamped 2.1.283  <n> pass  0 fail`, else the fixture panics with codes. Was "an interim seam (`stamped:false`) until `viola verify` exists". `StampedHome::unstamped(TestHome)` serves the unverified-path tests; `Wrapper::boot` always passes `--cli-version 2.1.283`, `--fixtures` stays per-test.
- `Wrapper::stop` / `stop_keep` wait for the exit and then for the recorded endpoint to be unconnectable (`wait_endpoint_gone`, the harness `endpoint_gone` rule); 9 root waits share `WITHIN` (was 8). An exit code is not the endpoint gone: on Windows the stopped wrapper's own pipe took a hook 25 ms after its exit line, as measured at ci#36532038635; why is recorded, not established.
- `boot` readiness checks `events.ndjson` lines 1-3 through `start_records` after snapshot, endpoint and heartbeat (missing `<name>:events`); was "`boot` checks lines 1–2 today".
**Why:** stamps come only from `viola verify` against the fake agent (§7 Seed strategies); the folded case_08 red showed "stopped" meant an exit code, not the endpoint gone. The pipe-name-collision hypothesis was falsified (the name carries the home).
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/

## 2026-09-29-verify-stamped-test-homes-and-harness — run --local-live specified; its suite, reason and codes closed
**Section:** §3 `run` (`--local-live` bullet; Output format); §3 `gate` (`--require`); §3 Closed enums; §12 Test Decisions Log (2026-09-29)
**Change:**
- `--local-live`: under `CI` (presence only, passed to `run_with` as `ci`) it is `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before any build, spawn or suite; otherwise, after the selected suites, `cargo build --workspace --features viola/fake-agent` into `target/harness`, then one `<harness bins>/viola --home target/e2e-home/viola-live-<pid>/home verify` against the real `claude`, both through the runner seam. Suite `local-live` passes 1 on exit 0 with each of the six literal row ids naming one `  pass` line and a `… 0 fail` summary; else it fails 1 with `build`, `verify-exit-<n>`, `verify-exit-none` or `row-missing`. It claims no H2 measurement.
- Output format `suite` gains `local-live`; `gate --require` takes the nine other values; Closed enums add suite `local-live` and its four codes, `run` reason `live-in-ci`, and the `boot` readiness missing codes list with `<name>:events`.
**Why:** the plan left the live run's binary unstated, so it builds first, as `--perf` does, and a verify that yields no exit code is named; a new closed value takes a Decisions Log entry.
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/

## 2026-09-29-verify-stamped-test-homes-and-harness — fake-agent default 2.1.283; perf session stamped; H2 owner re-named
**Section:** §7 Test Data & Fixtures → Fake agent (Modes); §10 Quality Gates → Perf session; §5 Integration Test Strategy → Module ↔ PTY row (H2)
**Change:**
- Fake agent `DEFAULT_CLI_VERSION` is 2.1.283, the recorded set's version (was "stays 2.1.0"); its default `--version` answer is `2.1.283 (Claude Code)`, and the harness `boot --cli-version` default is the same.
- The `--perf` session boots with `BootOptions { …, stamp: true }` and is stamped at boot step 4 (was "No unstamped session exists: nothing is stamped yet").
- The Module ↔ PTY row's H2 owner is the working-route entry "First live test and self-drive" (was "the real-CLI verify entry"); `run --local-live` does not claim it.
**Why:** the fake agent answers at the version its fixtures were recorded at, so the wrapper's version gate reads a stamped version; the H2 owner wording matches architecture [PTY].
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/

## 2026-09-29-sideloaded-conpty — the H2 pair, the sideloaded backend and its boundary tests
**Section:** §5 Integration → Module ↔ PTY; Critical Path 1 (the `run` start sequence); §9 CI → Pipeline structure (Coverage report row), Build failure conditions
**Change:**
- Module ↔ PTY: beside the inbox 13 of 200, the with/without pair from one windows-2025 run (image `windows-2025-vs2026` 20260828.587, ci#36563868040): sideloaded 0 of 200, inbox 14 of 200, no rate. Windows x64 root `viola run` tests host the child on the sideloaded `OpenConsole.exe`; viola-pty's tests, `OuterPty` and the harness `supervise` stay inbox. The boundary: `conpty_sideload` (5 cases) and the two-sided `restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load`. The piped driver answers DA1, and the forced-window resize test resizes on the wrapper's `process-start{claude-child}` line.
- Critical Path 1: the start sequence gains the ConPTY sideload after the pinned copy and plugin (fail-open).
- §9: on `windows-2025` the `ConPTY vendor verification` step runs before the coverage run, which covers the `conpty_sideload` binary; a verify or probe not ending its verdict fails the build.
**Why:** the chunk's measured acceptance and its new test surfaces (report Counts, Harness/gate surface, Coverage).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-sideloaded-conpty — the Windows x64 seeded-home carve-out
**Section:** §5 Setup / teardown lifecycle; §3 Test data bootstrap → Mechanism; §7 Seed strategies (On-disk product state); §12 Test Decisions Log (`2026-09-29`)
**Change:** was "every test home is a not-yet-existing path viola creates; on-disk state is never hand-written, the one exception the chaos tests"; now one carve-out: on Windows x64 `Wrapper::boot` and every piped or outer-PTY `viola run` start seed `<home>/bin/<key>/conpty/` through `tests/support/home.rs` `seed_conpty` (hard links, copy fallback, from one per-run `target/conpty-seed/<key>/` copy, byte-identical to the embedded companions), so the test creates those homes first. The product's first-start write stays covered by `tests/conpty_sideload.rs` and `run_viola_unseeded`, which never seed; the harness `boot` never seeds. The seed is §7's second exception.
**Why:** the first start's companion write cost a median 1 193 ms on the dev host under a parallel suite, which pushed first-start tests past their 7 s waits; the overseer chose a test-side seed and no bound was raised. The operator ruled the carve-out at this wrap (the overseer agreeing) with a route CARRY on the Epoch 6 entry "Home and code-bearing file integrity": once viola sets its own home DACL at creation, a seeded home would skip it, so the seed must then let viola create the home first.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-fake-agent-drift-contract — Harness prefixes: four, the cross-session tag included
**Section:** §4 viola-agent-claude (`prompt-submitted` normalisation, M2); §6 Critical Path 5 (the harness-injected turn)
**Change:** The M2 prefixes and Path 5's harness-injected turn were `<agent-message from=` / `<task-notification>`; now they are four — `<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message` — on the raw start with no trim. The escaped cross-session form is the one escaped tag that classifies; the tag mid-prompt or after a leading space stays `human`.
**Why:** the founder-ratified widening of the harness class (security-plan Decisions Log `2026-09-29`); `prompt_origin_files_the_cross_session_tag_as_harness` witnesses it, red on the unchanged two-entry list.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/

## 2026-09-29-fake-agent-drift-contract — The fake-agent drift contract, byte for byte; change-driven size; matchers and S8 moved
**Section:** §2 test pyramid (Contract row); §6 Contract suite; §7 Seed strategies and Fake agent (payload, `size` and `hook` receipt kinds, matchers); §12 Test Decisions Log (`2026-09-29`)
**Change:**
- The fake agent's hook sequences were "pinned with insta"; now `contract_fake_agent_drift` compares, per recorded `fixtures/claude/<ver>/` set (walked at run time, an empty walk fails), each print-mode hook's receipt `stdin_hex` byte for byte with that event's recorded fixture and the hook order with the spine literal SessionStart → UserPromptSubmit → Stop → SessionEnd, each `ran:true` / `exit_code:0`. insta stays for decision bodies; the seed table splits them from fake-agent transcripts, which have no snapshot.
- The S8 `annotations` assertion moves to "Dialog answers by dialog_id", where the question answer path lands.
- Fake agent: matchers were to land with this chunk; now with "Dialog answers by dialog_id" (no recorded fixture carries a `tool_name`). The UserPromptSubmit payload keeps the fixture's trailing newline. The `hook` receipt gains `stdin_hex` when it ran. The `size` receipt keeps its wording ("at start and whenever the size changed") and now holds by a watcher: written at once, then polled every 10 ms and written on every change, no key needed; interactive mode only.
**Why:** the recorded fixture files are the pinned artifact, and a snapshot copy would move with every `--record` refresh (P4 fork, the overseer agreeing); the contract witnessed the payload's missing newline red before its fix; the key-driven size sampling left a resize with no later key unreceipted (the H2 CARRY), witnessed red by `pty_resize_reaches_a_child_that_reads_no_key`.
**Kept:** insta for decision bodies; the jsonschema fixture walk unchanged.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/

## 2026-09-29-t15-07-57-wrap — registry migration (U35): the test-plan Decisions Log leaves the body
**Section:** §12 Test Decisions Log · §2 Test Strategy · §3 → 5-command implementation (its key file) · §4 Unit Test Strategy · §5 Integration Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:** the log moved verbatim to test-plan-amendments-archive.md (20 entries); 8 lifts:
- §2: why `crates/viola-e2e` is a test-only crate (tokio clients kept off the root and sync crates; never a tokio-ban root, no `wrappers` allowlist, no `--exclude`).
- §3 → 5-command implementation `boot` (hand-landed in its key file after the migration): the UI port race — `ui-port-taken`, never retried; `viola ui --port 0` requested from arch.
- §3 → 5-command implementation `pre-push` (the same): the operator pass runs it on the uncommitted tree before the pre-CI commit; a red stops the pass.
- §4: what the token-compare source-scan test asserts (compared only inside the single compare function).
- §5: the piped driver answers DA1 `ESC[c` with `ESC[?1;0c`; viola sends no DA1 answer.
- §9: caches — rust-cache only; npm and Playwright browsers uncached, and why.
- §9: no `concurrency:` block in `ci.yml` / `nightly.yml`; zizmor `concurrency-limits` declined (2 low).
- §10: why criterion 0.8.2 is not a gate; hyperfine is the perf gate.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/

## 2026-10-01-t12-19-55-wrap — web page entity and a11y lint: React + TypeScript
**Section:** §1 Test Scope Summary → the web page entity; §6 E2E Test Strategy → Playwright locator conventions; §9 CI Integration → E2E row (the a11y lint); §9 → Lint errors; §3 → 5-command implementation (Embedded-asset freshness)
**Change:**
- The web page entity was Lit 3.3.3 light-DOM `viola-*` elements; now React + TypeScript `viola-*` components in a built bundle, its toolchain OPEN (owned by the route's frontend-toolchain entry).
- The a11y lint runs eslint-plugin-jsx-a11y (was eslint-plugin-lit-a11y 5.1.1) over the `crates/viola-ui/` frontend sources; its version and source glob are OPEN.
- Component names scope text matches; the selector form of those names and of the `viola-*[data-*]` selectors follows the component mapping, OPEN.
- Embedded-asset freshness: no JS build step until the frontend-toolchain entry; how the bundle build joins step 1 is OPEN, owned by that entry.
**Why:** founder ruling of 2026-09-30, relayed by the overseer.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-02-epoch-2b-cleanup — test-data cleanup: owner record last
**Section:** §3 → Test data bootstrap (Cleanup)
**Change:** A root `TestHome` removes its home on drop through `tests/support/home.rs` `remove_owned`, which deletes `owner.json` last; the gone-owner sweep removes through the same function. Was "`TempDir` drop removes per-test homes". The keep path is stated as behaviour: a failing root test keeps its home only under `AGENT_RUN_KEEP_FAILED=1`, and CI's `AGENT_RUN_KEEP_HOMES=1` keeps every rstest home (was "calls `TempDir::keep()`"). Added: ownerless remnants still appear on runs failing at the D: dev volume's deadlines (0–3 per run, none on a C: copy), mechanism recorded, not established (M2, open).
**Why:** std's `remove_dir_all` stops at the first undeletable entry in listing order, so the record must go last for a part-way removal to stay reclaimable. The planned git-fixture read-only rule was not added: its premise was measured false (std deletes read-only files).
**Ref:** .andromeda/runs/2026-10-03T07-46-03-wrap/

## 2026-10-03-mutation-scoring-completion — pre-push native on the Linux host
**Section:** §3 → 5-command implementation (`pre-push` block; Closed enums `pre-push`) · §3 → Bootstrap phases (`ci-tool-install`) · §9 CI Integration (tool-pin paragraph) · §10 Performance budgets (Status)
**Change:**
- `pre-push` is Linux-host only, runs natively in the working tree (no clone, no sync), every child `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` from the repository root, `<home>` the passwd field 6 (two PATH-only probes). Stages `host · tools · linux-tests`, `ok:true` when `linux-tests` is green; the document is `{v,cmd,ok,reason?,detail?,stage,linux{run,browser,gate}}`, never a home or repository path.
- Closed enums: `reason` `pre-push-linux-only` (exit 2), `tool-missing`, `tool-pin-mismatch`, `linux-document-unreadable`; `detail` adds `passwd-home`; `stage` `host`, `tools`, `linux-tests`. Retired: `pre-push-windows-only`, `sync-failed`, `sync-mismatch`, the details `wsl-distro-ubuntu`, `source-path`, `patch`, `clone`, `fetch`, `reset`, `clean`, `apply`, `cache`, the stages `sync`, `cache`, `vm-release`, `windows-tests`, and the document's `sync`, `cache`, `vm`, `windows` sections.
- §9 and Bootstrap: `pre-push` checks the host's pins and installs nothing; the WSL provisioning sentences are retired. §10 perf Status: "`test` and the pre-push carry no perf step".
**Why:** the WSL gate retired with the Windows dev host (overseer, founder-delegated, at plan review: in place, no clone). These closed-enum changes are recorded here in place of a Decisions Log entry.
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/

## 2026-10-03-mutation-scoring-completion — run --mutants --package; the diff prefix pin; the boundary tier's form
**Section:** §3 → 5-command implementation (Command body; `run` step 4 Diff, Classification, Verdict, Test scope, Package; Output format; Closed enums; Test selection) · §2 Test Strategy (Mutation row) · §10 Quality Gates (Mutation gate)
**Change:**
- New `run --mutants --package <member>` (clap `requires = "mutants"`): one whole member, no Base, no `chunk.diff`, no Classification. The member must be `viola` or a `crates/<member>` whose manifest `[features]` declares `fake-agent`, named in lowercase ASCII, digits, `-`, `_`; anything else is `reason:"package-refused"`, exit 2, before any cargo. `cargo mutants --package <member> --features fake-agent [--file …] --test-tool=nextest --copy-target=true …` follows the root prebuild in `target/mutants`. Document `{"tested":N,"verdict":"package","package":"<member>"}` (+ `files`, + `scratch_bytes` on Windows). `mutants.verdict` adds `package`; `run` `reason` adds `package-refused`.
- Diff: both `git diff` argvs pin `--src-prefix=a/ --dst-prefix=b/` (a host `diff.mnemonicprefix` / `diff.noprefix` broke the header parse).
- Test scope: the viola-e2e scenarios run only against viola-e2e's own mutants (`--package viola-e2e`).
- §10: the boundary tier scores a whole member with `run --mutants --package <member>`. A mutant the host cannot compile or reach is "not measured here; owed to {route entry}" by coordinate, never "equivalent", and `missed == 0` reads over the measurable set. On the Linux dev host every mutation run takes a NOCOW btrfs `TMPDIR` (the `/tmp` quota and the reflink exec-bit loss, as measured at the chunk's `evidence/m3.md`).
**Why:** M3, a tested harness arm rather than a recipe (overseer, founder-delegated, at plan review). The not-measured vocabulary is the overseer's ruling. The `TMPDIR` rule is the overseer's direction.
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/

## 2026-10-04-windows-boundary-mutation-workflow — the mutants profile waits for running tests
**Section:** §3 → Bootstrap phases (derive for route / setup-project) · §3 → Test data bootstrap (Cleanup)
**Change:**
- `[profile.mutants]` was `fail-fast = { max-fail = 1, terminate = "immediate" }`; now `{ max-fail = 1, terminate = "wait" }`. `slow-timeout` (5 s × 2; viola-e2e 15 s × 2) is unchanged.
- New reason: the first failure stops scheduling and running tests finish, so their temp dirs and session guards drop. The slow-timeout kill bounds any hang below cargo-mutants' 20 s floor, so a caught mutant ends at the kill line and is never graded Timeout.
- The retired reason was that a plain `fail-fast = true` let a caught mutant hang into a Timeout grade; it now reads as history, bounded by the slow-timeout kill.
- Measured: 0 Timeout grades over 711 Linux viola-e2e and 508 Windows mutants; 170 vs 0 leftover temp dirs two-sided; after a full viola-e2e run, 38 `.tmp*` and 0 nested copies against 25 275 and 62. The 38 are 17 kill-path leftovers by design and 21 half-removed fixture git repos (a `terminate`-independent class). A full Linux viola-e2e run takes 78 m against 23 m, counts identical.
- Cleanup: the killed-test example was "nextest `terminate = \"immediate\"`"; it now names the slow-timeout kill or a mutant-made kill, with `wait` letting every other running test finish.
**Why:** a REVERSAL of the 2026-09-24 chunk-level locked choice, ratified by the overseer as operator (founder-delegated) on 2026-10-04 on the measured basis. It is the leak's mechanism fix, not a cleanup step. The 21-repo remainder is an `[inferred]` hypothesis owned by the next chunk.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/

## 2026-10-04-windows-boundary-mutation-workflow — the Windows leg of the boundary audit
**Section:** §9 CI Integration (Platform · Mutation row · the workflow / tool-pin paragraph · the concurrency note · Test report format `mutants.out`) · §10 Quality Gates (Mutation gate)
**Change:**
- Platform: `windows-mutants.yml` (dispatch-only) joins `ci.yml` and `nightly.yml`.
- Mutation row: was "none in CI"; now no push/PR job or gate. The audit's Windows leg is `windows-mutants.yml` (`workflow_dispatch` only, no inputs, `mutants (<package>)` per package on `windows-2025`, `fail-fast: false`, 120 min) running `scripts/agent-run.ps1 run --mutants --package <member> --file …`, report-only, no cache or upload.
- Workflow paragraph: three workflows. The new one's `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0` is asserted equal to the `test` job's line by `tests/contract_windows_mutation_scope.rs`.
- Concurrency note: three workflows, zizmor pedantic `concurrency-limits` 2 low → 3 low.
- `mutants.out` bullet: "no CI job runs mutation" → no push/PR job; the dispatch workflow uploads nothing.
- §10: the audit measures `cfg(windows)` code by dispatching the workflow, verdict `package`, never a gate. Its jobs read red until the audit classifies compiled-out `#[cfg(unix)]` twins (19 of 25 misses in run 37174673472; 0 timeouts over 508).
**Why:** founder ruling C2 (2026-10-04); the 2026-09-28 no-gate ruling stands.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/

## 2026-10-04-readiness-gate-and-timing-constants — the named spine bound, the vt100 property and fuzz target, the fixture-repo leak's cause
**Section:** §2 (Property-based row) · §3 → 5-command-implementation, bootstrap-phases-derive-for-route-setup-project · §6 (Property suite, cargo-fuzz paragraph) · §7 (Fake agent modes) · §10 (Spine deadline, perf table)
**Change:**
- §10 Spine deadline: the gate reads `viola_core::SPINE_DEADLINE` (1.0 s), imported by `viola-harness` `gate.rs` `perf()` (was a provisional `1.0 s` "until arch names it"); the hook's connect deadline is the provisional `CONNECT_DEADLINE` = 750 ms, asserted below it. The spine-hooks and `pre-tool-use` table rows read `max <` `viola_core::SPINE_DEADLINE` (1.0 s).
- §3 → 5-command-implementation: perf JSONs are judged against `viola_core::SPINE_DEADLINE`, breach text `{file} max {max} >= {bound}` (was `< 1.0` (`SPINE_DEADLINE_S`)); the fuzz-replay seed list adds `vt100_feed`: 6.
- §6 Property suite: the vt100 feed property landed at `cases: 512` (`screen::tests::feed_prop_a_caught_panic_poisons_until_resize`). cargo-fuzz: `vt100_feed` joined (6 synthetic seeds) with a silent panic hook over libfuzzer-sys's aborting one; a panic escaping the catch still aborts. §2 lists it among the replayed targets.
- §7 Fake agent: `--vt100-panic-bytes` lands with confirmed `send`'s chaos case; its bytes are measured (a 24×1 PTY + a wide character), so no new env seam.
- §3 → bootstrap-phases: the 21 half-removed fixture repos' cause is the detached `git maintenance run --auto` each fixture commit spawned; fixture repos run git with `-c maintenance.auto=false`. Measured on the `Pass` tests (4 and 5 per 200 rounds without, 0 and 0 with); the full mutation run under the fix is not measured.
**Why:** the readiness-gate chunk named the spine constant, landed the vt100 property and target, and carried CARRY 4's two-sided witness.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/

## 2026-10-04-confirmed-send-with-cl-1-records — Path 2 as landed; paste_text joins the fuzz targets
**Section:** §1 Test Scope Summary (the confirmed-`send` path) · §2 Test pyramid (Property-based row) · §6 Scenario Path 2 (Surfaces; the `local` and Playwright bullets) · §6 Property suite · §7 Fake agent (Modes) · §3 → `5-command-implementation`
**Change:**
- Path 2 as landed: `path2_send_confirms_with_cl1_events` covers cli, the wrapper channel and the receipt on three OSes; MCP `send` is owed to `:102`, SSE to `:131`, the web half and Playwright to `:139`.
- `local` was exit 0 `{confirmed:false, detail:"unconfirmable", cursor}`; now exit 13 `not-delivered`/`no-prompt-submitted` while no local-command row is compiled, never presumed delivered; `unconfirmable` (and the Playwright local line's `data-rb="unconfirmable"`) is owed to `:82`. §1's "a local command yields `unconfirmable`" says the same.
- Fuzz targets: `paste_text` joins (`validate_paste_text` against a per-char oracle over lossy UTF-8; 8 synthetic seeds, byte-exact under `.gitattributes`); the `validate_paste_text` property landed at 512 cases.
- Fake agent: `--vt100-panic-bytes` was "lands with confirmed `send`'s chaos case"; now built (after its `start` receipt it writes `e4 b8 ad` once and receipts nothing new).
**Why:** what confirmed `send` landed; the `local` outcome follows F2, the overseer's hold of the local-command rows with the typed probe.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/

## 2026-10-04-confirmed-send-with-cl-1-records — the second test-data carve-out: the chaos home (F4)
**Section:** §5 Integration (Setup / teardown lifecycle) · §3 → `test-data-bootstrap` (Mechanism; Cleanup) · §7 Test Data (Test data lifecycle, CI)
**Change:** homes were under `target/e2e-home` with one carve-out (`seed_conpty`); now a second: `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, a `viola-chaos-*` home under the system temp dir, because its forced vt100 feed panic writes a G2-counted `event:"panic"` line. That home is outside G2 (zero panics), G4 (schema conformance) and the secret scan; the test asserts its panic line and its `parse-rejected{vt100-feed, panicked}` line present itself.
**Why:** the founder's ruling, live, 2026-10-04, relayed by the overseer, naming all three scans.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/

## 2026-10-04-wait-and-last — Path 3 as landed
**Section:** §6 E2E Test Strategy → Scenario: Path 3 — `wait` / `last` (Surfaces involved)
**Change:** Path 3 now records its landed half: `tests/cli_wait_last.rs` (`path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn`, `last_survives_a_wrapper_restart`) over `fixtures/fake-scripts/path3.json`, and `tests/chaos_wait_vanish.rs`, cli + wrapper channel on all three CI OSes; the MCP steps owed to `:102`, `/api/sessions` to `:129`, the page STATUS to `:139`, the dialog kinds' end-to-end witness to `:78`.
**Why:** the scenario named four surfaces with no landed or owed status, overstating what this chunk covered.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — relayed dialog fixtures, the 2.1.287 default, Paths 3 and 4 as landed
**Section:** §2 Test Strategy (Contract row; the contract required-test line) · §6 Path 3 · Path 4 (surfaces, step 1) · §6 Security sweep → Windows `--home` outside `%USERPROFILE%` · §7 Fixture library · Recorded hook payloads · Fake agent (hook commands, modes) · Fixture hygiene · §3 → 5-command implementation (the root fixture chain)
**Change:**
- Fixture source was `viola verify` recordings only; now the spine is recorded and the dialog tier (`fixtures/claude/2.1.287/` `PreToolUse.*` / `PermissionRequest.*` for `ask-user-question` / `exit-plan-mode`, `RELAYED.md`) is relayed from the viola-lab prototype's live captures, `tool_input.plan` redacted, reviewed before commit, until `:82`'s re-probe; the contract suite checks both, the hygiene walk covers sets `2.1.283` and `2.1.287` and scripts `gated-turn` / `path3` / `path4`.
- The default CLI version was 2.1.283; now 2.1.287 (`DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home`, `Wrapper::boot`).
- The fake agent's matchers were "not evaluated yet"; now `hook_commands(…, tool)` evaluates a group's `matcher` against `tool_name`.
- Path 3: the `question` / `plan` end-to-end wake witness landed (was owed to `:78`); the `permission` one is owed to `:82`. Path 4 as landed: `tests/cli_answer.rs` over `path4.json` on three OSes (question with annotations, plan approve, plan revise via the PermissionRequest repeat, the question's repeat silent, a second concurrent dialog empty, `unknown-dialog` exit 13); `permission` unit / insta only, its e2e owed to `:82`; step 1 fixture names in the §2 form.
- Security sweep: the Windows creation half landed (`create_private_dir`, unit both OSes + `tests/cli_version_gate.rs`), the `RUNNER_TEMP` negative owed to `:111`; a Windows DACL test homes under `target/e2e-home`, never `%TEMP%`.
**Why:** print mode raises no dialog hook, so the founder ruled live for relayed fixtures (R1) and pulled the creation half forward (R3); the paths record what the chunk's tests cover.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — five perf rows on an unstamped session; the mutants build bound
**Section:** §2 Test Strategy → Performance / Load · §10 Perf run rules (perf session, status) · Performance budgets (`pre-tool-use` row) · §3 → 5-command implementation (`run --perf`, `gate --require perf`, `run --mutants` and its `--package` arm)
**Change:**
- Perf was four timed rows with `pre-tool-use` "untimed until the dialog-tier chunk" on a stamped session (`stamp: true`); now five rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`) on one unstamped session (`stamp: false`), so `pre-tool-use` times the unverified-CLI path; `gate --require perf` requires all five by name (`perf::ROWS`, re-exported as `run::PERF_ROWS`); the suite reads 7 passed on green (was 6).
- `run --mutants` (both arms) was `--build-timeout-multiplier=5`; now `--build-timeout=400` through the shared `MUTANTS_PROGRESS`, a fixed floor of 5 × the largest measured 78 s baseline, because the multiplier conflicts with it and derives a sub-second bound from a sub-second baseline.
**Why:** the dialog hook is now a registered event, so its latency row joins the gate; the build bound is the chunk's Red B fix.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-the-wheel — Path 5 as landed; the human-wheel controls row
**Section:** §6 E2E → Scenario: Path 5 (Surfaces involved) · §5 Integration → `tests/cli_controls_not_disableable.rs` · §3 → Log format
**Change:**
- Path 5: as landed, `tests/tui_wheel.rs` (the outer-PTY steps, the harness-turn case, the focus/mouse/resize case, the `^Z` case) and `tests/cli_wheel.rs` (`pause` / `release`, the `human-typing` refusals with detail `null` | `manual-pause`, hints without `release`, `release-from-driver` exit 20) cover tui, cli and the wrapper channel on all three CI OSes; on `windows-2025` the focus case asserts the platform fact (an injected mouse report takes the wheel, focus reports are swallowed), Unix the full assertion. The MCP `send` step and its refusal checks are owed to `:102`, the Playwright WHEEL cell to `:139`.
- The controls table: was "the four verb negatives … join when `send` / `answer` land"; now the human-wheel negative (`send` exit 10) has joined, the rest still to join.
- Log format: the `release-from-driver` corr rule names a `release` carrying a string `from` (another type is a plain `-32602`), matching obs-plan §3.
**Why:** the chunk landed Path 5's CLI and outer-PTY halves; the Windows clause is the founder's live ruling F-W3, relayed by the overseer; the Log format line keeps the tests↔obs §3 bind.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-04-the-wheel — the one-Rust-test example selects through --integration
**Section:** §3 → 5-command implementation (Test selection)
**Change:** the one-Rust-test example: was `scripts/agent-run.sh run --e2e --filter 'test(/path2_send_confirms/)'`; now `run --integration --filter 'test(/path2_send_confirms/)'`, a whole root test file through `binary(<stem>)`. `--e2e` selects nothing yet: it is a usage error (exit 2) until the first E2E binary lands with its filterset.
**Why:** measured this chunk — the plan's own gate written with `--e2e` exited 2 and was corrected to `--integration` before implement; `path2_send_confirms` is an integration-tier test.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-04-running-turn-refusal — Path 5 and Path 2 span the running turn
**Section:** §6 E2E → Scenario: Path 5 (As landed · step 8 · Verification signal) · Scenario: Path 2 (As landed)
**Change:**
- Path 5 step 8 boots the harness turn over the gated `fixtures/fake-scripts/path3.json`: a `send` while the harness turn runs, then its scripted `PostToolUse` and `Stop` released and a `send` after `turn-ended`.
- Path 5 signal: was "the harness-injected turns log `prompt-submitted{origin:"harness"}` and no `wheel` record" alone; now also, during the turn, `send` exits 13 `{"v":1,"refusal":"not-delivered","detail":"turn-running"}`, nothing typed, one `send-refused{refusal, detail}` with no `cursor` and no `send-issued`, and `send` exits 0 after `turn-ended`; with no Stop a bare `release` leaves the turn (13) and `pause` then `release` clears it.
- Path 5 As landed adds the harness-turn refusal and `tests/cli_wheel.rs` `path5_a_turn_left_running_is_cleared_by_pause_then_release`.
- Path 2 As landed: `path2_send_confirms_with_cl1_events` boots over `path3.json` and ends its first send's turn before the second; `send_after_a_confirmed_send_is_turn_running_until_turn_ended` covers the driver's own turn (exit 13, the `[/ ] unable … turn-running` line + `hint: a turn is running; viola wait builder first`, then 0 after `turn-ended`).
**Why:** a running turn now refuses `send`, so every test that sends twice must end the first turn with a scripted Stop (the fake agent's interactive submit fires none), and the closed hypothesis is witnessed on three CI OSes.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/

## 2026-10-04-running-turn-refusal — the turn-running control negative
**Section:** §5 CLI → `tests/cli_controls_not_disableable.rs`
**Change:** was "four control negatives"; now five: ESC in `send` text → 13 `control-character`; `answer` on an unstamped home → 12; human wheel → `send` 10; a running turn → `send` 13 `turn-running`; a 0770 `--home` → 21. As landed, the turn-running row (`send` during a running harness turn → exit 13 under every `FAKE_AGENT_HOOK_PANIC` setting) joined with this chunk.
**Why:** no setting may disable a control, and `turn-running` is now a control on a running turn, not only on a second send in flight.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/

## 2026-10-04-running-turn-refusal — the fake agent quiesces its hooks before it exits
**Section:** §7 Test Data & Fixtures → Fake agent
**Change:** was "It exits on `\x03`."; now it exits on `\x03` or at stdin EOF and, before exiting, waits for a hook still running and starts no other — as measured at this chunk.
**Why:** the agent is its PTY's session leader and its hooks sit in that terminal's foreground group, so its exit hung up an in-flight `viola hook` mid-exit: a truncated coverage profile, the `.profraw` WATCH's cause. Folded as a recorded widening on the overseer's founder-delegated word (provisional per the delegate rule); the WATCH closed on four consecutive green pre-push runs after the fix.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/

## 2026-10-05-real-cli-verify-probes — ten ledger rows, verify's interactive runs and the screen fixture class
**Section:** §2 Test pyramid (Contract row) · §3 → 5-command implementation (`boot` steps 4 and 5, `--local-live`) · §3 → Bootstrap phases (nextest) · §4 viola-agent-claude · §5 CLI (`cli_verify`, `contract_ledger_probes`) · §7 Fake agent · Fixture hygiene · Path owner sites (`send` local command, dialog captures, `permission` e2e)
**Change:**
- §3: boot step 4 runs verify against the fake agent with `--screens --turn-stop --trusted-root <workspace root>` (ten rows), and supervise passes `--screens --trusted-root <cwd>`. `--local-live` checks ten literal row ids (`LEDGER_ROWS`), unit-proven; its live firing at ten rows is owed to "First live test and self-drive".
- nextest `profile.mutants` overrides: was the `viola-e2e` override alone; now it also lists `test(/send_window_/)` and `test(/verify_window_/)` at 15 s × 2 (the ~11 s verify window cases carry no test-side deadline).
- §5: `cli_verify` went from 12 to 22 cases, `[NN/06]` → `[NN/10]`, and covers both interactive runs, the named refusal and no run dir left. `contract_ledger_probes` was "every set"; now 2.1.287 and 2.1.288 are stamped `10 pass  0 fail` and 2.1.283 is drift-only, with literal lists and every dir on exactly one list.
- §7 Fake agent: `--trusted-root`, `--screens` and `--turn-stop` (argv, no env); wrapper boots pass `--screens --trusted-root <cwd>`, and `boot_untrusted` passes no root. Print mode is the print probe's mode, no longer "the single probe".
- §7 Fixture hygiene: the walk adds the 2.1.288 set and walks `Screen.*.json` apart, against `schemas/claude-screen.v1.json`. It also refuses an email-shaped token and a seam-split username. Screens are signature-rows-only and checked scrub-as-detector, refused whole with a named code.
- §2 Contract row: screens are checked against `claude-screen.v1.json`. §4: signatures (`SIGNATURES`) feed the full gate on a verified CLI and verify's settle and record helpers.
- Owner sites: was `:84`; now local-command `unconfirmable` is owed to "Local-command and paste-framing rows", and the relayed dialog captures, the `permission` e2e case and its wake witness are owed to "Dialog rows and re-probe".
**Why:** the chunk built W1 + W5 of the founder's live three-way split (2026-10-05 05:58Z); the remaining work moved to the two new route entries.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/

## 2026-10-05-dialog-rows-and-re-probe — fourteen rows, recorded dialog variants, the verify_window_ class grown
**Section:** §1 contract trigger · §2 Test pyramid → Contract · §4 Dialog mapping · §5 `cli_verify` · `contract_ledger_probes` · §6 Path 3 / Path 4 · §7 Fixture library · Recorded hook payloads · Fake agent modes · Fixture hygiene · §3 → 5-command implementation · §3 → Bootstrap phases
**Change:**
- Rows: ten → fourteen; `[NN/10]` → `[NN/14]`; `contract_ledger_probes` ends `14 pass  0 fail` under `--dialogs`; `cli_verify` adds the no-replay four-dialog-row fail (the stale case count dropped); boot step 4 passes `--dialogs` (four runs); `LEDGER_ROWS: [&str; 14]`, the live firing at fourteen rows owed to "First live test and self-drive".
- Dialog tier: was relayed from the viola-lab prototype until this entry's re-probe; now recorded by verify's Runs C / D as `<Event>.<stem>-<n>.json` (12 per version), scrubbed and `unclean`-checked; the drift contract replays every variant; hygiene walks them; the relayed set superseded and kept.
- Fake agent: argv options three → five (`--dialogs`, `--stop-receipt-hold-ms`, capped at 1 000 ms).
- §4 / §6: the S7 approve is `allow` + `updatedInput` echoing the tool's input (was a bare `allow`); the `permission` end to end re-owed to "Permission end to end" over the recorded `PermissionRequest.permission-1.json`; `cli_answer` adds `v1-15`.
- Bootstrap phases: `[profile.ci]` gains two overrides — `verify_window_` tests 45 s (15 s × 3), the twelve verify-driven binaries 20 s (10 s × 2); a verify a test drives has no test-side bound; `WITHIN` 7 s stays elsewhere. The mutants `verify_window_` rationale reads four maximum waits (was ~11 s, two); kills unchanged.
**Why:** the chunk landed the rows and the re-probe. The 7 s bound gave way for verify-driven tests on the overseer's founder-delegated decision: four runs make a designed floor (about 1.0 s → 2.1 s median on the ubuntu coverage leg, seven 300 ms settles), not a regression. The approve form and the permission split are the founder's live rulings of 2026-10-05.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/

## 2026-10-05-permission-end-to-end — Path 4's permission kind end to end; the dialog wake witness complete
**Section:** §6 Path 3 → Surfaces involved; §6 Path 4 → Surfaces involved; §6 Path 4 → Steps (step 1); §7 Test Data & Fixtures (the fake-script hygiene walk)
**Change:**
- Path 4 Surfaces: was "the `permission` kind is unit / insta only (`dialog::tests`), its end-to-end case owed to 'Permission end to end'"; now the `permission` kind runs end to end over its own gated script `fixtures/fake-scripts/path4-permission.json` (the recorded `PermissionRequest.permission-1.json` / `PostToolUse.permission-1.json` under `fixtures/claude/2.1.287/`): `path4_permission_is_logged_once_woken_and_answered_by_id` (`allow` with no suggestion reaching the CLI, `v1-16`; `deny` + `message`; the PostToolUse `activity` line waking no `wait`; a `question` first raised by PermissionRequest logged once, its hook silent, a late `answer` exit 13 `unknown-dialog`) and `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` (exit 12, no body).
- Path 3 Surfaces: was "the `permission` wake stays unit-level, its end-to-end witness owed to 'Permission end to end'"; now the `permission` wake witness is end to end in `path4_permission_…`.
- Path 4 Steps: the prompt-2 permission step is, as landed, its own five-step gated script, not prompt 2 of `path4.json`.
- §7: the fake-script walk's committed set is `{gated-turn,path3,path4,path4-permission}.json`.
**Why:** the chunk landed both cases green on all three CI OSes; a separate gated script keeps `path4_dialogs_…`'s exact dialog and `activity` counts uncoupled.
**Ref:** .andromeda/runs/2026-10-05T15-56-38-wrap/
## 2026-10-06-local-command-and-paste-framing-rows — seventeen rows, the framing tier, the fake agent's two options, Path 2 step 3 as built
**Section:** §1 (Critical paths, confirmed send; the contract-test trigger) · §2 pyramid (Contract row) · §4 viola-agent-claude · §5 CLI (`cli_verify.rs`, `contract_ledger_probes.rs`) · §6 Path 2 (step 3, its verification bullets) · §7 (Fixture library, Seed strategies, Fake agent, Fixture hygiene) · §3 → 5-command implementation · §3 → Bootstrap phases
**Change:**
- Counts: seventeen `[NN/17]` step lines and `LEDGER_ROWS: [&str; 17]` (was fourteen); one stamped set, `2.1.287`, ending `17 pass  0 fail`; `2.1.283` and `2.1.288` drift-only. Boot step 4 and the contract pass `--dialogs --framing`. The live `run --local-live` firing is owed at seventeen rows.
- The framing tier joins the recorded set: `UserPromptSubmit.paste-<n>.json`, `SessionEnd.clear-<n>.json`, `SessionStart.clear-<n>.json`, recorded by Run B's three added pastes, four under `2.1.287`; the drift contract replays them byte for byte under `--framing`, and a set without them is run without the option and must exit 1.
- Fake agent: seven argv options (was five): `--framing` and the test-only hold `--paste-hint-ms` (capped at 8 000 ms). A replayed `paste-<n>` variant keeps its recorded `prompt`.
- §4: the unwrap's eight framing cases and the three named arm cases; a stamp of the fourteen older ids reads unverified.
- §6 Path 2 step 3: was "longer than the fixture's paste-wrap threshold, so the fake agent emits the form"; now the recorded `paste-1` variant replayed under `--framing` (the fake agent has no threshold); the wrapped half is `send_long_text_wrapped_by_the_cli_is_confirmed`, the literal-tag half is not built.
- `unconfirmable` for a local command is owed to "Local-command send outcomes" (was "Local-command and paste-framing rows"): the row is compiled, `send` does not consume it yet.
- Bootstrap key: the `verify_window_` class holds two cases; verify's quiet waits are ten (was seven when the kill was sized); no bound moved.
**Why:** the plan's inventory follows what the chunk landed and what its P4 split moved to the next entry. The 20 s kill stands on a measured reading: the longest verify-driven test read 9.98 s on the ubuntu leg in ci#37534758441.
**Kept:** `.config/nextest.toml` and `tests/support/verify.rs` still say "seven 300 ms settles" in a comment; the wrap touches no source, the correction is pinned on the next route entry.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
## 2026-10-06-local-command-send-outcomes — Path 2's local verdicts as landed
**Section:** §1 Critical paths → Path: confirmed `send` · §4 root bin (the send-confirmation matcher bullet) · §5 CLI (`tests/cli_verify.rs`) · §6 Path 2 → Verification signal (the `local`, Playwright and Human mode bullets) · §7 Fake agent (the Modes bullet)
**Change:**
- §1 and §6: was "`send` does not consume them yet … until then it is `not-delivered`" and "`local`: exit 13 … exit 0 … is owed"; now `local` is exit 0 `{confirmed:false, detail:"unconfirmable", cursor}` with `send-issued` then `send-confirmed{cursor, confirmed:false}`, no `send-refused`, no window, the command's text in no role log line (`send_window_local_command_is_not_presumed_delivered`, `/clear` and `/remote-control` each under `--json`).
- `/clear` on a verified CLI is confirmed by a `session-start` with cause `clear` and a new `agent_session_id` (`send_clear_on_a_verified_cli_is_confirmed_by_its_new_session`), proven on the recorded 2.1.287 `clear-1` variants; the live proof is owed to "First live test and self-drive".
- A slash text off the compiled list stays exit 13 `no-prompt-submitted` (`send_window_slash_text_off_the_list_is_not_delivered`).
- The forced-window pair `send_under_the_paste_hint_on_a_verified_cli` (two cases): `input-not-ready` with nothing typed under the fake agent's paste-hint hold, delivered without it.
- Playwright bullet: the "once "Local-command send outcomes" makes `send` return it" condition is retired; the web half stays owed to `:147`.
- Human mode bullet: `local` prints `[  ] unconfirmable  <name>  local command, no measured post-condition` on stdout, exit 0, nothing on stderr; a post-condition-confirmed `/clear` prints `[RB] read back`.
- §4: the unit cases `send_local_command_` (five functions, 13 cases) and the mirror writer `write_send_unconfirmable`.
- §5: `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal` beside the turn-screen modal case.
- §7: eight argv options (was seven); `--tag-turn-screen <phase>` added.
**Why:** the chunk landed `send`'s local-command outcomes, the measured readiness-gate reading and the control of the guard before verify's local-command paste.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/

## 2026-10-07-t05-47-07-wrap — H2's real-`claude` question re-pointed to "Windows-only live measurements"
**Section:** §5 Integration Test Strategy → the Module ↔ PTY row; §3 → 5-command-implementation (`--local-live`)
**Change:**
- Module ↔ PTY row: whether the real `claude` loses a key typed right after a resize is owned by the route entry "Windows-only live measurements", blocked on an interactive Windows host (was "First live test and self-drive").
- 5-command-implementation, `--local-live`: the H2 key-after-resize question belongs to the same entry (was "First live test and self-drive"). The live firing at seventeen rows stays owed to "First live test and self-drive".
**Why:** the founder split "First live test and self-drive" three ways and put the first live test on the Linux dev host (R-L1 and R-L2, live, 2026-10-07, relayed by the overseer). The overseer's answers at this wrap moved the Windows-only live items to a new Epoch 7 entry blocked on an interactive Windows host.
**Kept:** `/clear`'s live proof stays owed to "First live test and self-drive". The route coordinates in this plan moved by manifest only.
**Ref:** .andromeda/runs/2026-10-07T05-47-07-wrap/
## 2026-10-07-test-homes-off-the-contended-volume — the home base's keepers and the dev host's tmpfs backing
**Section:** §3 → 5-command implementation (`boot` step 2, `run --local-live`, `cleanup` step 6, `logs` Retention window) · §3 → Test data bootstrap (Mechanism, Cleanup) · §5 Integration Test Strategy → Setup / teardown lifecycle
**Change:**
- `boot` step 2: the base `target/e2e-home` is prepared before the `tempdir_in` by `Workspace::ensure_e2e_home`. A plain base is created as a directory; on Unix a linked base needs an absolute target, made 0700 when gone (one non-recursive create) and accepted only when one `lstat` reads a real directory with no group or other bit; Windows is the plain create. A refused base reads `reason:"build-failed"`; no reason added.
- `run --local-live`: the keeper runs after the build and before the verify. `verify-exit-none` was "no exit code"; now "no exit code, or a base the keeper refused, the verify not started". The four failure codes are unchanged.
- `cleanup` step 6: on the Linux dev host the `viola-session-*` removal goes through the link and deletes outside the working directory, only the dir the harness created.
- `logs` Retention window and Test data bootstrap Cleanup: on the dev host a kept home (`AGENT_RUN_KEEP_HOMES=1`, `AGENT_RUN_KEEP_FAILED=1`) is on tmpfs: gone at a reboot, aged out of `/tmp` after ten untouched days. Cleanup also: a `TestHome`'s drop and the owner sweep delete through the link; after a reboot the next start re-makes the target; after `cargo clean` the link is gone and `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/` reads it.
- Test data bootstrap Mechanism: `prepare_home_base`, called from `TestHome::new`, holds the same contract and panics with a fixed message naming the failed check and no path.
- §5: the `tempdir_in("<workspace root>/target/e2e-home")` statement holds by path; on the dev host the path is an operator-set link to `/tmp/viola-e2e-home-<uid>`. Not a third carve-out: these homes stay inside G2, G4 and the secret scan. No CI runner has a link; the pinned copy's `sync_all()` is unchanged.
**Why:** a start's pinned copy stalled behind other builders' writes; the backing was proved in one natural window at this chunk. The removals outside the working directory are a boundary widening, ratified by the founder, 2026-10-07T07:25Z, live, relayed by the overseer. Standing rule: a stalled-start red is a finding about the backing, stopped on and reported, never re-run for green; no kill, `WITHIN` or retry moved.
**Kept:** the two existing carve-outs and every CI statement about kept homes stand as written; the `--local-live` bullet's row list was not touched.
**Ref:** .andromeda/runs/2026-10-07T08-21-13-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the unwrap's frame and case list, the seventeen row ids, and the floor's Linux reading
**Section:** §4 Unit Test Strategy → viola-agent-claude (the `prompt-submitted` normalisation bullet; the R8 strip bullet) · §3 → 5-command implementation (the `--local-live` bullet) · §1 Test Scope Summary (the inherited-credentials-strip entity)
**Change:**
- §4 normalisation: the unwrapped frame is the two newlines before the open tag, the one after the close tag, and a second after it when text follows that is not the next pair's own two-newline frame; a third before, a second after at the prompt's end and a lone one before stay. Was "a second after … stay[s]" without the condition.
- §4 case list: the eight labelled cases of `prompt_text_drops_the_cli_framing_around_a_pair`; the wrapped-paste property, now with a typed tail after the pair; the five cases of `prompt_text_live_shape_normalises_as_measured` (typed then paste, paste then typed, two pastes under one id, a long and a short text whose last newline the CLI dropped, each pinned to the text less that newline); the two of `prompt_origin_live_shape_files_as_harness` (a `<task-notification>` typed at a prompt's very start; the unescaped three-attribute cross-session raw start).
- §3 `--local-live`: the parenthetical names all seventeen ids of `LEDGER_ROWS`. Was fourteen: `long-paste-wrapper`, `tag-escaping` and `local-command-clear` were missing. The count and the const are unchanged.
- §4 R8 strip and §1: a differing Linux dev-host reading on `claude` 2.1.287 stands beside the floor's Windows origin (12 names in a hook's environment, four outside the eleven, three of the eleven absent); the floor and the literal oracle stay the eleven.
**Why:** this chunk changed `unwrap_pastes` on a live measurement and added the measured shapes as literal oracles; the row list was stale since the three framing rows landed; the names reading is architecture's, pointed to here.
**Kept:** the literal-oracle rule: the two newline cases pin what the measured prompt normalises to, and the failing "equals the bytes sent" readings stay in the chunk's evidence.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/

## 2026-10-07-send-waits-out-the-paste-hint — kills, hold and cap follow the 8.5 s gate; Path 2 delivered under the hint
**Section:** §3 → Bootstrap phases (the key file, `[profile.ci]` and `[profile.mutants]`) · §4 (Screen signatures; Refusal ordering) · §5 (the `cli_verify` bullet) · §6 Path 2 (Verification signal) · §7 (Fake agent, Modes)
**Change:**
- §3 key file: `test(/verify_window_/)` is 20 s × 3, a 60 s kill, under `[profile.ci]` (was 15 s × 3, 45 s) and 15 s × 3, a 45 s kill, under `[profile.mutants]` (was 15 s × 2, 30 s; the three `mutants` overrides no longer share one line). The no-screen case waits out the gate's 8.5 s maximum four times: 34.3 s on the dev host, up to 35.3 s on CI (was "about 21 s" at 5 s). The paste-hint case holds 9 s (was 6 s) and reads 13.1 s to 13.7 s on CI (was "about 10 s").
- §4: the readiness-gate verdict is ready / wait / `input-not-ready`; on a verified CLI a quiet screen with no input-box literal waits until `GATE_MAX_WAIT`, 8 500 ms, pinned by a unit case. `send` reads human-typing and then turn-running a second time after the gate returns ready (four clock-driven unit cases).
- §5: the `verify_window_` hint case holds 9 s, past the 8.5 s maximum.
- §6 Path 2: `send_under_the_paste_hint_on_a_verified_cli` is delivered in both cases (was `input-not-ready` under the hold). New `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`: exit 10 `human-typing`, nothing typed, the key reaches the child. It cannot tell `send`'s first wheel read from its second, so its green is a floor. The refusal at the bound is unit-tier only.
- §7: `--paste-hint-ms` is capped at 10 000 ms (was 8 000 ms).
**Why:** The gate's bound moved to 8.5 s on the founder's live ruling of 2026-10-07T10:29Z (relayed by the overseer), and the `verify_window_` class waits that bound four times by design. Both kills moved in proportion on the overseer's answer of 2026-10-07T12:10Z, with a planted hang shown killed under each moved bound and passing with none. Standing rule: a kill line moves only for a floor that is by design, with that control recorded both ways.
**Kept:** `WITHIN` (7 s), the `mutants` profile's own 10 s kill and every other override; no end-to-end case blocks to the gate's bound.
**Ref:** .andromeda/runs/2026-10-07T12-57-41-wrap/
## 2026-10-07-a-send-ending-in-a-newline-is-confirmed — the matcher reads the typed text; the trailing-newline cases; a measured corrupt-profile mechanism and its witness
**Section:** §1 Test Scope Summary (the confirmed-`send` path and its receipt signal) · §4 What unit tests cover (viola-agent-claude, root bin) · §6 Scenario: Path 2 (Verification signal) · §10 Coverage thresholds → Stack adjustments
**Change:**
- §4 root bin: the matcher compares normalised `prompt-submitted.text` with the typed text (was "the sent text"). Seven tests carry `trailing_newline`: one LF and two confirmed, a refused character before the newline still `control-character` (pins the outcome, not the order), a prompt that keeps the newline claims nothing, three local-command cases. `send_local_command_` is eight functions, 15 cases (was five, 13).
- §4 viola-agent-claude: `typed_text`, nine labelled cases in one table; the two last-newline pins stay as the CLI's record.
- §1: a text is validated as received, then typed without its trailing LF; the list is consumed by exact equality of the typed text (was "exact text equality (no trim, …)"); the receipt holds inner newlines and no trailing LF.
- §6 Path 2: `send_text_ending_in_newlines_is_typed_without_them_and_confirmed` (two cases, three CI OSes), with what it reads. Under the fake agent the exit and the records are a floor; the receipt and `text` prove it. `/clear` and a newline is unit-tier only. Owed to "First live test and self-drive": a live send ending in newlines, a text of only newlines, a trailing CR.
- §10: the two earlier corrupt-profile runs (ci#36529038462, ci#36481260151) stay recorded, not established. Added, as measured: an instrumented child that exits by itself and is killed in its exit-time profile write leaves a short profile; 3 of 4 800 loaded runs of the root bin's start test did. The test's child is now `whoami`, uninstrumented; 4 800 runs then left one whole profile each. ci#37627485806 attempt 1 is closed by this mechanism; its writer, pid 10799, is not provable from the run. The witness `scripts/profraw-census.sh` is a dev-host script with recorded one-shot controls, no unit test and no gate.
**Why:** the remedy is the founder's ruling, live, 2026-10-07T15:21Z, relayed by the overseer. The red was folded into the chunk on the operator's word and closed by cause, never by its green re-run; the overseer's word at the wrap fixes the limit as stated. Trap: the wrapper puts its plugin flag first in a child's arguments, so a test binary can never be entered as its child.
**Kept:** nextest `retries`, the coverage ignore regex and the gate's required suites; no fake-agent option or fixture reproduces the dropped newline.
**Ref:** .andromeda/runs/2026-10-07T19-12-23-wrap/
## 2026-10-08-first-live-test-and-self-drive — `--local-live` alone runs no other suite; the live readings of Path 2 are measured
**Section:** §3 → 5-command implementation (`run`: the Command body sentence, the `--local-live` bullet) · §6 Path 2 (two Verification signal bullets)
**Change:** §3: the `--all` default holds with no selector and no `--local-live`; `--local-live` passed alone is a selection of its own (was: an unqualified "The default is `--all`" beside "after the selected suites"). The seventeen-row live firing was made once (suite `local-live` 1 passed, `stamped 2.1.287  17 pass  0 fail`), and because `verify` names no program it stamps the first `claude` on `PATH` (was: owed). Path 2: a send ending in newlines and `/clear` with a newline are confirmed live; a text of only newlines ends exit 13 `no-prompt-submitted` with `text_bytes` 0 and no `wheel` record; a text ending in one CR is filed `human`, moves the wheel and ends exit 13, with no test at any tier and no fix (was: all owed). The `/clear` live proof is taken (was: owed).
**Why:** wording brought to the harness as it fired and to the readings this chunk measured. Trap: the trailing-CR outcome has no pin at any tier.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-08-first-live-test-and-self-drive — Path 5 records the classifier's tables; the real CLI has two local routes
**Section:** §6 Path 5 (Surfaces involved) · §6 untestable list (first bullet) · §11 Test Strategy (the real-CLI ban, second sentence) · §1 Untestable zones (first bullet)
**Change:** Path 5 names the unit pins of the closed list as extended (the rstest table `classifier_terminal_reply_is_not_editing`, 8 cases; the whole-and-split test over 23 `REPLIES`; 27 negative controls) and says its reading on a real terminal is a local live measurement, not a suite (foot 1.28.0: seven typing before, none of 23 after; keys `n`, `t`, `m` still take the wheel). The three real-CLI sentences now say the suites reach the real CLI only through `run --local-live` / `viola verify`, and that a route entry's live readings of a real session are that chunk's own local measurement, not a test layer, not harness surface, never in CI (was: local only through `--local-live` / `viola verify`).
**Why:** the defect this chunk fixed was invisible at every CI tier (no tier writes a terminal reply), and three live sessions ran outside `viola verify` on the founder's ruling, so "only" was false.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-09-epoch-3-cleanup — the typed text, the `empty-text` refusal and the Path 2 readings
**Section:** §1 › Critical paths › confirmed `send` (the statement and its verification signal); §4 › `typed_text`, the send-confirmation matcher, Refusal ordering; §5 › wrapper-side paste validation; §6 › Path 2 › Verification signal
**Change:** the typed text drops every trailing CR and LF (was trailing LF only). `typed_text`'s table holds seventeen labelled cases (was nine) and the function has two product callers (was "Its only product caller"). Six of the seven `trailing_newline` tests take LF, CR and CRLF tails. The `send` order reads control-character → empty-text → human-typing and on, with five unit cases named for the new rung. §5 names the two cross-process cases, `send_empty_text_straight_to_the_wrapper_is_refused` and `send_empty_text_is_refused_at_once`. §6: the newline-tail case gains CR, CRLF and two-CR tails; the two 2026-10-08 readings (a newline-only text issued; a text ending in one CR filed `human`) stand as records of the earlier build; this chunk's live readings on 2.1.287 are added. Was: "no test at any tier pins the trailing-CR outcome, and no fix has landed"; now the unpinned, unfixed case is a CR or a CR LF inside a text, which the CLI submits as one LF.
**Why:** the chunk landed the founder's ruling of 2026-10-09 and measured the CRLF and two-CR endings live before pinning them.
**Kept:** the `send_local_command_` count is left as it stands: the chunk's report states no change to it.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/
## 2026-10-09-epoch-3-cleanup — the `mutants` override order and the `verify_window_` class
**Section:** §3 › Bootstrap phases (key file), `test-runner-install`: the `[profile.ci]` bullet, the `[profile.mutants]` bullet and its two sub-bullets
**Change:** in `[profile.mutants]` the `test(/verify_window_/)` override (15 s × 3, a 45 s kill) stands first, ahead of `package(viola-e2e)` and `test(/send_window_/)`, the first matching winning (was: `package(viola-e2e)` first and the `verify_window_` override "the last"). The class holds three cases in both profiles (was two): the harness test `verify_window_boot_with_an_unknown_cli_version_is_verify_failed` joins it by name and takes 45 s under `mutants` (was the package's 30 s) and 60 s under `ci` (was the profile's 120 s). Its floor as measured: four waits of 8577, 8561, 8563 and 8562 ms, 34.482 s under `ci`. The kill lines under `mutants` read 10 s, viola-e2e 30 s, the class 45 s. The order is pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`.
**Why:** the test waits out the gate maximum four times by design, above a 30 s kill, so the unit's unmutated baseline was killed under `mutants` on both hosts and the unit went unscored. The operator chose the class's name over an override of its own.
**Kept:** the `package(viola-e2e)` kill, `GATE_MAX_WAIT` and the periods are unchanged. A planted-hang control read TIMEOUT at 45.005 s under `mutants` and PASS at 84.513 s under the default profile.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/
## 2026-10-09-epoch-3-cleanup — a route entry cited by title
**Section:** §6 › Security control negatives
**Change:** the two mentions name the route entry "Home and code-bearing file integrity" (was: a bare `:127`, twice; the entry stands elsewhere on the route).
**Why:** the operator's direction that every stale bare route number found is cited by title.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/

## 2026-10-09-inner-cr-and-crlf-in-a-sent-text — the inner CR outcome is pinned at two tiers and read live
**Section:** §1 Critical paths (Path 2: the Path sentence and the fake-agent receipt bullet) · §4 Unit Test Strategy (the `typed_text` bullet; the send-confirmation matcher bullet) · §5 Integration Test Strategy (wrapper-side paste validation) · §6 Path 2 ("A text ending in newlines")
**Change:** the typed text is the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF; it holds no CR (was: the trailing removal "and nothing else"). §4: the ending table is `typed_text_drops_every_trailing_newline` with fourteen cases (was seventeen; three that pinned a CR as kept removed); beside it the ten-case table `typed_text_every_inner_cr_is_one_lf`, the test `typed_text_copies_only_a_text_with_an_inner_cr` and the 512-case property `typed_text_prop_holds_no_cr_and_every_inner_cr_is_one_lf`; the matcher bullet names `send_text_whose_inner_cr_is_one_lf_is_typed_and_confirmed` (five cases) and the two-case `send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing`. §5: the accepted half `channel_paste_accepts_lf_cr_tab_and_multibyte` expects the typed text, its inner CR as one LF. §6: `send_text_whose_inner_cr_is_one_lf_is_typed_with_lf_and_confirmed` (`tests/cli_send.rs`, three cases) and the live readings on 2.1.287 (five inner shapes confirmed after the control `rule-inner-lf`), with the not-measured list (was: "not confirmable … no test at any tier pins the inner outcome, and no fix has landed"). The receipt bullet says the fake agent's prompt holds no CR.
**Why:** the chunk landed the founder's ruling of 2026-10-09T16:51Z (relayed) and pinned it. A fake-agent green proves the outcome only by reading the typed bytes, so the live readings are stated as a local measurement, not a suite. The LF typed inside a paste has no ledger row or probe: owed to `v1-34` on the working-route entry "Paste newline ledger row".
**Kept:** the seven `trailing_newline` tests and their count; the earlier readings `after-inner-crlf` and `after-inner-cr` as records of the build before the rule.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-09-epoch-3-cleanup-ii — the shared root helpers named, the first whole-member viola-e2e score, the waits counted by pattern
**Section:** §2 Test directory + naming conventions (the sync E2E suites bullet) · §10 Mutation gate · §3 → 5-command implementation · §3 → Bootstrap phases (derive for route / setup-project)
**Change:**
- §2: the helper list is `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}` (was without `cli.rs`). `events.rs` holds the one `events.ndjson` reader, its wait and the `boot` that waits for the session-start record; `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), stdout and stderr two captures beside the exit code. No root test file outside `tests/support/` defines its own copy.
- §10: the first whole-member `viola-e2e` score on the Linux dev host: 718 mutants in 5356 s (89 min 16 s), 656 caught, 2 missed, 0 timeout, 60 unviable. The 2 missed are in `prepare` (`crates/viola-e2e/src/harness/run/mutants/scratch.rs`), reachable only on a Windows host, recorded not measured here and owed to the route entry "Windows mutation grade".
- §3 → 5-command implementation: `WITHIN` is "shared by the root waits on a child", 22 sites of `Instant::now() + WITHIN` in 16 files, the rule stated in architecture §Occupied Resources → Filesystem (was "shared by all 9 root waits on a child"); `wait_endpoint_gone` is named without "the ninth wait".
- §3 → Bootstrap phases: beside "78 m against 23 m under `immediate`" stands this chunk's whole-member run, 718 mutants in 5356 s with 0 timeout.
**Why:** the chunk lifted the per-file scaffolding into two shared files, took the member's first whole-unit score on the dev host, and moved the number of waits on `WITHIN`; the 9 was a named list no wrap had re-taken. `events.rs` was named in §2 before the file existed.
**Kept:** "78 m" and "0 Timeout grades over 711 Linux viola-e2e mutants" stand as measured at their chunk. The mutation gate's rule and the boundary tier's form are unchanged.
**Ref:** .andromeda/runs/2026-10-10T01-25-38-wrap/

## 2026-10-10-windows-mutation-grade — `run --mutants` leaves host-excluded missed mutants out and names them in `host_excluded`
**Section:** §3 → 5-command implementation (`run` step 4 Verdict · Exit code semantics · Output format · `gate`) · §3 → Bootstrap phases (derive for route / setup-project) · §2 Test Strategy (Mutation row) · §10 Build failure conditions · §11 Test Anti-Patterns → Quality
**Change:**
- Verdict: the run's own `outcomes.json` is read by record first. A record whose summary is `MissedMutant` and whose whole span sits under a `cfg` predicate proved false for the host the harness was built for (on a node covering the span, or on the `mod` declaration that brings the file in) is host-excluded: it leaves the missed count, `survived` and `failures`; `tested` still counts it. Decided keys: `unix`, `windows`, `target_family`, `target_os`, `target_arch`; any other key is unknown; `not` / `all` / `any` are three-valued. The host's facts are compile-time constants. A caught, unviable or timed-out record is never left out, and anything the reader cannot read keeps the mutant counted. The requirement is `missed − host-excluded == 0 && timeout == 0 && unviable <= caught` (was `missed == 0 && …`).
- `survived` = missed − host-excluded + timeout (was missed + timeout), in Output format and in `gate`'s `run-summary.json` line.
- Output format: a counted, scoped or package `mutants` object carries `"host_excluded"`, an array of `{"name","cfg"}`, repo-relative and never an absolute path, only when at least one mutant was left out.
- Exit code semantics, §2's Mutation row, §10's failure condition and §11's ban each read "missed" as missed beyond the host-excluded ones. §11 also bans trusting cargo-mutants' summary line, which still counts a left-out mutant as missed.
- Bootstrap phases: `viola-e2e`'s dependency line gains syn `=2.0.119` and proc-macro2 `=1.0.107`.
**Why:** the chunk moved the classification of mutants a host never compiles from a hand record into the harness, so that zero missed reads over the measurable set without an argued list. The harness document, not cargo-mutants' own lines, is the verdict.
**Kept:** `unviable <= caught` and `unviable-exceeds-caught` are judged as before. No `verdict`, `reason`, `suite` or `event` value is added, and `suites[]` keeps its field names. Founder Direction 1's words ("surviving mutants are red") stand; they read true under the new `survived`.
**Ref:** .andromeda/runs/2026-10-10T08-56-51-wrap/

## 2026-10-10-windows-mutation-grade — nine Windows mutation jobs, run 38036448183 green, the `prepare` mutants measured
**Section:** §9 CI Integration (Pipeline structure, Mutation row) · §10 Quality Gates & Coverage Targets (Mutation gate)
**Change:**
- §9 Mutation row: the audit's Windows leg is nine `mutants (<label>)` jobs on `windows-2025`, one per `matrix.include` item, counted as its `- package:` items; the root package `viola` is four of them, split by file; each runs over that item's Windows-gated files (was "one `mutants (<package>)` job per package"). The row's dispatch wording is unchanged.
- §10: `missed == 0` reads after the harness has left out the host-excluded records. After the overseer's sentence on mutants a host cannot compile or reach, two sentences say that the harness now applies the `cfg`-attribute case itself and names each such mutant in `mutants.host_excluded`, while a mutant behind a const compiles on both hosts, is never left out and stays a record by coordinate.
- §10: "Its jobs read red until the audit classifies the `#[cfg(unix)]` twins" is retired. The first run's reading stays as dated history (37174673472: 19 of 25 misses were such twins, 0 timeouts across 508 mutants). Run 38036448183 read all nine jobs green over 642 mutants: 537 caught, 75 unviable, 30 left out as host-excluded (the sum of the nine documents' `host_excluded` lengths), 0 missed, 0 timeout, the longest job 49 min against 120 min.
- §10: that run is named as one dispatch outside the boundary audit, made on 2026-10-10 for this chunk alone, on the founder's own word.
- §10: the two missed `prepare` mutants are no longer "owed to the route entry "Windows mutation grade"": both read caught on `windows-2025` in run 38036448183, job `mutants (viola-e2e)`; on the Linux dev host they still read missed.
**Why:** the chunk split the `viola` job per file, taught the harness the exclusion, and its one dispatch measured what the route entry owed. The dispatch was allowed by the founder for this chunk only, relayed verbatim by the operator on 2026-10-10.
**Kept:** founder ruling C2 and the overseer's ruling of 2026-10-04 stand word for word. The workflow stays report-only and never a gate. The four `src/cmd/run.rs:385:5` mutants, which no host this project builds on compiles, are not written into §10: they are a route matter.
**Ref:** .andromeda/runs/2026-10-10T08-56-51-wrap/

## 2026-10-10-self-healing-state — the torn append as landed: a stop and a shortened log, the appender heals, the reader counts three names
**Section:** §1 Test Scope Summary (Coverage triggers, chaos-test) · §2 Test Strategy (Test pyramid, Chaos / Fault row) · §4 Unit Test Strategy (What unit tests cover, viola-state) · §6 E2E Test Strategy (Chaos suite)
**Change:**
- §6 Chaos suite, torn append: the wrapper is stopped and its log shortened with `File::set_len`, in place of a kill mid-append with `Process::kill_with(Signal::Kill)`; the count is read through the product reader, 1 before the heal and 1 after; the next start's first record starts on a fresh line with every earlier byte unchanged; the role file holds one `state-recovered` line, which the case holds to the diag-line schema itself because a local run removes its test homes (`tests/chaos_torn_append.rs`). The `skipped.torn_lines` reading on `list --json` and `/api/sessions` is owed to "The board: viola list" and, for the page, to Epoch 8.
- §1 chaos-test trigger and §2 Chaos / Fault row: the same fault, a stopped wrapper's log shortened with `File::set_len` (was "kill the wrapper mid-write" / "kill mid-write"); the reader counts and rewrites nothing, the next append heals (was "readers count `torn_lines` and heal").
- §4 viola-state: the reader's three counts and what each takes (`three_counts`); the heal is the appender's, with `append_event_at`'s L + 1, `try_append_event` under a held lock, and one `state-recovered` line per heal (`torn_tail`). Was "Torn-line healing counts the line in `torn_lines` and never panics" and "An over-long line is counted as torn".
**Why:** the chunk built the fresh-line half of the chaos case and the counts. A stop leaves the same file on disk as a kill and no short coverage profile.
**Kept:** the observable, "the next append starts on a fresh line", is unchanged; the other chaos bullets stand.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/

## 2026-10-10-self-healing-state — the snapshot read and the replay as landed; Scenario E5 owed; the crate-level suite named; the root waits at 23 in 17
**Section:** §1 Test Scope Summary (Coverage triggers, chaos-test) · §4 Unit Test Strategy (What unit tests cover, viola-state) · §5 Integration Test Strategy (Cross-module patterns, On-disk) · §6 E2E Test Strategy (Scenario E5) · §3 → 5-command implementation
**Change:**
- §4 viola-state: the classified read tells the snapshot, no file, an unreadable one and an unsupported `v` apart, reading `v` first; an unsupported `v` or a parse failure goes to replay through the read-or-replay function, one `state-recovered` line each and none for a present or absent snapshot (`snapshot_cause`, `replay_recovers`); library code, no product reader yet. Replay: a field no line gave is absent, `links` replays empty until "Session links", no file is written.
- §1 chaos-test trigger, snapshot bullet: the same limits (`links` empty until "Session links", no file written, the reader owed first to "viola revive").
- §6 Scenario E5: owed, not built. It needs `link` ("Session links"), a budget pause ("Budget governor"), `list --json` ("The board: viola list") and a reader of the replay (first "viola revive"). Two signals are set against what landed, for the building entry to restate: every start appends `wheel{driver, start}`, so `wheel:"human"` does not hold after step 3; the replay writes no file, so no snapshot is rebuilt from it. The scenario's own lines are unchanged.
- §5 On-disk: `crates/viola-state/tests/` is two files, `state_events.rs` and `state_replay.rs`, with their cases named; the root `path2` and two `path4` cases read a product-written log back with zero on all three counts.
- §3 → 5-command implementation: `Instant::now() + WITHIN` reads 23 sites in 17 files (was 22 in 16).
**Why:** the chunk landed the replay below any surface and measured the second start's first record. The operator approved E5 as owed at the P5 review.
**Kept:** E5's steps and verification lines stand as written under the new as-landed bullet.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/

## 2026-10-10-viola-revive — revive in the test plan: its exit-1 causes, the replay's first reader, the instance check's cases
**Section:** §1 Test Scope Summary (Coverage scope, CLI verbs; Surfaces under test, cli; Coverage triggers, chaos-test) · §4 Unit Test Strategy (viola-state: Snapshot envelope bullet, a new instance-check bullet) · §5 Integration Test Strategy (On-disk, strict-modes refusals) · §6 E2E Test Strategy (Scenario E5, as landed; Exit-cause matrix)
**Change:**
- §1: both verb lists name `revive`; it takes no `--json`, exits 0, 1 or 2, and has the human signal only (`--list` rows `<ts>  <cause>  <id>`).
- §4, §1 chaos-test and §6 E5: `viola revive` is the first product reader of the read-or-replay function (was "no product reader takes the replay yet" and a reader owed first to the route entry "viola revive", per "2026-10-10-self-healing-state — the snapshot read and the replay as landed; Scenario E5 owed; the crate-level suite named; the root waits at 23 in 17"). The four other snapshot readers still call `read_snapshot`, so E5's step 3 does not take the replay. E5 stays owed for `link`, a budget pause and `list --json`. A replayed snapshot carries no `cwd`; a revived start's first record is `wheel{holder:"driver", cause:"start"}`.
- §4: the instance check `strict::check_instance` with its unit cases `strict_instance_*` (five widened modes on Unix, a tree viola wrote, a home with no instance directory, a path that cannot be statted) and two `cfg(unix)` root cases; not covered: a widened DACL on an instance tree on Windows, owed to the route entry "Home and code-bearing file integrity".
- §5: strict-modes is exit 1 for `run`/`ui`/`mcp` and for `viola revive`, CLI 21 for every other verb (was those three; every CLI verb 21).
- §6 Exit-cause matrix: a third list, revive's exit-1 causes in preflight order (`strict-modes-failed`, `already-live`, `no-session` with two pairs, `cwd-missing`), landed as cases of `tests/cli_revive.rs` outside `cross_exit_causes.rs`, with no `--json` document; joining the table is owed to "Exit-cause code catalogue", the document to "CLI machine contract".
**Why:** the chunk landed the verb and its tests at another site than the matrix names.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-viola-revive — the fake agent's resume payload and ten options; the root waits at 26 in 19 and the kill path's endpoint wait
**Section:** §7 Test Data & Fixtures (Fake agent: the hook-commands bullet, Modes) · §3 → 5-command implementation (`run` step 2)
**Change:**
- §7: under `--resume <id>` the fake agent sets `source` to `resume` and `session_id` to the id given on the recorded launch SessionStart, with `--fork-session` beside it the compiled id `0f0e0d0c-0b0a-4908-8706-050403020100`, the trailing newline kept; without `--resume` every payload is the recorded bytes (was: only UserPromptSubmit's `prompt` is set, and no other payload is built). Ten argv options (was eight; the list is the count's rule): eight for verify's runs, two for revive's tests.
- §7: one payload is set without a recorded fixture. No recorded SessionStart has source `resume`; the live payload on `claude` 2.1.287 holds 10 key names against the rewritten 5. No reading became a fixture or a ledger row; the recorded payload is owed to the route entry "Paste newline ledger row".
- Key file: `Instant::now() + WITHIN` reads 26 sites in 19 files (was 23 in 17). The kill path (`Wrapper::kill`) waits for the wrapper's exit, then for the child it recorded to be gone by pid and start time, then on `holder_gone` (on Unix a connect refused or the path absent, on Windows the pipe not found); the stop path keeps `unconnectable`.
**Why:** the two set fields are the operator's word at the chunk's P4. The plan's `wait_endpoint_gone` after a kill was disproved by measurement: a killed wrapper leaves its socket file on Unix, so a wait on the file being absent never ends.
**Kept:** the key file's `cleanup` sentence on a force-kill is unchanged: what the harness reads after a force-killed wrapper on Unix is not measured, and it is carried as a labelled hypothesis on the route entry "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-statusline-pass-through — Path 6 and Path 1 as landed: the named source, the pass-through and the override rewrite on Unix only
**Section:** §1 Test Scope Summary (Coverage scope: the hook entity; Critical paths: `run` start sequence, budget governor; Coverage triggers) · §3 → 5-command implementation (`boot`) · §6 (Scenario: Path 1; Scenario: Path 6, steps 1 and 2 and its Verification signal)
**Change:**
- Path 6 step 1 and the `boot` key: the statusline source was "a §12 open question" with the pass-through bullets blocked; now it is named (`<home>/statusline-source.json` whenever it exists, else in the default home the user's `.claude/settings.json`, else none), and the pass-through bullets landed as root tests on a planted source: `tests/hook_statusline.rs` cases 1 and 5 and `path6_wrapped_statusline_prints_the_user_output_unchanged` in `tests/cli_instance_state.rs`.
- `agent-run boot --statusline-echo` is not built, and neither is a `statusline-source-unresolved` refusal (was "exits 2 with `reason:"statusline-source-unresolved"`"); both are owed to the working-route entry "Budget governor", and the harness-session form of Path 6 step 1 waits for it.
- Path 6's verification, and the §1 statements of the same claim: the marker equality is Unix's (`/bin/sh -c`, a 5 s PROVISIONAL bound); on Windows `run` writes no override and the arm runs no command; exit 0 with empty stderr holds on every OS; `budget.json` is written only when the payload holds a `rate_limits` object. Step 2 says the reader takes `resets_at` as epoch seconds or RFC 3339.
- Path 1, and the §1 statements of the same claim: the `settings.json` rewrite on restart is asserted on Unix (whole, 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written. Landed as `run_rewrites_the_settings_override_each_start` and the unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only`.
**Why:** the chunk named the source and built the pass-through on Unix; the Windows half is cut on the founder's word (live, 2026-10-10, relayed verbatim by the operator) and owed to the working-route entry "Windows-only live measurements".
**Kept:** Path 6 stays a critical path that §2 wants on a harness-booted session; the root tests are where its pass-through signal stands until the flag is built.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-statusline-pass-through — the fake agent's statusline options, the reader's property and seeds, the stamped-home rule on Windows, three counts
**Section:** §4 (viola-state: the snapshot readers) · §5 (Setup / teardown lifecycle: the Windows x64 seed carve-out) · §6 (Property suite) · §7 (Fake agent: hook commands, receipt kinds, Modes; Seed strategies) · §3 → 5-command implementation (`run --fuzz-replay`) · §3 → Bootstrap phases (test-runner-install)
**Change:**
- Fake agent: twelve argv options (was ten; the rule is architecture's list of them). `--settings <file>` and `--statusline-stdin <file>`: the settings file's `statusLine.command` is split on ASCII white space and run only with both options and only when absolute, with the stdin file's bytes. One mode word, `statusline-echo`, landed (was "lands with its consumer chunk"). A `statusline` receipt kind: the `hook` kind's fields without `event`.
- Property suite: the statusline reader's property landed at `cases: 512` with `proptest-regressions/statusline.txt` committed (was "joins with Statusline pass-through"); its strategy draws whole seconds across the date range. `hook_stdin` has 12 synthetic seeds (was 10; rule `ls fuzz/corpus/hook_stdin | wc -l`), in §6 and in the `run --fuzz-replay` key, and the target reaches the statusline reader.
- Seed strategies: a row for the statusline source, written by the test through `plant_statusline_source`, never by viola; default-home cases through `TestHome::default_of_user` and `Wrapper::boot_as_user`.
- Seed strategies and the §5 carve-out: on Windows x64 a home `seed_conpty` made is not one viola created, and `hook statusline`'s instance check refuses it; a case that needs a viola-created home on every OS starts from `stamped_home`.
- §4: five other snapshot readers call `read_snapshot` (was four; rule `read_snapshot(` under `src/`, outside test modules). test-runner-install: thirteen verify-driven binaries under the 20 s kill (was twelve; rule: the `binary(…)` names on that filter line), `hook_statusline` added.
**Why:** each moved with the chunk's code. The Windows rule was measured on CI: three cases refused from seeded homes while the stamped-home case passed; the refused home's DACL was not read, and that reading is owed to the working-route entry "Home and code-bearing file integrity" (the operator's answer at this wrap, 2026-10-10).
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-budget-governor — Path 6 landed in part, expiry as a pure-function case
**Section:** §6 (Scenario: Path 6, steps 1 and 7, the signals, a new As-landed bullet; Scenario: E5) · §4 (root bin, Budget governor) · §7 (Time-sensitive data) · §8 (the Time bullet, the Time mocking row) · §1 (the critical-path signal list) · §3 → Bootstrap phases (Dev-deps)
**Change:** Path 6 gains an As-landed bullet: its cli and ipc signals are seven `path6_` cases of `tests/cli_budget.rs` (`binary(cli_budget)`), each from a stamped home; a window with an `"unknown"`, unparseable or passed `resets_at` does not pause, `release --budget` with no pause records nothing, an override does not outlive its wrapper; step 6 is `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often`. Owed by title: the harness-session form, step 7's two-instance leg (its rule held by `gate_release_on_one_instance_leaves_another_paused`) and `list --json` to "The board: viola list", the web signals to Epoch 8, the MCP form to "MCP server for drivers"; the not-gated clause for `answer`, `wait`, `last` and `pause` holds by the diff alone. Step 1: `boot --statusline-echo` is owed to "The board: viola list"; was to "Budget governor". `release --budget` gives `{budget_paused:false, override_until}` when a pause stood. E5's budget leg landed (`e5_the_replay_reads_what_the_wrapper_wrote`, two `state_replay_` cases); `link` and `list --json` are still owed. Override expiry is a unit case of the pure `decide`, which takes the instant as an argument (`gate_decide_an_expired_override_is_dropped`, `gate_release_override_ends_at_its_instant`); was "`budget_override_until` expiry via mock_instant", at §4 and at Path 6 step 7. `budget_override_until` leaves §8's clock-seam list, and the gate's three evaluation points are named as the exception to "`chrono::Utc::now()` is read only through the clock seam". mock_instant is marked as in no manifest yet at its three strategy sites.
**Why:** the chunk landed the gate's cli and ipc half and nothing of the harness-session, board or web half. No crate depends on mock_instant, so the expiry could not be a mock_instant case; the plan chose an instant as an argument over a new clock seam. The owners of what is still owed are the plan's cut and the operator's directions at the wrap.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-budget-governor — the strict-modes cases without a log line, the gate's negatives
**Section:** §5 (strict-modes refusals; out-of-range `Percent`; the `cli_controls_not_disableable.rs` bullet) · §6 (Exit-cause matrix, `revive`) · §4 (viola-state: the Snapshot envelope bullet, the instance check bullet) · §1 (the CLI verbs entity)
**Change:** `viola revive`'s strict-modes refusal is the stderr pair alone with no log line on the start arm as on `--list`: no `process-start`, no `process-exit`, no child, the home's mode left as found; was "its start arm with one `process-exit` of detail `strict-modes-failed`". The hook's refusal: the statusline arm reads and opens nothing after a refused check. The exit-cause list puts the strict-modes check first, ahead of the wrapper log, then three preflight readings; its cases assert no line added to `run-<name>.ndjson`, and the group-writable-home case reads the home's mode while the revive still runs, because a revive whose check wrongly passes runs on as a wrapper. Three `cfg(unix)` root cases hold the refusal (two on the start arm, one on `--list`); was two. The read-or-replay function is called in the preflight's second reading; was "its third". `revive`'s exit 1 is "a refusal (the strict-modes check ahead of the log, or a preflight reading)". Out-of-range `Percent`: the product falls back to that key's default and never refuses to start; `setting_does_not_disable_the_budget_gate`, three rows, each exit 11 and exactly one `parse-rejected{parser:"config-json", detail:"malformed"}` line; was "may fall back to the default or refuse to start (exit 1)". The controls table lists the budget-gate negative as joined.
**Why:** the chunk moved both instance checks ahead of any log and landed the threshold fallback, so the cases assert an absence where they asserted a line, and one open outcome is now decided.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-budget-governor — the state-file ban scoped, two counts, the boot flag's owner
**Section:** §2 (Agent-runnable invariants, Self-bootstrapping fixtures) · §7 (the `budget.json` row) · §11 (Test Data, the NEVER hand-write ban) · §3 → Bootstrap phases (the nextest overrides) · §3 → 5-command implementation (`boot`, `run`)
**Change:** the ban on a hand-written `ledger/stamps.json`, `snapshot.json` or `budget.json` binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version. Was an unscoped NEVER at all three sites, with `budget.json` "only from `viola hook statusline` stdin". The 20 s kill filter line holds fourteen verify-driven binaries (the rule: the `binary(…)` names on that line); `cli_budget` joined; was thirteen. Root waits `Instant::now() + WITHIN` under `tests/`: 28 sites in 20 files; was 26 in 19. The `boot` key: `--statusline-echo` is owed to "The board: viola list" (was "Budget governor"), and the `statusline-source-unresolved` refusal is dropped with no owner, its premise gone.
**Why:** the reader's negative cases need an over-cap, an unparseable and a `v` 2 file, which no writer produces, and the approved plan asked for those cases beside a constraint that forbade them. The operator answered the escalation at the wrap: scope the rule. The flag's owner is the plan's cut, confirmed by the operator's direction at the wrap.
**Kept:** a booted-wrapper test still feeds `budget.json` only through `viola hook statusline`; the scope admits no hand-written file anywhere.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-session-links — links in the replay, fifteen kinds, the kill line
**Section:** §1 Coverage triggers (chaos-test) · §4 (viola-state) · §5 (Cross-module patterns, On-disk) · §3 → Bootstrap phases (test-runner-install)
**Change:** Was "`links` replays empty until the link kinds land" at two sites (§1, §4); now the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items. §4 states the rule `viola_state::replay::apply_link_line` (a `link` line with string `driver` and `driven` adds the pair with `since` the line's `ts` unless held, an `unlink` line removes it, any other line changes nothing; inline cases `link_rule_*`) and that a `link` line with no string `ts` gets `since: "unknown"`, on the operator's word. §5: `state_events.rs` round-trips the 15 kinds (the rule: the length of `EventKind::ALL`; was 13), and a line of a kind no build knows is counted and not returned; `state_replay.rs` gains two link cases (a pair keeps its first `since` until an `unlink` and takes a later one after it; a snapshot cut short is replayed with the logged link). The 20 s kill filter line holds fifteen verify-driven binaries (the rule: the `binary(…)` names on that line; was fourteen): `cli_links` joined, each of whose five cases starts from a stamped home.
**Why:** The two kinds and the replay's link half landed with their unit and crate-level cases, and one root test file joined the stamped-home class.
**Ref:** .andromeda/runs/2026-10-11T00-33-08-wrap/

## 2026-10-10-session-links — Path 6's four verbs, E3 and E5 as landed
**Section:** §6 (Scenario: Path 6 — budget governor; Scenario: E3; Scenario: E5)
**Change:** Path 6: was "holds by the diff alone, and its case is owed to the route"; now that `answer`, `wait`, `last` and `pause` are not gated is held by the root case `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`): under one closed gate an automated `send` is refused exit 11 `budget-paused` / `five-hour` with one `send-refused` record, and the four verbs each give the exit code and document they give under an open gate. E5: was "still needs `link` … and `list --json`"; the link leg landed on the implicit link alone (`e5_the_replay_reads_the_link_the_wrapper_wrote` in `tests/cli_links.rs`, beside two `state_replay.rs` cases; `links_survive_a_wrapper_restart` holds the start's own restore, which is not the replay); the scenario still needs `list --json`. E3 gains an As landed bullet, "owed, built in part": steps 1 and 3 cannot run because `viola link` and `viola unlink` are not built (both methods answer `-32601`) and are held for the founder's word, the route entry "The board: viola list" carrying the hold; what step 2 leaves is held by four root cases of `tests/cli_links.rs`; no case reads an `unlink` record a product path wrote; `/api/links` and the Playwright signals are owed to the Epoch 8 entries.
**Why:** The folded budget test and the implicit link's root cases landed; the explicit verbs did not.
**Kept:** E3's steps and signals stand as the scenario. Nothing of the unbuilt part (the cli half with `already_linked`, the raw-channel name negative) is written as landed.
**Ref:** .andromeda/runs/2026-10-11T00-33-08-wrap/

## 2026-10-11-the-board-viola-list — the board's readings landed: E5, Path 6, Chaos; the literal pin and the kill line
**Section:** §4 Unit Test Strategy (root bin escaping; snapshot read-or-replay); §5 Integration Test Strategy (Drivers, CLI trycmd bullet, driver table); §6 (Scenario E5 As landed, Path 6, Chaos suite torn append and forward-compat, Cross-surface parity, E3 As landed); §3 → Bootstrap phases (the `[profile.ci]` kill line); §3 → Status endpoint shape; §3 → 5-command implementation (`boot --statusline-echo`)
**Change:**
- §4: `wait` / `last` keep `\n` and `\t`; `list` rows escape every control character, `\n`, `\t`, DEL and C1 included (was: all three keep them). The read-or-replay function has a second product caller, `board::read`.
- §5: the `viola list` board is pinned by literals in `tests/cli_list.rs` (`CAPTION`, `list_human_board_is_the_literal_rows`, the empty-board case) and the `bay_line_*` / `board_row_*` units, not a trycmd case (was: trycmd); trycmd keeps the send mirror and help.
- §6 E5 is landed in part (was: owed, not built): the `list --json` leg (`e5_list_reads_a_live_wrapper_through_the_replay` ×2); steps 1–3 wait on the five other readers taking the replay, owed to "Server verification before any frame".
- §6 Path 6: `list --json`'s `budget`, `budget_paused` and the board's cell landed (`path6_list_shows_the_reading_and_the_pause`, three CI runners); the harness-session form, step 7's two-instance leg and `--statusline-echo` are owed to "End-to-end path binary and harness legs" (was: the board entry).
- §6 Chaos: `torn_lines` on `list --json` landed; the forward-compat `list --json` leg landed (`chaos_forward_compat.rs`); `/api/sessions` stays Epoch 8's. Parity names the `CAPTION` literal. E3's hold moved to "Explicit session links".
- The kill line: eighteen verify-driven binaries (was fifteen; the rule: the `binary(…)` names on `.config/nextest.toml`'s filter line, every binary that drives verify): + `cli_list`, `chaos_forward_compat`, `chaos_torn_append`.
- Status endpoint: the interim's premise is now "while `status` runs no `viola list`" (was: `viola list` unbuilt); its owner is "End-to-end path binary and harness legs".
**Why:** the board as built; owners per the review card the founder approved.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/

## 2026-10-13-t06-38-35-wrap — E3's explicit steps held on the Epoch 6 entry
**Section:** Scenario: E3 — `link` / `unlink` transfer markers (As landed)
**Change:** steps 1 and 3 stay unrunnable: `viola link` / `viola unlink` are not built, their boundary widening is not ratified, and no explicit verb is built before Epoch 6. The hold's entry "Explicit session links" now stands in Epoch 6 after "Server verification before any frame"; was "the Epoch 5 entry … carries the hold". Step 2's root cases are unchanged.
**Why:** the founder's own answer of 2026-10-13 («Не делать до Эпохи 6»), relayed verbatim by the Viola overseer through the operator's directive; E3's explicit steps move with the entry.
**Ref:** .andromeda/runs/2026-10-13T06-38-35-wrap/

## 2026-10-13-epoch-4-cleanup — moved unit tests re-cited after the send and human split
**Section:** §4 Unit Test Strategy (root bin: the budget governor and the send-confirmation matcher bullets); §5 Integration Test Strategy (the `viola list` board)
**Change:** `send_budget_paused_*` sit in `src/run/send/budget.rs`; of the six send-module `trailing_newline` tests three sit in `src/run/send/classify.rs`, two in `src/run/send/relabel.rs` and one in `src/run/send.rs`; the two `inner_cr_is_one_lf` tests in `src/run/send/relabel.rs`; the eight `send_local_command_` functions in `src/run/send/classify.rs`; the board units `bay_line_*` and `board_row_*` in `src/human/board.rs`. Was `src/run/send.rs` and `src/human.rs`. Every test kept its name and cases (223 test lines before and after).
**Why:** the pure-move split of `src/run/send.rs` and `src/human.rs` by phase.
**Kept:** the wrapper's `send` (`typed_text`'s product caller) and the three `send_empty_text_*` cases stay in `src/run/send.rs`.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/

## 2026-10-13-epoch-4-cleanup — the Windows scratch lay-down and the Windows-target clippy
**Section:** §3 → 5-command implementation (`run` step 4, Command, the Windows host scratch arm); §9 Gate commands (the Lint row)
**Change:**
- `run` step 4: the scratch is removed and re-created through `viola_state::fs::create_private_dir`, so it takes the protected user + SYSTEM DACL and every temp copy and test home inside inherits owner-only, as under `%TEMP%`. Pinned by the `#[cfg(windows)]` unit `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` on CI's `test (windows-2025)` leg with no dispatch (a tree inside a wiped scratch passes `check_instance`; the same tree in a plain-created sibling reads `Writable`), as measured at ci#38375200217. The root package's Windows baseline reading green is not yet measured (the next `windows-mutants.yml` dispatch). Was: "removes and recreates it" with no mode.
- The Lint row: beside CI, a chunk plan may list a check-only Windows-target clippy (`CARGO_TARGET_DIR=target/wincheck … --target x86_64-pc-windows-msvc -- -D warnings`) as a chunk gate entry on the Linux dev host, nothing linked; neither a CI step nor a `pre-push` stage.
**Why:** run 38215104120's root-package baseline failed inside a plainly re-created scratch; CI lints `cfg(windows)` code the dev host does not build.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/

## 2026-10-13-epoch-4-survivors — the Epoch 4 survivors disposed, the resize miss recorded
**Section:** §10 Quality Gates → Mutation gate
**Change:**
- The Epoch 4 boundary audit's Linux whole-member runs read 47 missed; their ledger is the chunk's `evidence/survivors.ndjson`: 29 disposed by a nextest case in the package that owns the mutated file, 7 of them after a behaviour-preserving restatement that made the site reachable on Linux (`viola_state::fs`'s retry decision passed in by the caller, `viola send`'s issue-line decision a named function), each shown red under its hand-applied mutant and green restored with no mutation run made; the 2 `prepare` rows stand as recorded "not measured here"; the 16 in `src/cmd/verify/typed.rs` (`framing_turns` 5, `SharedPty` 5, `Run`'s ending sequence 6) owed to the route entry "Epoch 4 survivors II". The Epoch 5 boundary audit's own run confirms the 29 kills.
- Beside the Linux mutation `TMPDIR` sentence: the audit ran two cargo-mutants jobs (`-j 2`) under one `TMPDIR` and read viola-pty's `PortablePty::resize` → `Ok(())` missed, while its three resize tests, run alone under that mutant, read red; that the other job's unmutated run wrote the awaited line into the shared report file is recorded, not established. The report files now carry the test process's id, so two jobs never share one.
**Why:** the founder's split of the entry (by dialog, relayed verbatim by the operator): this chunk takes the 31 reachable by a unit case or a small restatement, the Run B driver restatement left for its own entry; and his standing direction that the mutation workflow runs at the epoch boundary only, so each kill is shown by hand and read by the next boundary audit.
**Ref:** .andromeda/runs/2026-10-13T16-59-54-wrap/

## 2026-10-13-epoch-4-survivors-ii — the 16 typed.rs survivors disposed
**Section:** §10 Quality Gates → Mutation gate
**Change:** the 16 in `src/cmd/verify/typed.rs` (`framing_turns` 5, `SharedPty` 5, `Run`'s ending sequence 6) were disposed by this chunk in its ledger `evidence/survivors.ndjson`. Each was disposed after a behaviour-preserving restatement inside `typed.rs`:
- the two framing waits became the named predicates `added_turn_ended` and `local_command_answered`;
- every `clock.now() + PROBE_DEADLINE` became the one function `probe_until`;
- `SharedPty` became generic over its `Pty`, reached through a test-only recording double;
- the Ctrl-C ending and the pump wait became the plain functions `ctrl_c_ending` and `ended_by`.

Each was then killed by a case in `typed.rs`'s `mod tests`, shown red under its hand-applied mutant and green restored, with no mutation run made. None is owed. The Epoch 5 boundary audit's own run confirms the 45 kills: the 29 of chunk 2026-10-13-epoch-4-survivors and the 16 of this chunk. This replaces "owed to the route entry "Epoch 4 survivors II"" and "confirms the 29 kills", per the heading "2026-10-13-epoch-4-survivors — the Epoch 4 survivors disposed, the resize miss recorded", whose other claims stand.
**Why:** the founder's split moved the Run B driver's survivors to their own entry. His standing direction holds that the mutation workflow runs at the epoch boundary only, so each kill is shown by hand and read by the next boundary audit.
**Ref:** .andromeda/runs/2026-10-13T18-08-14-wrap/

## 2026-10-13-interrupted-verify-cleanup — verify's stop table, the Windows matrix count
**Section:** §5 Integration Test Strategy → CLI (the `tests/cli_verify.rs` bullet) · §9 CI Integration → Pipeline structure (the Mutation row)
**Change:**
- `tests/cli_verify.rs` gains `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` (`cfg(unix)`, rstest cases `sigint` and `sigterm`). It runs a verify started and not waited (`tests/support/verify.rs` `verify_started`, no `--turn-stop`, a cwd and `TMPDIR` the case owns), signalled by `libc::kill` at the pid it started once the receipt shows its third `cwd` line and first `prompt` line.
- The case asserts: the print probe's, Run A's and Run B's dirs gone while planted foreign dirs stand with their bytes; stdout empty; the exact plain pair; exit 1; no `ledger/stamps.json`; every spawn pair closed; the log ending `process-exit{exit_code:1, detail:"interrupted"}` with no `panic` line, schema-valid.
- Unit cases in `stop.rs`, `typed.rs` and `version_gate.rs` back it. On Windows the console routine is called directly (three cases on `windows-2025`).
- The Mutation row's Windows leg was nine jobs with the root package four; now ten jobs with the root package five, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`.
**Why:** The stop path is new behaviour proven against the fake agent only, with no live start (`inputs#I1`). Real Windows console delivery is owed to "Windows-only live measurements" (the P5 card's row, the operator's word). The matrix line is forced by `tests/contract_windows_mutation_scope.rs`. The root `Instant::now() + WITHIN` count is unchanged at 28 in 20 files (the rule: the pattern under `tests/`): the receipt watch reuses `fake::wait_for`.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/

## 2026-10-13-paste-newline-ledger-row — eighteen ledger rows in the harness and the suites, `paste-3`, Run B's four added pastes
**Section:** §2 (contract trigger · Contract tier row) · §3 → 5-command implementation (boot step 4 · `local-live`) · §3 → Bootstrap phases (profile.ci sizing) · §4 (the ledger framing arms) · §5 (`cli_verify` · `contract_ledger_probes`) · §6 Path 2 (the inner-LF control) · §7 (Recorded hook payloads · fake agent pastes and `--framing` · the resume payload · the hygiene walk)
**Change:**
- Was seventeen ledger rows; now eighteen (`LedgerRow::ALL`, `paste-newline` last). Boot stamps all eighteen. `LEDGER_ROWS: [&str; 18]`, unit-proven by the `LIVE_PASS` proof. The tmpfs live home stamped at seventeen reads `row-missing` until its next `run --local-live` re-stamps it.
- `cli_verify`: `[NN/18]` lines; without the framing replay the three paste rows pass (`17 pass  1 fail`); after a tag-turn modal neither the newline text nor `/clear` is pasted (`16 pass  2 fail`). `contract_ledger_probes`: eighteen ids, `18 pass  0 fail`.
- Run B: was three added pastes; now four (the newline text recorded as `paste-3`). Framing variants: was four; now five under `2.1.287`, walked by the hygiene contract. `--framing` replays `paste-1` / `paste-2` / `paste-3`.
- §4: four framing arms, `paste-newline` with its three `check_paste_newline_` cases.
- §6: the `rule-inner-lf` control is now covered by `paste-newline`, stamped on live 2.1.287 on Linux; Windows owed to "Windows-only live measurements".
- §3 Bootstrap: Run B makes four added turns; 34.69 s against 35.4 s, no kill line moved.
- The recorded resume payload: was owed to "Paste newline ledger row"; now to "Owed ledger rows".
**Why:** The fifth paste is the founder's own answer to P4 fork 1, relayed verbatim by the operator (`inputs#I2`); the wrap's card confirm is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. The owed payload's move is his fork-3 answer (`inputs#I2`). Counts carry their rule (`LedgerRow::ALL`).
**Kept:** the seventeen-row live firing of chunk 2026-10-08 and the 2026-10-06 sizing of three added turns stand as dated records.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/

## 2026-10-13-owed-ledger-rows — the relied set, the statusline arms, the StatusLine fixtures and the kept workspace.repo
**Section:** §2 (contract trigger, Contract row); §3 → 5-command implementation (boot step 4, `run --local-live`); §4 viola-agent-claude; §5 `cli_verify`, `contract_ledger_probes`; §7 Recorded hook payloads, Fake agent, Fixture hygiene; §11 Test Data
**Change:**
- Step lines: one `[NN/MM]` per relied row, `MM` = `LedgerRow::relied()`'s length (21 on Unix, 18 on Windows). Was eighteen.
- `cli_verify`'s per-OS literals:
  - no framing replay: `17 pass  4 fail` on Unix, `17 pass  1 fail` on Windows;
  - tag-turn modal: `19 pass  2 fail` on Unix, `16 pass  2 fail` on Windows.
- `contract_ledger_probes` lists each relied id once (`ROW_IDS` 21) and ends at the relied count.
- Boot stamps every relied row; on Unix, Run B carries its one probe `--settings`.
- `run --local-live`'s `LEDGER_ROWS` is `[&str; 21]` under `cfg(unix)` and the 18-id form under `cfg(not(unix))`, keeping it out of the Windows mutation scope. The 21-row live firing is recorded (`passed 1`, census empty).
- §4: three statusline arms with nine `check_statusline_*` cases, beside the shell-verdict, variant, file-name and relied-set cases.
- Fixtures:
  - §2: the drift contract replays the `StatusLine` variants byte for byte under `--framing`, and jsonschema checks them against `schemas/claude-statusline-fixture.v1.json`.
  - §7: two variants under `2.1.287`, scrubbed, refused if unclean and hygiene-walked.
- The fake agent's `--framing` replays them through an absolute `--settings` command. `"$0"` is handed over as `/bin/sh`, with no shell. The first-Stop replay comes after the turn screen, and no option is added. The resume payload is owed to "Resume and compact ledger rows" (was "Owed ledger rows").
- §7 Fixture hygiene and §11: a `workspace.repo` naming this repository's own remote is kept in a fixture. The scrub does not rewrite it and the walk does not refuse it. Was: fixtures scrubbed of usernames, no exception.
**Why:**
- Run B's probe `--settings` is the founder's own answer to P4 fork 3, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer).
- The kept `workspace.repo` is his own word (`inputs#I3`, relayed verbatim by the operator, given after it was shown to him), worded by `inputs#I4`. Nothing else in the hygiene rule widens.
**Kept:** `:753`'s `stamped 2.1.287  18 pass  0 fail` is a dated reading; "eighteen verify-driven binaries" counts nextest binaries.
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/

## 2026-10-14-resume-and-compact-ledger-rows — Run E, the resume variant and /compact in the tests
**Section:** §1 Test Scope Summary; §3 → `5-command-implementation`, `bootstrap-phases-derive-for-route-setup-project`; §4 Unit Test Strategy; §5 Integration Test Strategy; §6 Path 2; §7 Test Data & Fixtures
**Change:**
- **§4:**
  - The `resume-session` arm gains five `check_resume_session_*` cases, `resume_target_*` and `resume_variants_*`.
  - Relied is 22 / 19, and the older-stamp oracle covers the 21 / 18-id stamps.
  - The local-command decision names the eight `send_local_command_` functions. Was "eight functions, 15 cases"; the master now states no case count, because the code's `#[case]` rows are the count.
- **§5 `cli_verify`:**
  - five interactive runs; MM 22 / 19; `STEPS_PASS` / `ROW_IDS` `[&str; 22]` on Unix and 19 on Windows;
  - summaries 18/4 (Windows 18/1) and 20/2 (Windows 17/2); seven spawn pairs;
  - the stop case plants `.viola-verify-0-resume`;
  - new `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, its killed child asserted on `duration_ms` only.
- **§5 `contract_ledger_probes`:** `ROW_IDS` is 22 and n is 22 / 19.
- **§1:** Run E records the resume variant, and the drift contract replays it.
- **§6 Path 2:** `/compact` is `unconfirmable` at once (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`).
- **§7:**
  - The fake agent's `--resume` builds on `SessionStart.resume-1.json` where the set holds it (2.1.287, 10 key names), else `SessionStart.default` (2.1.288). The owed-payload sentence is retired.
  - The hygiene walk and the Recorded payloads row carry the variant; the 2.1.287 set is 32 files.
- **§3:**
  - Boot's stamp runs Run E (22 / 19).
  - `--local-live`'s `LEDGER_ROWS` is 22 (19-id form elsewhere) and `LIVE_PASS` names `resume-session`.
  - The twenty-two-row live firing was six starts and read `22 pass  0 fail`.
  - The no-screen `verify_window_` case went 34.42 s → 42.98 s with Run E's settle, against kills of 60 s and 45 s (mutants). No kill moved.
**Why:**
- The literals and cases follow Run E, the founder's fork 2 answer (`inputs#I1`).
- The case count is dropped on the overseer's founder-delegated word (`inputs#I7`). Its rule was unrecoverable, and by `#[case]` rows it read 14 → 24 → 32.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/

## 2026-10-16T14-58-10-wrap — the GUI surfaces and their tests leave 0.1.0; the harness keeps what the tree holds
**Section:** §1 Test Scope Summary (entities, surfaces, triggers, coverage counts); §2 Test Strategy; §3 Test Harness Contract → 5-command implementation, Status endpoint shape, Log format, PID file, Test data bootstrap, Bootstrap phases (and the index labels); §4–§6 (viola-ui unit tests, GUI HTTP integration, drivers, selector strategy, Paths 1–7, E1, E3, E4, security sweep, cross-surface parity, bay layout states); §7–§11
**Change:**
- Was an api-service and a web-spa surface: `viola ui` boot with `--ui`, `/health` / `/ready` / `/api/*` reads, the GUI token exchange, SSE E4, Playwright web assertions inside Rust paths, the loopback-GUI security sweep, `http_` / `sse_` tests, the viola-ui unit tests and axum-test integration. Now all of it is cut.
- E4 keeps its heading over the stub line «The frontend is deferred to 0.2.0; its technology is not chosen.». E3 keeps its CLI link half.
- The status shape is `viola list --json` alone. Its item, `budget` and `skipped` shapes are the `list` payload contract.
- `cleanup` has no UI port or url-file step.
- Recounted: six surface entries, fifteen §1 entities, six crates, six parser surfaces and six fuzz parser targets (a seventh seed target), four expansion scenarios.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen.
**Kept:** By the operator's two answers at this wrap (overseer, technical, founder-delegated): the masters stay true to the tree. Each item below is owned by the route entry "Remove the browser leg".
- The browser test leg: `run --browser`, the `playwright` suite and gate value, `browser-missing`, Node, npm and the CI browser job, with one marker line in §9.
- The harness leftovers: `status`'s `null` `ui` and `api_sessions_equal_list`, `cleanup`'s `null` `port_free` / `url_file_removed`, the `token-query` / `gui-token` / `cookie` scan classes, `ui-restart` forwarding, the `fixed-port` nextest group and the `cookie_file` field.
- The closed obs vocabulary in the bound Log format key (the `ui` process, `ui-<port>`, `http-request`, `liveness-changed`, `sse-*`), marked reserved: no emitter in 0.1.0.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/

## 2026-10-16-remove-the-browser-leg — §3 harness: the browser step, the GUI keys and the GUI scan forms leave
**Section:** §3 → 5-command implementation (`boot`, `run`, `status`, `cleanup`, `logs`, Internal harness subcommands, `secret-scan`, `gate`, `pre-push`, Closed enums; Browser-side controls removed); §3 Test Harness Contract (Exit codes, Session record); §3 → Status endpoint shape; §3 → PID file; §3 → Bootstrap phases (test-runner-install, ci-tool-install); §3 → Test data bootstrap; the §3 index labels
**Change:**
- `run` has no `--browser`: step 3 reads none in 0.1.0 and the mutation step keeps its number 4. The `run` `suite` enum is the nine `nextest-unit · nextest-integration · doctest · nextest-e2e · mutants · coverage · perf · fuzz-replay · local-live`; `gate --require` takes the other eight (`gate::SUITES`), so `playwright` is usage `unknown-suite`; JUnit inputs are the four `nextest-unit · nextest-integration · nextest-e2e · coverage`; `archived` holds `junit-{nextest-unit,nextest-integration,coverage}.xml` and `outcomes.json`; `reason` has no `browser-missing`.
- `status` is `{v, cmd, ok, state, pid, uptime_ms, last_error, list, instances}`; `cleanup` reports `{cleaned, processes_gone, endpoint_gone, home_removed, killed}` and removes no cookie file; the session record is `{v, session, home, instances, supervisor_pid, supervisor_started_at}` (an older record with `ui` / `cookie_file` still parses: no `deny_unknown_fields`). `ui-restart` is not forwarded.
- `secret-scan` classes: `claude-stripped` (twelve), `cookie` (the case-insensitive `cookie:` header), `content-canary`, `mode`, `unreadable` (was also `token-query` `?t=`, `gui-token` and `viola_<digits>=`).
- `pre-push`: PATH `<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`; `tools` checks the passwd home, `cc` and the `rust-toolchain.toml` / `test`-job pins; `linux-tests` runs `run --coverage` then `gate --require coverage,doctest`; the document is `{…,"stage",linux{run,gate}}`; no `node` detail.
- Bootstrap: no Node side, no `fixed-port` group, no Node / `npm ci` / Chromium install; `jq`'s scripts are G2, `release-check.sh`, `orphans-check.sh`.
**Why:** This chunk removed the browser test leg and the GUI harness leftovers (founder ruling 2026-10-16, item 3); the scan forms on the operator's P4 answer as the overseer (technical, founder-delegated; `inputs#I1`).
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-remove-the-browser-leg — Log format: the `ui` process, the four web events and the a11y rows leave
**Section:** §3 → Log format (Process logs, Required fields, Harness-side a11y rows removed, Constraints); §1 Test Scope Summary (Test harness requirements `run`, `logs`, Log format; Coverage triggers); §5 Module ↔ DB; §6 Non-path suites (Security sweep, Error sanitization and secret scan)
**Change:**
- Process logs write `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; `process` is `run|hook|mcp|cli`; the `corr` rules read `process-start`, `process-exit`, `panic` and `state-recovered`, `parse-rejected` (obs-plan D-03, D-05). Was also `ui-<port>`, the `ui` process and `http-request`, `liveness-changed`, `sse-opened`, `sse-closed`, marked reserved until the route entry "Remove the browser leg".
- The harness-side a11y rows (a Playwright fixture's `e2e-web/test-results/a11y/*.ndjson` against `e2e-web/schemas/a11y-row.v1.json`) are gone.
- No line may contain a `Cookie` header or a stripped `CLAUDE*` value (was also the GUI token and `?t=`); §1 and §6 restate it.
- §1: `run` item 3 reads none in 0.1.0; `logs` prints the per-process logs of `run` / `hook` / `mcp` / `cli`; no browser-suite runner. §5: the concurrent-append check names a colliding second `run` on the live `run-<name>` file. §6: the security sweep runs in nextest alone.
**Why:** This chunk removed the reserved web half of the closed obs vocabulary with the browser leg (founder ruling 2026-10-16, item 3). The key is kept equal to obs-plan §3's copy in the same pass.
**Kept:** §1's quotation of Founder Direction 3 («run/hook/mcp/ui»), a quote.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-remove-the-browser-leg — §2, §5, §7, §9–§11: no browser runner, one JUnit path, deny probes 12/12
**Section:** §2 (Test pyramid E2E row, Agent-runnable invariants, Test directory + naming conventions, Trigger → layer map V9 row); §5 Setup / teardown lifecycle; §7 (Fake agent, Test data lifecycle); §9 (Caches, the stand-until paragraph, Pipeline structure: Lint, Supply-chain, Integration, E2E, Coverage report rows; tool install; Matrix builds; Test report format; Build failure conditions); §10 Zero-flakiness budget; §11 (CI, Universal)
**Change:**
- E2E runs against harness-booted `viola run … -- viola-fake-agent` through the outer PTY and the rmcp client; no Playwright, Chromium or Node, no `pw.json` / `pw-junit.xml`, no browser-suite directory or naming rule, no Playwright fake-agent consumer.
- §9: rust-cache is the only cache; the E2E row is `agent-run run --e2e` on all 3 OSes; the Integration row runs in parallel (the `fixed-port` group left, nothing is shared between tests); the per-OS test job gates `coverage,doctest`; `junit-${{ matrix.os }}` uploads `target/nextest/ci/junit.xml`, one path; Supply-chain has no npm lockfile audit and nightly runs `advisories` alone besides `fuzz`; the `jq` scripts are three; the pre-push `tools` stage checks no Node; no browser or a11y-lint failure condition.
- `scripts/deny-probes.sh` ends `deny-probes: 12/12 banned, control clean`: one probe per ban, twelve bans (was 13/13 with the axum `http2` ban).
- §10: nextest `retries = 0`; `#[ignore]` is no parking place (Playwright `retries: 0` and `test.skip` left). §11: serialising tests (`max-threads = 1`) is banned outright; the cache ban names rust-cache alone; the `playwright`-suite ban and the recorder ban's runner parenthetical are gone.
**Why:** This chunk removed the browser test leg (founder ruling 2026-10-16, item 3); the axum ban and its probe on the operator's P4 answer as the overseer (technical, founder-delegated; `inputs#I1`). The deny-probe count was 13/13 by the same rule, one probe per ban.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — test data: the fake agent's agents files, the test-scoped PATH, the Agents fixture class, the agents property and fuzz target
**Section:** §2 (Test pyramid: Property-based and Contract rows; Test directory conventions, `tests/support`); §3 → 5-command implementation (`boot --agents-mode`, `--fuzz-replay` seeds); §4 (`viola-agent-claude`: the agents parser bullet); §6 (Property suite and its cargo-fuzz paragraph); §7 (Seed strategies: the `claude agents --json` row and Recorded hook payloads; the fake agent's modes; the hygiene walk)
**Change:**
- The `claude agents --json` seed: `viola-fake-agent` placed as `claude` in a test-scoped bin dir (`tests/support/fake.rs` `claude_bin` / `claude_in`; `empty_bin` for no `claude`) answers from files beside its exe — `agents-mode` (`recorded` · `oversize` · `malformed`), `agents-rows.json`, the live `sessions/<pid>.json` registry — with no env var (was `FAKE_CLAUDE_AGENTS_MODE` via `Command::env`). Every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on`; a plain `spawn` of `list` under an empty bin dir). The fake takes two mode words (was one), `agents` built on the recorded `Agents.recorded-1.json`; units `agents_output_*`, `live_sessions_skip_a_dead_or_reused_pid`, `launch_session_replays_the_recorded_id_and_cwd`, `planted_rows_of_a_malformed_rows_file_are_none`. `boot --agents-mode` (still not built, owed to "End-to-end path binary and harness legs") will write that file, no env.
- Run B's agents read is recorded as `Agents.recorded-1.json` (Run B's own row, one today under `2.1.287`; the set holds 33 files by `ls fixtures/claude/2.1.287 | wc -l`, was 32), schema-checked against `schemas/claude-agents-fixture.v1.json`, hygiene-walked (`agents_fixtures_pass_hygiene` and four planted cases) and shape-compared with the fake's rows (`fake_agent_agents_replay_matches_every_recorded_agents_set`).
- The agents parser as landed: `AgentsList { rows, skipped, tolerated }` or `AgentsFailure` (`Oversize` · `Malformed`); cases `agents_parse_*`, `agents_status_maps_*`, `agents_row_joins_*`, `agents_failure_details_*`; its property at `cases: 512` with `proptest-regressions/agents.txt` committed, its oracle comparing against the input re-parsed (a float does not round-trip through serde_json text); fuzz target `claude_agents_json` (6 synthetic seeds, binary attribute) joins the five.
**Why:** Files beside the exe are the founder's own answer to P4 fork 2 (`inputs#I1`, relayed verbatim by the operator): no third env seam. The test `PATH` keeps a test from reading the host's live sessions (11 on the dev host at P3).
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — tests: agents-join at 23 / 20 rows, Run B's agents read, revive's session-live, the board's witnesses
**Section:** §3 → 5-command implementation (`boot` step 4, `--local-live`); §4 (`viola-agent-claude` ledger arms; `viola-state` read-or-replay); §5 (CLI: the `viola list` board bullet, `tests/cli_verify.rs`, `contract_ledger_probes`); §6 (E1; exit-cause matrix: the unwrapped-name cause and `revive`'s exit-1 causes; `list` row integrity)
**Change:**
- Relied rows 23 on Unix, 20 on Windows (was 22 / 19): `[NN/23]` / `[NN/20]` ending `agents-join`; `STEPS_PASS`, `ROW_IDS` and `LEDGER_ROWS` `[&str; 23]` / 20-id forms; boot step 4 stamps them, Run B making its one agents read once its first turn has settled, before the framing pastes. A run without the dialog replay reads `19 pass  4 fail` on Unix (was 18), without the framing replay `19 pass  1 fail` on Windows, after a tag-turn modal `21 pass  2 fail` / `18 pass  2 fail`. An uninterrupted verify closes eight spawn pairs (was seven), the eighth `agents-probe`. The `agents-join` arm: `check_agents_join_*` beside `agents_variant_keeps_only_run_b_s_own_row`.
- `revive`'s preflight has four readings (was three; the logged session the second of four); `session-live`: `revive_of_a_session_open_in_another_claude_is_session_live` (exit 1, the pair, one `process-exit` `session-live`, no child), `revive_passes_the_session_live_reading` (`--fork`), `revive_whose_agents_read_fails_starts`, and the `preflight_order_*` units.
- The board: `cli_list` `list_prints_planted_rows_as_unwrapped_rows`, `list_status_of_a_joined_wrapper_is_idle_then_busy`, `list_of_an_answer_that_does_not_parse_equals_the_no_claude_board`, `list_under_a_name_logs_the_agents_read`, `list_without_a_name_logs_no_agents_read`, with `board_row_*` and `board_join_*` (was: separator not built). E1 / the exit-21 unwrapped-name cause: `e1_send_naming_an_unwrapped_row_is_unreachable` in `cli_list`, exit 21 only; the hint is owed to "Exit-cause code catalogue". Row integrity landed: `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode`, the Windows terminal case reading names as text under a 200-column terminal, the no-ESC assertion Unix-only, on the 20 s kill line.
**Why:** Run B's read and `revive`'s read are the founder's own answers to P4 forks 1 and 3 (`inputs#I1`, relayed verbatim by the operator); the card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. The hint's owner is the operator's P5 card (`inputs#I2`).
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-17-end-to-end-path-binary-and-harness-legs — tests: run --e2e, the E2E regex widened, the integration complement
**Section:** §2 test locations; §3 Exit codes; §3 → 5-command implementation (`run` steps 1 and 2, Test selection); §3 → Bootstrap phases (derive for route / setup-project) (`[profile.ci]` verify-driven override)
**Change:**
- `run --e2e` is built: suite `nextest-e2e`, run third after unit and integration, in the default and `--all`, through the same nextest shape and target dir, a `--filter` intersecting it; alone it runs no doctest; the run-archive list stays unit, integration, coverage (was: a usage error until the first E2E binary).
- The E2E regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (was `^(path|…)_`, which does not select a numbered Path binary): 20 binaries / 157 tests at this chunk's end. The integration layer is `kind(test) & !binary(…)`, its complement (was `kind(test)` until the first E2E-prefixed binary). An unmatched `binary()` regex is still a parse error, re-measured on 0.9.146 (0.9.133 kept as the first reading).
- §2: the viola-e2e form is `tests/{path<N>,mcp,cross}_<topic>.rs`; the root package carries numbered Path binaries too (`tests/path1_run_start.rs`); both report in the E2E layer.
- §3 Exit codes: `run --e2e` leaves the not-built example; the rule stands.
- The verify-driven 20 s override names twenty binaries (was eighteen): `path1_run_start` joined; `cross_list_rows`, already on the line, was missing from the list.
**Why:** The plan's binary names did not match the written regex; the widening is the Viola overseer's technical answer, founder-delegated, PROVISIONAL until the founder's Epoch 5 batch. Numbered Path binaries name the test plan's numbered Paths, and the widened pattern matches no other binary.
**Kept:** The "unbuilt selector is a usage error" rule, without its example.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/

## 2026-10-17-end-to-end-path-binary-and-harness-legs — tests: boot's two flags, harness_session, status over viola list
**Section:** §3 → 5-command implementation (`boot`, `run` step 2's boot models, `status`, Closed enums); §3 → Status endpoint shape; §3 Session record
**Change:**
- `boot --statusline-echo` is built (was owed): conflicts with `--unstamped` (usage `arguments`); after step 4 it writes `<home>/statusline-source.json` (`replace_private` 0600, command `<bin dir>/viola-fake-agent[.exe] statusline-echo <home>/fake/statusline.marker`, forward slashes) and makes `<home>/fake/`; a path outside ASCII letters, digits and `_ - . / :` fails `build-failed`; no env var, no `budget.json`.
- `boot --agents-mode recorded|oversize|malformed` is built (was owed): a stale `bin/agents-mode` removed before step 4, the value written (0600) after step 4 and the plant, before `supervise.json`, because Run B's agents read goes through that copy (a mode written first failed `agents-join`); `agents_mode` (serde default) in `session.json` and `supervise.json`.
- `harness_session(test, label, opts) -> HarnessSession` is a plain function and drop guard (was: an rstest fixture with a fixture-owned supervisor thread): library `boot`, `stamp:true`, the spawned `supervise` kept; `SessionOptions` default `overseer` + `builder`, no echo, `recorded`; `Drop` cleans up, then keeps the home only under `AGENT_RUN_KEEP_HOMES=1`, or `AGENT_RUN_KEEP_FAILED=1` while panicking. "One of two fixtures" reads "one of two boot models".
- `status` runs `<bin dir>/viola --home <home> list --json` from `supervise.json`'s bin dir on the session `PATH`, `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed, stdout through `take(MAX_FRAME + 1)`. The interim (`list` null, state from wrapper liveness) retires: `ready` = exit 0 and every booted instance `wrapped:true, liveness:"live"`; `degraded` = one `stale` or absent, or no `items`; `down` = non-zero (`list-exit-<code>`), no exit / spawn / `supervise.json` (`list-exit-none`, a new closed value), or exit 0 with no JSON object (`list-exit-0`, `list:null`).
**Why:** Fork 2 (the spawned `supervise`, so `cleanup`'s pid + start-time proof stays exact) is the overseer's answer as the founder's delegate, PROVISIONAL until his Epoch 5 batch; a gone instance is omitted from `list`, so an absent booted one is not `ready`.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/

## 2026-10-17-end-to-end-path-binary-and-harness-legs — tests: Path 1 as one flow and Path 6's harness-session leg landed
**Section:** §6 Path 1 (As landed); §6 Path 6 (step 1, As landed); §3 → 5-command implementation (`run` step 2, the `viola_e2e::fixtures` copy)
**Change:**
- Path 1: steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs` (`path1_run_start_one_home_in_order`), in the E2E layer and on the verify-driven 20 s kill line, its helpers in `tests/support/start.rs`; wall 4.69 s (5.14 s instrumented) on the Linux dev host against the 10 s `mutants` kill. The `viola_e2e::fixtures` copy still waits for its first consumer: Path 1 is not one.
- Path 6: `boot --statusline-echo` and the harness-session form landed (was owed) as `crates/viola-e2e/tests/path6_budget_harness.rs` `path6_two_instance_release_over_a_harness_session`; step 7's leg asserts the lifted `builder` and the still-paused `overseer`, one override record for `builder` alone, the WHEEL cell, and the Unix / Windows statusline branches. Only the MCP form stays owed to "MCP server for drivers".
**Why:** Fork 1 (Path 1 on the root chain, not a hand-mirrored fixture copy) is the overseer's answer as the founder's delegate, PROVISIONAL until his Epoch 5 batch.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/

## 2026-10-17-cli-machine-contract — tests: revive's machine form, the completeness case and the home-resolution cases landed
**Section:** §1 CLI verbs entity; §1 Surface cli; §5 On-disk strict-modes refusals; §5 Wrapper channel (out-of-range `Percent`); §5 CLI (`cli_controls_not_disableable`, the completeness case, a new home-resolution and machine-contract bullet); §6 Exit-cause matrix (exit 1 `revive`, `revive --list`)
**Change:**
- `revive` takes `--json` on both arms (was: "takes no `--json`" / "the human signal only … no JSON document"); `tests/cli_revive.rs` holds one `revive_json_*` case per code (`already-live`, `no-session` ×2, `session-live`, `cwd-missing`, `strict-modes-failed` on both arms) and `revive_list_json_*`; `revive_list_under_a_name_logs_its_start_and_exit` pins `--list`'s `cli` log (was: "no log line"). A post-preflight `run` start refusal prints no document and no case asserts one; joining the rstest table and that document are owed to "Exit-cause code catalogue" (was: the document owed to "CLI machine contract").
- The completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` landed (was: listed in the file's header): the `Options:` section of `viola --help`, the `VIOLA_*` literals walked from the product sources (`viola-e2e` left out) and `schemas/config.v1.json`'s properties, equal to `CONFIG_KEYS` (`config_keys_are_the_config_schema_properties`); new rows `viola_dir_does_not_disable_the_home_scan`, `viola_variable_does_not_disable_the_send_paste_control`, `config_key_does_not_disable_the_identity_strip`; `setting_does_not_disable_the_budget_gate` gains `seven_day_over_one_hundred` and `v_two` (was: "three rows"; no count written).
- New §5 bullet: `tests/cli_machine_contract.rs` (each verb's document and literal exit; the resolution order, two homes, a home through a link) with the `resolve_home_*` / `canonical_home_*` / `plain_drive_*` units; a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`, and a path oracle compares `resolved(home)`.
**Why:** the chunk's tests as landed; the home resolution they pin is NOT ratified (security-plan's entry).
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-output-discipline — viola --help pinned by literal; cli_output_plain's new cases; the colour variables in Vector 6
**Section:** §5 Integration Test Strategy (Human-mode layouts; CLI: trycmd cases, `tests/cli_output_plain.rs`, `tests/cli_controls_not_disableable.rs`); §6 E2E Test Strategy (surface table, cli row); §7 Test Data & Fixtures (Human-mode expected output)
**Change:**
- Was: `viola-help` a trycmd `tests/cmd/*.toml` case; now `viola --help` (the grouped verb table) is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (`help_is_the_grouped_verb_table`) — §5 Human-mode layouts, §5 CLI, §6 cli tool cell and §7 each name it beside the `viola list` board's literals.
- `tests/cli_output_plain.rs` adds `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs` (each served verb's `--help` exit 0, its usage program `viola` or `viola.exe`; no `hook`, `help`, `link`, `unlink`, `plugin`) and, on Unix, no CR not followed by LF; the parser `sgr_attributes` (with `redraws`, `sets_a_foreground`) lives in `tests/support/sgr.rs`, its self-check case staying in that file.
- §6 cli row verdict: bold NAME and amber `DIALOG` under the outer PTY (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`; the parsed attribute set on windows-2025); dim `stale` at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity".
- Vector 6: `NO_COLOR`, `TERM`, `COLORTERM` are cases of `setting_does_not_disable_the_send_paste_control`, whose helper removes the three inherited values first.
**Why:** measured this chunk — no manifest holds trycmd and no `tests/cmd/` exists; the help is pinned by literal. The stale-dim witness's owner is the halt's answer (a delegate's, the Viola overseer). The other trycmd claims are carried to "Version done-check", not amended here.
**Kept:** the remaining trycmd / assert_cmd tooling claims (§3, §5, §6, §7, §10) stand until "Version done-check" decides tool or text.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/

## 2026-10-17-mcp-server-for-drivers — the MCP tests as landed
**Section:** §1 Test Scope Summary (ipc-internal MCP surface, Signal); §5 Integration Test Strategy (MCP; Wrapper channel impostor bullet); §6 E2E Test Strategy (Drivers per surface, MCP stdio row; Path 2, 3, 4, 5, 6 As landed; Path 3 step 7; Non-path suites → Security sweep)
**Change:**
- §1 Signal: was `{"error":"instance-unreachable"|"wrapper-fault"}`; now `instance-unreachable` with `detail: null`, `wrapper-fault` with `{code,message,data}`, `invalid-params` with a closed field word and no frame, each text `refused: <code>`; an internal handler fault JSON-RPC `-32603` `internal error`, `data: null`, an unknown tool `-32602`; `tools/list` exactly the five tools with `ttlMs` 0 and `cacheScope` `private`.
- §5 MCP: was "using the sync channel client inside the handler under test"; now `crates/viola-mcp/tests/handlers.rs` drives the transport-free `viola_mcp::Tools` against a `viola_channel::Server` test wrapper (snapshot through `write_snapshot`), the handlers using the Tokio client. Impostor bullet and Path 3 step 7: the MCP body carries `detail: null`, equal to the CLI's less `v`.
- §6 Drivers row: was `TokioChildProcess` and dev-dep features `client` + `transport-child-process`; now the rmcp client over a test-owned `viola mcp` child, stdin closed and its exit awaited (the transport killed the server 3 s after a close and truncated a coverage profile), `viola-e2e` taking rmcp `client` only as a normal dependency.
- Paths 2–6: was "owed"; the MCP steps landed in `crates/viola-e2e/tests/mcp_tools.rs` — `mcp_tools_path2_path3_send_wait_last_match_the_cli`, `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, `mcp_tools_path6_budget_paused_send` — each compared with the CLI's `--json` less `v`; the `wait` clamp by `handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`.
- Security sweep: was `mcp_schema_violations.rs`; now `handlers_schema_violations_write_no_frame`, the unit `tool_inputs_are_revalidated_before_any_frame` and `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`.
**Why:** the chunk built the MCP surface and its tests; the child-process transport was measured to truncate a coverage profile.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — the CI gates: the channel tree form, rmcp features, deny-probes 13/13
**Section:** §9 CI Integration (Pipeline structure, Lint and Supply-chain rows; Pipeline fails on, Lint and Supply-chain errors)
**Change:**
- Lint row: the sync-crate `cargo check` target set is four (`viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`) — was five with `viola-channel`; was "the rmcp release-graph assertion … has no subject until `viola-mcp` exists"; now the per-OS step `rmcp features (exactly server + transport-io, non-vacuous)` runs `bash scripts/rmcp-features.sh` after `Lint probes`, last line `rmcp-features: server transport-io`.
- Supply-chain row: after the sole-root loop, `bash scripts/channel-tokio-free.sh` in the same step (no `tokio v` line without the feature, read from the printed line never the exit; its control one; `channel-tokio-free: ok`); `deny-probes.sh` ends `13/13 banned, control clean` — was `12/12`: the twelve bans plus `channel-tokio-edge` with its channel control.
- Fails on: a lint error adds a red `rmcp-features.sh`; a supply-chain error adds a red `channel-tokio-free.sh`, and the deny-probes line reads 13/13.
**Why:** the chunk landed both scripts and the probe; `viola-channel` left the sole-root list because no listed crate depends on it.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — harness keys and the time mechanism
**Section:** §3 → Bootstrap phases (derive for route / setup-project); §3 → 5-command implementation (`harness_session`); §7 Test Data & Fixtures (Time-sensitive data); §8 Mocking & Stubbing Discipline (Time mocking)
**Change:**
- Bootstrap phases: the `ci` profile's 20 s kill line holds twenty-one verify-driven binaries (the named list on that filter line, `mcp_tools` joined; its cases read 0.01–5.9 s; `path6_budget_harness` not on it) — was twenty; `viola-e2e` takes rmcp `>=3.4.1, <3.5` with `client` only and tokio `=1.53.1` `rt` + `process` — was `client` + `transport-child-process` and tokio `test-util`.
- 5-command implementation: was "`harness_session` … with `stamp:true`"; now `stamp: !opts.unstamped`, `SessionOptions::unstamped` default false (the in-test form of `boot --unstamped`; Path 7's MCP leg boots an unstamped `builder` alone).
- §7 and §8: no manifest names tokio `test-util` (as landed: `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`); `viola-mcp` owns no timer — its `wait` bound is the pure `wait_timeout`, a unit case — so no `start_paused` test exists.
**Why:** the counted list is the filter line's names (the count's rule); the unstamped option serves the Path 7 MCP leg; the time qualifier keeps a planned mechanism from reading as built.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-optional-plugin-install — the plugin install suite and the reshaped help pin
**Section:** §5 Integration Test Strategy → Cross-module patterns covered → CLI
**Change:**
- `help_lists_only_served_verbs`: was "each served verb answers `--help` exit 0 … no `plugin` in the table"; now each served verb path answers (`plugin install --help` included), its usage the program then the verb's words, the table's words read from the group rows alone (`plugin install` one entry of `setup:`), no bare `plugin`.
- New: `tests/cli_plugin_install.rs` pins `viola plugin install [--json]` against `CARGO_BIN_EXE_viola`, each case in a fresh home (the tree and its `.mcp.json`, the re-run, the modes under `umask 000`, the `ok` document, the planning form, no escape byte, the widened-home refusal, the self pair, `viola plugin` exit 2), its units in `marketplace.rs`, `src/cmd/plugin.rs` and `src/human/verbs.rs` named.
**Why:** the chunk added the verb and moved the help pin; the master names the functions and states no case count.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-18-compact-post-condition-ledger-row — twenty-four rows, Run B's five added pastes, the compact variant
**Section:** §1 (Coverage triggers, Critical paths); §2 (Contract row); §3 → 5-command implementation, → Bootstrap phases; §4 (relied set, the `local-command-compact` arm, the older-stamp oracle); §5 (`cli_verify`, `contract_ledger_probes`); §7 (Seed strategies, Fixture hygiene, Fixture library, Fake agent)
**Change:** relied 24 on Unix, 21 on Windows (`local-command-compact` appended last); `cli_verify` `[NN/24]` / `[NN/21]`, `STEPS_PASS` / `ROW_IDS` `[&str; 24]` / `[&str; 21]`; `contract_ledger_probes` `ROW_IDS: [&str; 24]`; `LEDGER_ROWS` / `LIVE_PASS` 24 / 21; boot's Run B five added compiled pastes (`/compact` before `/clear`), five added turns since this chunk, no kill line moved; the framing variants `paste-<n>`, `compact-<n>`, `clear-<n>` — six under 2.1.287, the set 34 files; `fake_agent_compact_replay_matches_every_recorded_compact_set`; the arm's three `check_local_command_compact_*` cases; the older-stamp oracle at the twenty-three older ids (eighteen + `resume-session` + `agents-join` + `local-command-compact` verifies on Windows only); the no-framing summary `19 pass  5 fail` / `19 pass  2 fail`, the tag-turn modal `21 pass  3 fail` / `18 pass  3 fail`; the fake agent's `--framing` compact replay (the last fired SessionStart's id, the one built payload beside `--resume`'s). Was: 23 / 20, four added pastes, five variants, 33 files, the twenty-one / eighteen oracle, `19 pass  4 fail` / `19 pass  1 fail`, `21 pass  2 fail`.
**Why:** the chunk built the row, its probe and its replay (report Symbols / APIs, Counts); the sixth Run B paste is the founder's own answer to P4 fork 1 («Шестая вставка в Run B», `inputs#I3`, relayed verbatim by the operator); this wrap's answer-card confirm of the proposals inside it is a delegate's (delegate the Viola overseer), PROVISIONAL until his own word in the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-18-compact-post-condition-ledger-row — /compact's same-session confirmation and its own window at the unit and E2E tiers
**Section:** §1 Critical paths; §4 (the local-command decision, the trailing-newline cases); §6 Path 2
**Change:** `/compact` on a verified CLI waits only for a `session-start` of cause `compact` and the remembered (same) id, inside `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL), never claimed by a `prompt-submitted`; twelve `send_local_command_` functions in `src/run/send/classify.rs` (four `…_compact_*` added) and `send_compact_confirm_window_is_its_own_bound`; eight `trailing_newline` tests, four in classify.rs, four local-command cases; Path 2: `send_compact_on_a_verified_cli_is_confirmed_by_its_same_session` (on the recorded `compact-1`) and `send_compact_on_an_unverified_cli_is_unconfirmable_at_once`; the `local` outcome lists `/compact` only on an unverified CLI; a confirmed `/compact` prints `[RB] read back`. Was: `/compact` unconfirmable at once on any version (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`, retired), eight functions, seven tests, three local-command cases.
**Why:** the chunk built the claim arm and the window (report Symbols / APIs, Retired); the counts re-derived from the tree at this wrap.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-18-plugin-precedence-live-readings — the VIOLA_DIR home step ratified
**Section:** §5 Integration Test Strategy (Home resolution and the machine contract)
**Change:** was "the CLI verbs' `VIOLA_DIR` step and the canonicalisation are NOT ratified, the founder's word owed"; now they are a widening the founder ratified (security-plan §Input Validation).
**Why:** follows the security-plan ratification of the whole home step by the founder's own word, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-19T07-00-05-wrap/

## 2026-10-19-dialog-hold-for-the-driver — the hold's unit cases, sixteen kinds, six wake kinds, the controls table at ten rows
**Section:** §1 Test Scope Summary (Path `wait` / `last`); §4 Unit Test Strategy (Conventions → Test grouping; What unit tests cover → viola-core, root bin, the channel `wait` handler); §5 Integration Test Strategy (Cross-module patterns → On-disk; CLI, `tests/cli_controls_not_disableable.rs`).
**Change:**
- The wake set is six (rule: `EventKind::WAIT_WAKE`). Was five; `dialog-expired` is added, at §1 Path 3 and at §4's channel `wait` handler.
- `state_events.rs` round-trips 16 kinds (rule: the length of `EventKind::ALL`). Was 15; `dialog-expired` carries `dialog_id` and `dialog_kind`.
- The controls table's `SETTINGS` holds ten rows (rule: `SETTINGS` in the file). Was nine; the new row is `dialog_hold_s`, with the booted case `setting_does_not_unbound_the_dialog_hold` (`0`, `3601`, `-1`, `"3600"`, `false`, each one `parse-rejected{config-json, malformed}`).
- §4 viola-core gains the `DialogHold` bounds case and the counts `ALL` 16 / `WAIT_WAKE` 6. §4 root bin gains the dialog-hold cases on the injected clock: `read_hold`, the 120 s hold, the expiry's one `dialog-expired` and `dialog-answered{deadline_hit:true}`, no record on a hand-back or an answer, `pending_dialog` before the line, the hook bound, the timeout margin and the `wait` line form.
- §4 Test grouping: `viola-core` (no rstest dev-dependency) writes table-driven cases as literal tables, beside `viola-e2e`.
**Why:** The chunk made the dialog hold configurable and recorded its expiry as a wrapper event that wakes `wait`. These are the tiers that prove it: the clock-driven unit case §11 Universal requires for the hold, the on-disk round trip, and the control row for the new key. The `dialog_hold_s` row records a boundary widening the founder answered himself to P4 fork Q1 (`inputs#I2`). The wrap's answer-card confirm was the Viola overseer's (delegate viola-overseer), PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/

## 2026-10-19-overseer-switch-to-viola — the `option_details` and `background_tasks` cases named, with the switch's live readings
**Section:** §4 Unit Test Strategy (viola-agent-claude); §5 Integration Test Strategy (On-disk, `state_events.rs`); §6 Scenario: Path 3 — `wait` / `last`; §6 Scenario: Path 4 — dialog → `answer`.
**Change:**
- §4 names `question_option_details_carry_description_and_preview`, `question_option_details_are_null_when_absent_or_not_a_string` (`dialog.rs`), `turn_ended_background_tasks_counts_the_array` and `turn_ended_background_tasks_is_null_when_absent_or_not_an_array` (`hook.rs`).
- §5 On-disk names `state_events_background_tasks_and_option_details_are_known_keys` and `EventKind::TurnEnded`'s data keys `last_assistant_message`, `background_tasks`.
- Path 3 names `hook_a_recorded_stop_carries_its_background_task_count` (the 2.1.287 fixture's empty array reads 0; non-zero and `null` held at the unit tier) and the live reading on 2.1.296, a local measurement and no suite: `wait --json` woke on `turn-ended` with `background_tasks` 0, and `last` read the turn.
- Path 4 names `hook_a_recorded_question_carries_its_option_details` (the 2.1.287 fixture holds no `preview`, so previews read `null`) and the live reading on 2.1.296: a `question` with descriptions and previews answered by a product `viola answer`, `dialog-answered{deadline_hit:false}` with no `from`.
**Why:** The chunk's two additive data keys carry tests at the unit and integration tiers, and the plan asked for the switch's live readings stated as a local measurement, not a suite.
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-19-newer-claude-cli-rows — the long-turn screen, the second literal and the 2.1.302 set in the suite
**Section:** §4 Unit Test Strategy (viola-agent-claude); §5 Integration Test Strategy (CLI `cli_verify`, `contract_ledger_probes`); §7 Test Data & Fixtures (Fake agent modes; Fixture hygiene walk; Seed strategies, Recorded hook payloads); §2 Test Strategy (the Contract row).
**Change:**
- §4: a bullet for the `input-box-signature` arm, which reads three Run B screens (`ready`, `turn`, `long_turn`, `TypedRun::long_turn` never written to `measured`), with the cases `check_input_box_signature_long_turn_holding_the_literal_passes`, `…_holding_a_modal_fails` and `…_of_none_fails`. The `SIGNATURES` bullet names two input-box literals, `for agents` and `(shift+tab to cycle)`, pinned by the `resting_footer` cases (was one, `for agents`).
- §5 `cli_verify`: the long-turn modal case `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`, at `19 pass  5 fail` on Unix and `16 pass  5 fail` on Windows, its receipt holding the probe prompt and the long paste only. `contract_ledger_probes`' stamped sets are `2.1.287` and `2.1.302` (was `2.1.287` alone).
- §7: the fake agent has thirteen argv options, eleven of them after the statusline two, nine for verify's interactive runs (was twelve, ten and eight). `--long-turn-screen <phase>` sits beside `--tag-turn-screen`, with parse cases `opts_long_turn_screen_*`. The hygiene walk and the Recorded hook payloads row include `2.1.302`: its dialog, framing, status line, resume and agents variants and its screens (was `2.1.287` only for framing, status line, resume and agents).
- §2 Contract row: the dialog variants are 12 under each of `2.1.288`, `2.1.287` and `2.1.302`; the framing variants are six under each of `2.1.287` and `2.1.302`.
**Why:** The suite now reads what 2.1.302 showed: the long turn's settled screen, a second compiled literal, and a recorded set that CI replays and stamps.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/

## 2026-10-19-newer-claude-cli-rows — the readiness stall report in wait_ready
**Section:** §5 Integration Test Strategy (Readiness stall report).
**Change:** New. When `tests/support/home.rs` `wait_ready` reaches its deadline, it appends the stalled wrapper's state to the watch report before the panic, so the report reaches the JUnit. The state is the wrapper's last `run-<name>.ndjson` event; on macOS `/usr/bin/sample <pid> 1`'s call-graph stack lines (bounded at 4 s, at most 120); on Linux `/proc/<pid>/wchan` and the `stat` state (`note_stall`, `wrapper_pid`, `process_state` per OS). It signals nothing and adds nothing to a passing boot. It is the witness for the macOS readiness stall inside `run.start`, whose phase is not yet named.
**Why:** The stall recurred on 1 of 4 macOS attempts read and has not been located. The founder's own word, relayed verbatim by the operator, closes the chunk with the criterion unmet and a route WATCH as its owner, the report as its witness. That is his departure from §10's in-chunk quarantine for this flake; §10 is not amended.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/

## 2026-10-20-t06-58-31-wrap — the founder's Epoch 5 batch: the E2E pattern, Path 1, harness_session and the owed-rows card his
**Section:** 2. Test Strategy (the E2E layer's regex) · 5. Integration Test Strategy (`tests/cli_verify.rs`; `tests/cli_plugin_install.rs`) · 6. E2E Test Strategy (Scenario: Path 1) · §3 → `5-command-implementation` (boot, `run --e2e`, `harness_session`, Path 1)
**Change:**
- The E2E regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, Path 1 at the root tier and `harness_session` keeping the spawned `supervise` stand as built; was "the Viola overseer, technical, founder-delegated, PROVISIONAL" and "the overseer as the founder's delegate, PROVISIONAL until the founder's Epoch 5 batch" (five sites, the key file's four included); now the marks are gone.
- Run B's probe `--settings` stands as the founder's own answer; was "the wrap's card confirm provisional, delegate viola-overseer" (here and in the key file); now gone. `plugin_install_prints_the_two_commands` pins "the printed lines by literal"; was "the planning form by literal".
- In this file's history the provisional marks of these entries are superseded 2026-10-20, their other claims standing, none pruned: 2026-10-13-paste-newline-ledger-row — eighteen ledger rows in the harness and the suites, `paste-3`, Run B's four added pastes; 2026-10-13-owed-ledger-rows — the relied set, the statusline arms, the StatusLine fixtures and the kept workspace.repo; 2026-10-14-resume-and-compact-ledger-rows — Run E, the resume variant and /compact in the tests (the case count dropped); 2026-10-16T14-58-10-wrap — the GUI surfaces and their tests leave 0.1.0; the harness keeps what the tree holds; 2026-10-16-unwrapped-sessions-on-the-board — tests: agents-join at 23 / 20 rows, Run B's agents read, revive's session-live, the board's witnesses; the three 2026-10-17-end-to-end-path-binary-and-harness-legs entries; 2026-10-17-cli-output-discipline — viola --help pinned by literal; cli_output_plain's new cases; the colour variables in Vector 6 (the stale-dim witness's owner); 2026-10-18-compact-post-condition-ledger-row — twenty-four rows, Run B's five added pastes, the compact variant; 2026-10-19-dialog-hold-for-the-driver — the hold's unit cases, sixteen kinds, six wake kinds, the controls table at ten rows.
**Why:** the founder's own answers to his Epoch 5 batch, 2026-10-20, relayed by the Viola overseer through the operator's directive, and his own word widening it to every Epoch 5 delegate mark, the halt answers included; the axum ban of 2026-10-16-remove-the-browser-leg stays a separate item. The bodies take writing rule 7's form, by the overseer's technical answer at this wrap.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-epoch-5-cleanup — the mutants profile's paste-hint class and the verify_window_ kill at 60 s
**Section:** §3 → `bootstrap-phases-derive-for-route-setup-project` (test-runner-install: `[profile.ci]`'s Run E sentence, `[profile.mutants]`, its fail-fast kill list, the viola-e2e override; Dev-deps in `viola-e2e`)
**Change:**
- `[profile.mutants]` holds four overrides, first match winning: `verify_window_` at `{ period = "20s", terminate-after = 3 }` (a 60 s kill, the class's `ci` kill) stands first, then `package(viola-e2e)`, `send_window_` and `send_under_the_paste_hint_`, each at `{ period = "15s", terminate-after = 2 }` (30 s). Was three overrides, `verify_window_` at 45 s.
- The paste-hint class holds the two `send_under_the_paste_hint_` cases, which wait out the product's gate under a held hint by design, 8.7–9.9 s on three OSes under `ci`. `mutants_paste_hint_override_holds_the_class_above_the_default_kill` pins its kill above the default.
- The no-screen `verify_window_` case reads 42.952 s alone on the dev host and 42.874 / 43.078 / 43.714 s on the CI legs, below the 60 s kill (was «34.3 s … below the 45 s kill»). The planted-hang controls: TIMEOUT 60.003 s / PASS 112.961 s for the class, TIMEOUT 30.003 s / PASS 48.413 s for the paste-hint class.
- The fail-fast kill list reads 10 s; viola-e2e 30 s; the `send_window_` and paste-hint classes 30 s; the `verify_window_` class 60 s. The viola-e2e harness case takes that class's 60 s kill. Run E's sentence reads «the class's 60 s kill (the `mutants` profile's too)».
- Dev-deps in `viola-e2e` add libc `=0.2.189` under `cfg(unix)`, for the keeper's `statvfs` headroom read.
**Why:** both floors are by design and measured across runs, so each kill moved with its planted-hang pair, never by raising the default; the replay contracts got faster instead (their sets run at once).
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/

## 2026-10-20-epoch-5-cleanup — the headroom floor, the hook hangup named, the scratch control made, E4 retitled
**Section:** §3 → `test-data-bootstrap` (Mechanism: `prepare_home_base`) · §3 → `5-command-implementation` (`boot` step 2; `run --mutants`' scratch witness) · §7 Test Data & Fixtures (the fake agent's exit bullet) · §10 Quality Gates & Coverage Targets (Stack adjustments) · §6 E2E Test Strategy (Scenario: E4)
**Change:**
- Test data bootstrap and `boot` step 2: a linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` is refused with `e2e-home backing: {free} MiB free, below the {floor} MiB floor`. The decision is the plain `headroom_refusal(free, floor_mib)`, unit-tested on every OS. There is one compiled constant per keeper, set by no variable, flag or config key, its basis the EDQUOT reading of about 8 GiB plus 1 GiB. A plain base reads none; `boot` reads it as `build-failed`.
- The scratch witness: the plain sibling is granted the Users group's write explicitly (`icacls … /grant *S-1-5-32-545:(OI)(CI)(W)`, exit asserted), so it reads `Writable` whatever its parent grants. Was a plain-created sibling that inherited the grant.
- The fake agent's exit bullet: on Unix every command it spawns (each hook, the status line command) runs in a process group of its own. A hangup of its PTY, at its exit or when a killed wrapper closes the PTY, no longer reaches a hook in its exit-time coverage write. Was: «a hook is in its PTY's foreground process group».
- Stack adjustments: a second measured mechanism, the hangup cutting a `viola hook`'s profile, with 0 refused of 800 in 200 runs of `chaos_wait_parked_wrapper_killed_exits_21` after the change. The census witness gains `--case <binary stem> <test name> …`.
- E4 reads «none in 0.1.0» over an `N/A` gap marker (was «SSE `Last-Event-ID` resume»).
**Why:** the floor stops a run before the quota'd tmpfs's EDQUOT. The census named the truncated profile's killer two-sided. A control made explicitly holds inside cargo-mutants' owner-only copy too. E4 named a retired web scenario.
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the `Mcp.` exchange: its walk, its seed row and its replay
**Section:** §1 Test Scope Summary (the MCP stdio Signal) · §2 Test Strategy (Contract row) · §6 E2E Test Strategy (the MCP stdio driver row) · §7 Test Data & Fixtures (Fixture library; Seed strategies; Fixture hygiene: the walk, its classes, the recorder rule)
**Change:**
- §7: a seed row "Recorded MCP exchange" — `Mcp.<variant>.json` (`Mcp.tools-list-1.json` in `2.1.302`) recorded by the test-only `viola-mcp-tap`: `record` between one live `claude -p` start and a direct spawn of `viola mcp`, each line through `take(MAX_FRAME)` into a 0600 capture never committed; `fixture` through the `--record` scrub and refusal (the four closed codes, the file name only). The fixture library names the exchanges. `claude_fixtures` leaves `Mcp.` files out and `mcp_fixtures` walks them against `schemas/claude-mcp-fixture.v1.json` with the path, username and email checks (`mcp_fixtures_pass_hygiene`; planted `planted_clean_mcp_exchange_passes`, `planted_dirty_mcp_exchange_is_rejected`, `planted_mcp_schema_violation_is_rejected`). The recorder rule covers the tap's `fixture` step. Was: `verify --record` the one source.
- §2: the jsonschema clause checks every `fixtures/claude/*/Mcp.*.json` against `schemas/claude-mcp-fixture.v1.json`. Was: every file but the status line, agents and screen classes against `claude-fixture.v1.json`.
- §6 / §1: `tests/mcp_recorded_exchange.rs` (`binary(mcp_recorded_exchange)`) writes the recorded client frames to a `viola mcp` child, takes the `tools/list` response by its request's id, and holds the served result to the recorded key and type shape, `ttlMs` and `cacheScope` included, its tool names in order. Was: `tests/mcp_tools.rs` the suite's one case file.
**Why:** item (7); the tap's recording is the founder's own answer to its P4 widening fork, each answer relayed verbatim, and he confirmed at this wrap's answer card, relayed verbatim, that §7's seed row, library and recorder rule lie wholly inside it.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the moved units re-cited and the orphans probe line
**Section:** §5 Integration Test Strategy (the `tests/cli_list.rs` board bullet; the `tests/cli_verify.rs` bullet) · §9 CI Integration (Lint row; the Lint errors bullet) · §10 Quality Gates & Coverage Targets (Mutation gate: the Epoch 4 `typed.rs` dispositions)
**Change:**
- The board units are the `src/human/board/` units (was `src/human/board.rs`, twice); the stop's unit cases sit in the `src/cmd/verify/typed/` module (was `typed.rs`); the Epoch 4 dispositions name `typed.rs`'s code under `src/cmd/verify/typed/`, "inside that module" and "a case in its unit tests" (was "inside `typed.rs`", "`typed.rs`'s `mod tests`").
- The orphans probe fires on each of its two planted orphans, last line `orphans-check probes: 2/2 fired, control clean`, and the Lint errors trigger reads `2/2 fired`. Was: `1/1 fired`.
**Why:** the size split moved code with every case's name kept. The probe line was a stale literal from before this chunk: the script already held two planted cases, and its log reads 2/2.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-20-epoch-5-cleanup-ii — `prepare` killed on every host, the Epoch 5 survivors disposed
**Section:** §10 Quality Gates & Coverage Targets (Mutation gate paragraph)
**Change:**
- `prepare` takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`), so both arms are reached on every host, and its `Ok(None)` and deleted-`!` mutants read killed on the Linux dev host by its scratch case, as measured at the chunk's survivors ledger. Was: "reachable only on a Windows host, and are recorded not measured here … On the Linux dev host they still read missed". The Epoch 4 note reads the 2 `prepare` rows "stood as recorded (killed on every host since)".
- The Epoch 5 boundary audit's 16 survivors were each disposed by a nextest case in the package that owns the mutated file, 10 killed and 6 after a behaviour-preserving restatement, each red under its hand-applied mutant and green reverted byte-exact, with no mutation run made and none equivalent or recorded.
**Why:** item (4) closes the audit's survivors so the next audit reads them closed.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-25-u55-door — test-plan: split into its body and 9 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 9 category files; the map .andromeda/test-plan.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-25 for the door on viola's masters (F2), given after two review rounds, the reviewer's wording then applied verbatim on two N rows (README step 3 (a), no third agent pass), at 1.41 M (0.97× its cost guide); relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/a2a855dc-9f94-4016-87d5-2f52c0246ab5/scratchpad/test-plan.card.md

## 2026-10-25-windows-endpoint-admission — the Windows listener's admission negatives
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Security control negatives, two new cases; the body's Security control negatives line) · §5 Integration Test Strategy (integration-test-strategy.md: Module ↔ IPC, Test approach) · §2 Test Strategy (test-strategy.md: the trigger map's IPC vector (V1) row)
**Change:**
- §6 gains **Windows listener remote-client rejection** (`#[cfg(windows)]`, IPC access control (Windows)): `tests/security_negatives_channel.rs` `security_negative_viola_listener_rejects_remote_clients`, on the windows-2025 CI `test` leg, binds `viola_channel::Server::bind`, opens a client end with `CreateFileW(FILE_READ_ATTRIBUTES)`, reads `NtQueryInformationFile(…, FilePipeLocalInformation)` and asserts the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) set in `NamedPipeType`; its two controls, test-owned `CreateNamedPipeW` pipes read through the same client-end instrument, tell the bit apart: `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (`PIPE_ACCEPT_REMOTE_CLIENTS`) clear, `security_negative_control_reject_remote_pipe_reads_the_bit_set` (`PIPE_REJECT_REMOTE_CLIENTS`) set.
- §6 gains **Windows listener DACL denial**: `security_negative_viola_listener_denies_an_anonymous_open` opens the viola listener `GENERIC_READ` under `ImpersonateAnonymousToken` and asserts `ERROR_ACCESS_DENIED` (5); its control `security_negative_control_default_pipe_admits_an_anonymous_read` makes the same open of a test-owned default-descriptor pipe, and it succeeds.
- The body's Security control negatives line names the Windows listener rejecting remote clients and denying an anonymous open, each beside its controls (was: neither named).
- §5 Module ↔ IPC: after the canonical SDDL read-back, the pipe's admission itself (remote clients rejected, an anonymous open denied) is asserted in §6 Security control negatives (was: the read-back alone).
- §2 the IPC vector (V1) row's plan location adds Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial) (was: §5 Module ↔ IPC; §6 Scenario 1).
**Why:** the chunk landed the five Windows cases and every one passed on CI's `test (windows-2025)`; the controls tell the two outcomes apart, so nothing is owed to "Windows-only live measurements". §6's naming was the plan's expected amendment.
**Kept:** the Module ↔ IPC Tools line (windows-sys 0.61.2) is unchanged: the root's `cfg(windows)` dev-dependency features are architecture's record (Crate dependency direction). The cases' bind-before-impersonate order stays the test code's own, and their `viola-test-chan-<pid>-<label>` pipe names stay architecture's registered namespace.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — the control pipes and the anonymous principal as fake servers
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Fake servers row)
**Change:**
- The tool cell adds windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) (was: the interprocess 2.4.4 squatter listener and `viola-fake-agent` as the fake CLI alone).
- The pattern cell adds: in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; "no HTTP fake servers" stays.
**Why:** the chunk's five Windows cases take test-owned `CreateNamedPipeW` pipes as their controls and an anonymous-token open as the foreign principal; the row named neither.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — the viola-channel untestable zone narrowed to the other-euid peer
**Section:** §1 Test Scope Summary (the body's `viola-channel` entity line; test-scope-summary.md: the `viola-channel` entity, Reason if not fully testable)
**Change:**
- Body: was "the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners"; now the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`, and only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- Detail: was rejecting remote pipe clients and denying access through the Windows pipe DACL among the parts hosted GitHub runners cannot drive; now the Windows listener's admission is asserted on CI `test (windows-2025)` (`tests/security_negatives_channel.rs`): remote clients rejected by the pipe-type read-back beside an accept-remote and a reject-remote control, and the DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`) by an anonymous-token open denied on the viola listener beside the same open admitted on a default-descriptor pipe. The cross-user part that cannot be driven (no second OS account) is rejecting a peer with a different euid, asserted at the peer-credential check boundary.
**Why:** the clause was disproved by measurement: false for remote pipe clients and the DACL on CI's `test (windows-2025)`, each witnessed beside a control telling the outcomes apart; the rest of it (a peer of another euid; a second OS account) stands.
**Kept:** the entity stays partially testable; the security excerpt's note that the Windows DACL cases are only "partially testable with stubs" stands as a quotation of the excerpt, whose stub cases are the home strict-modes ones (§1, the `viola-state` entity: a foreign-owner SID, a volume without persistent ACLs).
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — nextest the only supported runner, three plain-run reds with no owner
**Section:** §2 Test Strategy (test-strategy.md: the Unit level, a new Runner item)
**Change:**
- The Unit level gains Runner: nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
**Why:** the founder's own answer to a P4 fork, live by dialog in the overseer session: plain `cargo test` is unsupported and its known-cause reds are recorded with no owner, so no route entry is owed for them.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — scoped log captures through with_scoped_default
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: a new Scoped log captures item and its two sub-items)
**Change:**
- Every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`): it registers once per process a pinned, always-interested `tracing::Dispatch` over a bare `tracing_subscriber::registry()` (no layer, no filter) in a `static OnceLock`, never the global default, then runs `tracing::subscriber::with_default`; so no scoped capture is ever the only registered dispatcher, and a line whose callsite another, subscriber-less thread registered first is not lost. The binary's one global install is `set_global_default` in `src/run/gate.rs`.
- Its witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first` (`src/main.rs`) runs on every OS under nextest: red on a bare `with_default` (the capture stays empty), green on the helper.
- The three crate-level scoped captures sharing the mechanism, in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`, run on a bare `with_default`; never observed red, they have no owner.
**Why:** the carried capture loss is explained: a callsite first registered on a subscriber-less thread while the scoped capture is the only registered dispatcher loses the line; the witness reproduces it and the helper removes it. The three crate captures are left out with no owner on the founder's own yes to the P5 review card's Beyond-the-plan row, live by dialog in the overseer session.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — viola-e2e's tokio pin in the bootstrap phases
**Section:** §3 Test Harness Contract (the key file Bootstrap phases (derive for route / setup-project): the test-runner-install Dev-deps, the `in viola-e2e` bullet)
**Change:**
- The `viola-e2e` bullet reads tokio `=1.53.3` with default features off and `rt` + `process` (no `test-util`) (was `=1.53.1`).
**Why:** the report's Dependencies fact: the `[workspace.dependencies]` exact pin and the lock moved tokio `=1.53.1` → `=1.53.3`, a version RUSTSEC-2026-0386 lists as patched, `default-features = false` kept; its Crates / modules fact: tokio's per-crate features unmoved (`viola-e2e` `rt` + `process`).
**Kept:** every other pin on the bullet (rmcp `>=3.4.1, <3.5` with `client` only, syn `=2.0.119`, proc-macro2 `=1.0.107`, libc `=0.2.189`) and the bullet's no-`test-util` clause.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the tokio test-util version in the time-mocking discipline
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Time item, the Time mocking row)
**Change:**
- The Time item reads `#[tokio::test(start_paused = true)]` from tokio 1.53.3 `test-util` (was 1.53.1).
- The Time mocking row reads tokio 1.53.3 `test-util` (was 1.53.1).
**Why:** the report's Dependencies fact: the workspace pin and the lock moved tokio 1.53.1 → 1.53.3; Crates / modules and Schema / config changed nothing, so no manifest gained `test-util`.
**Kept:** the row's qualifier "in no manifest yet, as landed at chunk 2026-10-17-mcp-server-for-drivers", and the rest of the item and the row.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the tokio pin in the time-sensitive fixtures row
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: the Time-sensitive data row)
**Change:**
- The row's parenthesis reads "(as landed at chunk 2026-10-17-mcp-server-for-drivers; tokio `=1.53.3` with default features off, …)" (was `=1.53.1`, joined by a colon); no history clause is written into the body.
**Why:** the report's Dependencies fact: the workspace exact pin and the lock moved tokio `=1.53.1` → `=1.53.3`; its Crates / modules fact: tokio's per-crate features unmoved.
**Kept:** the features clause (`viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`), the no-manifest-names-`test-util` fact, and `viola-mcp` owning no timer (`wait_timeout(given)` = `min(given, 50 000)`, a unit case), so no `start_paused` test exists.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-server-verification-before-any-frame — the macOS-target check-only clippy, a dev-host recipe
**Section:** §9 CI Integration (test-plan.md: Pipeline structure, the Lint line; ci-integration.md: the Lint stage)
**Change:**
- The Lint stage gains, beside the Windows-target recipe, a check-only macOS-target clippy on the Linux dev host: `CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`; it reproduced CI's `lint (macos-latest)` error line for line, as measured at the chunk's operator pass; neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
- The master's Lint line: the `--acyclic` review and the dev-host Windows- and macOS-target check-only clippy runs are not gates (was "a dev-host Windows-target clippy").
**Why:** the chunk's pre-CI run was red on `lint (macos-latest)` alone: an unused `super::*` import in the `last.rs` and `pause.rs` test modules once their cases were gated off macOS, which no local lint could see; the macOS-target check reproduced the line exactly, and the fix gated the two modules whole.
**Kept:** the Windows-target recipe's own clause that a chunk plan may list it as a gate entry; the macOS recipe takes no such clause.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — E5 steps 1–3 and the three other snapshot readers, held on "Home and code-bearing file integrity"
**Section:** §6 E2E Test Strategy (test-plan.md: Scenario: E5, As landed; e2e-test-strategy.md: the E5 procedure, the `list --json` leg) · §4 Unit Test Strategy (unit-test-strategy.md: viola-state, the Snapshot envelope item)
**Change:**
- The other snapshot readers are three (`run`'s collision check, `src/cmd/hook.rs` once, its event and dialog paths sharing the one read, and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules), each still on `read_snapshot`; was five (`src/cmd/client.rs`, the collision check, `src/cmd/hook.rs` twice, `statusline.rs`), per "2026-10-11-the-board-viola-list — the board's readings landed: E5, Path 6, Chaos; the literal pin and the kill line". The CLI frame verbs and the MCP frame tools read the snapshot through `viola_state::liveness::live_endpoint`, whose one `read_snapshot` call feeds the server check.
- E5 steps 1–3 stay unbuilt while the collision check calls `read_snapshot`: every snapshot reader stays on `read_snapshot` (option 1), and their move onto `read_snapshot_or_replay`, with what a client does with an unreadable or newer snapshot, is one CARRY on the route entry "Home and code-bearing file integrity"; was owed to "Server verification before any frame" (the e2e line's "the review card the founder approved" dropped).
**Why:** measured at this tree: `src/cmd/client.rs` holds no `read_snapshot(` call (its read moved into `live_endpoint` at chunk 2026-10-17-mcp-server-for-drivers), and this chunk joined the hook's two reads in one `connect`; the entry closes without moving any reader. Option 1 of fork CARRY 7/8 is the founder's own ruling, given by dialog at his wake on 2026-10-26 and relayed by the overseer; it supersedes the overseer's provisional delegate answer and the wrap directive's hold, so it is recorded as his word, not PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the hook fail-open matrices' impostor rows
**Section:** §6 E2E Test Strategy (test-plan.md: Non-path suites, Security sweep; e2e-test-strategy.md: the Security sweep procedure)
**Change:**
- The hook fail-open matrix is named by its functions, `hook_fails_open_silently_within_the_spine_bound` and `hook_dialog_tier_fails_open_silently` (was "the 11-case matrix").
- Its causes gain, on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`), each exit 0 with no body, at both sites.
- The impostor rows, each under `cfg_attr(not(target_os = "macos"), …)`: `case::impostor` in the first, `question_impostor` and `permission_impostor` in the second, driving `stop`, `pre-tool-use` and `permission-request` against a same-user server on the recorded endpoint while the snapshot records another live process (the fake agent's `--hold-stdout-internal` holder): each exits 0 with empty streams under the 1.0 s `Instant` bound, no frame reaches the server, and one `hook-decision` line carries `detail:"server-verify-failed"` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`).
- Not a refusal case: a hook reaching an exiting wrapper's own Windows pipe passes the server check by design, the exiting wrapper being the recorded process; whether sysinfo still lists a terminated Windows process whose pipe handle is open, and SessionEnd's outcome in that window, are one CARRY on the route entry "Windows-only live measurements".
**Why:** the chunk added the three impostor rows to the two existing matrices; a master states no test-case count, so the count gives way to the function names. The exiting-wrapper window's re-pin is a Beyond-the-plan row of the P5 review card, on the overseer's founder-delegated word.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the exit-cause matrix as landed: no `cross_exit_causes.rs`, the server-verification cause
**Section:** §6 E2E Test Strategy (test-plan.md: Non-path suites, Exit-cause matrix; e2e-test-strategy.md: the Exit-cause matrix procedure)
**Change:**
- `cross_exit_causes.rs` is marked not built in the master and recorded as absent, with no rstest table joining the causes: each built cause is a case of the binary its row names.
- The server-verification exit-21 cause, on Linux and Windows, is the six-verb server-identity negative and its control in `tests/security_negatives_server_identity.rs`, not a `cross_exit_causes.rs` row.
- macOS: the client reads no peer pid and compares no server (the recorded gap); the client euid check and the `#[cfg(target_os = "macos")]` case loosening the per-user socket dir to 0755 → exit 21 are not built, one CARRY on the route entry "Unix endpoint and home hardening" (was the client checking euid and the socket dir, the 0755 case written as asserted).
- The cause's hint is its own, cause `server-verify` in `src/human/hints.rs` (the design-system surface cli-send string, byte for byte), apart from a dead instance's not-running hint; its line is `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`; its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.
**Why:** the report disproved the case's home: it landed in the new security-negative binary, six verbs in two modes, and `cross_exit_causes.rs` does not exist. The macOS re-pin is a Beyond-the-plan row of the P5 review card, on the overseer's founder-delegated word; its clause, read at first as contradicting the line it rewrites, was applied with its companion on the overseer's founder-delegated answer at the wrap's halt.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-identity security negative and the `security_negatives` suite prefix
**Section:** §6 E2E Test Strategy (test-plan.md: Non-path suites, Security control negatives; e2e-test-strategy.md: the Security control negatives procedure) · §2 Test Strategy (test-strategy.md: the naming pattern, the sync E2E suites)
**Change:**
- Security control negatives gain, on Linux and Windows, a CLI frame verb writing no frame to a server that is not the recorded wrapper (`tests/security_negatives_server_identity.rs`, beside its control reaching the recorded wrapper).
- Its procedure, **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification): `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` runs `send`, `wait`, `last`, `answer`, `pause` and `release`, each under `--json` and in human mode, against a same-user server bound on the recorded endpoint while `snapshot.json` records another live process (the fake agent's `--hold-stdout-internal` holder): exit 21 with the cause's `--json` document (stderr empty), hint and `process-exit` line, no `channel-request` line, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`). Its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each verb in both modes against the recorded wrapper itself, which receives one frame each.
- The root sync E2E suites read `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs` (was without `security_negatives`).
**Why:** the chunk's new test binary, green on the host and on CI's ubuntu-latest and windows-2025 legs; it retires on Linux and Windows the server-identity half of the dated liveness-only gaps for the six verbs.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the Wrapper channel's server-mismatch cases and the MCP impostor case
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the Wrapper channel cases)
**Change:**
- A server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21 and the hook exits 0 with no body, on Linux and Windows: the CLI half in `tests/security_negatives_server_identity.rs`, the hook half the impostor rows of `tests/hook_fail_open.rs` (§6 Security sweep). macOS reports no peer pid, so there the check passes with no comparison, the recorded gap; its euid and directory check is owed to the route entry "Unix endpoint and home hardening". Was "macOS verifies euid + dir only", and the MCP half "the same impostor case in `cross_exit_causes.rs`, the crate with the rmcp client", `call_tool("send")`.
- The MCP half: `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (`crates/viola-mcp/tests/handlers.rs`, a held process as the recorded pid) drives the transport-free `Tools` handle, no rmcp client: MCP `send`, `wait`, `last` and `answer` against a server that is not the recorded wrapper each get `isError` with `{"error":"instance-unreachable","detail":null}` and text `refused: instance-unreachable`; no frame reaches the server and no log line is written.
**Why:** the report disproved the MCP case's home: it landed in the crate's handler tests over the `Tools` handle, and `cross_exit_causes.rs` does not exist; the macOS client reads no peer pid, and its euid check belongs to that route entry.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-identity impostor: its held process, its listener, its test-built snapshot
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Process mocking and Fake servers rows) · §7 Test Data & Fixtures (test-plan.md: Seed strategies, On-disk product state; test-data-fixtures.md: the On-disk product state row, a new data item, the fake agent's modes)
**Change:**
- Process mocking: a held live process whose pid an impostor's snapshot records is a real process, never a double: in root tests the fake agent's `--hold-stdout-internal` holder, in `crates/viola-mcp/tests/handlers.rs` its own `Held` child, entered at `handlers_held_child_entry` only when its `HELD` variable is set, each killed and reaped by a `Drop` guard.
- Fake servers gain the impostor listener: in the server-verification negatives (Linux and Windows) a server on the recorded endpoint while the snapshot records another held live process stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener in `tests/security_negatives_server_identity.rs` and `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is the handlers impostor case); the hook unit impostor varies the recorded start time instead (a second early, or an unparseable `started_at`), a root-bin unit test having no portable second process.
- On-disk product state: three exceptions to "never hand-written" (was two), the impostor cases added: `served` and `Impostor::serve` write an `InstanceSnapshot` themselves, recording a held live process's pid beside an endpoint the test binds. The master's seed line names the impostors' test-built `snapshot.json`.
- The `--exit-no-eof` holder's re-exec mode `--hold-stdout-internal` (sleeps 10 s, writes nothing) is also spawned directly by the two root impostor files, killed and reaped by each file's `Held` `Drop` guard.
**Why:** an impostor needs a snapshot recording a live pid that is not the server's, which no product run writes; the chunk's held process is the holder mode, not a bare fake-agent spawn, and its hook unit impostor varies the start time.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the hook perf rows with the server check in the timed path
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the Perf session)
**Change:**
- Every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host, as measured at the chunk's `run --perf`.
**Why:** the chunk put the check ahead of every hook frame; `run --perf` stayed green, every hook row far under the 1 s `SPINE_DEADLINE` with zero panics, against 1.23–1.6 ms read before the check landed.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — fake-agent copies planted one after another (ETXTBSY)
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: a new data item)
**Change:**
- A test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child: on the tmpfs `target/e2e-home` the hard link fails, so each copy is written, and a child one thread forks inherits another thread's open write handle (ETXTBSY). In `fake_agent_agents_replay_matches_every_recorded_agents_set` (`tests/contract_fake_agent_drift.rs`) it reads 0 red of 40 standalone runs, as measured at the chunk's ETXTBSY evidence.
**Why:** the chunk's pre-push red was ETXTBSY in that test, 6 of 40 standalone runs red before the sequential planting; the test-only fix was folded into this chunk by the standing rule that a red met during a chunk folds into it, on the overseer's founder-delegated word, recorded in-intent.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — where the server verification is asserted: the viola-channel entity and the V1 row
**Section:** §1 Test Scope Summary (test-scope-summary.md: the `viola-channel` entity, a new As landed item) · §2 Test Strategy (test-strategy.md: the trigger table, the IPC vector (V1) row)
**Change:**
- As landed (server verification): the pid/start-time server verification is asserted on Linux and Windows — the six CLI frame verbs by `tests/security_negatives_server_identity.rs` (an impostor gets no frame and the verb exits 21, beside a control reaching the recorded wrapper), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed (§6 Exit-cause matrix).
- The V1 row's plan location gains §5 Wrapper channel (the MCP impostor case), root `security_negatives_server_identity.rs` (off macOS) under Security control negatives, and the hook fail-open impostor rows (off macOS) under Security sweep.
**Why:** the chunk built the verification's negatives on all three frame-writing surfaces, each compiled out on macOS, where there is no peer pid to compare.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server check's captured-span cases compiled out on macOS
**Section:** §2 Test Strategy (test-plan.md: Test pyramid, Unit; test-strategy.md: the Unit level, Tier-specific depth)
**Change:**
- Unit runs on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS (was the mutants tests alone), where the server check reads no peer pid: each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the test-only `cmd::client::refused_inside` helper they share (under `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`) and the `last.rs` and `pause.rs` test modules, gated whole; the decision table `server_verdict_rows` and the macOS `server_pid` case still run there.
**Why:** macOS reports no peer pid, so the check passes there with no comparison and these cases have nothing to refuse. The helper's gate is two attributes because a first `#[cfg(all(test, not(target_os = "macos")))]` gate put its `cfg!(windows)` ahead of the file's first `#[cfg(test)]` and the Windows mutation-scope contract read the file as Windows-gated product code; the two modules are gated whole because a per-case gate left an import unused on macOS.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server pid and the pure server verdict, unit cases
**Section:** §4 Unit Test Strategy (test-plan.md: the viola-channel and viola-state lines; unit-test-strategy.md: viola-channel, viola-state)
**Change:**
- viola-channel: the connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), asserted on each OS by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature); `SERVER_PID_REPORTED` is false on macOS.
- viola-state: `ServerVerdict { Matches, Mismatch, Unverifiable }` from the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`, pinned by the decision table `server_verdict_rows`; with no peer pid reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap. `verify_server_reads_the_observed_start_time_from_the_os` covers the joined `verify_server`'s OS read (`process_start_time`, none for a zombie). `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.
- The master's lines gain the connected stream's server pid on each OS (viola-channel) and the pure server verdict's decision table (viola-state).
**Why:** the chunk's new channel and state surface; the decision table is named without its row count, since a master states no test-case count.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the root bin's server-check unit cases
**Section:** §4 Unit Test Strategy (test-plan.md: the root bin line; unit-test-strategy.md: root bin, two new items)
**Change:**
- The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`, the server cause `during` `connect`).
- The hook's server check: `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the dialog path writes no frame and emits no decision), and a SessionEnd whose check fails takes the direct append (`append_session_end`), held by a mismatch row of the hook's unit cases; the unit impostor varies the recorded start time (a second early; an unparseable `started_at`) rather than recording another live pid, the integration rows' shape.
- The master's root bin line gains the server check refused inside each frame verb's and the hook's span, with each closed unreachable cause logged.
**Why:** the chunk's root-bin surface; the seven captured-span cases and their helper were added beyond the plan on the operator's check, each red with the check removed, and a root-bin unit test has no portable second process to record.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the fake agent's exit ignores SIGHUP
**Section:** §7 Test Data & Fixtures (test-plan.md: the fake agent's exit)
**Change:** The fake agent's exit gains its own SIGHUP disposition beside its commands' process groups: the agent, its PTY's session leader, cannot leave that group (`setpgid` refuses a leader), so on Unix its exit ignores SIGHUP from its first step (`\x03` or stdin's end); a hangup that lands while it exits (a killed wrapper, or portable-pty's SIGHUP-first kill) no longer cuts its own exit-time coverage write, and one that lands before the exit begins still ends it; as measured on the Linux dev host. Was: the exit stated only its commands' own process groups.
**Why:** a folded test-only fix of the coverage merge's red (one cut-short fake-agent profile at this wrap's light gate), whose mechanism the hooks' own process groups did not reach; the contract states the exit as it now behaves, on the overseer's founder-delegated answer, delegate the overseer, 2026-10-26.
**Kept:** the hooks' own process groups and the `--exit-no-eof` holder's per-OS output reading (test-plan-detail/e2e-test-strategy.md) stand as they were: the holder still runs in its own group.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-explicit-session-links — `channel_links` on the 20 s kill line
**Section:** §3 → Bootstrap phases (derive for route / setup-project) (bootstrap-phases-derive-for-route-setup-project.md: test-runner-install, the `[profile.ci]` kill line)
**Change:**
- The `ci` profile's 20 s kill line holds twenty-two verify-driven binaries (the rule: the `binary(…)` names on that filter line; was twenty-one, per 2026-10-17-mcp-server-for-drivers — harness keys and the time mechanism): `channel_links` (`tests/channel_links.rs`) joined, each of its cases booting a wrapper (`booted_wrapper`).
- The list's reading is no longer stamped "at chunk 2026-10-17-mcp-server-for-drivers"; that chunk now dates `mcp_tools`' joining.
**Why:** the chunk added `binary(channel_links)` to the kill line's filter. The report's claim that no master lists the kill line's binaries was false: this key file enumerates them, so its count moves by its rule.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' result lines and outputs pinned
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the surface table's cli row; the Exit-cause matrix, exit 21 no instance running) · §5 Integration Test Strategy (integration-test-strategy.md: CLI) · §7 Test Data & Fixtures (test-data-fixtures.md: Human-mode expected output) · §4 Unit Test Strategy (unit-test-strategy.md: root bin, the link verbs)
**Change:**
- §6 cli row and §7: the three `viola link` / `viola unlink` result lines join `viola --help` and the `viola list` board as human layouts pinned by literal in their test files: `tests/cli_links.rs` `link_verbs_print_the_design_lines` (the printed time checked by shape), and the units `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`).
- §5 CLI: `help_lists_only_served_verbs` reads no `hook`, `help` or bare `plugin` in the table (was "no `hook`, `help`, `link`, `unlink` or bare `plugin`", per 2026-10-17-cli-output-discipline — viola --help pinned by literal; cli_output_plain's new cases; the colour variables in Vector 6). New: `tests/cli_links.rs` pins `link_verbs_print_the_design_lines` (the three lines byte for byte, `{"v":1,"ok":…}` under `--json`), `link_verbs_against_a_stopped_instance_exit_21` and `link_verbs_log_the_channel_pair_and_no_names` (the wrapper's `channel-request` / `channel-response` pair carrying `method` `link` / `unlink`, and no driver or driven name in its process log).
- §6 exit 21, no instance running: for the two verbs a `tests/cli_links.rs` case, `link_verbs_against_a_stopped_instance_exit_21` (both verbs in both modes: the not-running hint last, or the `instance-unreachable` document with stderr empty), not a `cross_exit_causes.rs` row.
- §4: `link_reply_is_read_by_its_verbs_shape` and `link_line_is_built_from_the_names_and_the_reply` (`src/cmd/link.rs`) beside the two `verbs.rs` cases; the line's time is the reply's `since` as UTC clock time, and a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through `escape_message`.
**Why:** the chunk built both verbs with three plain result lines and their help row; the report reads each of these outcomes met by the cases named.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — E3 built: the explicit verbs' hold retired
**Section:** §6 E2E Test Strategy (test-plan.md: Scenario E3, As landed; e2e-test-strategy.md: the Scenario E3 and Scenario E5 procedures)
**Change:**
- E3 As landed: was "owed, built in part" — steps 1 and 3 could not run, `viola link` / `viola unlink` not built (the wrapper answering both methods `-32601`) and held, their widening not ratified, no explicit verb before Epoch 6, the hold on the route entry "Explicit session links"; now "built": the verbs and the wrapper's `link` / `unlink` methods are served. The driver `overseer` is a self-reported `VIOLA_NAME` and is not booted, so step 1 boots only `builder`.
- E3 itself: `e3_two_links_and_a_send_leave_one_link_record` (`tests/cli_links.rs`, from `stamped_home`): two `link` calls leave one `link` record and one `since`, the second answering `already_linked: true`; an issued `send` from the self-reported driver adds no `link` record; `unlink` appends one `unlink` record (`source: wrapper`, `{driver, driven}`) and empties the snapshot's `links`, a second `unlink` appending nothing; a later `link` is a new pair with a later `since`.
- `links_an_unlink_survives_a_wrapper_restart`: a wrapper started over a log whose pair the methods linked and then unlinked restores an empty set (its first snapshot's `links` empty, no line appended).
- Was "no case reads an `unlink` record a product path wrote, because none writes one" (per 2026-10-10-session-links — Path 6's four verbs, E3 and E5 as landed); now a product path writes it: the `unlink` method, `LinkSet::remove`'s one product caller. Step 2's four root cases stand.
- E5's link leg: was "no `viola link` verb exists; the explicit verbs are not built"; now the served verb can drive step 1's `link`, and steps 1–3 stay unbuilt for the `read_snapshot` reason.
**Why:** the chunk served both methods and built both verbs in the form the founder's own answer to its P4 fork named (inputs#I3, live by dialog, relayed verbatim by the operator), ratifying the widening the hold waited on; the report retires the hold and the `-32601` answer. The two E3 procedure items, read at first as contradicting the line they sit under, were applied with their companion on the overseer's founder-delegated answer at the wrap's halt (inputs#I6, PROVISIONAL).
**Kept:** E3's body steps (boot `overseer` and `builder`) stand as the spec; the deviation is stated in the As landed lines.
**Supersedes:** 2026-10-13-t06-38-35-wrap — E3's explicit steps held on the Epoch 6 entry
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' instance check before any log
**Section:** §1 Test Scope Summary (test-scope-summary.md: the CLI verbs entity) · §4 Unit Test Strategy (unit-test-strategy.md: the instance check) · §5 Integration Test Strategy (integration-test-strategy.md: On-disk) · §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix, exit 21 the home fails strict-modes)
**Change:**
- §1 As landed (`link` / `unlink`): of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens; a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).
- §5 On-disk: their refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found.
- §6: a `tests/cli_links.rs` case, not a `cross_exit_causes.rs` row: `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`) makes the driven instance's directory group-writable while its wrapper runs; both verbs in both modes exit 21, the verb's `cli` log gains no line, the wrapper gets no frame, and the mode reads back as found. Remove-the-guard control A, the check moved, read red 1 of 1.
- §4: the instance check's root refusal cases gain that case beside the three in `tests/cli_revive.rs`.
**Why:** the founder's own answer to the chunk's P4 fork (inputs#I3, live by dialog, relayed verbatim by the operator) put the instance check first, its refusal exit 21. The order trap the case holds: opening a role file narrows an existing home to 0700, so a check placed after it passes a 0770 home. The wrapper-only self-pair refusal is the operator's answer of chunk 2026-10-10-session-links.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — eight CLI frame verbs under the server check
**Section:** §1 Test Scope Summary (test-scope-summary.md: As landed (server verification)) · §2 Test Strategy (test-strategy.md: Unit, Tier-specific depth) · §4 Unit Test Strategy (unit-test-strategy.md: the server check before any frame) · §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's server-verification cause; Security control negatives, Server identity)
**Change:**
- The CLI frame verbs read eight, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` (was six, per the 2026-10-26-server-verification-before-any-frame entries on the exit-cause matrix, the server-identity security negative and where the server verification is asserted): §1 names them as the verbs `tests/security_negatives_server_identity.rs` asserts; §6's server-verification exit-21 cause is the eight-verb negative, which runs the eight under `--json` and in human mode, and its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the eight in both modes.
- §4: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` (was the six, per 2026-10-26-server-verification-before-any-frame — the root bin's server-check unit cases).
- §2: the `last.rs`, `pause.rs` and `unlink.rs` test modules are gated whole off macOS (was `last.rs` and `pause.rs`, per 2026-10-26-server-verification-before-any-frame — the server check's captured-span cases compiled out on macOS); a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS.
**Why:** the two verbs take the liveness pre-check and the server check through `cmd::client::connect` like the other six, and the identity binary grew to eight verbs in two modes, green on CI's Linux and Windows legs. The module-gate rule is the chunk's implement deviation: a whole-module gate fits only a module whose every case is gated.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the served `link` / `unlink` methods and the raw-channel name negative
**Section:** §1 Test Scope Summary (test-scope-summary.md: the ipc-internal wrapper channel surface) · §2 Test Strategy (test-strategy.md: the trigger table, IPC vector (V1)) · §4 Unit Test Strategy (unit-test-strategy.md: root bin, the served link methods) · §6 E2E Test Strategy (test-plan.md: Non-path suites, Security control negatives; e2e-test-strategy.md: Security control negatives, Raw-channel `link` / `unlink` name validation)
**Change:**
- §1 As landed: both are served by the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written; a failed `record` / `remove` is `-32603`; `link` answers `ok` `{since, already_linked}` and `unlink` an empty `ok` object.
- §6 the raw-channel negative, as landed: `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`, `binary(channel_links)` on the 20 s kill line) writes its frames through `viola_channel::Client::request`, past every CLI check and the server check. For `link` and `unlink` each, a `driver` or `driven` of `../x`, `BUILDER`, `a/b`, an empty string, a 33-character name or a number, a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven` each get `-32602` with `data: null`, no `link` / `unlink` record and the snapshot's `links` empty; its control, a valid `link`, answers `already_linked: false` and appends one `link` record. The master's suite list names the file.
- §2 the V1 row gains root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written.
- §4: `links_set_method_link_twice_writes_one_line_and_answers_the_same_since`, `links_set_method_unlink_then_link_takes_a_later_since`, `links_set_method_unlink_of_an_absent_pair_writes_nothing`, `links_set_method_refuses_a_bad_pair_with_invalid_params`, `links_set_method_a_from_changes_nothing`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged` (`src/run/links.rs`) and `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs`).
**Why:** the chunk served both methods (before, both answered `-32601`), neither reading `from`; the raw-channel negative is its new binary, green with the served methods' units.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — an unserved method's response line carries no `method` key
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Wrapper channel)
**Change:**
- New: an unserved method's conn-less response line carries no `method` key, because the channel's `method_label` names only the closed method list: `channel_wrapper_logs_each_call_with_corr_and_conn` (`tests/channel_endpoint.rs`) sends the unserved `anything` and asserts the key absent; a test asserting a `method` value on such a line for a name outside that list is wrong by construction.
**Why:** measured this chunk, disproving the plan's assumption that such a line logs its `method`: the test's unserved example was `unlink`, now served, and its assertion became the key's absence.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link line's `since` outside the clock seam
**Section:** §8 Mocking & Stubbing Discipline (test-plan.md: Time; mocking-stubbing-discipline.md: Time) · §7 Test Data & Fixtures (test-plan.md: Time-sensitive data; test-data-fixtures.md: Time-sensitive data)
**Change:**
- §8: `chrono::Utc::now()` is read only through the clock seam with two exceptions as landed (was one, the budget gate's three evaluation points): `LinkSet::line` stamps the link line's `since` with it directly, so root cases assert `since` by shape only, through `is_clock_time` in `tests/cli_links.rs`, itself held by `is_clock_time_reads_the_shape_only`. The master's Time line names `LinkSet::line` beside the budget gate.
- §7: time-sensitive data is injected behind `viola_core::Clock` except a link line's `since`, which the wrapper stamps from the wall clock and root cases assert by its shape only (§8); the detail's row states the same.
**Why:** the report's Decisions: `LinkSet::line` stamps `since` with `Utc::now()` directly, outside the clock seam. The detail's clause, read at first as contradicting the run it rewrites (one exception to two), was applied on the overseer's founder-delegated answer at the wrap's halt (inputs#I6, PROVISIONAL).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the seeded home made private; the seed's hard-link grant hazard
**Section:** §3 → Test data bootstrap (test-data-bootstrap.md: Mechanism, the Windows x64 seed) · §5 Integration Test Strategy (test-plan.md and integration-test-strategy.md: Setup / teardown lifecycle, the seed carve-out) · §7 Test Data & Fixtures (test-data-fixtures.md: On-disk product state; a new row, A widened seeded home (Windows x64))
**Change:**
- `seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` (owner-only modes and DACL) before `bin/<key>/conpty/`, so a seeded home passes `check_instance` and, on windows-2025, the three `hook_statusline` cases pass. Was "a home that seed made is not one viola created … `hook statusline`'s instance check refuses it", per "2026-10-10-statusline-pass-through — the fake agent's statusline options, the reader's property and seeds, the stamped-home rule on Windows, three counts", and "a seeded home skips [viola's own home DACL], so that entry owns making the seed let viola create the home first (route CARRY)", per "2026-09-29-sideloaded-conpty — the Windows x64 seeded-home carve-out"; that CARRY is gone from the body.
- The companions are hard links to one shared seed file: an inheritable `icacls` grant (`(OI)(CI)`) on a seeded home or its `bin/` reaches the shared `target/conpty-seed/<key>/` companion and widens every seeded home until removed, so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded (`run_refuses_a_widened_home_tree_before_its_log`), as measured on windows-2025. The new §7 row holds it, per-test; the §5 master carve-out reads "into a home `seed_conpty` makes private" with the unseeded rule.
**Why:** the chunk's `seed_conpty` makes the home and the seed dir with `create_private_dir`, and the seeded homes passed on the green Windows leg; the two red Windows runs measured the grant reaching every seeded home through the hard links, and the fix commit started that test's home unseeded.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the start's and verify's home-tree checks before any log
**Section:** §1 Test Scope Summary (test-plan.md: Critical paths, Path 1; test-scope-summary.md: Path `run` start sequence and its signals) · §6 E2E Test Strategy (e2e-test-strategy.md: Path 1; the Exit-cause matrix, exit 1 for `run` and for `revive`; test-plan.md: Exit-cause matrix) · §5 Integration Test Strategy (integration-test-strategy.md: On-disk, a new item)
**Change:**
- Path 1: the start runs `check_home_tree` before the wrapper log opens: the home (read rule), `check_instance`'s set, `ledger/`, `bin/`, `plugin/`, then `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists. A refused tree exits 1 with the home pair, or the instance pair when the refusal is at instance level, opens no log, writes no line and starts no child. Was a sequence opening at the collision check, its exit-1 list ending at a tampered pinned exe; now a refused home tree joins it.
- `tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`: after one clean start, a 0770 home or a widened `instances/<name>`, `bin/` or pinned copy refuses the next `viola run` before its log, the widening as found. `tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step`: a widened `ledger/` on a stamped home refuses `viola verify` with the home pair, no step line or summary, `stamps.json` byte for byte.
- Exit 1, `run`: "the `--home` fails strict-modes at `run`" now names the home-tree check, a read-granted pre-existing home included on every OS, and beside it `viola verify` over a refused home tree (the home, `ledger/`, `bin/`, `plugin/`); the master's matrix names `tests/run_cli.rs` and `tests/cli_verify.rs` for these exit-1 refusals.
- Exit 1, `revive`: its strict-modes cause also covers the home, the instance directory or its `diagnostics/` read by another user (`cfg(unix)`), and the start arm runs `run`'s home-tree check before the wrapper log (under `--json` the `strict-modes-failed` document).
**Why:** the chunk's `run` and `revive`'s start check the tree before `open_wrapper_log`, and `verify` checks the home, `ledger/`, `bin/` and `plugin/` before its log; the report reads the criterion met on the entry's binaries and the unit tier.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the four readers on the replay, their unit cases and signals
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-state, the Snapshot envelope and server check items, a new replayed-readers item; test-plan.md: viola-state, root bin) · §6 E2E Test Strategy (e2e-test-strategy.md: Path 6, step 6) · §1 Test Scope Summary (test-scope-summary.md: Path `run` start sequence, the collision check; Path budget governor, the statusline signal)
**Change:**
- Was "the three other snapshot readers (`run`'s collision check, `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`) still call `read_snapshot`" and `live_endpoint` "all from its one `read_snapshot` call", per "2026-10-26-server-verification-before-any-frame — E5 steps 1–3 and the three other snapshot readers, held on "Home and code-bearing file integrity""; now the four call `read_snapshot_or_replay`: the collision check takes the caller's one read and over a replayed snapshot reads liveness from the beat alone; hook `connect` (the event and dialog paths sharing the read) reads a replayed or absent snapshot `channel-unreachable`, fail open; statusline `serve` finds no `statusline_command`; `live_endpoint` proceeds only on a present snapshot, a replayed or absent one or a read error `None`. `revive`'s preflight reads once for its collision check and its recorded-directory reading. No product reader is left on `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules).
- New unit item: `live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line`, `connect_over_a_replayed_snapshot_is_unreachable`, `statusline_over_a_replayed_snapshot_runs_nothing`, `collision_check_over_a_replayed_snapshot_reads_the_beat_alone`; the §4 master's root bin line gains the readers over a replayed snapshot, its viola-state line `live_endpoint` over one.
- Path 6 step 6 and the statusline signal: `statusline_command` also never runs over a replayed snapshot (unreadable or of an unsupported `v`); the arm runs and prints nothing.
**Why:** the founder's own answer to the chunk's P4 fork Q1 (inputs#I3, live by dialog in the overseer session, relayed by the overseer) moved the readers now, his boundary-widening ratification; the wrap's halt confirmed these items on the overseer's founder-delegated answer (inputs#I6, PROVISIONAL), the ratification staying his inputs#I3.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — E5 built: steps 1–3 and the six replay readers
**Section:** §6 E2E Test Strategy (test-plan.md: Scenario E5, Verification signal and As landed; Chaos suite; e2e-test-strategy.md: the E5 procedure; Chaos suite) · §1 Test Scope Summary (test-scope-summary.md: the chaos trigger, snapshot corruption) · §2 Test Strategy (test-strategy.md: the trigger map's Filesystem (V7) row, E5)
**Change:**
- As landed: was "in part", steps 1–3 unbuilt while the collision check called `read_snapshot`, the readers' move a CARRY owed to the route entry "Home and code-bearing file integrity", per "2026-10-26-server-verification-before-any-frame — E5 steps 1–3 and the three other snapshot readers, held on "Home and code-bearing file integrity""; now "built": steps 1–3 run as `tests/chaos_snapshot_replay.rs` (`a_start_over_an_unreadable_snapshot_replays_the_log`, `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`). The replay's product readers were two and are six (`viola revive`, `viola list`'s `board::read`, hook `connect`, statusline `serve`, `run`'s collision check, `liveness::live_endpoint`), each after its strict-modes check; `read_snapshot` has no product caller under `src/`; the CARRY is gone.
- Signals: was "the rebuilt snapshot has `links`, `wheel:"human"`, `budget_paused:true` and `agent_session_id`"; now the replay reads `links`, `budget_paused:true` and `agent_session_id` with `dialog_pending:false` and writes no file, and every start appends `wheel{holder:"driver", cause:"start"}`, so after step 3 the wheel is the driver. "No refusal of start" holds once the heartbeat is older than `STALE_AFTER`; a fresh beat over a replayed snapshot refuses the start `already-live` (exit 1). The master's set-against note ("two of the signals above are set against what landed, for the entry that builds it to restate") is removed; the procedure's two signals read as built.
- Chaos suite: a start over a truncated or `v:99` snapshot replays the log, and a fresh beat over it refuses `already-live` (`chaos_snapshot_replay.rs`); the snapshot-corruption item and the V7 row name the file for steps 1–3.
**Why:** the readers moved on the founder's own answer to P4 fork Q1 (inputs#I3, live by dialog, relayed by the overseer), his boundary-widening ratification, confirmed at the wrap's halt on the overseer's founder-delegated answer (inputs#I6, PROVISIONAL); the chaos item, read as contradicting the procedure line it sits beside, was applied with its companion (the halt's Q2, inputs#I6).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the hook's instance check on its event and dialog paths
**Section:** §6 E2E Test Strategy (test-plan.md and e2e-test-strategy.md: Security sweep, the hook fail-open matrix) · §5 Integration Test Strategy (test-plan.md: Hooks, a new bullet; integration-test-strategy.md: Hook exec ↔ channel ↔ disk)
**Change:**
- The event and dialog paths run the instance check before any `config.json` read or obs init: an instance it refuses, or a `VIOLA_DIR` naming no existing instance directory (which canonicalises to no session), means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append. The two `--capture` arms stay ahead of it, unchanged.
- The fail-open matrix was oversize stdin, malformed JSON, clap error, channel failure, forced panic, a missing `VIOLA_NAME` and the impostor server; it gains the `no_instance_dir` row and `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`, one rstest function over `stop`, `session-end`, `pre-tool-use` and `permission-request`, not a row of each matrix. The `tests/hook_fail_open.rs` cases that expect lines run in a viola-made instance dir (`Env::Made`).
**Why:** the chunk moved `canonical` / `trusted` into `src/cmd/hook.rs`, shared by both arms and taken before obs init; the report reads the criterion met, `hook_fail_open` and `hook_statusline` green.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the six CLI frame verbs and the MCP frame tools under the instance check
**Section:** §1 Test Scope Summary (test-scope-summary.md: the CLI verbs entity; the MCP surface, a new As landed) · §6 E2E Test Strategy (e2e-test-strategy.md: Exit-cause matrix, exit 21; Security control negatives, a new item; test-plan.md: Exit-cause matrix, Security control negatives) · §5 Integration Test Strategy (integration-test-strategy.md: Wrapper channel, a new MCP case; On-disk, the link-verb item; CLI, `cli_controls_not_disableable`)
**Change:**
- Was "of the eight CLI frame verbs only `link` / `unlink` run `check_instance`", per "2026-10-26-explicit-session-links — the link verbs' instance check before any log"; now all eight run it on the named instance through `src/cmd/client.rs` `refuse_untrusted` before any role file or log opens: exit 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and no line.
- `send`, `wait`, `last`, `answer`, `pause` and `release`: `tests/security_negatives_strict_modes.rs` `frame_verb_refuses_a_widened_instance_before_any_log` (three CI OSes, both modes, stderr empty, no frame, no line, the mode or ACL as found), the Exit-cause matrix's exit-21 cause for the six (not `cross_exit_causes.rs` rows). The exit-21 home row reads Unix a widened mode, Windows a Users grant through `icacls` on windows-2025; was a 0770 home and a synthetic-descriptor stub.
- MCP `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check: `isError` with `{"error":"instance-unreachable","detail":null}`, no frame, no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, over stdio after an `initialize` handshake).
- `SETTINGS`' `--home` row is the case `setting_does_not_disable_the_home_check` (`cfg(unix)`: under each row `viola send` against a 0770 home exits 21 with the instance pair before any log).
**Why:** the founder's own answers at the chunk's P4 (inputs#I3, live by dialog, relayed by the overseer, as the report records them): the six verbs print `link`'s instance pair, and a refused check writes no line, as `link` / `unlink`. The new negatives item, read as contradicting the owed clause beside it, was applied with its companion on the overseer's founder-delegated answer at the halt (Q2, inputs#I6, PROVISIONAL).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the security control negatives: Windows `--home` DACL, read-granted home, single writers, the ConPTY companion
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Security control negatives; test-plan.md: Security control negatives) · §5 Integration Test Strategy (test-plan.md: Drivers, Windows DACL read-back; integration-test-strategy.md: Module ↔ PTY; On-disk, two new items) · §1 Test Scope Summary (test-plan.md: Vector 7) · §2 Test Strategy (test-strategy.md: the V7 row)
**Change:**
- Windows `--home` outside the profile: was "as worded here stays owed to that route entry", read back with `GetNamedSecurityInfoW`; now `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (windows-2025) reads the home `viola verify` made under `target/e2e-home` back through `icacls <home> /save` (`Get-Acl` printed nothing) and requires it protected with exactly the user and SYSTEM: `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, canonical SDDL spelling the RID-500 runner user `LA`. Drivers: the root tests set and read grants through `icacls`, its exit asserted.
- `a_home_another_user_may_read_refuses_run`: a home made before viola that another user may read (Unix 0750, Windows a Users read grant) refuses `viola run` on all three OSes, exit 1, home pair, no `diagnostics/`, the grant as found.
- Single writers: was the wrapper SIGSTOPped (suspended on Windows); now stopped (`Wrapper::stop`), the case `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`, both files byte-identical after every other writer-capable entry point, three OSes. The V7 row lists the binary's cases, umask 000 among them.
- ConPTY companion owner/DACL (Windows x64): `tests/conpty_sideload.rs` `run_on_a_companion_another_user_may_write_falls_back_to_conhost`, the child on the inbox ConPTY, `sideload_fallback` `strict-modes-failed`, the exit unchanged; the Windows leg's kept homes hold one such fallback, that test's. `conpty_sideload`'s "5 cases" count is dropped; Vector 7 gains the bullet.
**Why:** the report's test criteria read met on three legs; the `LA` spelling, `Get-Acl`'s silence and the `icacls /save` read-back were measured on windows-2025 and fixed across the chunk's two fix commits.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — a home a test makes before viola: `create_private_dir`, the plain home refused, the per-OS oracle
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Setup / teardown lifecycle; CLI, `cli_verify`) · §7 Test Data & Fixtures (test-plan.md: On-disk product state; test-data-fixtures.md: On-disk product state, a new seed-strategy item) · §1 Test Scope Summary (test-scope-summary.md: Test data strategy, a new constraint)
**Change:**
- A test that makes a home before viola makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`, as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do: a `create_dir_all` home is refused, on Unix (0755) by the read rule, on windows-2025 `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`); that home's canonical SDDL is not recorded. Was "the refused home's DACL was not read, and that reading is owed to the working-route entry "Home and code-bearing file integrity"", per "2026-10-10-statusline-pass-through — the fake agent's statusline options, the reader's property and seeds, the stamped-home rule on Windows, three counts".
- A catch-site case holds a viola-made home with a regular file where the checked entry goes (`tests/cli_wait_last.rs`, `diagnostics/`; `tests/cli_verify.rs`, `ledger/`). A home planted under a regular file takes a per-OS oracle: on Unix a strict-modes refusal, exit 1 with the home pair; on Windows the path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`); `verify_under_a_regular_file_prints_only_the_internal_error_line` forces the catch-site line on every OS.
**Why:** the read rule now refuses a 0755 home; the Windows reading was measured on windows-2025, disproving the "not read" claim; the plain home's SDDL could not be recorded from a passing run, so the case asserts the refused ACE instead; the Windows `NotFound` stat came from the first red Windows run.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the strict-modes refusals' exits, each refused check writing no line
**Section:** §5 Integration Test Strategy (test-plan.md: On-disk; integration-test-strategy.md: On-disk, the strict-modes refusals item)
**Change:**
- Master: was "exit 1 for `run` / `mcp` / `viola revive`, hook 0 with no output, and CLI 21 for every other verb"; now exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair); a refused check at `run`, `viola revive`, `viola verify`, the frame verbs, the hook and the MCP frame tools writes no line.
- Detail: exit 1 also for the start home-tree check of `run` and `viola revive` (the home pair, or the instance pair at instance level; under `revive --json` the `strict-modes-failed` document; before the wrapper log, so no line and no child) and for `viola verify`'s (the home pair before any step: no step line, no line, `stamps.json` unchanged); hook 0 now also states the event and dialog paths, their instance refused or absent, read no `config.json`, open no role file, read no stdin and write no frame, no line and no SessionEnd append; the MCP frame tools return `instance-unreachable` with `detail: null`, no frame or line.
**Why:** the founder's own answer at the chunk's P4 (inputs#I3, live by dialog, relayed by the overseer, as the report records it): a refused check at every new entry point writes no line, as `link` / `unlink`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the home-widening helpers and the Filesystem faults row
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: a new Filesystem faults row) · §7 Test Data & Fixtures (test-data-fixtures.md: a new row, Widened test homes) · §2 Test Strategy (test-strategy.md: Sync (tokio-free) E2E suites, the support helpers)
**Change:**
- `tests/support/home.rs` holds `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened. The §7 row: widened test homes (the strict-modes negatives) are set by the test on the real file, never by viola, per-test.
- §8 Filesystem faults: no stub (a real tempfile home, real modes and ACLs); a widened or read-granted home tree is refused by the product's own check, never a stubbed one: on windows-2025 an instance tree granted Users write, or a `diagnostics/` granted Users read, is refused beside a `create_private_dir` tree that passes; the pure verdicts (`windows_read_verdict` among them) take synthetic descriptors at the unit tier; a test that widens a home tree above `bin/<key>/conpty/` starts its home unseeded.
**Why:** the chunk added the four helpers to `tests/support/home.rs` for its strict-modes negatives, and its Windows cases made the grants for real.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the hook perf rows with the instance check in the timed path
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the Perf session)
**Change:**
- Every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init, beside its server check; with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at this chunk's `run --perf`. Was `max` 2.2–2.4 ms over 30 runs each with the server check alone, per "2026-10-26-server-verification-before-any-frame — the hook perf rows with the server check in the timed path".
**Why:** the chunk put the instance check ahead of every hook's obs init; `run --perf` stayed green (7 passed, 0 failed), every hook row under `SPINE_DEADLINE`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — remove-the-guard witnesses bounded by `WITHIN`
**Section:** §2 Test Strategy (test-strategy.md: Tier-specific depth (Comprehensive, viola))
**Change:**
- Beside the mutation rule stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at this chunk).
**Why:** during implement a remove-the-guard witness hung 27 minutes under nextest's default profile; the chunk bounded the G4 and G10 witnesses. Trap for later chunks: a neutralised guard that lets a product process start needs a bounded witness.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the read rule and the instance check's widened set
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-state, the strict-modes items and the instance check) · §1 Test Scope Summary (test-plan.md: Vector 7)
**Change:**
- Was "Unix strict-modes checks `mode & 0o077`"; now strict-modes has the write rule and the read rule: `unix_read_verdict(mode, uid, euid)` is the write rule plus a refusal of `mode & 0o077` (`unix_read_verdict_needs_the_write_rule_and_no_foreign_bit`); `windows_read_verdict` is the write rule plus a refusal of any allow ACE granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to an untrusted SID, inherit-only ACEs included (`windows_read_verdict_refuses_any_read_right_to_another_sid`, `windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule`, `cfg(windows)` `read_rights_are_the_named_flags`).
- `check_instance`: was the home, `instances/`, the instance directory, `snapshot.json` and `events.ndjson`; now, a folder before what it holds, the home (read rule), `instances/` (write), the instance directory (read), its `diagnostics/` (read), `snapshot.json` (write), `events.ndjson` (write on Unix, read on Windows). Its call sites (`check_instance(` under `src/`) are three: `src/cmd/hook.rs` `trusted`, `src/cmd/client.rs` `refuse_untrusted`, `src/cmd/revive.rs`.
- Was "Not covered: a widened DACL on an instance tree on Windows … owed to the route entry "Home and code-bearing file integrity""; now `cfg(windows)` cases green on windows-2025: a tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and the plain-home case; the SDDL-stub item adds that these two grants are also made for real.
- Vector 7 gains: a home, instance directory or `diagnostics/` another user may read → refused (Unix a group or other bit; Windows a read grant to another SID).
**Why:** the chunk built the read rule and widened the instance check's set; the Windows cases ran green on the windows-2025 leg, closing the owed instance-tree case.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the tree, diagnostics, stamps and pin checks at the unit tier
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-state, new items; root bin, a new diagnostics-opens item; test-plan.md: viola-state, root bin) · §1 Test Scope Summary (test-plan.md: Vector 7)
**Change:**
- `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`, `check_diagnostics_of_what_viola_wrote_or_of_nothing_passes`, `check_diagnostics_refuses_a_widened_mode`, `check_diagnostics_refuses_a_symlinked_role_file_and_folder` (a symlink refused, a folder held to the read rule, a file to the read rule on Unix and the write rule on Windows, an absent path passing) and `strict_instance_holds_the_content_folders_to_the_read_rule`.
- Stamps and pin: `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key`, `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).
- Root bin, `src/obs.rs`: a `diagnostics/` folder or a role or detail file failing `strict::check_diagnostics` is a failed open, checked before `ensure_private_dir` narrows anything, the writer ending at `std::io::sink` and a detail line dropped (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`).
- §4 master: viola-state was "… over synthetic SDDL descriptors, and the instance check"; now the write and the read rule over synthetic SDDL descriptors and, on windows-2025, real grants, the instance check, the home-tree and diagnostics checks and the stamps and pin strict-modes refusals; root bin gains the diagnostics opens refusing a widened or symlinked path and leaving it as found.
- Vector 7 gains: a widened `ledger/`, `bin/`, `plugin/` or pinned copy → `run` exit 1 before any log, a widened `ledger/`, `bin/` or `plugin/` → `verify` exit 1 before any step; a symlinked `diagnostics/` folder or role file → refused, the open failing.
**Why:** the chunk added `check_home_tree`, `check_diagnostics`, `StampsError` and `PinError::StrictModes`, and checks the diagnostics opens before any narrowing; `update_stamps` checks before its create as well as under the lock, since a check under the lock alone comes after the create narrows a widened `ledger/`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — Path 7's MCP leg on the default pair, unstamped
**Section:** §3 → 5-command implementation (registry key file: run step 2's `harness_session` bullet) · §6 E2E Test Strategy (e2e-test-strategy.md: Scenario: Path 7 — unverified CLI version gate, a new As landed item)
**Change:**
- 5-command implementation: was "whose Path 7 MCP leg boots an unstamped `builder` alone", per "2026-10-17-mcp-server-for-drivers — harness keys and the time mechanism"; now "whose Path 7 MCP leg boots the default `overseer` + `builder` pair unstamped, `SessionOptions { unstamped: true, ..SessionOptions::default() }`".
- Path 7 gains an As landed item: the MCP leg is `mcp_tools_path7_unverified_cli_answer` (`crates/viola-e2e/tests/mcp_tools.rs`), which boots the default `overseer` + `builder` pair unstamped, so it also witnesses two first starts on one fresh unstamped home both landing (green on all three CI OSes, as measured at ci#40439188053).
**Why:** booting `builder` alone was the test-side workaround for the pinned copy's first-start race; with the absent-copy write now a no-clobber placement the workaround is retired and its race comment removed, and the default-pair session is ready on all three CI OSes.
**Kept:** Path 7's step-1 procedure, the CLI leg's agent-run form (`agent-run boot --session p7 --unstamped --instance builder`), which this chunk did not change.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the pinned copy's no-clobber placement: its forced-window witness, test parameter and unit cases
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: a new procedure block, Forced-window witness, filesystem placement) · §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: a new row, Forced-window test parameters) · §4 Unit Test Strategy (unit-test-strategy.md: viola-state, a new item; test-plan.md: viola-state)
**Change:**
- §5: a filesystem-placement forced-window witness (no PTY spawn), beside the PTY ones: `place_private_new_placed_copy_execs_at_once` (`crates/viola-state/src/fs.rs`, Unix) execs the placed copy right after a `Placed::Written` placement, through `place_private_new_with`'s `placed` hook. With the guard removed (the copy persisted while its write handle is still open) it is red on the Linux dev host with `ExecutableFileBusy` (raw 26, `ETXTBSY`), and green with the guard; it is green on `ubuntu-latest` and `macos-latest`. Its Windows form `place_private_new_placed_copy_runs_at_once` is green-only on `windows-2025` (ci#40439188053), and the red leg is read on the Linux dev host only.
- §8: the `placed` hook is a test parameter, not a seam: no env read, no feature, no mockall double, nothing added to the test-seam list.
- §4 detail, `fs::place_private_new`: `place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp` (0700 / 0600 on Unix, the mode set before any byte), `place_private_new_over_a_present_name_is_present_and_leaves_it` (`Placed::Present`, the target byte-identical, no temp left), `place_private_new_under_a_missing_dir_fails` and the two forced-window cases (§5). The lost placement, `pin::place_absent`: `place_absent_over_a_copy_another_start_placed` (the same bytes reused; other bytes `PinError::HashMismatch`, the file byte-identical) and `place_absent_on_a_free_path_writes_the_copy`.
- §4 master, viola-state: was "… and the stamps and pin strict-modes refusals."; now adds the pinned copy's no-clobber placement (`fs::place_private_new`) and the re-hash of a copy another start placed (`pin::place_absent`).
**Why:** the pinned copy's absent-copy write is now a no-clobber placement of a closed temp file, so a concurrent first start's agent can no longer exec the copy while another start holds it open for write. No standing forced-window site held a filesystem-placement witness, which made it the plan's expected §5 amendment. The hook adds no env var, flag or feature.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the plain home's refused Windows entry: no fixed Users SID
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Setup / teardown lifecycle, the plain-home item) · §7 Test Data & Fixtures (test-data-fixtures.md: On-disk product state; test-plan.md: Seed strategies, On-disk product state) · §1 Test Scope Summary (test-scope-summary.md: Test data strategy, the `create_private_dir` item) · §4 Unit Test Strategy (unit-test-strategy.md: viola-state, the instance-check item)
**Change:**
- The refused entry of a home made with `create_dir_all` before viola, still `Writable` on Windows: was an inherited `S-1-5-32-545` (Users) write ACE, per "2026-10-26-home-and-code-bearing-file-integrity — a home a test makes before viola: `create_private_dir`, the plain home refused, the per-OS oracle"; now an inherited write grant to a SID strict-modes does not trust, whichever SID holds it (read by its right, in integration-test-strategy.md, test-data-fixtures.md and test-plan.md). The unit-test-strategy item now points to its oracle below.
- The refusal's measurement: integration-test-strategy.md was "as measured at chunk 2026-10-26-home-and-code-bearing-file-integrity on `windows-2025`: ci#40398601594 for the refusal"; now "as measured on `windows-2025`: ci#40439188053 for the refusal". test-data-fixtures.md was ci#40398601594; now ci#40439188053. The seeded homes' ci#40403389629 stands.
**Why:** the claim was disproved by measurement. On one windows-2025 runner, Users held read only and Authenticated Users held the inherited write. The product's `Writable` verdict was right, and the oracle failed because it demanded the Users SID. The case now reads the grant by its right and is green on windows-2025. Folding this red (CI red 2) into the chunk as a test-only oracle fix is overseer1's word, founder-delegated. It is PROVISIONAL and owed the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — runner-inherited grants, and the plain-home oracle that names no SID
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: a new values paragraph, Runner-inherited grants) · §4 Unit Test Strategy (unit-test-strategy.md: viola-state, a new item, the plain-home case's Windows oracle)
**Change:**
- §5 records: on one image (`windows-2025-vs2026` 20261021.278.1), `windows-2025` runners differ by host in a temp dir's inherited ACL. On one (westus3) Users holds the write; on another (northcentralus) Authenticated Users (`S-1-5-11`, mask `0x001301BF`) holds it and Users has read only (ci#40403389629 and ci#40425866096). So an oracle for a runner-inherited grant names no SID: it reads the grant by its right, beside a control that cannot pass vacuously.
- §4: the plain-home case's Windows oracle is the `cfg(windows)` test helper `strict::win::tests::inherited_untrusted_writes(home)`. It returns the allow ACEs that are inherited, carry a `WRITE_RIGHTS` bit and name a SID `trusted_sid` does not trust, so it reads the grant by its right whichever SID holds it. Its control, a home `instance_tree()` makes, reads none, so the oracle cannot pass vacuously (green on `windows-2025`, ci#40439188053).
**Why:** an oracle that names one SID for a runner-inherited grant depends on the host it lands on. This sets the rule for later chunks: read such a grant by its right, with a control that cannot pass vacuously.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the fake agent's `hook` receipt carries `spawn_os_error`
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: the fake agent's receipt kinds, `hook`)
**Change:**
- The `hook` receipt: was {`event`, `command_absolute`, `ran`, plus `exit_code`, `stderr_len`, `stdout_hex` and `stdin_hex` when it ran}; now it also carries `spawn_os_error` when the spawn of an absolute `command` failed: the raw OS error as an integer, `null` when the OS gives none.
**Why:** the fake agent gained the field so that the pinned copy's first-start race was read by its errno on the product run itself, not inferred from `ran:false`. Each failed boot read `ETXTBSY`, which confirms the race's hypothesis. A spawn that succeeds and the non-absolute arm are unchanged.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the serde_json `unbounded_depth` ban and deny-probes 14/14
**Section:** §9 CI Integration (ci-integration.md: the Supply-chain stage's Runs; test-plan.md: Build failure conditions, Supply-chain errors)
**Change:**
- Supply-chain Runs: `deny.toml`'s bans now hold a `[[bans.features]]` table that denies serde_json's `unbounded_depth`. `scripts/deny-probes.sh` ends `deny-probes: 14/14 banned, control clean`: the thirteen bans one probe each, the new ban's row `serde_json-unbounded-depth` firing `error[feature-banned]`, plus probe `channel-tokio-edge` with its channel control. Was `13/13` and the twelve bans, per «2026-10-17-mcp-server-for-drivers — the CI gates: the channel tree form, rmcp features, deny-probes 13/13».
- Supply-chain errors: the deny-probes failure reads a line not ending `deny-probes: 14/14 banned, control clean` (was `13/13`).
**Why:** the chunk made serde_json's depth limit a `deny.toml` feature ban with a firing probe; `cargo deny check` is clean and the probes read 14/14. The count's rule: the `probes=(` rows of `scripts/deny-probes.sh` (now 13) plus the planted channel probe. The Runs clause naming the ban and the new count were written as one edit of the line on the operator's answer at the P2 halt (the overseer as operator, founder-delegated): the contradiction read against the ban clause was the stale 13/13 count the same apply retires.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the feed-panic passthrough witnessed on the outer terminal
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the drivers table's tui row, the Chaos suite's forced vt100 panic mode; test-plan.md: the tui driver, the Chaos suite) · §1 Test Scope Summary (test-scope-summary.md: the tui surface's Signal; test-plan.md: the tui surface, the security-vector-coverage (child spawn) trigger)
**Change:**
- The forced vt100 panic case is `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs`): `send` exits 13 `input-not-ready`, a typed key reaches the child with nothing typed, and the outer terminal shows the child's own panicking bytes (`e4 b8 ad`) on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` on every OS. Was: human passthrough still reaches the receipt, with no outer-terminal reading.
- On Windows the case asserts the literals' absence only; the byte oracle for the panicking wide character is owed to the route entry "Windows-only live measurements".
- The tui surface's signal and the tui driver's verdict sources gain, for this case alone, a byte search of the outer terminal's drained master-reader stream, never a parse of the rendered child screen (was: `events.ndjson`, the receipt and exit detection only). The child-spawn trigger's forced-panic item and the Chaos suite line name the passthrough as witnessed on the outer terminal.
**Why:** the chunk's chaos test now reads the outer terminal, green on ubuntu, macOS and windows-2025. ConPTY re-renders the child's output and a wide character on a 1-column screen has no guaranteed rendering, so the Windows byte reading rides the P5 card's CARRY on «Windows-only live measurements», approved by delegate overseer1 and PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the `viola mcp` stdin line cap's tests
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the ipc-internal (MCP stdio) row, the Security sweep) · §1 Test Scope Summary (test-scope-summary.md: the MCP surface's as-landed entries; test-plan.md: the security-vector-coverage (MCP) trigger) · §4 Unit Test Strategy (unit-test-strategy.md: a new viola-mcp group; test-plan.md: the viola-mcp bullet)
**Change:**
- §1: `viola mcp` reads stdin through `CappedLines` capped at `MAX_FRAME`; a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line holding no byte of it; on an initialized session the earlier lines are answered, exit 0, stderr empty, stdout only the earlier frames. The MCP trigger's required tests add this negative.
- §6: `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`) spawns and owns a `viola mcp` child from a fresh home, writes raw lines from its own writer thread (`initialize`, `notifications/initialized`, one line past `MAX_FRAME`, stdin left open), drains stdout and stderr on their own threads, awaits exit under an 8 s kill deadline: exit 0, stderr empty, stdout only the `initialize` response, exactly one such line in `mcp.ndjson`, valid against `schemas/diag-line.v1.json`, the line's canary in no file under the home.
- §4, a new viola-mcp group: `stdin::tests` hold a line of exactly the cap whole, its `\n` counted, and one byte more releasing no byte, setting the cut flag and reading as end of stream, at the literal `16 * 1024 * 1024` too; the count restarts after each newline; a split over-long line is cut, a split fitting one joined (the six `capped_lines_*` cases); the `serve` child cases, each killed at 5 s: `serve_ends_the_session_at_a_line_past_max_frame`, `serve_on_a_closed_stdin_is_the_initialize_error`.
**Why:** rmcp 3.4.1's stdio transport reads each line unbounded, so the bound belongs to the reader viola hands it. A line is refused whole: a truncated one could dispatch a padded request, a skipped one leaves a request unanswered. Green on the three CI OSes (ci#40458923095).
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the bounded-reader enumeration in the Contract suite
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Contract suite; test-plan.md: the Contract suite)
**Change:**
- `tests/contract_bounded_readers.rs`, a `contract_` binary in the E2E layer (`run --e2e`), holds two literal tables, never the product's own list: `BOUNDARIES`, one row per external reader naming its bounded read, its cap, depth and type rule and the existing test that witnesses it; `EXCEPTIONS`, the reads of viola's own executable (`checked_exe`, `pin_exe`), no external input. Its tests: `boundaries_each_name_a_bounded_read_that_stands` (each row's read in its file, its witness a defined test, each exception a read that stands), `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one`, `serde_json_unbounded_depth_is_banned_in_deny_toml`.
- The census (its file carrying the `andromeda:walks-tree` token) walks the product source roots, comments, strings and test items left out: a reader call passes when `take(` stands in its own statement up to the call, or an `EXCEPTIONS` needle stands on the call's whole source line. Its self-check reads a planted unbounded read as a hit; with the `replace_private_shared` compare bound undone it reads red at exactly `crates/viola-state/src/fs.rs:354`.
**Why:** the chunk's criterion that every external reader is size-capped needs an enumeration no new reader slips past. The exception needle is matched against the whole line because the executable argument it names lies past the call's `(`, outside the statement.
**Kept:** neither master states the tables' row counts: a master names the functions, and the code's rows are the count.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the bounded compare read of `replace_private_shared`
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-state; test-plan.md: the viola-state bullet)
**Change:**
- viola-state adds the bounded compare: `fs::replace_private_shared`'s fallback, through the private `holds_exactly`, reads the target through `take(bytes.len() + 1)`, never the whole file. `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` holds its outcome to a whole-file compare's on every OS for an equal, a longer, a shorter, a different and an absent target; the refusal of a longer target under a read-only directory runs on Unix only, Windows using a read-only file.
- test-plan.md's viola-state bullet names the bounded compare read (at most one byte past the bytes, its outcome a whole-file compare's).
**Why:** the compare read was the one product read of a home file with no cap; one byte past the compared bytes decides equality as a whole read does. The Windows shape matches the existing case's stand-in.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — two owed test gaps, owned by «Sanitised error surfaces»
**Section:** §4 Unit Test Strategy (test-plan.md: the viola-mcp bullet) · §6 E2E Test Strategy (test-plan.md: the Contract suite)
**Change:**
- §4 viola-mcp, not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ.
- §6 Contract suite, Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the same route entry, as a PREREQ (a cap-driving case for each row).
**Why:** both are the chunk's own deviations, which the wrap directive (the operator's request; the overseer, founder-delegated) asked be given an owner or a test-plan note. The next route entry takes both as a PREREQ by the standing rule that a missing test for a clause a chunk landed is folded into the next chunk.
**Kept:** each gap is stated once, in test-plan.md with its owner; the e2e detail's notes restating them were rejected as twins.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-31-sanitised-error-surfaces — the `ci` kill line's verify-driven binaries
**Section:** §3 Bootstrap phases (registry row `bootstrap-phases-derive-for-route-setup-project.md`: test-runner-install, `[profile.ci]`)
**Change:**
- The verify-driven binaries on the `ci` profile's `{ period = "10s", terminate-after = 2 }` line (a 20 s kill) read twenty-four (was twenty-two).
- The `binary(…)` names now list `channel_error_bodies` and `cli_error_bodies`, on the line because each one's witness starts from `stamped_home`.
**Why:** the `ci` filter gained the chunk's two new binaries; a count of `binary(` over `.config/nextest.toml` reads 24.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error bodies' surface witnesses and the planted value
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the driver table's cli, wrapper-channel and MCP stdio rows; the suite-error-sanitization `[[test]]` item) · §7 Test Data & Fixtures (test-data-fixtures.md: a new row)
**Change:**
- cli row: `tests/cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text` holds each CLI `--json` failure it drives, across dead, refused and live rows, to one document, a literal exit, empty stderr, no ESC and a clean body.
- wrapper-channel row: `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` writes seven fault rows to the channel of a wrapper booted with a planted value, its bodies clean and the three `detail` words read on the lines.
- MCP stdio row: `mcp_tools_error_results_hold_no_path_or_upstream_text` (an unstamped session, no row reaching a stamped path) and `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` stand in `tests/mcp_tools.rs`. rmcp's client reads the wire's `"data":null` as `None`, so the live unknown-tool case asserts `None`; on the wire it is the product's `Some(Value::Null)`.
- `channel_error_bodies` and `cli_error_bodies` are `[[test]]` tables with `required-features = ["fake-agent"]`, each on the `ci` profile's 20 s kill line.
- §7's new row, strategy per-test (rstest `stamped_home`): `booted_with_the_value` starts from `stamped_home` and boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env` (`Wrapper` takes no planted value); the value reaches that one child through the PTY spawn's `env_set`, as `tests/tui_env_strip.rs` does; `assert_error_body_clean` takes the planted values and reads a hit as class `planted`, reporting place and class, never bytes.
**Why:** the chunk added these witnesses; `Wrapper` offered no way to plant the value, so the boot went through the PTY spawn. An oracle asserted against the wire form read red through rmcp's client, hence `None`. The row's "no planted value" wording, in place of "no environment", is the overseer's answer as the founder's delegate at the P2 halt, PROVISIONAL.
**Kept:** §7's standing line on `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on its one child: that wording keeps it true.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `viola mcp`'s exit at a line past the cap before `initialize`
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the MCP stdio row) · §1 Test Scope Summary (test-scope-summary.md: the MCP surface, **As landed (the line before `initialize`)**) · §4 Unit Test Strategy (unit-test-strategy.md: viola-mcp; test-plan.md: viola-mcp)
**Change:**
- test-plan.md §4 was: not driven by any test, ending `serve` with `ServeError::Initialize` (exit 11), owner the route entry "Sanitised error surfaces", as a PREREQ. Now: the case is driven, the owner clause is gone, and it ends `serve` with `ServeError::Initialize`, as a closed stdin before `initialize` does.
- Exit was 11; now 11 is the unit child's own code and `viola mcp` itself exits 1, both streams empty, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line.
- Its holders: unit `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` (`crates/viola-mcp/src/lib.rs`), E2E `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`).
**Why:** measured: the E2E case reads exit 1 with both streams empty and that one line; 11 is the unit child's own code, and the process exit for `Role::Mcp` with `Failed` is 1. The process exit stays 1 and the masters move, as the plan leaned. The owed clause is discharged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the after-every-test error-body check, replaced and not built
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: suite-error-sanitization; test-plan.md: Error sanitization and secret scan) · §2 Test Strategy (test-strategy.md: the `tests/support` item, the trigger table's error sanitization row)
**Change:**
- Was: after every E2E test, the `events` support helper checks. Now: no helper runs after each E2E test; none is built, replaced by the three surface witnesses, the enumeration and the census of writers (`tests/contract_error_bodies.rs`), reading the check in `tests/support/events.rs`.
- The check: `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes, self-tested by `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read` (`tests/cli_error_bodies.rs`); test-strategy's `events.rs` description names it.
- Every `-32603` stays exactly `"internal error"` / `null`, held at the unit layer (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`): no scanned home drives one, no failpoint or counted panic stands for one.
- `v1-47`'s "after every end-to-end test" reads "after each job's end-to-end run", PROVISIONAL: `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()`, the `viola-chaos-*` home out.
- test-plan.md names the holders: the no-`Cookie`, stripped-value and canary clauses are `viola-harness secret-scan`'s, after each job's end-to-end run; the 0600 clause `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`'s (`secret_scan.rs` reads `cookie:` whatever its case).
- The trigger row names `cli_error_bodies.rs`, `channel_error_bodies.rs` and `mcp_tools.rs` with their witness cases, and `contract_error_bodies.rs`.
**Why:** the check was measured absent before this chunk and is not built. `v1-47`'s reading is the delegate overseer's word, PROVISIONAL, owed the founder's own word; escalated at the P2 halt as a contradiction, it was applied on the overseer's answer as the founder's delegate, PROVISIONAL the same. The chaos home stands out by the founder's ruling of 2026-10-04, as the report records. The report names no holder for the log, 0600 and canary clauses; the disposition records them named by main's direction.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the error-body enumeration in the contract suite
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Contract suite)
**Change:**
- `tests/contract_error_bodies.rs` (a `contract_` binary, `run --e2e`) is the error-body enumeration: the tables `CLI_DOCUMENTS`, `MCP_RESULTS`, `MCP_FIELD_WORDS`, `CHANNEL_FAULTS`, `CHANNEL_METHODS`, `NOT_ERROR_BODIES` and `PLUMBING` over `PRODUCT_ROOTS`, its file carrying the `andromeda:walks-tree` token.
- Its guards: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `channel_faults_are_the_seven_literal_bodies`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`, with the census self-check `census_finds_an_unnamed_writer_and_an_interpolated_message`.
**Why:** the chunk's new contract binary, green at the final HEAD's run. `PLUMBING` names the census lines that build or route a document a row names, so a row stays one body.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the reply screen and the fault screen
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the MCP screen case) · §1 Test Scope Summary (test-scope-summary.md: the `viola-channel` entity's **As landed (the fault screen)**, the MCP surface's **As landed (a peer's reply)**) · §4 Unit Test Strategy (unit-test-strategy.md: a new viola-core group, viola-channel, viola-mcp, root bin; test-plan.md: viola-channel, viola-mcp, root bin)
**Change:**
- viola-core group: `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first (`code_word_literal_table`).
- `viola_channel::sanitised_fault` reads a peer's error for `reply_of` and `of_reply`: each of the five known codes takes its fixed `message`; `-32602` keeps only an integer `data.supported` with a version-text `data.wrapper`, or a `data.reason` exactly `release-from-driver`, any other reading `invalid params` with `data: null`; every other known code reads `data: null`; a code outside the five reads `wrapper fault` with `data: null`; an error without an integer `code` reads nothing. The human line reads `error: wrapper fault  <code>`.
- Its units: `sanitised_fault_keeps_each_wire_body`, `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`, `sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters`, `sanitised_fault_without_an_integer_code_is_nothing`.
- `of_reply` and `reply_of`: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null` / `None`, a non-string `detail` is no known reply (`of_reply_maps_each_reply_shape`, `reply_of_reads_the_three_shapes`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`, `mcp_tools_error_results_hold_no_path_or_upstream_text`).
- A code word outside the closed refusal list stays visible in the document and in `refused: <word>` by design, its exit `unknown`'s 14 by `reason_of`.
- test-plan.md §4's three broad lines name these screens.
**Why:** `sanitised_fault` replaces the two private `fault_detail` copies, so a peer's `message`, `data`, `refusal` or `detail` reaches no document or text. `wrapper fault` is a new fixed output word on `--json` and MCP; the human line is unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the exit-2 paths that write no document
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: oversize `answer`) · §1 Test Scope Summary (test-scope-summary.md: the cli **Signal**; test-plan.md: the cli surface)
**Change:**
- Oversize `viola answer` stdin was: exits non-zero with one `--json` error document before connecting. Now: exits 2 before connecting, with the fixed `UNUSABLE_RESPONSE` line (`src/cmd/answer.rs`) on stderr whatever `--json` says and no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`).
- The cli **Signal** gains the exception: the exit-2 paths of `viola answer` (`UNUSABLE_RESPONSE`) and `viola send` (`UNUSABLE_TEXT`) write that fixed line to stderr, with no document; test-plan.md's cli surface carries the same exception.
**Why:** measured: these exit-2 paths write the fixed line whatever `--json` says, disproving the one-document claim.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `channel-response`'s closed `detail` and the wire bodies
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: `tests/cli_links.rs`) · §1 Test Scope Summary (test-scope-summary.md: the wrapper-channel surface's **As landed (the error bodies)**, the fault screen's `ProtocolError::cause` sentence) · §4 Unit Test Strategy (unit-test-strategy.md: viola-channel)
**Change:**
- A `channel-response` line's `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other such line, on both the server's and the client's line at WARN, with no name or param on it, held by `link_verbs_log_the_channel_pair_and_no_names` and `schemas/diag-line.v1.json`.
- `ProtocolError::cause` gives those words and is `None` for the other four faults: `protocol_error_cause_tells_the_three_invalid_params_apart`, `answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`.
- The seven `ProtocolError` wire bodies stand literal with `v` 1, a too-new request answered `-32602` with the wrapper's version (`protocol_error_body_is_the_literal_wire_object`, `channel_error_bodies_hold_no_path_or_upstream_text`). No `-32603` is driven in a scanned home; the unit layer holds it.
**Why:** the schema closes `detail` from any string to the three words; the wire form does not change and the seven bodies are unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the bounded readers' cap-driving witnesses
**Section:** §1 Test Scope Summary (test-scope-summary.md: the budget path's statusline signal) · §4 Unit Test Strategy (unit-test-strategy.md: root bin) · §6 E2E Test Strategy (test-plan.md: Contract suite)
**Change:**
- The Contract suite's clause was: Owed, the five rows name a witness that reads the bounded read without driving past its cap, their caps held by the row's needle and the census alone, owner the route entry "Sanitised error surfaces", as a PREREQ. Now: As landed, the five `BOUNDARIES` rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` each name a cap-driving case.
- The cases: `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap` and `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command` (`src/cmd/hook/statusline.rs`), `drain_keeps_cap_bytes_of_an_endless_pipe` (`src/run/version_gate.rs`), `read_thresholds_keeps_no_key_past_the_frame_cap` (`src/run/budget.rs`), `read_hold_keeps_no_key_past_the_frame_cap` (`src/run/dialog.rs`), `env_keep_keeps_no_key_past_the_frame_cap` (`src/run/env.rs`).
- The statusline signal gains: the shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command.
**Why:** the chunk renamed the five rows' `witness` to cap-driving cases, discharging the owed clause; the cases and the census read green.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `resolve_program`'s error after `Refusal`
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-agent-claude)
**Change:**
- `resolve_program` returns `Result<PathBuf, AgentError>`, `AgentError` holding `BatchScriptChild` and `ProgramNotFound`, their messages and outputs held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.
**Why:** `viola_agent_claude::Refusal` is removed and folded into `AgentError` (`ProgramNotFound` was `Refusal::NotFound`), its two messages and the outputs unchanged.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `ci` kill line's verify-driven binaries
**Section:** §3 Bootstrap phases (registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md: test-runner-install, `[profile.ci]`)
**Change:**
- The `ci` profile's second override holds the twenty-five verify-driven binaries (was twenty-four), its `binary(…)` name list gaining `cross_exit_causes` between `contract_ledger_probes` and `cross_list_rows`, as `.config/nextest.toml` reads; its `{ period = "10s", terminate-after = 2 }` (the 20 s kill) stands.
**Why:** `.config/nextest.toml`'s `ci` kill line gained `binary(cross_exit_causes)` with the new table. The report read no master as stating this count, its pattern requiring the noun right after a number; the spelled-out count here disproves that reading, its rule the `binary(…)` names on the filter line.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — MCP's `instance-unreachable` body names its cause
**Section:** §1 Test Scope Summary (test-scope-summary.md: the MCP surface's Signal and As landed (the instance check), the server verification's MCP half); §5 Integration Test Strategy (integration-test-strategy.md: the impostor case, the widened-instance case, MCP's mapping, the On-disk strict-modes item); §6 E2E Test Strategy (test-plan.md: Cross-surface parity)
**Change:**
- The MCP `isError` body reads `{"error":"instance-unreachable","detail":"<cause>"}` (was `detail` null), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`; its text stays `refused: <code>`.
- A refused instance check on the frame tools carries `strict-modes-failed` (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, the On-disk item); a server that is not the recorded wrapper carries `server-verify-failed` (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`); each still with no frame and no line.
- New: MCP's mapping in `crates/viola-mcp/src/tools.rs` `frame` and `crates/viola-mcp/src/result.rs` `of_reply`: a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `server-verify-failed` (off macOS), `Connect` is `instance-dead`, `Closed` or `Io` is `endpoint-lost`, no frame and no line; `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` hold each cause its word.
- Cross-surface parity gains a row: for each cause of exit 21, the MCP `{error, detail}` equals the CLI `--json` document less `v`, text `refused: instance-unreachable`, beside the standing `{refusal, detail}` row.
**Why:** the founder's own word (`inputs#I2` Q1, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him) put the cause in `detail` on CLI and MCP, `error`, `v` and MCP's text unmoved; his `inputs#I2` Q4, the same authority, holds MCP to the exit-21 causes its five tools can meet, its rejects lineless and `isError` their record. The rows' reading inside those answers, and the mapping's item applied over its contradiction with a standing line (Q5), are `inputs#I6`, delegate the overseer, PROVISIONAL; the ratification stays the founder's. The no-endpoint reading is `inputs#I2` Q2, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — MCP's lost-cause witness, its row after `cleanup` and the unserved row
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Path 3); §5 Integration Test Strategy (integration-test-strategy.md: the MCP cases); §7 Test Data (test-data-fixtures.md: On-disk product state, the impostor cases' group)
**Change:**
- Path 3 gains `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (`crates/viola-e2e/tests/mcp_tools.rs`), MCP's lost cause.
- The `mcp_tools` row after `cleanup` reads its cause from the directory: a home kept by `AGENT_RUN_KEEP_HOMES` reads `instance-dead`, a removed one `unwrapped-name`, its body compared with the CLI's.
- The unserved row in `crates/viola-mcp/tests/handlers.rs` records, through `record_live(instance, endpoint, pid)`, an endpoint no one ever bound, because a Windows `Serving` has no `Drop` and its pipe lives in the accept thread until the process exits.
- The On-disk product state row's hand-written exceptions read four (was three), the MCP handler suite's own snapshots joining; `record_live` stands beside the server-identity impostor cases.
**Why:** a kept home, as on CI, holds the stopped instance and a removed one does not, so the row reads the directory. A served row would hold a Windows pipe for the process's life, so the row binds nothing. The Path 3 and `record_live` items, read as contradicting standing lines, were applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's CLI document, one per cause
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Path 3 step 7, the Exit-cause matrix's exit-21 head and its strict-modes and server-verification causes, Security control negatives' strict-modes item); §5 Integration Test Strategy (integration-test-strategy.md: On-disk, the link verbs); §1 Test Scope Summary (test-scope-summary.md: the server verification, the CLI's As landed (the instance check)); §2 Test Strategy (test-strategy.md: the Filesystem (V7) row)
**Change:**
- Exit 21's document reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (was `detail` null for every cause); the five closed causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, on the eight frame verbs, `list` and `plugin install`.
- Path 3 step 7: exit 21 with `<cause>`, `endpoint-lost` for a wrapper killed under a parked `wait`, and after `cleanup` `instance-dead` in a kept home or `unwrapped-name` in a removed one.
- A refused instance check reads `strict-modes-failed`: the six frame verbs' negative `frame_verb_refuses_a_widened_instance_before_any_log`, the link verbs, the eight verbs' instance check, the V7 row; the frame verbs' case now stands beside the table's `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log` (was: not `cross_exit_causes.rs` rows).
- The failed server check reads `server-verify-failed` (was every exit-21 cause's one document); its `--json` is named in the scope summary's eight-verb record.
**Why:** the founder's own word, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): `detail` turns from null to the closed cause word. The rows' reading inside his answer is `inputs#I6`, delegate the overseer, PROVISIONAL, the V7 row and the five causes' item being applied at the same record's Q5; the ratification stays the founder's.
**Kept:** `error` stays `instance-unreachable`, `v` stays 1 and every refusal stays exit 21; `{"v":1,"error":"<code>","detail":null}` stays the exit-1 documents' form.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `cli` catch site's document under `--json`
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Error sanitization and secret scan); §4 Unit Test Strategy (unit-test-strategy.md: the root bin)
**Change:**
- New rule: under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout and leaves stderr empty, witnessed by `cli_verbs_print_the_internal_error_document_under_json` (`tests/cli_wait_last.rs`) and `a_frame_verb_that_cannot_open_its_log_is_internal_error` (`tests/cross_exit_causes.rs`).
- Without `--json` the line `error: internal error` stands byte for byte, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- `json_asked` (`src/main.rs`) reads `--json` from argv only before a `--`, held by `json_asked_reads_the_flag_before_a_separator`.
**Why:** the founder's own word, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): the catch site's document under `--json` replaces its stderr line, exit 1 and the human line staying. The e2e item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the exit-cause table is built
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's head, As landed and self-checks; test-plan.md: Exit-cause matrix, Security control negatives); §7 Test Data (test-data-fixtures.md: Human-mode expected output)
**Change:**
- The matrix reads built (was `cross_exit_causes.rs` not existing, no rstest table): `tests/cross_exit_causes.rs`, a root `cross_` binary, `required-features = ["fake-agent"]`, on the `ci` profile's kill line, rstest `#[case]` by mode, one negative test per cause it holds.
- Every case asserts the exit code, the `--json` document and a last `hint:` line naming its cause, a fault carrying none. Each cause is asserted in both modes where a mode exists, with each logged cause's one `process-exit` line and its own `detail`.
- Exact strings are literals in the file's constants (was pinned with trycmd once the product defined them); `CAUSES`' codes, exits and last `hint:` lines join the Human-mode row.
- The causes its top comment names outside the table are server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, and `viola mcp`'s two silent exits: each a case of the binary its row names.
- Self-checks: `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.
- Security control negatives' body gains `viola list`'s whole-board refusal opening no log and `viola plugin install`'s creating nothing (`strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`).
**Why:** the chunk built the table and retired the matrix's not-built lines. `every_code_is_a_code_word` was added beyond the table's planned list. The both-modes item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Kept:** a cause named outside the table keeps its own binary's case, so the matrix's remaining "not a `cross_exit_causes.rs` row" lines, for the link verbs' two cases and the server identity's, stand.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stopped instance and an unwrapped name
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-21 list; test-plan.md: Scenario E1); §4 Unit Test Strategy (unit-test-strategy.md: viola-state's `without_endpoint`)
**Change:**
- No instance running for the name: the endpoint is gone after `cleanup` with the instance's directory standing, `detail` `instance-dead`, the table's `a_stopped_instance_is_instance_dead`.
- The name holds no instance directory, as an unwrapped `claude agents --json` row or a name never started: `detail` `unwrapped-name`, its own hint; `a_name_never_started_is_unwrapped_name` and `e1_send_naming_an_unwrapped_row_is_unreachable` hold it (was exit 21 only, its hint not built, owed to "Exit-cause code catalogue").
- `viola_state::liveness::without_endpoint(instance_dir)` reads by `symlink_metadata` a directory as `InstanceDead` and anything else (absent, a regular file, a symlink) as `UnwrappedName`, opening nothing and writing no line; `without_endpoint_maps_the_directory_reading` (`src/cmd/client.rs`) holds the client's mapping.
**Why:** the cause in `detail` is the founder's own word, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him). Unwrapped reading by the instance directory is `inputs#I2` Q2, delegate the overseer, PROVISIONAL. This chunk built the hint the E1 line owed, so its owed clause is spent.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a wrapper lost under an open call
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-21 list, the Chaos suite; test-plan.md: Chaos suite); §8 Mocking (mocking-stubbing-discipline.md: Process faults); §1 Test Scope Summary (test-scope-summary.md: the chaos trigger)
**Change:**
- New matrix row: a wrapper lost with the call open is `detail` `endpoint-lost`, `during:"call"`, its own hint, the table's `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`.
- The chaos line and the chaos trigger for an endpoint vanishing during `wait` name `endpoint-lost` (was exit 21, `instance-unreachable`, with no cause); `tests/chaos_wait_vanish.rs` kills through `support::start::kill_wrapper`.
- The new Process faults row: a wrapper lost under an open call is a real kill, `kill_wrapper` (`tests/support/start.rs`) for the table's lost row and for `tests/chaos_wait_vanish.rs`, `kill_by_pid` (`crates/viola-e2e/tests/mcp_tools.rs`) for the MCP case; no double stands in.
**Why:** a wrapper that goes away mid-call is a cause of its own, apart from a dead instance, whose `during` moved to `connect`. The cause word is the founder's own, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him); the lost cause's hint is `inputs#I2` Q3, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder reads `stale-heartbeat`
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-1 list and `already-live`, the Chaos suite; test-plan.md: Chaos suite); §5 Integration Test Strategy (integration-test-strategy.md: `Frozen`); §8 Mocking (mocking-stubbing-discipline.md: Process faults); §1 Test Scope Summary (test-scope-summary.md: the run start path)
**Change:**
- A stale holder refuses the start exit 1 with `stale-heartbeat`, printed and logged (was `already-live`); a live holder or a fresh replayed beat keeps `already-live` (`a_live_holder_is_already_live`).
- A frame verb against a stale instance reads `instance-dead`, because `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1's cause alone.
- Witnesses: `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (`tests/cli_instance_state.rs`).
- `Frozen` is a guard on Windows and on Unix, a `SIGSTOP` on Unix; both stale cases stand on it, and the Process faults row names it: a stale holder is a real wrapper, no double.
- The SIGSTOP chaos item and the body's chaos line name `stale-heartbeat` on the refused `run`.
**Why:** the stale holder's own code retires `already-live` as its refusal. `Frozen` was Windows-only, so a stale case on it would have read nothing on Linux or macOS. The scope summary's item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola revive --json`'s ten codes and the closed gap
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-1 `revive` causes); §1 Test Scope Summary (test-scope-summary.md: the CLI verbs entity, the cli surface)
**Change:**
- A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints one exit-1 document under `revive --json`, before `spawn_child`, each a table row: `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, `a_tampered_pinned_copy_is_pinned_hash_mismatch`, `a_held_endpoint_is_squatted_name`, `a_revive_with_no_claude_on_its_path_is_internal_error` (was no document, owed to "Exit-cause code catalogue").
- Its codes read ten (was five): `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`; its exit 1 now also covers a refusal of the start.
- A `pump_child` error prints no document, the child having held the terminal; a program not found at the start is the `internal-error` row.
- The script-child row is `verification-matrix.json#v1-39`'s Windows witness; `run` passes `json` false to `Launch`, so its human pairs stand byte for byte.
- The `tests/cli_revive.rs` cases stand beside the table's `strict_modes_on_a_start_is_exit_1_and_opens_no_log`, `a_revive_with_no_such_session_is_no_session`, `a_session_open_elsewhere_is_session_live` and `a_recorded_directory_gone_is_cwd_missing` (was not rows of the table).
**Why:** the founder's own word, `inputs#I2` Q4 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): `run` takes no `--json`, so its start's causes reach the log and the hint and, for that start, `revive --json`. The gap was the CARRY of chunk 2026-10-17-cli-machine-contract. The rows' reading inside his answer and the ten-code, `pump_child` and witness items (Q1, Q5) are `inputs#I6`, delegate the overseer, PROVISIONAL; the ratification stays the founder's.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the socket-directory refusal and macOS's server check stay with their route entry
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Security control negatives, the Unix socket directory refusal); §1 Test Scope Summary (test-scope-summary.md: the server verification)
**Change:**
- New: the socket-directory refusal has no code; `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site, owned by the route entry "Unix endpoint and home hardening" with macOS's server check.
- The macOS server-verification cases read owed to that route entry (was owed to §6 Exit-cause matrix).
**Why:** `v1-18`'s requirement listed a socket-directory refusal the code does not hold. The founder's own word, `inputs#I2` Q4 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him), keeps both carve-outs with this route entry; the row leaving that code out of the chunk is `inputs#I3`, delegate the overseer, 2026-10-31, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the bare temp-dir home and the liveness-check case
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Setup / teardown lifecycle, Bare temp-dir home)
**Change:**
- New: a bare temp dir reads 0755 under the umask and the read rule refuses it; `tool_inputs_that_pass_reach_the_liveness_check` reaches the check it names from a viola-made `empty_home` and reads `unwrapped-name`.
**Why:** before this chunk the case's bare temp dir never reached the liveness check, its strict-modes refusal sharing one body with a dead instance. The trap for later chunks: an oracle moved by a shared body can hide a test that never reached its subject.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola verify`'s refusal codes
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the verify items); §7 Test Data (test-data-fixtures.md: the fixture-hygiene group)
**Change:**
- `viola verify` writes its refusal's code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` and `unclean-fixture`, beside `interrupted`.
- A refused home tree under `verify` stays lineless, a failing row is no refusal and no code, the fixture's file name enters no code, and `verify` takes no `--json`.
- Witnesses: `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` (`tests/cli_verify.rs`), the second under the `verify_window_` prefix because it runs two whole verifies, and `verify_with_no_claude_on_its_path_is_program_not_found` (`tests/cross_exit_causes.rs`).
- A recording refused as unclean is `verify`'s `Ended::Refused`: its `process-exit` reads `detail` `unclean-fixture`, a member of the `process-exit` `detail` enum in `schemas/diag-line.v1.json`.
**Why:** the founder's own word, `inputs#I2` Q4 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): `verify` has no `--json`, so its causes reach the log and the hint.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the root `cross_` binary and the shared start helpers
**Section:** §2 Test Strategy (test-strategy.md: the source layout; test-plan.md: File naming)
**Change:**
- `cross_` was a `viola-e2e` prefix alone; the root package now also holds a `cross_` binary, `tests/cross_exit_causes.rs` (sync, `[[test]]` `required-features = ["fake-agent"]`), which the E2E layer regex selects as it selects `path1_`.
- New: `tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`.
**Why:** the new table is a root `cross_` binary, which the E2E regex takes as it took `path1_`, and no harness verb moved for it; the setups moved to `tests/support/start.rs` because a root test file holds no copy of its own of what two share.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the closed causes at the unit tier and the two built hints
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-core, the root bin's server check and hints)
**Change:**
- New: `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify` and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words.
- `unreachable_logs_its_cause` logs four closed `Unreachable` causes, `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (was two); `endpoint-lost` is `during` `call`, the other three `during` `connect`.
- New: the `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`; the hint cause of `instance-dead` keys on `not-running`, a word apart from the log's.
**Why:** every word is written by the two clients. A strict-modes check refused before a log opens writes no line (`inputs#I2` Q4, the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog), and the logged causes are the other four. The `UnreachableCause` item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL. The `endpoint-lost` hint is `inputs#I2` Q3, delegate the overseer, PROVISIONAL.
**Kept:** `not-running` stays the hint cause's key, not renamed to the log's `instance-dead`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
