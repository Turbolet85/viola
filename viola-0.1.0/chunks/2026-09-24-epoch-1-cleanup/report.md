# Report — 2026-09-24-epoch-1-cleanup

**Chunk:** Epoch 1 cleanup. The MAX_FRAME value is pinned by a mutation-witnessed test; harness and fake-agent functions are held at cognitive
≤ 15; run.rs is split under 800 lines; the test clone pairs are removed; the Rust gate deferral is closed.
**Date:** 2026-09-25T11:30:00Z
**Commits:** `d14f234 chore(2026-09-24-epoch-1-cleanup): operator pre-CI commit`. It is the fourth form of the pre-CI commit (bf87d71 →
74dadd4 → d14f234, each amended and force-pushed on the founder's rulings; see Decisions). Parent: `9df9e45`.

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --name-only 9df9e45` over source, route and harness paths)
  - `crates/viola-core/src/lib.rs` — test `max_frame_is_sixteen_mib`.
  - `src/obs.rs` — test-module changes only: the boundary test `read_diagnostics_level_reads_at_most_sixteen_mib` and helper calls.
  - `src/main.rs` — a new inline `#[cfg(test)] pub(crate) mod test_support` (`one_line`, `assert_home_level`, `diag_detail_validator`),
    plus test-module call sites.
  - `tests/cli_fake_agent.rs` — a local helper `hooked()` and `PROMPT_FIXTURE`.
  - `src/bin/viola-fake-agent.rs` — `main` split into `script_steps`, `read_stdin` and `main`.
  - `crates/viola-e2e/src/bin/viola-harness.rs` — `main` split into `boot_cmd`, `run_cmd`, `cleanup_cmd`, `logs_cmd` and `gate_cmd`;
    `Cmd::Run` now holds a clap `#[derive(Args)] RunArgs`.
  - `crates/viola-e2e/src/harness/run.rs` (now dispatch plus shared test fixtures) and NEW `crates/viola-e2e/src/harness/run/{nextest,
    coverage,doctest,fuzz,mutants}.rs`.
  - `crates/viola-e2e/tests/cli.rs` — a `Booted` drop guard.
  - Also: the route (`working-route.md` stamp, `master-route.md` pending record), the chunk folder (scope, research, plan, `metrics.py`,
    `evidence/`) and the run dirs.
- **Symbols / APIs:**
  - `viola_e2e::harness::run` keeps every name that was `pub` at 9df9e45 through `pub use` re-exports, EXCEPT one:
    `pub fn fuzz_host_supported() -> bool` became `pub const FUZZ_HOST_SUPPORTED: bool = cfg!(target_os = "linux")`.
    - Callers: `run_with` only. Basis: `grep -rn fuzz_host_supported --include=*.rs` → 0 hits after the change.
  - The external users of `harness::run` are unchanged: `bin/viola-harness.rs:13` (`Selection`, `run_forwarding`, `run_with`) and
    `harness/gate.rs:10` (`COVERAGE_FLOORS`, `leg_verdict_path`). Basis: code-graph query 5 at phase P3, 11 rows.
  - New private helpers: `run_with` → `test_suites`, `tool_arms`, `document`; the harness bin handlers; the fake-agent helpers.
  - No product (`viola` bin) runtime symbol changed. `src/main.rs` and `src/obs.rs` changed only inside `#[cfg(test)]`.
- **Crates / modules:**
  - Modules added in `viola-e2e`: `harness::run::{nextest, coverage, doctest, fuzz, mutants}`, all private, each with an inline
    `#[cfg(test)] mod tests`.
  - Also added: a `#[cfg(test)] mod test_support` inline in `run.rs`, and a `#[cfg(test)] pub(crate) mod test_support` inline in `src/main.rs`.
  - No crate or workspace member added.
- **Dependencies:** none added or bumped. Basis: `cargo deny check` green; `Cargo.toml` / `Cargo.lock` not in the diff.
- **Schema / config:** none.
- **Spec-master edits:** none in the chunk's own diff.
- **Counts / qualifiers moved:** (basis: `metrics.py` over the audit population, before / after)
  - `run.rs` tokei code lines 1 562 → 472; files over 800: 1 → 0.
  - Functions over cognitive 15: 3 → 0 (831 → 846 functions).
  - In-scope clone pairs 9 → 0; total jscpd pairs 16 → 7.
  - viola-core `lib.rs` mutation 4 missed → 0 (12 caught).
  - The harness `--in-diff` mutant count for this chunk: 143 at implement, 148 after the swamp rule.
  - No master states these counts. They live in the code-audit ledger, whose next record owns them.
- **Dev-tool versions:** none changed.
  - cargo-nextest was re-read on the dev host at 0.9.146 (the CI pin) on 2026-09-24. The handoff's 0.9.133 line was stale.
  - cargo-mutants 27.1.0, cargo-modules 0.27.0, rust-code-analysis-cli 0.0.25, tokei 14.0.0 and jscpd 5.0.16 were re-read unchanged.
- **Harness / gate surface:**
  - `run --mutants` invokes `cargo mutants … --test-tool=nextest --copy-target=true` plus `--caught --unviable
    --build-timeout-multiplier=5`. cargo-mutants' stdout goes live to the harness's stderr (`Stdio::from(std::io::stderr())`) instead of
    being captured by `Command::output()`. Every per-mutant outcome line (repo-relative name plus build/test seconds) now reaches the CI
    step log as it happens.
  - **New verdict rule:** `mutants_suite` marks the suite red when `unviable > caught`, with the new `failures[]` code
    `unviable-exceeds-caught` and `failed` = survivors + 1. Under `--leg` such a leg is NOT deferred to the union (`failed != survived`).
  - No new `mutants.verdict` value and no new leg-verdict `outcome` value.
  - The five agent commands, the internal subcommands, every document shape, the exits and the `reason` / `detail` codes are unchanged.
    `run --browser` is still a usage error.
  - The CI workflow is unchanged (`ci.yml` not in the diff).
- **Cross-project / external claims:** CI runs on this chunk's pre-CI commits:
  - **36046091888** (bf87d71): cancelled at 21:56Z by the founder.
    - `mutants (windows-2025)`: 2 h 45 m silent after `Auto-set test timeout to 210s`; no verdict artifact.
    - `mutants (ubuntu-latest)`: `143 mutants tested in 17m: 139 caught, 4 unviable`.
  - **36117447745** (74dadd4, after the force-push): both mutation legs `base-missing`. `github.event.before` = bf87d71 was reachable from no
    ref in the checkout.
  - **36118112104** (74dadd4, after the rewind + re-push; base 9df9e45):
    - ubuntu: `143 … 139 caught, 4 unviable`.
    - windows: `143 mutants tested in 29m: 8 caught, 135 unviable`, `ok:true`. A false green, turning after
      `viola-harness.rs:157:5: replace cleanup_cmd -> ExitCode with Default::default()`.
    - `mutants-verdict`: never started (GitHub billing annotation).
  - **36126924953** (**d14f234**, base 9df9e45 on both legs; the repo is now public): **completed success, all 15 jobs**.
    - ubuntu `148 mutants tested in 10m: 144 caught, 4 unviable`; windows `148 mutants tested in 26m: 142 caught, 6 unviable`.
    - `mutants-verdict` success.
  - The rewind push also triggered run 36118098013 on 9df9e45 (empty diff), which is not a witness.
- **Reverted / negative API facts:** the `LEAK_PROBE_STUB` stub in `cleanup_cmd` and the `&& false` neutralisation of the swamp rule were
  temporary remove-the-guard controls, reverted (the `viola-harness.rs` diff is empty against the pre-CI commit).
- **Insufficient fixes (written, kept, not the remedy):** the streaming plus flags change (pass 2) was written to make the windows stall
  diagnosable. It did so: it exposed the unviable swamp in run 36118112104. It did not prevent the swamp. The remedy is the `cli.rs` guard
  plus the swamp rule (pass 4).
- **Spec claims disproved by measurement:**
  1. test-plan §10 Mutation gate, `.andromeda/test-plan.md:1496`: "`unviable` mutants are reported but do not fail the gate". MEASURED
     insufficient. A windows leg where 135 of 143 mutants were unviable because their builds failed (a leaked process held
     `viola-harness.exe`; relink `Access is denied. (os error 5)`, reproduced locally) read `ok:true`. The harness now reds `unviable > caught`.
  2. test-plan §3 `run` step 4 Verdict (`:557`) and Exit code semantics (`:563`) state `missed == 0 && timeout == 0` as the whole counted
     verdict. The implemented rule adds `unviable ≤ caught` (same evidence).
  3. test-plan §3 `run` step 4 Command (`:555`) gives the `cargo mutants` invocation without `--caught --unviable
     --build-timeout-multiplier=5` and does not say the output streams live. That is stale against the harness at d14f234 (the Harness
     bullet above).
- **Expected amendments (from plan):** none listed. The plan's list was empty, and its claim of "no contract changes" is superseded by the
  harness changes folded here at the operator's direction.
  - New since the plan: items 1–3 of Spec claims disproved (test-plan §3, §10).
  - Search: `grep -n "unviable" .andromeda/test-plan.md` → :655, :663, :1496 (+ :566 in the leg-verdict shape);
    `grep -n "copy-target" .andromeda/*.md` → test-plan :555 only; `.claude/docs/commands.md:41`; `.claude/docs/tests-summary.md:42`.
  - The failure codes (`outcomes-missing`, …) are stated in prose at :556 / :563, not as a closed enum. :663 enumerates only
    `mutants.verdict` and the leg `outcome`, so `unviable-exceeds-caught` belongs beside `outcomes-missing` in :563's prose, not in :663.
- **Coverage of new surfaces:**
  - `harness run --mutants` progress stream + swamp rule → validation n/a · instrumentation n/a (a test harness, print-exempt) · PII n/a
    (repo-relative names and durations only; no `--all-logs`) · tests unit (`run_mutants_prints_every_outcome_and_bounds_each_build`,
    `mutants_suite_is_red_when_unviable_outnumbers_caught`, `run_mutants_leg_with_an_unviable_swamp_is_red_not_deferred`) · a11y n/a ·
    tokens n/a.
  - `tests/cli.rs` `Booted` guard → tests (integration; leak control recorded) · the rest n/a.
  - `read_diagnostics_level` cap witness → validation `Read::take(MAX_FRAME)` witnessed ✓ · tests unit · the rest n/a.

## Deviations from intent
1. **All prior `pub` items re-exported**, where plan step 8 said to keep "the five external names". The harness library API is unchanged
   except `FUZZ_HOST_SUPPORTED`. Justification: zero cost, and no consumer breaks.
2. **`fuzz_host_supported()` → `const FUZZ_HOST_SUPPORTED`**, instead of step 9's "add a killing test". Its mutant `-> false` is equivalent
   on windows (`cfg!(linux)` is already false), so no windows test could kill it. The test oracle became `std::env::consts::OS == "linux"`.
3. **Smoke fired** though the plan stated no smoke leg. `boot_cmd` (default instances, real build) had no test. It ran boot → status
   `ready` → cleanup `processes_gone:true`, and 4 pids were verified gone.
4. **Harness mutation leg changed** (streaming, `--caught --unviable --build-timeout-multiplier=5`, the swamp rule) and a **`cli.rs`
   guard** added. None of this was in the plan. It folds the CI red of this chunk, at the overseer's direction ("fold, do not carry").
   `cli.rs` was in the plan's modify-set as conditional.
5. **Operator passes 1–4** instead of one. Pass 1 was cancelled (stall); pass 2 went base-missing after a force-push; pass 3's windows leg
   was false green and billing blocked the union; pass 4 was green. Two force-pushes and a rewind of `build/viola-0.1.0` (never `main`)
   were on the founder's explicit rulings, each with `--force-with-lease`.

## Decisions & corrections
- **Overseer/founder decisions:**
  - fold every red in this chunk;
  - every new guard test carries its remove-the-guard mutation run (done: the MAX_FRAME pin via cargo-mutants; the obs.rs witness, the
    flags test and the swamp tests by manual neutralisation);
  - the push form: amend + `--force-with-lease`, then rewind to 9df9e45 and fast-forward (so `github.event.before` = the chunk base);
  - the swamp rule is `unviable > caught`, picked by measurement across the chunk-7, chunk-8 and this chunk's legs;
  - **the reduced partial-verdict upload for cancelled legs is DECLINED** (a decision, not a deferral): per-mutant streaming already makes
    a stall diagnosable from the job log, and the `mutants.out` security decision stands;
  - the two windows-only unviable fake-agent `main` mutants (`viola-fake-agent.rs:463:5`, `:482:8`, each 1 s build; ubuntu caught both)
    are recorded, not chased. **Hypothesis, not measured:** a relink lock from the `--exit-no-eof` test's grandchild holding
    `viola-fake-agent.exe` for `HOLD_FOR` = 10 s.
- **Measured corrections of this session's own claims:**
  - the scope premise "every moved run.rs mutant was once caught" held only for the two-leg union (`fuzz_host_supported`);
  - the recommendation "a force-push makes the merge-base fall back to 9df9e45" was false: the replaced commit is absent from a
    `fetch-depth: 0` checkout, and `resolve_base` refuses it (`base-missing`);
  - "the build timeout caused the unviables" was false: the builds failed in 8–12 s, below the 390 s bound.
- **Sweep hazards / host lessons found:**
  - a process stop keyed on a command-line substring also matched this session's own tool shell (the command text carried the
    substring). Stop by exact `ExecutablePath` only;
  - `gh api …/commits/{sha}/check-runs` returns check-runs from EVERY run on the sha, including superseded ones after a force-push, so
    a CI read keyed on a sha re-used across runs mixes verdicts. A new sha per push keeps it one run;
  - an out-of-line `#[cfg(test)] mod x;` file is an orphan to cargo-modules 0.27.0 as the orphans gate runs it;
  - `Command::output()` never reaches EOF while a leaked grandchild holds the inherited pipe write end (measured: the pipe still open
    61.7 s after the child exited);
  - on windows a test that boots a supervisor and whose cleanup a mutant disables leaks processes that lock their `.exe`, so every later
    cargo-mutants build fails → unviable swamp.
