# Report — 2026-09-24-fake-agent-and-test-data-fixtures

**Chunk:** fake agent scripted modes, control file and receipts, temp-home fixture chain, fixture scrub walk, property seeds
**Date:** 2026-09-24T08:47:00Z
**Commits:** since last_wrap (2026-09-24T07:23:46Z): `dc01bd9 chore(2026-09-24-three-os-ci-headless-harness-skeleton): record green CI witness`. This chunk's work is uncommitted and rides the wrap commit.

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --stat HEAD` + `git status --short`)
  - Modified:
    - `src/bin/viola-fake-agent.rs` (+628 lines)
    - `crates/viola-e2e/src/harness/run.rs` (+153)
    - `crates/viola-core/src/lib.rs` (+54)
    - `crates/viola-core/Cargo.toml`
    - `Cargo.toml`
    - `Cargo.lock`
    - `tests/run_cli.rs`
  - New:
    - `tests/support/{mod,home,fake,hygiene}.rs`
    - `tests/cli_fake_agent.rs`
    - `tests/contract_fixture_hygiene.rs`
    - `fixtures/fake-scripts/gated-turn.json`
    - `schemas/fake-script.v1.json`
    - `crates/viola-core/proptest-regressions/lib.txt`
- **Symbols / APIs:**
  - **Fake agent CLI surface (test-only bin, feature `fake-agent`):**
    - value flags: `--cli-version <v>` (default 2.1.0), `--report-version <v>` (changes only the `--version` answer), `--script <path>`, `--control <path>`, `--receipt <path>`, `--fixtures <dir>`, `--plugin-dir <dir>`
    - mode flags: `--suppress-prompt-submit`, `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof`
    - internal re-exec flag: `--hold-stdout-internal`
    - unknown arguments are ignored
  - **Fake agent receipt format** (ndjson, one `write_all` per line):
    - every line carries `"v":1` + kebab `kind`
    - `start` {cli_version}
    - `env` {names, sorted, NAMES only}
    - `fds` {fds} (Unix only)
    - `key` {hex} (one per byte outside a paste)
    - `prompt` {text, hex, bare_esc, origin human|harness, submit fired|suppressed|local-command|no-fixture|no-hooks}
    - `hook` {event, command_absolute, ran, + exit_code/stderr_len/stdout_hex when ran}
    - `step` {index, event}
  - **Fake agent behaviour:**
    - Bracketed paste `ESC[200~…ESC[201~` + CR = one prompt.
    - A fresh ESC that breaks a partial escape match restarts paste detection.
    - Hooks are read from `<plugin-dir>/hooks/hooks.json` (documented Claude Code hooks shape; every `type:"command"` entry for the event, matchers NOT evaluated).
    - The payload = the bytes of `<fixtures>/<cli-version>/<Event>.<variant>.json`, with only the top-level `prompt` key set for UserPromptSubmit (variant `default` for prompt-driven submits).
    - An absolute command is spawned directly with `args`. A non-absolute command is never run: `command_absolute:false, ran:false`.
    - `--inject-harness-turn` = one gated step before the script, firing a `<task-notification>`-prefixed UserPromptSubmit with `origin:"harness"`.
    - Scripts `{"v":1,"steps":[{event,variant,gate}]}` with events from the 9 registered hook events; an unreadable script exits 2.
    - A gated step waits for one more `\n`-terminated control line past the byte offset (agent-side 10 ms poll).
    - `--exit-no-eof` spawns a re-exec of itself holding the inherited stdout for a bounded 10 s, then exits 0.
  - **Harness `viola-e2e::harness::run` (pub):**
    - new `diff_paths`, `diff_files`, `rust_delta`
    - private `no_rust_delta`
    - `mutants()` now returns `(Suite, Value)` (sole caller `run()` @ `run.rs:82`, graph query in phase research)
    - the `run` document's `mutants` object is `{"tested":N,"verdict":"counted"}`, or `{"tested":0,"verdict":"no-rust-delta","diff":"target/agent-run/chunk.diff","files":N}` for a diff with no `.rs` path
    - a Rust-delta run removes `mutants.out/outcomes.json` before invoking cargo-mutants
    - the former `diff.trim().is_empty()` arm is removed (an empty diff is Rust-free)
  - **Root test support (sync, `tests/support/`):**
    - `TestHome` (tempdir prefix `viola-test-` under `<workspace>/target/e2e-home/`, home at `<dir>/home`, never pre-created; Drop keeps via `keep_decision(AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED, panicking)`)
    - rstest fixtures `home`, `fake_agent_path`, `stamped_home` (interim: `stamped:false`, writes no `ledger/stamps.json`), `booted_wrapper` (boots `builder`)
    - `Wrapper::boot(stamped, name, script, extra)` spawns `viola --home <home> run <name> -- <fake> --control <home>/fake/<name>.control --receipt <home>/fake/<name>.receipt.ndjson [--script <workspace-resolved>]`; readiness = interim `process-start` self + claude-child, bounded 20 s
    - hygiene checker `check(bytes, schema, user) -> Result<(), Violation{absolute-path|username|schema}>`
  - **Env vars:**
    - `AGENT_RUN_KEEP_FAILED` (NEW, test-only, read by `tests/support/home.rs`)
    - `AGENT_RUN_KEEP_HOMES` (existing; now also read by `tests/support/home.rs`, not only `viola-harness`)
    - `FAKE_CLAUDE_AGENTS_MODE` NOT introduced (deferred)
- **Crates / modules:** no crate added. New root test binaries `cli_fake_agent`, `contract_fixture_hygiene` (`[[test]]`, `required-features = ["fake-agent"]`); root `tests/support/` module. No `viola_e2e::fixtures` module (deferred).
- **Dependencies:**
  - `[workspace.dependencies]`: `jsonschema =0.57.0` (`default-features = false`), `proptest =1.11.0` (`default-features = false`, `features = ["std"]`), `rstest =0.27.0` (`default-features = false`)
  - root dev-deps: `jsonschema`, `rstest`
  - `viola-core` dev-deps: `proptest`
  - Cargo.lock: 62 new packages (`git diff HEAD -- Cargo.lock | grep '^+name'`). The `cargo tree --workspace -e all … | grep -cE '^(reqwest|rustls|aws-lc-rs|aws-lc-sys|ring) '` probe = 0.
  - No tokio added. No product (normal) dependency changed.
- **Schema / config:** new `schemas/fake-script.v1.json` (draft 2020-12, tolerant: unknown fields allowed, `v` const 1, closed 9-event enum). New committed `crates/viola-core/proptest-regressions/lib.txt` (proptest header, no seeds). No `config.json` key.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** none — verified. No master states a count this chunk moved.
- **Dev-tool versions:** none. cargo-mutants re-read at 27.1.0 (dev host, `cargo mutants --version`).
- **Harness / gate surface:** the `run --mutants` verdict shape gains the `verdict` closed enum (`counted` | `no-rust-delta`), plus `diff` and `files` on the no-rust-delta arm. No agent-run command or flag changed. `ci.yml` unchanged.
- **Cross-project / external claims:**
  - CI run 35973118026 on sha `dc01bd9cd426ab3659ad41a5656dff619dee0fd9`: `mutants` concluded `failure` (`outcomes-missing` on a Rust-free diff, the log's `INFO Diff changes no Rust source files`). All three `test` legs concluded `success`. This chunk folds that red.
  - The cargo-mutants releases page (github.com/sourcefrog/cargo-mutants/releases, v25.3.0) documents the "diff changes no Rust source files" message, but not its exit code or outcomes behaviour. The dev-host measurement is the basis: exit 0, and `mutants.out/` left untouched.
- **Reverted / negative API facts:**
  - A sentinel-`variant` encoding of the harness turn was written, then replaced by an explicit `harness` field on `Step` before gates ran.
  - `std::thread::sleep` in the chain's readiness probe was replaced by `yield_now` (no-sleep test rule).
- **Insufficient fixes (written, kept, not the remedy):** none
- **Spec claims disproved by measurement:**
  1. architecture.md:374 "`target/e2e-home/viola-session-*/home`: every harness and test home". The root rstest homes use the prefix `viola-test-*` (this chunk's `tests/support/home.rs`; the prior `tests/run_cli.rs::scratch` already did). Only harness sessions use `viola-session-*` (`crates/viola-e2e/src/harness/boot.rs:126`).
  2. architecture.md:359 "`AGENT_RUN_KEEP_HOMES` … read by `viola-harness` and never by `viola`": now also read by the root test chain (`tests/support/home.rs`). The "never by `viola`" half still holds.
  3. test-plan §3 `run` step 4 (test-plan.md:553-556) and §10 Mutation gate (test-plan.md:1445): the verdict reads `outcomes.json` for every non-empty diff. Measured false for a Rust-free diff: cargo-mutants 27.1.0 exits 0 and writes no `mutants.out/`, so CI read `outcomes-missing`, and a dev host read a STALE `outcomes.json` (P5 baseline: `ok:true "tested":8` on this chunk's docs-only untouched tree).
- **Expected amendments (from plan):**
  - test-plan §3 `run` step 4 (classifier, `verdict` enum, stale removal, `files`): **carried** (Harness / gate surface + Spec claims disproved 3). Sites: `grep -c outcomes.json` test-plan=8 (§3 run step 4 at :556, §10 at :1445, plus mentions), obs-plan=1 (:1237 lists the artifact only, no verdict claim: no change), others 0.
  - test-plan §12 Decisions Log (new closed value): **carried** (same fact). Site: test-plan §12.
  - test-plan §7 Fake agent (receipt kinds, hooks from `<plugin-dir>/hooks/hooks.json`, non-absolute command not run, `default` variant, three modes deferred): **carried** (Symbols / APIs). Sites: `grep -c -- --receipt` test-plan=2 · `vt100-panic-bytes` test-plan=1 · `statusline-echo` test-plan=3 · `FAKE_CLAUDE_AGENTS_MODE` test-plan=3; 0 in the other six masters.
  - test-plan §7 Fixture hygiene (`schemas/fake-script.v1.json`; `fixtures/claude` walk deferred): **carried** (Schema / config). Sites: `grep -c "Fixture hygiene"` test-plan=1 · `fake-scripts` test-plan=1; 0 elsewhere.
  - test-plan §3 `run` step 2 (`viola_e2e::fixtures` copy deferred to its first consumer): **carried** (Crates / modules). Site: `grep -c "exists twice"` test-plan=1; 0 elsewhere.
  - architecture §Occupied Resources + §Infrastructure Patterns directory tree (`fixtures/fake-scripts/`, `schemas/`, `crates/viola-core/proptest-regressions/`, `<home>/fake/<name>.control` / `.receipt.ndjson`, `target/e2e-home/viola-test-*/home`, `AGENT_RUN_KEEP_FAILED`): **carried** (Files, Env vars, Spec claims disproved 1-2). Sites: `AGENT_RUN_KEEP_HOMES` architecture=2 (:359 env list, :448 CI jobs) · `e2e-home` architecture=2 (:359, :374) · `proptest-regressions` architecture=0, test-plan=6 · `schemas/` architecture=0, test-plan=5, obs-plan=13, a11y-plan=4 (existing `schemas/` mentions are obs/a11y schema files; this chunk's file is a new one).
  - `matrix#v1-06` notes (Rust-free diffs pass with an explicit `no-rust-delta` verdict; stale `outcomes.json` removed before a Rust-delta run; fold of CI run 35973118026, sha dc01bd9): **ledger-note — owner P7.3**.
- **Coverage of new surfaces:**
  - fake-agent receipt (harness test format) → validation n/a (test double) · instrumentation n/a (test-only, not a diag line; never written under `diagnostics/`) · PII redacted✓ (env NAMES only; sentinel-value test) · tests unit+integ · a11y n/a · tokens n/a
  - fake-agent hook invocation → validation ✓ (absolute-path check; exec-form direct spawn, no shell) · instrumentation n/a · PII n/a · tests integ (decoy-on-PATH test) · a11y n/a · tokens n/a
  - `run --mutants` verdict arm → validation n/a · instrumentation n/a (harness JSON document) · PII n/a · tests unit (3, incl. two throwaway-workspace runs) · a11y n/a · tokens n/a
  - fixture hygiene checker → tests integ (walk + planted inputs per class) · rest n/a
  - `ViolaName` property test → tests unit (4 proptests, 512 cases) · rest n/a

## Deviations from intent
1. `Input::feed`: a fresh ESC that breaks a partial escape match restarts paste detection. This is not in the plan. A unit test showed that without it, a paste following a broken escape is read as keys.
2. The chain's readiness probe spins on `yield_now` under the 20 s deadline instead of an interval sleep, to keep the no-sleep test rule.
3. The `booted_wrapper` fixture boots the fixed name `builder`. Variations go through `Wrapper::boot(stamped, name, script, extra)`, since the rstest fixture takes no per-test arguments.
4. Step-15 test (c) builds a Rust delta with no fresh outcomes via an untracked `scripts/tool.rs` that no package builds (cargo-mutants finds no source and writes no outcomes). The planted stale file is removed and never read. This is a real end-to-end run, not a mock.
5. The injected harness turn also writes a `step` receipt (index 0, event UserPromptSubmit), which the plan implied rather than stated.

## Decisions & corrections
- **Operator (P1):**
  - Fold the CI mutants red into this chunk, not carry it unowned.
  - Constraint (verbatim): "a diff with no .rs files passes only with an explicit no-rust-delta verdict (the diff is checked and named in the output); a diff WITH .rs files and no outcomes stays red. Unit-test both."
- **Operator (P4), consumer-first, "no shapes invented before a recorded fixture; each deferral pinned as a CARRY on its consumer":**
  - core modes now; `--vt100-panic-bytes`, `statusline-echo`, the `agents --json` modes deferred
  - root chain copy only; the `viola_e2e::fixtures` copy deferred
  - property persistence + a header-only tracked regressions file
- **Measured:** cargo-mutants on a Rust-free non-empty diff exits 0 and leaves `mutants.out/` untouched. A harness that reads `outcomes.json` afterwards reads the previous run's counts (a stale false-green on any dev host).
- **Measured:** a `format!("{a}{b}")` inline capture inside `proptest!` fails to compile ("there is no argument named"). Macro hygiene blocks inline format captures, so build the string outside the format macro.
- **Sweep hazard:** the hygiene pattern `/home/` matches ANY path containing that segment (`relative/home/x` is a hit). This is correct per test-plan §7, but easy to mis-assert in a test.
- **Test race found:** a receipt test waited for the `start` line and asserted the `env` line written just after it. The rule is to wait on the line you assert.

## Outcome
- **Acceptance criteria**, re-asserted against the diff:
  - `run --unit` / `run --integration` `ok:true` on this host: MET. The CI legs are owed to the operator push (gates below).
  - Script order driven by control appends and read from `step` receipts, no sleep: MET (`fake_agent_gated_steps_wait_for_control_lines`, `chain_boots_the_fake_agent_under_viola_with_a_gated_script`).
  - Paste = one prompt, keys byte-exact incl. `ESC[I`/`ESC[O`: MET.
  - suppress, local-command and harness-turn modes: MET.
  - Absolute hook runs with `args` and carries `prompt`; the relative-command decoy never runs: MET.
  - Sentinel value never in the receipt: MET.
  - `--report-version` changes only the answer, replay follows `--cli-version`: MET.
  - `--exit-no-eof` exit recorded by handle-wait while stdout is held (piped): MET, with a control test that EOF arrives without the flag.
  - Chain homes under `target/e2e-home/`, keep decision over all 8 inputs, no `stamps.json`: MET.
  - Hygiene classes each proven, committed scripts pass, the walk is non-vacuous by `#[files]` construction: MET.
  - `ViolaName` agrees with the oracle over 512 cases, regressions file present: MET.
  - Mutation verdict arms both unit-tested: MET.
  - The chunk's own `run --mutants`: MET (below). The CI `mutants` success is owed to the operator push.
  - No reqwest, rustls, aws-lc or ring crate in the graph: MET.
- **Gates** (implement P2, run dir `.andromeda/runs/2026-09-24T08-27-34-implement`, full block after one fix iteration):

  | Gate | Verdict |
  |---|---|
  | `cargo build --workspace --features fake-agent` | green |
  | `cargo fmt --all --check` | green |
  | `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` | green |
  | `bash scripts/agent-run.sh run --unit` | green |
  | `bash scripts/agent-run.sh run --integration` | green |
  | `run --unit --filter 'test(/no_rust_delta\|outcomes_missing\|rust_delta/)'` | green (3 tests) |
  | `run --unit --filter 'test(/viola_name_prop/)'` | green (4 tests) |
  | `run --integration --filter 'binary(cli_fake_agent)'` | green |
  | `run --integration --filter 'binary(contract_fixture_hygiene)'` | green |
  | cleanup / boot / status / cleanup `--session gate-smoke` | green |
  | `cargo tree … \| grep -cE '^(reqwest\|rustls\|aws-lc-rs\|aws-lc-sys\|ring) '` | green, exit 1, last line 0 |
  | `grep -rnE "(std::)?env::set_var" tests crates/viola-e2e src` | green, exit 1, no output |
  | `git check-ignore -q target/e2e-home/viola-test-probe/home` | green |
  | `test -s crates/viola-core/proptest-regressions/lib.txt` | green |
  | `AGENT_RUN_CHUNK_BASE=dc01bd9… bash scripts/agent-run.sh run --mutants` | green: 81 mutants, 33 caught, 48 unviable, 0 missed, 0 timeout, `"verdict":"counted"`; `outcomes.json` fresh at 10:43:39 +0200 |
  | `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` | not run, leg operator: the operator pushes after this wrap's commit |
  | `gh api …/commits/$(git rev-parse HEAD)/check-runs …` | not run, leg operator: read after that push |
- **Smoke:** boot path changed (boot copies the fake agent). Session `impl-smoke` with two instances reached `ready`, cleanup `processes_gone:true`, 4 pids verified gone.
- **Outcome basis:** implement's P4 report as given in this session's conversation. No operator directive between implement and this report.
- **Process hygiene:**
  - Sessions `gate-smoke` and `impl-smoke`: terminated (harness cleanup; Get-Process confirms the impl-smoke pids are gone).
  - `viola` 12172 and 10348 are the operator's viola-lab prototype (`run viola-builder` / `wait viola-builder`), not this run's: left running, the operator's.
