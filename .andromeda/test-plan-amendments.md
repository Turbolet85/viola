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