- **Wrap obligation (CARRY):** propose a `playbook.md` rule keeping obs-plan §1 (the verbatim obs-scope copy, `obs-plan.md:485`) out of the
  cascade sweep (`runs/2026-09-24T17-37-22-evolve-diagnose/proposals.md` P9).

## Outcome
- **Acceptance criteria** (re-asserted against the diff):
  - MAX_FRAME pin + viola-core mutation last line `0` ✓.
  - Boundary witness red with the guard removed, restore grep `1` ✓.
  - `metrics.py cognitive` `0` ✓ · `size` `0` ✓ (run.rs 472) · orphans `5/5 targets clean` ✓ · `clones` `0` ✓ (LOG_FORMAT_EVENTS kept).
  - Panic-line / schema assertions unchanged in strength (the same keys plus `panic_location`, `thread`, no `corr`; the same schema) ✓.
  - Harness contract unchanged (`run --integration` green, `cli.rs` green) ✓. Fake-agent receipts unchanged (`cli_fake_agent.rs` green,
    names kept, print-allow probe `1`) ✓.
  - Gate deferral closed ✓.
  - In-diff mutation: host `148 … 144 caught, 4 unviable` `ok:true` ✓. CI `success,success,success` on run 36126924953 ✓.
  - `cargo deny` green, no new crate / dep / env / path ✓.
  - Pushed-sha check-runs `success` alone ✓.
  - **Criterion contradicted by the diff:** plan Implementation notes "Expected amendments (wrap): none … no contracts change". The
    harness mutation contract did change (Harness bullet), so this is routed to P2 as Spec claims disproved 1–3. No matrix-linked criterion:
    the chunk claims 0 capabilities.
