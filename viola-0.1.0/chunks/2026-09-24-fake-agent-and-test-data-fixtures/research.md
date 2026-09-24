# Codebase Research — 2026-09-24-fake-agent-and-test-data-fixtures

## Scope
- **Depth:** moderate · **Reads:** 11 (run.rs ×2 windows, viola-core lib.rs, tests/run_cli.rs, 3 manifests, nextest.toml,
  .gitignore, cookbook, test-plan §3 run lines 530-561) · **Globs/Greps:** 6
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 0 Session Additions entries;
  `.claude/rules/testing.md` — read in full, 1 Session Additions entry applied (every function needs a test-observable
  effect under the zero-missed gate; cargo-driving tests use a throwaway temp project named `viola`).
- **Platform issues consulted:** (filled at P5, check 9) query `cargo-mutants "Diff changes no Rust source files"
  in-diff outcomes.json`. The fetched cargo-mutants releases page (github.com/sourcefrog/cargo-mutants/releases)
  states under v25.3.0: "A specific clearer error if a valid non-empty diff changes no Rust source files, and so
  matches no mutants". It documents neither that case's exit code nor whether `mutants.out/`/`outcomes.json` is
  written. Under v26.0.0: "`start_time` and `end_time` fields in `outcomes.json`". The upstream note therefore
  describes a message, not the verdict contract; the host measurement (exit 0, `mutants.out/` untouched) is the
  basis for step 15. No runner-specific issue is involved: the failure reproduces on this host with the same
  tool version (cargo-mutants 27.1.0).

## Files inspected
- `src/bin/viola-fake-agent.rs` (full, 30 lines) — the first slice: `cli_version` (`--cli-version`, default `2.1.0`),
  `--version` → `"{ver} (Claude Code)\n"`, else reads stdin byte by byte until `\x03` or EOF. No `#![allow]` yet, no
  receipt, script or control handling.
- `crates/viola-e2e/src/harness/run.rs` (1-330, 330-440, 500-640) — `run()` builds the JSON document
  (`run.rs:92-100`); `mutants()` (`run.rs:363-419`) writes `target/agent-run/chunk.diff`, prebuilds the root package,
  runs cargo-mutants, then reads `mutants.out/outcomes.json`; the only no-outcomes pass arm is
  `Err(_) if diff.trim().is_empty()` (`run.rs:409`); otherwise `failures:["outcomes-missing"]` (`run.rs:410-417`).
  Unit tests pin `mutants_suite` (`run.rs:535-544`), `mutants_exit_reason` (`run.rs:518-532`), `chunk_diff`
  (`run.rs:606-615`) with a throwaway `git_repo()` helper (`run.rs:568-588`).
- `crates/viola-core/src/lib.rs` (full) — `ViolaName` nutype (`lib.rs:9-13`) with `is_valid_name` (`lib.rs:15-25`):
  ASCII lowercase first byte, ≤32 bytes, `[a-z0-9-]`. Table tests exist (`lib.rs:27-89`); no proptest.
- `tests/run_cli.rs` (full) — `VIOLA`/`FAKE` via `env!("CARGO_BIN_EXE_…")` (`run_cli.rs:10-11`), `run_viola`
  (`run_cli.rs:13-28`), `scratch()` = `tempdir_in(<manifest>/target/e2e-home)` with prefix `viola-test-`
  (`run_cli.rs:31-39`); the home passed to viola is `tmp.path().join("home")` — not pre-created (good).
  `fake_agent_stops_at_ctrl_c_and_reads_no_further` (`run_cli.rs:224-248`) uses a bounded yield loop, not a sleep.
- `Cargo.toml` (root) — `[features] fake-agent = []`; `[[bin]] viola-fake-agent` `required-features`; `[[test]]
  run_cli` `required-features = ["fake-agent"]`; dev-deps only `serde_json`, `tempfile`. No `viola-e2e` dev-dep.
- `crates/viola-e2e/Cargo.toml` — deps clap/serde/serde_json/sysinfo/tempfile/thiserror/viola-core; **no tokio yet**;
  own `[lints.rust]` (no workspace print bans). `fake-agent = []` no-op feature.
