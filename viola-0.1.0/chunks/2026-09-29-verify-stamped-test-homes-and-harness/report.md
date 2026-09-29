# Report — 2026-09-29-verify-stamped-test-homes-and-harness

**Chunk:** Verify-stamped test homes and harness. Test homes and harness boot are stamped through viola verify. The chunk also lands recorded-fixture readiness, contract ledger probes, the local-live run, the fake agent at the recorded version, and a fold of the 31dd995 hook_fail_open unreachable-endpoint red.
**Date:** 2026-09-29
**Commits:** since `last_wrap` 2026-09-29T06:31:35Z, one commit: `702a3b4 chore(2026-09-29-verify-stamped-test-homes-and-harness): operator pre-CI commit, for the run this chunk's verdict reads`. Base: its parent `31dd995` (`git log --format='%h %s' 31dd995..HEAD`).

## Changes (structured — detectors read this)
- **Files** (basis `git diff --name-only 31dd995 -- src crates tests schemas` plus the docs; 32 source, test and schema files, +898/−122):
  - Product: `src/bin/viola-fake-agent.rs`, `src/cmd/verify.rs`, `src/run/version_gate.rs`,
    `crates/viola-agent-claude/src/{lib,hook,ledger}.rs`, `schemas/diag-line.v1.json`.
  - Harness (viola-e2e): `src/harness/{boot,supervise,run,pre_push}.rs`,
    `src/harness/run/{perf,browser,coverage,fuzz,mutants}.rs`, `src/bin/viola-harness.rs`, `tests/{cli,harness_lifecycle}.rs`.
  - Root tests: `tests/support/{home,fake}.rs`; `tests/{channel_endpoint,cli_fake_agent,cli_instance_state,cli_verify,cli_version_gate,contract_diag_schema,hook_events,hook_fail_open,run_cli,tui_passthrough}.rs`; new `tests/contract_ledger_probes.rs`.
  - Docs: `.claude/docs/gotchas.md` (a new "stopped wrapper's pipe" entry), `.claude/docs/services/viola.md` (two bullets).
  - Chunk: `scope-record.md`, `evidence/{item8-witness-guard,entry-6-freshness-plan-defect,operator-pass}.md`.
    `plan.md` gate 6 is retargeted at this wrap (below).
