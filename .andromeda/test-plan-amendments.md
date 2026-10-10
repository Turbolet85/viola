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