- `crates/viola-e2e/src/harness/boot.rs` (grep) — `InstanceSpec` parses `<name>[:<fake-agent-args>]`
  (`boot.rs:31-37`); boot copies `viola-fake-agent` from `bin_dir` to the session's `claude` (`boot.rs:120-121`).
- `.config/nextest.toml` — `[profile.mutants]` terminate-immediately + 15 s × 2 slow-timeout; `fixed-port` group.
- `.gitignore` — `target/` covers `target/e2e-home/` and `target/agent-run/`; `mutants.out/` ignored.
- `.andromeda/test-plan.md:530-561` — §3 `run` steps 1-4 (chain-twice rule at :542, mutation verdict at :553-557).
- No `.cargo/mutants.toml` and no `deny.toml` exist at HEAD (`ls .cargo` → absent; `ls deny.toml` → absent).

## Graph impact (rust plane, regenerated: 387 nodes / 1551 edges; trace `tree-query-{marker}.json`; lines +1)
- **mutants** — 1 caller: `run()` @ `crates/viola-e2e/src/harness/run.rs:82`. Private; changing its verdict logic
  threads nowhere else.
- **mutants_suite** — `mutants()` @ `run.rs:407`, `run.rs:409`; test @ `run.rs:537`, `run.rs:540` (pub, same file only).
- **chunk_diff** — `mutants()` @ `run.rs:368`; test @ `run.rs:610`, `run.rs:614`.
- **scratch** — 6 sites, all `tests/run_cli.rs` (`:64 :122 :133 :144 :161` + one crate-level ref `:237`) — the
  migration set onto the root chain.
- **run_viola** — 7 sites, all `tests/run_cli.rs`.
- **cli_version** — 1 caller: `main()` @ `src/bin/viola-fake-agent.rs:19`.
- **crate_edges** — `viola → viola-core`, `viola-e2e → viola-core`; nothing depends on `viola-e2e` (leaf test crate).

## Patterns detected
- **No-vacuous-pass by construction** (`run.rs:553` letter, `run.rs:363-371`): a missing base or diff is a typed
  `reason`, never an empty diff handed to the tool. Item 7 extends the same shape to the Rust-free diff.
- **Counts decide, exit only gates** (`run.rs:326-332`, `run.rs:399-401`): exit 0/2/3 defer to counts; others fail.
- **Throwaway git/cargo projects in unit tests** (`run.rs:568-588`, `run.rs:621-640`): cargo/git-driving tests never
  touch this workspace (testing.md Session Addition).
- **Per-child env only** (`run_cli.rs:19`, `boot.rs` PATH per child `boot.rs:280`): `Command::env`, never `set_var`.
- **Bounded observation without sleep** (`run_cli.rs:231-238`): a deadline loop with `yield_now`.

## Conventions to follow
- **Test names** `<subject>_<condition>_<expected>` (testing.md §Naming; e.g. `run_cli.rs:161`).
- **Root integration binaries** need `[[test]] … required-features = ["fake-agent"]` when they spawn the fake agent
  (`Cargo.toml` `[[test]] run_cli`).
- **Harness JSON document** `{"v":1,"cmd":"run","ok":…,"suites":[…],"mutants":{"tested":N}}` (`run.rs:92-100`) —
  item 7's verdict lands in this document.

## Mechanism equalities (verified at HEAD)
- **Item 7, face 1 (CI):** for `chunk.diff` = a non-empty diff touching no `.rs` path, cargo-mutants 27.1.0 prints
  `INFO Diff changes no Rust source files`, exits **0**, and writes no `mutants.out/` → `read_json` errs, `diff` is
  non-empty → `outcomes-missing`, `ok:false`. Witness: run 35973118026 job `mutants` log (sha `dc01bd9`,
  `AGENT_RUN_CHUNK_BASE=966b7aa…`), and local repro: `cargo mutants --workspace --features fake-agent --in-diff
  <14-line diff of viola-0.1.0/verification-matrix.json 966b7aa..dc01bd9>` → exit 0, same INFO line.
- **Item 7, face 2 (dev host):** the same local run left `mutants.out/outcomes.json` untouched (mtime 09:57:35 +0200,
  from an earlier run; `lock.json` 09:56:30) → on any host with a prior `mutants.out/`, `mutants()` would parse the
  stale file and return the EARLIER run's counts. So the verdict must be decided by the harness's own classification of
  `chunk.diff` (Rust paths present or not) BEFORE the tool runs, and a Rust-bearing diff must only accept an
  `outcomes.json` produced by this invocation.