- **Symbols / APIs:**
  - **Fake agent.** `src/bin/viola-fake-agent.rs` `DEFAULT_CLI_VERSION` moved `"2.1.0"` → `"2.1.283"`, the recorded
    set's version; the fake agent's default `--version` answer is now `2.1.283 (Claude Code)`. The harness constant
    `viola_e2e::harness::boot::DEFAULT_CLI_VERSION` moved the same way (callers: `viola-harness` `boot --cli-version`
    default, `run/perf.rs`).
  - **Root test seam** (`tests/support/`):
    - `fake::RECORDED_CLI_VERSION = "2.1.283"` is the root chain's one copy, a test literal.
    - `home::stamped_home` now runs `viola --home <h> verify -- <fake> --cli-version 2.1.283 --fixtures <ws>/fixtures/claude`
      through `support::verify::verify`. `stamped` is true only on exit 0 plus a last line
      `stamped 2.1.283  <n> pass  0 fail`; otherwise the fixture panics with exit and match codes. The interim
      `stamped:false` seam is gone.
    - `StampedHome::unstamped(TestHome)` is new: `stamped:false`, writes nothing. Constructed by `cli_version_gate`,
      `hook_events`, `hook_fail_open` and `cli_fake_agent`'s missing-program case.
    - `Wrapper::boot` always passes `--cli-version 2.1.283` before per-test extras. `--fixtures` stays per-test.
    - `Wrapper::stop` / `stop_keep` now also wait until the snapshot's recorded `endpoint` is unconnectable, after the
      wrapper's exit.
    - New `home::unconnectable(endpoint)` is a root copy of the harness `cleanup::unconnectable` rule: Windows
      `viola_channel::Client::connect` → `Connect(NotFound)`; Unix, socket path absent.
    - New `home::wait_endpoint_gone(endpoint, label, observe)`: polls, hands each reading to `observe`, notes through
      `Watch`, fails at `WITHIN` (7 s), never sleeps. The root waits on a child are now 9 (the 8 before, plus this one).
  - **Error fold** (`viola-agent-claude`):
    - `AgentError` moved from `hook.rs` to the crate root (`lib.rs`) and gained the variant `StampsMalformed`
      (`Display` "the capability stamps are malformed").
    - `StampError` is removed (`! grep -rn StampError crates src tests` → exit 0, no output).
    - `ledger::verified` → `Result<bool, AgentError>`. Its one production caller is
      `src/run/version_gate.rs::stamps_verdict`, which maps `AgentError::StampsMalformed | AgentError::Malformed` →
      `(false, Some("malformed"))`. The outcome set is unchanged.
    - The crate still also holds the thiserror enum `Refusal` (`lib.rs`: `BatchScriptChild`, `NotFound`, from
      `resolve_program`), which predates this chunk.
  - **`viola verify` spawn pairs** (`src/cmd/verify.rs`):
    - A private `run_logged(subject, …)` wraps `run_bounded`: a `process-start{subject}` before, then
      `process-exit{subject, child_exit_status, duration_ms}` after.
    - It wraps the `--version` read as `subject:"version-probe"` and the print-mode probe as `subject:"verify-probe"`
      (a NEW subject value).
    - `run_bounded` itself stays unlogged. `run`'s version gate keeps exactly one `version-probe` pair. Without
      `VIOLA_NAME` no subscriber is installed and both are no-ops.
    - With an instance, `cli-<name>.ndjson` reads: `process-start{self}`, a version-probe pair, a verify-probe pair,
      `process-exit{self}`.
  - **Harness `boot`** (`viola-e2e`):
    - `BootOptions.stamp: bool`, and clap `boot --unstamped` (default: stamp).
    - Step 4 is `stamp()`, between the fake-agent copy and the `supervise.json` write. It runs
      `<bin dir>/viola --home <home> verify -- <session bin>/claude --cli-version <v> --fixtures <ws root>/fixtures/claude`
      with stdin null, stderr null, stdout read for the verdict only, and PATH = `session_path`.
    - Step 4 passes only on exit 0 plus the new parser `pub fn stamped_line(stdout)` (last line
      `stamped <non-space version>  <digits> pass  0 fail`). Anything else returns
      `{"v":1,"cmd":"boot","ok":false,"reason":"verify-failed","instance":null,"exit_code":<verify exit|null>,"missing":[]}`,
      exit 1, with no supervisor started.
    - New `pub fn start_records(instance_dir)`, a readiness stage after snapshot, endpoint and heartbeat:
      `events.ndjson` complete lines 1-3 are `wheel{data.cause:"start"}` → `budget-gate` →
      `session-start{source:"hook"}`. The missing code is `<name>:events` (NEW).
  - **Harness `supervise`:** `spawn_wrapper` passes `--fixtures <ws root>/fixtures/claude` after `--cli-version`, so
    each booted fake agent fires the plugin's SessionStart hook.
  - **Harness `run`** (`viola-e2e`):
    - `Selection.local_live` and clap `run --local-live`. `run_with(ws, sel, filter, chunk_base, ci: bool, runner)`
      gained the `ci` parameter; the bin reads `std::env::var_os("CI").is_some()` and `run::run` does the same.
      Callers updated: `pre_push.rs`, and the `run/{browser,coverage,fuzz,mutants,perf}.rs` tests (6 companion
      files, `ci:false`).
    - Refusal: `--local-live` with `ci` returns `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before
      any build, spawn or suite.
    - Without `CI`, after the selected suites, it runs `cargo build --workspace --features viola/fake-agent` into
      `target/harness` through the runner seam. It then runs one `<harness bins>/viola --home
      target/e2e-home/viola-live-<pid>/home verify` (the real `claude`, program default, no `--`) through the runner
      seam.
    - Suite `local-live` (NEW suite name) passes 1 when verify exits 0, every literal row id (six, `LEDGER_ROWS`)
      names exactly one step line ending `  pass`, and `stamped_line` holds.
    - Otherwise it fails 1 with one closed failure code: `build`, `verify-exit-<n>`, `verify-exit-none` (no exit
      code) or `row-missing`.
- **Crates / modules:** none added or removed. A new root integration binary: `tests/contract_ledger_probes.rs`.
- **Dependencies:** none.
- **Schema / config:** `schemas/diag-line.v1.json` `$defs.subject.enum` gains `"verify-probe"` (now
  `self|claude-child|version-probe|verify-probe|agents-probe|statusline-shell`). No config key and no env var for the
  product. The harness reads `CI` (presence only) to refuse `--local-live`; that is test-side, test-plan.md:561.
- **Spec-master edits:** none (implement and the operator pass touched no master).
- **Counts / qualifiers moved:**
  - The root waits on a child sharing `WITHIN` go from 8 to 9 (test-plan.md:542 "all 8 root waits"; basis: the new
    `wait_endpoint_gone` in `tests/support/home.rs`).
  - The fake agent's `DEFAULT_CLI_VERSION` goes 2.1.0 → 2.1.283 (test-plan.md:1379).
  - Harness boot readiness goes from 3 stages to 4 (events lines 1-3; test-plan.md:526 "`boot` checks lines 1–2 today").
- **Dev-tool versions:** none.
- **Harness / gate surface:** `boot --unstamped`; boot step 4 and `verify-failed` (reason already listed at
  test-plan.md:529); readiness `<name>:events`; `run --local-live`, `reason:"live-in-ci"`, suite `local-live` and its
  four failure codes; `supervise --fixtures`; `run_with`'s `ci` parameter. The shims `scripts/agent-run.{sh,ps1}` are
  unchanged (they forward flags). No CI step changed (`grep -c -- '--local-live' .github/workflows/ci.yml` → 0).
- **Cross-project / external claims:**
  - CI: ci#36538832471 measured `702a3b466ef3`: `verdict: green · checks 15/15 · wall 248 s` (`ci.py conclusion`,
    polled 9× over 250 s; overseer-verified).
  - Per-OS job logs (`gh run view 36538832471 --log`) read PASS on ubuntu-latest, windows-2025 and macos-latest for
    `case_08_unreachable_endpoint`, `stop_wait_holds_while_the_endpoint_answers` and
    `contract_ledger_probes_pass_over_every_recorded_set`.
  - The folded red came from ci#36532038635 on `31dd995`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes:** item 8's fix is test-side and does not close the product window. `viola hook` still reads a
  `hook.event` written into an exiting wrapper's pipe as delivered, so `SessionEnd`'s direct-append fallback is
  skipped in that window. It is documented in `.claude/docs/{gotchas.md, services/viola.md}` and not owned by any
  entry yet.
- **Spec claims disproved by measurement:**
  - (1) "a stopped wrapper's endpoint is gone once its exit is observed" is the implicit premise of the root
    `stop_keep` and of `hook_fail_open` case_08's `Env::Stopped`. ci#36532038635's kept home `viola-test-B3yvcs`
    shows the stopped wrapper's own pipe accepting the test's hook 25 ms after its `process-exit` line; research.md
    §Item 8. Why the pipe outlives the observed exit is NOT established.
  - (2) The overseer's hypothesis "a pipe-name collision with a concurrent test" (scope.md §Take-up direction) is
    FALSIFIED: the endpoint name carries the home (`crates/viola-channel/src/endpoint.rs:29-37`), and the hook reached
    its own home's endpoint.
  - (3) The plan's acceptance says "`viola-agent-claude` defines one thiserror enum, `AgentError`". The crate
    holds two: `AgentError` and the pre-existing `Refusal` (lib.rs, the `resolve_program` refusal). Arch's one-enum
    wording (architecture.md:97, :155, :641) names `StampError` as the only exception and does not mention
    `Refusal`, while architecture.md:461 lists `Refusal` among the crate's exports.
  - (4) Plan gate 6's `artifact = 'target/agent-run/artifacts/'` targets a directory. `gate.py` judges its mtime, which
    NTFS does not move when `run` overwrites the files in it: STALE on 3 of 3 runs while the files were fresh
    (`evidence/entry-6-freshness-plan-defect.md`).
- **Expected amendments (from plan)** — searches by `sweep_expected.py` over the seven masters, as hits@lines:
  - test-plan §3 `run` step 2, the interim seam and the "8 root waits": carried (Symbols: stamped seam, 9 waits).
    Search `interim|8 root waits`: test-plan 4@542,544,964,1935 (542 is the site; 544 is harness-`cleanup` `null`
    fields, a different fact; 964 and 1935 are to be read by the detector), architecture 4@97,155,397,641,
    security-plan 3@207,234,719.
  - test-plan §3 Readiness signal, "boot checks lines 1–2 today": carried (Symbols: `start_records`). Search
    `lines 1.2 today`: test-plan 1@526.
  - test-plan §3 Closed enums, `live-in-ci`, suite `local-live`, `<name>:events`, plus a §12 Decisions Log entry:
    carried (Harness / gate surface). Searches: `live-in-ci` test-plan 1@561; `local-live` test-plan
    5@533,561,970,1000,1557; `:events` 0 hits; Closed enums at test-plan:665. Also carried: the four `local-live`
    failure codes (`build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing`) and `boot` reason `verify-failed`
    (already at test-plan:529).
  - test-plan §7 Fake agent `DEFAULT_CLI_VERSION` 2.1.283: carried (Symbols). Search `2\.1\.0` test-plan 1@1379.
  - obs-plan §6 `subject` catalog `verify-probe`, plus a §12 Decisions Log entry: carried (Schema / config).
    `version-probe`: obs-plan 4@854,867,1059,1110, architecture 1@93; `statusline-shell` obs-plan @947,1059,1110.
    The `verify-probe` subject has 0 hits; the 3 architecture hits are the plugin name `viola-verify-probe`, a
    different fact.
  - architecture §Occupied Resources (Filesystem `diagnostics/`), verify's `cli-<name>.ndjson` spawn pairs: carried
    (Symbols). `cli-<name>` hits architecture 2@390,605, obs-plan 15, test-plan 4@698,710,923,1728.
  - architecture [Error Handling], §Conventions (Rust error types), §Inherited Defaults (Errors), §Standard Contracts
    (Ledger stamps envelope), `StampError` retired: carried (Symbols). `StampError` hits architecture 4@97,155,251,641
    (97 [Error Handling], 155 Conventions, 251 Ledger stamps envelope, 641 Inherited Defaults). See also claim (3), `Refusal`.
  - architecture §Occupied Resources (Repository), `target/e2e-home/viola-live-*/`: carried (Symbols, local-live).
    `e2e-home` hits architecture 4@378,410,519,572.
  - architecture [PTY] H2 note, "owned by the real-CLI verify entry" → names "First live test and self-drive": not
    carried by code. It is a wording correction, since `--local-live` does not claim the H2 measurement. Search
    `real-CLI verify entry`: architecture 1@47, test-plan 1@926.
- **Coverage of new surfaces:**
  - `viola verify` spawn pairs → validation n/a · instrumentation log✓ (`process-start/exit`, schema-valid, codes only)
    · PII n/a (no argv or content logged) · tests integ (`cli_verify::verify_with_an_instance_logs_both_spawn_pairs`,
    `contract_diag_schema::diag_line_schema_takes_only_catalogued_spawn_subjects`) · a11y n/a · tokens n/a.
  - `AgentError::StampsMalformed` → validation ✓ (a closed enum, fixed Display) · instrumentation log✓ (the existing
    `parse-rejected{parser:"ledger-stamps", detail:"malformed"}`) · PII n/a · tests unit (`ledger` tests,
    `version_gate` `stamps_verdict` cases) · a11y n/a · tokens n/a.
  - harness `boot --unstamped` / step 4 / `start_records` → validation ✓ (closed reasons and missing codes) ·
    instrumentation n/a (a test harness; one JSON document) · PII n/a · tests unit (`stamped_line`, `start_records`,
    readiness cases) and integ (`harness_lifecycle::{harness_session_boots_reports_logs_and_tears_down,
    boot_with_an_unknown_cli_version_is_verify_failed, boot_unstamped_writes_no_stamps}`) · a11y n/a · tokens n/a.
  - harness `run --local-live` → validation ✓ (a closed suite and codes; `CI` presence only) · instrumentation n/a ·
    PII n/a (verify stdout is forwarded to the harness stderr, never into the document) · tests unit (5
    `run::tests::run_local_live_*`) and integ (`crates/viola-e2e/tests/cli.rs::run_local_live_under_ci_is_refused`) ·
    a11y n/a · tokens n/a.
  - root `stamped_home` / `wait_endpoint_gone` → test infrastructure; tests integ
    (`channel_endpoint::stop_wait_holds_while_the_endpoint_answers`, `contract_ledger_probes`, the consumers).

## Deviations from intent
- **`run_with`'s `ci` parameter** (the plan's form) reached 6 unlisted callers, recorded as companions. The
  scope record, as P1 `gate.py scope` read it (`clean — changed 32 · listed 26 · recorded 6 (companion 6)`):
  - companion · `crates/viola-e2e/src/harness/run/perf.rs` · serves `boot.rs` (`stamp: true`) and `run.rs` (the `ci`
    argument) · self
  - companion · `crates/viola-e2e/src/harness/pre_push.rs` · serves `run.rs` (a caller of the changed `run_with`) · self
  - companion · `crates/viola-e2e/src/harness/run/browser.rs` · serves `run.rs` (same) · self
  - companion · `crates/viola-e2e/src/harness/run/coverage.rs` · serves `run.rs` (same) · self
  - companion · `crates/viola-e2e/src/harness/run/fuzz.rs` · serves `run.rs` (same) · self
  - companion · `crates/viola-e2e/src/harness/run/mutants.rs` · serves `run.rs` (same) · self
- **`--local-live` builds before it verifies.** The plan left the binary unstated. It uses the `--perf` precedent (a
  runner build into `target/harness`), so it adds the failure codes `build` and `verify-exit-none` beside the plan's
  `verify-exit-<n>` and `row-missing`.
- **`cli_fake_agent.rs` beyond line 199.** `hooked()` reads the default version, so lines 95, 150, 165 and 170 moved
  to the recorded version too. Its old `!stamps.json.exists()` assertion was removed, because the home is now stamped.
- **`cli_instance_state.rs:160`.** The plan listed it as keeping its assertions. It asserted `cli_verified:false` on
  the now-stamped `booted_wrapper` and read `true` (red on the Windows run and the pre-push Linux leg), so the
  oracle moved to `true`.
- **`cli_verify.rs`.** `verify_dispatch_error_keeps_the_chain_in_the_detail_file`'s exact role shape and
  `verify_with_an_instance_logs_its_start_and_exit`'s first-`process-exit` find now account for the two spawn pairs.
- **The witness server.** The item-8 witness binds an unserved `Server`: a served one keeps its Windows pipe in the
  accept thread after `Serving` drops.
- **`boot_that_never_gets_ready_times_out_and_stops_its_supervisor`** boots with `stamp: false`: its stand-in `viola`
  (a fake-agent copy) cannot verify.
- **`services/viola.md`.** Its "verify's spawns write no pair yet" bullet was rewritten to the landed fact.
- **Wrap: plan.md gate 6 retargeted** on the operator's word. The `artifact` key moves from the directory
  `target/agent-run/artifacts/` to the file `target/agent-run/artifacts/junit-nextest-integration.xml`, a file every
  default run writes and the one whose archive names `contract_ledger_probes`. The word: "Retarget gate 6 freshness
  to the file, not the directory".

## Decisions & corrections
- Operator (P4 fork, relayed in the plan): item 8's fix is test-side, with a deterministic witness, no measurement
  pushes and no product seam. `Wrapper::boot` passes `--cli-version` always, while `--fixtures` stays opt-in.
- Operator and overseer at the operator pass, on hygiene, for the phase run dir's refused rows:
  - "Delete uncited, rewrite log" (overseer: "uncited copies are noise in the tree (clean as you go); the cited job
    log and p5-dryrun get placeholder rewrites with line counts kept").
  - "Rename to .rs.txt" for the P5 control `baseline/control/writer.rs` (overseer: "agreed, same precedent as the
    hook-perf control file"). Recorded in `evidence/operator-pass.md`.
- Operator: "The entry-6 freshness red is a plan-target defect: record it for the wrap to retarget the freshness check
  to a file the run writes, not a directory, rather than accept a standing red." At this wrap: "Retarget gate 6
  freshness to the file, not the directory, and carry the Refusal-enum line."
- Operator, at this wrap, to insert at route-resolve (founder-ruled 2026-09-29, live), right after this entry and
  before "Fake-agent drift contract": Sideloaded ConPTY. Ship Microsoft NuGet `conpty.dll` + `OpenConsole.exe`
  beside the pinned `viola.exe`, version and hash pinned, signature checked, loaded only from that path. Acceptance:
  the H2 200-loop on windows-2025 with and without it.
- Sweep hazards:
  - `verify-probe` greps hit the probe plugin's name `viola-verify-probe` (architecture 91, 100, 359), not the new log
    subject.
  - gate.py hygiene's POSIX `/home/{user}/` form fires on a repo-relative `…/home/instances/builder/…` (a kept test
    home's layout) written in evidence prose.
  - A `run_with(` call written across lines ends with `&mut runner` on its own line. `pre_push.rs` passes `runner`
    without `&mut`, so a single-line `, &mut ` pattern misses it.
- The Bash guard refuses any command carrying a doubled backslash (a sed with an escaped newline literal, and an
  evolve record note). Such an edit goes through Edit, a Write-tool script, or `chr(92)`.

## Outcome
- **Acceptance, re-asserted against the diff:**
  - v1-14 (security, tests, arch): every stamped root test home is stamped by verify against the fake agent at
    2.1.283 (`stamped_home`), and every harness `boot` without `--unstamped` is stamped at step 4. The census
    `! grep -rnE '(fs::write|replace_private|File::create|OpenOptions)[^;]*stamps' tests crates/viola-e2e scripts` read
    exit 0, no output. `grep -c -- '--local-live' .github/workflows/ci.yml` reads 0. `CI=true … run --local-live`
    exits 2 `live-in-ci` before any spawn (gate 7 and `cli.rs::run_local_live_under_ci_is_refused`). CI is green on
    three OSes. MET; matrix `implemented`, ref `crates/viola-e2e/tests/cli.rs::run_local_live_under_ci_is_refused`.
  - `stamped_home` reads `stamped` only from exit 0 plus the summary line; `unstamped` writes no stamps; its three
    consumers keep their verdicts. MET.
  - Harness boot: default boot is stamped (smoke `cli_verified:true`, both instances) with events 1-3 as specified;
    `--unstamped` reads `cli_verified:false`; `9.9.9` is `verify-failed`, `exit_code` 1, no `supervise.json`. MET.
  - `run` passes and `contract_ledger_probes` is named in the archived JUnit (`target/run-archive/359/`), with six
    literal rows and `6 pass  0 fail` over `fixtures/claude/2.1.283`. MET.
  - Item 8: the stop waits for the endpoint to be gone. The witness passes, and reads red with the wait neutralised
    (`evidence/item8-witness-guard.md`). The final HEAD's CI is green on three OSes with case_08 among the passed.
    Cause as required: "the stopped wrapper's own endpoint still accepted a connect after its exit was observed; why
    it outlives the exit is not established". The overseer's collision hypothesis: falsified. MET.
  - "`viola-agent-claude` defines one thiserror enum, `AgentError`, holding `StampsMalformed` …; `StampError` is
    absent": the `StampError` half is MET. The "one thiserror enum" half is UNMET against the diff, because the
    pre-existing `Refusal` enum remains (claim (3)). It is not matrix-linked, so it routes to P2. Carried by the
    operator's word as the Refusal-enum line.
  - A malformed stamps file still yields exactly one `parse-rejected{ledger-stamps, malformed}` and
    `cli_verified:false` (the existing cases, green). MET.
  - Verify with `VIOLA_NAME` writes one version-probe pair and one verify-probe pair, all schema-valid; `run`'s gate
    keeps one version-probe pair; verify's stdout and stderr are unchanged. MET.
  - Verify output carries no ESC (`assert_plain`, unchanged). MET. `run --browser` is green on three OS legs (CI
    `test` jobs, 15/15). MET. Final CI green with `coverage,doctest,playwright`: MET (ci#36538832471).
- **Gates** (implement's final whole-block run over the final tree; the operator pass re-read CI):
  - `cargo fmt --all --check`: green.
  - `cargo clippy … -D warnings`: green.
  - `bash scripts/agent-run.sh run --unit`: green (728 unit).
  - `… run --integration --filter 'binary(channel_endpoint) & test(/stop_wait_holds…/)'`: green (1 passed; the
    remove-the-guard run read red, then green restored).
  - `… run --integration --filter 'binary(cli_verify) & test(/verify_with_an_instance_logs_both_spawn_pairs/)'`:
    green (1 passed).
  - `bash scripts/agent-run.sh run`: `red · artifact STALE`. `exit 0` ✓ and `contains "ok":true` ✓ (728 + 225); the
    `artifact` atom judged the directory, claim (4). It is dispositioned by the retarget to the integration JUnit
    file at this wrap; the light gate re-reads it.
  - `CI=true bash scripts/agent-run.sh run --local-live`: green (exit 2, `live-in-ci`).
  - The stamps-writer census `! grep -rnE …`: green (no output).
  - `grep -c -- '--local-live' .github/workflows/ci.yml`: green (exit 1, `0`).
  - `! grep -rn StampError crates src tests`: green.
  - `cargo deny check`: green.
  - `bash scripts/agent-run.sh pre-push`: green (also re-run green before the push).
  - Smoke entries (`cleanup` / `boot` / `logs --process run` `cli_verified:true` / `logs --kind session-start` /
    `cleanup` / `boot --unstamped` / `logs` `cli_verified:false` / `cleanup`): all green.
  - `gate.py hygiene` (operator leg): refused 10, all in the phase run dir, handled on the operator's word, then
    `clean`.
  - `git … push origin HEAD` (operator leg): `31dd995..702a3b4`.
  - `ci.py conclusion --sha HEAD --wait 1800` (operator leg): `verdict: green` 15/15, ci#36538832471 on `702a3b4`.
  - Smoke: a hand boot of overseer + builder, stamped, events 1-3, `status` ready, `cleanup` processes and endpoints
    gone.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. Commits: `702a3b4` (the one push). Final HEAD's CI: ci#36538832471, green
  15/15 (`evidence/operator-pass.md`). Implement's report stays the basis for the gate runs and deviations.
- **Process hygiene:** harness sessions `p-vsth-smoke`, `p-vsth-unstamped` and `p-vsth-p3` were started by
  implement and terminated (cleanup `processes_gone:true`, `endpoint_gone:true`). The pre-push WSL work was started
  by implement and the operator pass and has terminated (no viola or cargo process left in the distro). 6 ×
  `viola.exe` of `additional/viola-lab/prototype` belong to other sessions and were left running on the operator's
  word.
