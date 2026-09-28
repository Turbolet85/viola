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
