# tests extract

## Relevance
relevant — the chunk lands the `viola hook` surface (a §1 entity and cli surface), the third Path 1 event, the hook fail-open matrix, two property/fuzz parser surfaces and the first `run --perf` arm; tier is Comprehensive (per test-plan §1 Test tier).

## Constraints
- The hook surface's signal is fixed: exit always 0, stderr always empty, stdout empty or exactly a decision body, and side effects are checked as an `events.ndjson` line plus a hook trace under `diagnostics/` (per test-plan §1 Surfaces under test, cli `viola hook`). Root `tests/hook_*.rs` must cover every event name in the arch Hooks list: exit 0, empty stderr, empty stdout for async hooks, and an immediate exit 0 when `VIOLA_NAME` is absent. These `hook_*` binaries run in the integration layer, not E2E (per test-plan §5 Cross-module patterns, Hooks; §6 Non-path suites preamble). Which events register in this chunk and which wait for the dialog entry is research's question. The test list follows that answer.
- The fail-open matrix lives in root `hook_fail_open.rs` (assert_cmd). Its cases are oversize stdin, malformed JSON, clap error, channel failure, a forced panic and a missing `VIOLA_NAME`, plus the `--home` strict-modes refusal (hook exit 0, no output). The forced-panic trigger compiles only under feature `fake-agent`, and its name is a row in `cli_controls_not_disableable.rs` (per test-plan §1 Coverage triggers, hook stdin; §6 Non-path suites, Security sweep; §5 On-disk strict-modes refusals). The server pid / start-time mismatch case must also give hook exit 0 with no body (per test-plan §5 Wrapper channel bullets; §1 Coverage triggers, IPC).
- `prompt-submitted` normalisation is a §4 unit case in `viola-agent-claude`. The M2 prefixes `<agent-message from=` / `<task-notification>` must give `origin:"harness"`, and both M3 `<pasted_content id=…>` and M4 `<\pasted_content` must normalise. The hook payload parse is a `#[files]` walk over `fixtures/claude/<ver>/`, and unknown fields are counted, never fatal (per test-plan §4 What unit tests cover, viola-agent-claude). The walk depends on recorded fixtures, which land with :51. Whether any `fixtures/claude/<ver>/` set exists today is research's question.
- The property suite requires proptest 1.11.0 at `cases: 512` with committed `proptest-regressions/`, covering (a) the hook stdin parser, statusline JSON with arbitrary `resets_at` included, and (b) the `prompt-submitted` normalisation round-trip. Each parser target joins cargo-fuzz in `fuzz/fuzz_targets/<parser>.rs` with a committed, non-empty `fuzz/corpus/<target>/`, seeded from the §7 recorded payloads it accepts or synthetic seeds where none applies (per test-plan §6 Property suite; §3 `run` `--fuzz-replay`; §2 Property-based row).
- The concurrent-append check requires separate processes appending at once to `hook-<name>.ndjson` (parallel hook processes) and to the shared `detail-hook` file, with at least one detail line over 4 KiB, on all 3 OSes. Every line must parse as one JSON object (per test-plan §5 Boundary types, Module ↔ DB; §12 obs D-21 items).
- `run --perf` must follow the §10 perf run rules:
  - Build and home: it uses its own release build into `target/perf`, never with `LLVM_PROFILE_FILE` set. Two booted sessions live under `target/e2e-home/`: `perf-stamped-<pid>` and `perf-unstamped-<pid>`.
  - Invocation: hyperfine 1.20.0 `-N --warmup 3 --runs 30 --export-json target/agent-run/artifacts/perf-<hook>.json` with `VIOLA_NAME`/`VIOLA_DIR` exported and the fixture on `--input`, so no timed hook takes the no-instance no-op.
  - Checks: zero `event:"panic"` lines in the role files before `cleanup`, and a missing tool gives `reason:"tool-missing"`.
  - Gate: on `max` only, `max < 1.0 s` provisional until arch names the spine-deadline constant.

  (Per test-plan §10 Performance budgets; §3 Bootstrap `ci-tool-install`; §3 `gate`.) Research has two questions here. First, whether the stamped session can boot before `viola verify` exists (:51). Second, whether the `pre-tool-use` unverified row (`hook.dialog`) belongs here or at :62.
- Harness grammar grows per chunk: until its chunk lands, `--perf` is a usage error, never a vacuous pass. `boot` readiness adds `events.ndjson` line 3 `kind:"session-start"`, `source:"hook"` when this chunk lands (per test-plan §3 Exit codes preamble; §3 `boot` Readiness signal).