- **Chain location:** test-plan.md:542 fixes two copies; the root `Cargo.toml` has no `viola-e2e` dev-dep and must not
  gain one (viola-e2e becomes tokio-based with rmcp/reqwest per test-plan §3 test-runner-install).

## New files to create
- `tests/support/mod.rs` + `tests/support/{home.rs,fake.rs}` — the sync root chain (`home`, `fake_agent_path`,
  `stamped_home` interim seam, `booted_wrapper`) and fake-agent helpers (control append, receipt read)
  (test-plan §2 names `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs}`; `outer_pty.rs`/`events.rs` have no
  consumer yet).
- `crates/viola-e2e/src/fixtures.rs` — the `viola_e2e::fixtures` copy of the chain.
- `fixtures/fake-scripts/<scenario>.json` — ≥1 synthetic scenario script (feeds the scrub walk's non-vacuity).
- A tolerant JSON schema for fixture/script files (location per arch tree — P4 decides).
- Root integration tests for the fake agent (e.g. `tests/cli_fake_agent.rs`) and the scrub walk
  (e.g. `tests/contract_fixture_hygiene.rs`).
- `crates/viola-core/proptest-regressions/` — per proptest's default `SourceParallel` persistence.

## Files to modify
- `src/bin/viola-fake-agent.rs` — the §7 contract (script, control, receipt, modes, hook reader, agents --json).
- `crates/viola-e2e/src/harness/run.rs` — `mutants()` verdict: classify `chunk.diff`; no-rust-delta verdict naming the
  diff; stale-outcomes guard; unit tests for both arms (callers: only `run()` @ `run.rs:82`).
- `crates/viola-e2e/src/lib.rs` — `pub mod fixtures;`.
- `crates/viola-e2e/Cargo.toml` — rstest (+ tempfile present) for the fixtures module.
- `crates/viola-core/Cargo.toml` — `[dev-dependencies] proptest`; `crates/viola-core/src/lib.rs` — the proptest.
- `Cargo.toml` (root) — `[workspace.dependencies]` rstest `=0.27.x`, proptest `=1.11.0`, jsonschema `=0.57.0`
  **with `default-features = false`**; root dev-deps; new `[[test]]` entries with `required-features`.
- `Cargo.lock` — new dev-deps.
- `tests/run_cli.rs` — migrate the 6 `scratch()` sites onto the root chain (`scratch`: 6 hits · 6 changed).
- `outcomes-missing` sweep (`grep -rn outcomes-missing` excluding `.andromeda/runs`): 1 hit (`run.rs:413`) · kept.

## Open questions
- jsonschema 0.57.0's DEFAULT features are `resolve-http, resolve-file, tls-aws-lc-rs, idna` (`cargo info
  jsonschema@0.57.0`): `tls-aws-lc-rs` pulls `aws-lc-rs` (a C-building crate, banned by the arch "no C-building
  crates" invariant) and `resolve-http` pulls reqwest. → blocks: plan-decision (decisive lean: `default-features =
  false`; verify with `cargo tree -e normal,dev -i` at /implement). `deny.toml` does not exist yet, so `cargo deny
  check` cannot witness it this chunk.
- proptest writes `proptest-regressions/` only when a case FAILS; with a correct `ViolaName` there is nothing to
  commit. → blocks: plan-decision (how "committed property seeds" is satisfied without a vacuous empty dir).
- The fake agent under the zero-missed mutation gate: every mode body in `src/bin/viola-fake-agent.rs` needs a root
  test that observes its effect (testing.md Session Addition) — `--vt100-panic-bytes`, `--inject-harness-turn` and
  `statusline-echo` have no product consumer yet. → blocks: plan-decision (build all §7 modes each with a direct
  observing test, or defer consumer-less modes).

## Extract overrides (poisoned-by-premise)
- arch extract Constraint "Code placement" ("root `tests/support/` may reach `viola_e2e::fixtures` only as a
  dev-dependency") leaned on scope item 4's withdrawn wording → OVERRIDDEN by test-plan.md:542 (two copies, no root
  dev-dep on `viola-e2e`).