- **Gates** (/implement block, final run 4, `implement-2026-09-24T18-26-58`), each green:
  - `python -X utf8 viola-0.1.0/chunks/2026-09-24-epoch-1-cleanup/metrics.py size` · `… cognitive` · `… clones` (last line `0` each);
  - `cargo fmt --all --check`;
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
  - `bash scripts/agent-run.sh run --unit`;
  - `… run --unit --filter 'test(/max_frame|read_diagnostics_level/)'`;
  - `… run --integration`;
  - the viola-core `cargo mutants -p viola-core --file crates/viola-core/src/lib.rs …` (last line `0`);
  - `grep -c 'take(MAX_FRAME)' src/obs.rs` (`1`) and the fake-agent print-allow grep (`1`);
  - `bash scripts/orphans-check.sh --probe` and `bash scripts/orphans-check.sh`;
  - `cargo deny check`;
  - `AGENT_RUN_CHUNK_BASE=9df9e45… bash scripts/agent-run.sh run --mutants` (`"verdict":"counted"`, `ok:true`).

  Operator entries (driven by the session on the founder's rulings, results in `evidence/operator-pass-{1..4}.md`):
  - `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` — pass 1 as written; passes 3–4 in the ruled
    rewind form;
  - the all-check-runs read: pass 4 last line `success`;
  - the mutants check-runs read: pass 4 `success,success,success`.

  Smoke: boot → status `ready` → cleanup, green.
- **Outcome basis:** implement's P4 report plus the operator directives after it (passes 1–4, the leak and swamp decisions) and their
  artifacts: `evidence/{metrics-before,metrics-after,remove-the-guard,mutants-max-frame,ci-windows-mutants-stall,operator-pass-1..4,
  leak-guard-and-swamp-rule}.md|txt`.
- **Process hygiene:**
  - Implement's census: the smoke session's 4 pids were terminated (verified by `Get-Process`).
  - Leak probes (`target/leak-probe`): each round's leaked `viola-harness.exe` / `viola.exe` / fake `claude.exe` were stopped by exact
    image path. One earlier stop by command-line substring also ended this session's own tool shell.
  - Re-measured at this wrap: 0 processes whose image lies under `D:\dev\projects\viola\`.
  - Left running, not this chunk's: the operator's viola-lab prototype `viola.exe` (12172, 44520) and the operator's Claude sessions.