## Patterns to follow
- Hook-level tests use assert_cmd against `CARGO_BIN_EXE_viola` with fixture stdin and a `std::time::Instant` deadline bound, which asserts the §10 deadlines at unit speed. Env goes per child through `Command::env`. Any `.env_clear()` re-adds `LLVM_PROFILE_FILE` (per test-plan §6 Drivers per surface; §10 closing paragraph; §7 Test data lifecycle).
- The fake agent runs every `type:"command"` hooks.json entry by absolute path only. A non-absolute command reads `command_absolute:false, ran:false` in the receipt. For UserPromptSubmit, only the top-level `prompt` key is set on the fixture payload. The M6 witness is read from that receipt (per test-plan §7 Fake agent).
- Naming and layout:
  - integration binaries use the `hook_` prefix, and test functions follow `<subject>_<condition>_<expected>` (e.g. `hook_oversize_stdin_exits_zero_silent`);
  - table-driven cases use rstest `#[case::label]`;
  - fixtures are `<Event>.<variant>.json`, keyed by the CLI's PascalCase event name, with no mapping table.

  (Per test-plan §2 Test directory + naming conventions; §4 Conventions.)
- On-disk state is produced only by real `viola run` / `hook` against the fake agent in a fresh home under `target/e2e-home/`. Faults come only from explicit process or file operations. That fits the CARRY 5 witness: a real handle holding `snapshot.json` open across the replace, which must be red on today's code (per test-plan §7 Seed strategies; §11 Universal, deterministic replay; §10 Zero-flakiness budget).
- Mutation scope: a mutant in `viola-agent-claude` / `viola-core` / `viola-state` must be killed by that crate's own tests. The `viola-e2e` scenarios never run under mutation. The chunk diff is mixed, so the verdict is `counted` (per test-plan §3 `run` step 4, Test scope and Classification).

## Anti-patterns to avoid
- NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote. Never parse the rendered child screen, and never use `sleep` for synchronisation: waits key on `events.ndjson` offsets (per test-plan §11 Mocking, §11 E2E).
- NEVER import the product's own tables as the oracle. Write the M2 prefixes, the hook-to-kind map, and the literal and wrapper forms as literals in the test. NEVER hand-write `snapshot.json` or `budget.json` (per test-plan §11 Unit, §11 Test Data).
- NEVER gate hook deadlines on the hyperfine `mean`, retry a perf breach, or pass `gate --require perf` with a missing `perf-*.json` (per test-plan §11 Quality; §10 Verdict; §3 `gate` breaches).

## Contract bindings
- tests ↔ obs:
  - obs owns the role-file and `detail-hook.ndjson` line schemas (`diag-line.v1.json`, and `diag-detail.v1.json` incl. `drift_report`);
  - tests own the G4 schema-conformance check body and the canary scan: a canary in hook stdin must never reach home-level `diagnostics/`;
  - the perf zero-panic assertion reads obs role files, and the obs-overhead row requires perf to take the real logging path.

  (Per test-plan §6 Schema conformance / Error sanitization; §10 Perf session.)
- tests ↔ security: Vector 4 (hook stdin fail-open, strict-modes before `statusline_command`) is realised by `hook_fail_open.rs` and the security-negatives Unix-socket-dir case, where `viola hook` exits 0 with no body and no frame (per test-plan §1 Coverage triggers; §6 Security control negatives).
- tests ↔ arch: the spine-deadline constant (arch request 3) replaces the provisional `1.0 s` in `gate`, and the hook-to-kind map is the §5 Hooks event list the tests enumerate (per test-plan §10 Spine deadline).

## Acceptance criteria contributions
- Path 1 and `boot` readiness:
  - `events.ndjson` records 1–3 are `wheel{cause:"start"}` → `budget-gate` → `session-start{source:"hook"}`;
  - after a restart, `plugin/` hooks.json holds no sentinel and every hook `command` is the absolute `pinned_bin` path (M6, via fake-agent receipt `command_absolute:true`);
  - harness `boot` readiness requires line 3.

  (Per test-plan §6 Scenario Path 1; §3 `boot` Readiness signal.)
- Fail-open and E1:
  - each `hook_fail_open.rs` case exits 0 with empty stdout and empty stderr: oversize stdin, malformed JSON, clap error, channel failure, forced panic, missing `VIOLA_NAME`, strict-modes;
  - every hook event with no `VIOLA_NAME` exits 0 silent and creates no `instances/` dir.

  (Per test-plan §6 Non-path suites, Security sweep; §6 Scenario E1.)
- Property and fuzz: the hook stdin parser and `prompt-submitted` round-trip properties pass at `cases: 512` with `proptest-regressions/` committed. `run --fuzz-replay` passes over each new target's non-empty seeded corpus, and the concurrent-append check with a >4 KiB `detail-hook` line passes on all 3 OSes (per test-plan §6 Property suite; §3 `--fuzz-replay`; §5 Module ↔ DB).
- Gates:
  - `run --perf` writes `perf-<hook>.json` per timed hook, and `gate --require perf` reports no breach (`max < 1.0 s`, zero panic lines);
  - the chunk passes coverage (lines 85 / functions 95 / regions 80, per OS) and a counted mutation union with `missed == 0`, `timeout == 0`, `unviable <= caught`.

  (Per test-plan §10 Performance budgets / Coverage thresholds / Mutation gate.)
