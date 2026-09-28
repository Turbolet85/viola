# Report — 2026-09-28-mutation-testing-to-the-epoch-boundary

**Chunk:** Mutation testing to the epoch boundary — per-chunk, pre-push and CI mutation gates retired; cargo-mutants and
the harness mutants arm kept for code-audit
**Date:** 2026-09-28T21:10Z
**Commits:** (since `last_wrap` 2026-09-28T18:39:59Z; basis `git log --format='%h %s' 537ac36..HEAD`)
- `d5deb01` chore(…): operator pre-CI commit, for the run this chunk's verdict reads
- `a7c1560` chore(…): measure the macOS harness mutants phases, nested build apart from first and second binary execution (measurement only)
- `9da5f67` fix(…): the two harness mutants tests leave macOS, measured runner-side in run 36481260151, and the measurement code is removed
- `17b93c7` fix(…): the oversize send also takes ENOTCONN on the write, measured on macOS in run 36482322449

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-status 537ac36` + the working tree; 20 changed per `gate.py scope`, base `537ac366`)
  `.github/workflows/ci.yml` · `Cargo.toml` · `Cargo.lock` · `crates/viola-e2e/Cargo.toml` ·
  `crates/viola-e2e/src/bin/viola-harness.rs` · `crates/viola-e2e/src/harness/{gate.rs, mod.rs, pre_push.rs,
  pre_push/linux.rs, run.rs, run/browser.rs, run/coverage.rs, run/fuzz.rs, run/perf.rs, run/mutants.rs,
  run/mutants/scratch.rs}` · DELETED `crates/viola-e2e/src/harness/cfg_legs.rs`, `crates/viola-e2e/src/harness/run/mutants/leg.rs` ·
  `crates/viola-e2e/tests/cli.rs` · `tests/channel_endpoint.rs` · chunk folder (scope/research/plan/scope-record/evidence/report).
- **Symbols / APIs:**
  - CI: jobs `mutants (${{ matrix.os }})` (matrix ubuntu-latest, windows-2025) and `mutants-verdict` REMOVED from
    `ci.yml`, with their comment block; the only `actions/download-artifact` use (v8.0.1, pinned SHA) left with
    `mutants-verdict`. Nothing else `needs:` either. The `test`-job `tool:` line keeps `cargo-mutants@27.1.0` (the
    harness self-tests use it; it is the WSL provisioning input) — byte-unchanged; the `msrv` job's cargo-mutants
    install unchanged. The `!target/agent-run/chunk.diff` upload exclusion stays (comment reworded: "`run --mutants`'
    diff").
  - `viola-harness run`: flag `--leg <name>` REMOVED (clap now exits 2 `usage`/`arguments` for it); usage detail
    `invalid-leg` (run and gate) REMOVED; `scoped-leg` refusal REMOVED. `--file` kept.
  - `viola-harness gate`: flag `--mutants-legs` REMOVED (clap `arguments` exit 2). `gate()` signature is now
    `gate(artifacts, require)` — the `root` and `legs` parameters removed (only the union read `root`). `SUITES` keeps
    `mutants`; the single-run `survived` breach stays.
  - `Selection::from_flags`: no selector or `--all` selects **unit + integration only**; `mutants` runs only when
    `--mutants` is named (was: in the default and `--all`).
  - `run_with(ws, sel, filter, chunk_base, runner)` — the `leg: Option<&str>` parameter removed; callers: `run`,
    `viola-harness run_cmd`, pre-push `windows_tests`, and tests in browser/coverage/fuzz/perf/mutants.
  - `run --mutants` document: no `leg` key; no `artifacts/mutants-verdict-<leg>.json` is written (writer
    `write_leg_verdict`, `leg_verdict`, `leg_verdict_path` REMOVED with `run/mutants/leg.rs`). Everything else of the arm
    is kept: base derivation (`AGENT_RUN_CHUNK_BASE` override, else the derived base), classification
    (`no-rust-delta` / `test-only-rust-delta` / `counted` / `scoped`), host mutation scratch
    `<repo parent>/viola-mutants-scratch` on Windows (`scratch_bytes` in the run document stays), `target/mutants`,
    `[profile.mutants]`, verdict (`missed == 0 && timeout == 0 && unviable <= caught`), run archive.
  - `harness::cfg_legs` module (`compiled_legs`, a `syn` visitor over `#[cfg]`s) REMOVED; `gate::union`,
    `gate::parse_legs` REMOVED.
  - `run/mutants/scratch.rs` `host_scratch_bytes` REMOVED (its only production callers were pre-push's two reads).
  - `pre-push`: stages now `tools → sync → cache → linux-tests → vm-release → windows-tests`, `ok:true` when
    `windows-tests` is green. Stages `linux-leg`, `windows-leg`, `union` REMOVED; stop reasons `verdict-missing`,
    `base-mismatch` REMOVED; document sections `legs`, `gate` REMOVED; `cache` fields `scratch_bytes`,
    `scratch_bytes_after`, `windows_scratch_bytes`, `windows_scratch_bytes_after` REMOVED (`bytes`, `cap`, `cleaned`,
    `bytes_after` kept). Distro side: the `TMPDIR=<distro home>/viola-pre-push-scratch` assignment and the scratch dir
    (wipe/recreate 0700) REMOVED — every WSL call is now `env -i HOME=… PATH=…` with NO further assignment
    (`Linux::cmd_env` folded into `cmd`; `linux_harness` lost its `env` parameter). The `tools` stage keeps its
    cargo-mutants pin check (the distro's `run --coverage` runs the real-cargo-mutants harness tests).
  - Tests: `tests/channel_endpoint.rs` `send_oversize` also tolerates `ErrorKind::NotConnected` on the oversize write
    (a test tolerance; no product source changed). `run/mutants.rs` tests
    `run_mutants_passes_when_the_change_is_tested` and `run_mutants_reports_survivors_of_an_untested_change` carry
    `#[cfg(not(target_os = "macos"))]` (never `#[ignore]`).
- **Crates / modules:** removed modules `viola_e2e::harness::cfg_legs`, `viola_e2e::harness::run::mutants::leg`; no crate
  added or removed.
- **Dependencies:** REMOVED from `viola-e2e`: `syn` (=2.0.119, features full/parsing/printing/visit) and `proc-macro2`
  (=1.0.107, `span-locations`), their `[workspace.dependencies]` pins and comments, and viola-e2e's
  `[package.metadata.cargo-machete] ignored = ["proc-macro2"]`. `Cargo.lock` −2 lines (both stay in the graph as
  `serde_derive`'s). No dependency added or bumped. `cargo deny check` and `bash scripts/deny-probes.sh` green.
- **Schema / config:** `.config/nextest.toml` byte-identical to `537ac36` (`[profile.ci]` 120 s kill, `retries = 0`,
  `[profile.mutants]` unchanged; basis `git diff --quiet 537ac36 -- .config/nextest.toml` exit 0). No schema file changed.
- **Spec-master edits:** none (this wrap's P2 applies them).
- **Counts / qualifiers moved:** (basis: CI reads by `ci.py conclusion`)
  - ci.yml jobs 9 → 7; check-runs per push 18 → 15 (ci#36480299135 on `d5deb01` 15/15; ci#36483042659 on `17b93c7`
    15/15). Stated at architecture :572 ("9; 18 check-runs per push").
  - SHA-pinned actions in ci.yml: `actions/download-artifact` gone (security-plan names the pinned list).
  - macOS `test` job testcase count 917 → 915 (the two excluded tests); ubuntu 917, windows 932 unchanged
    (ci#36483042659 JUnit).
  - pre-push document keys: `v cmd ok stage sync cache linux vm windows` (implement's pre-push, operator entry 24 ×4).
- **Dev-tool versions:** none — cargo-mutants 27.1.0, cargo-nextest 0.9.146, cargo-llvm-cov 0.9.1 re-read at their pins
  (ci.yml :41, the pre-push `tools` stage).
- **Harness / gate surface:** CI steps `Mutation leg (whole chunk)`, `Upload leg verdict`, `Gate verdict (union of the
  legs)` removed with their jobs; harness verbs `run --leg`, `gate --mutants-legs` removed; pre-push stages/sections
  above; default `run` selection above. The 5-command agent surface and every status/log shape unchanged.
- **Cross-project / external claims:**
  - CI runs read (Turbolet85/viola): ci#36480299135 on `d5deb01` green 15/15 (wall 463 s) · ci#36481260151 on `a7c1560`
    RED (test ubuntu-latest, the measurement push) · ci#36482322449 on `9da5f67` RED (test macos-latest) ·
    **ci#36483042659 on `17b93c7` green 15/15 (wall 242 s)** — the final HEAD before this wrap commit; the overseer
    verified the same run.
  - `/andromeda-code-audit` collectors.md C1 (outside this repo) runs `cargo mutants -p {unit}` itself and names a
    project union only by pointer — basis: research.md, read at phase.
- **Reverted / negative API facts:** the step-8 measurement code (`timed()` phase lines, cargo-mutants `-L debug
  --all-logs`, the in-test `measure()` nested build + double execution, a `[[profile.ci.overrides]]`
  `success-output`/`junit.store-success-output` block in `.config/nextest.toml`) shipped in `a7c1560` and was removed
  in `9da5f67` — measurement only, per the plan.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The CARRY hypothesis "the nested cargo inherits the outer `cargo llvm-cov nextest` jobserver or coverage
    `RUSTFLAGS`" (scope.md CARRY 2; the capability-ledger chunk's `evidence/macos-mutants-baseline-carry.md`) —
    measured false on ci#36481260151: no `CARGO_MAKEFLAGS`/`MAKEFLAGS`/`MFLAGS`/`RUSTFLAGS`/`CARGO_ENCODED_RUSTFLAGS`
    reaches the nested cargo; what it inherits is `RUSTC_WRAPPER`, `__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS`,
    `CARGO_LLVM_COV`, `LLVM_PROFILE_FILE`. The second candidate (XProtect first-launch scanning) also measured false:
    first launch 3 ms. Evidence: `evidence/macos-mutants-phases.md`.
  - The plan's claim that the macOS time is "the cold compile of the one-file throwaway crate" — measured narrower:
    the same cold build takes 0.41 s outside cargo-mutants; ~77 s is spent only in cargo-mutants' baseline build in
    its copied tree. Same evidence file.
- **Expected amendments (from plan):** (site search: `sweep.py` pattern set over the seven masters — hits below per
  master; 0 hits in design-system and layout-templates for every pattern)
  - test-plan §10 / §12 macOS exclusion of the two tests (runner-side arm) — **carried** (Symbols: the `cfg`; Spec
    claims disproved). Sites: test-plan `Mutation gate` 10 hits (:1448 :1511 :1809 :1825 :1853 :1856 :1877 :1891 :1905
    :1928).
  - test-plan §1/§2/§3/§9/§10/§11/§12 mutation leaves chunks, pushes and CI — **carried** (Symbols; Harness). Sites:
    test-plan `mutants-verdict` 17 · `--mutants-legs` 9 · `--leg` 11 · `invalid-leg` 5 · `scoped-leg` 4 ·
    `linux-leg|windows-leg` 3 · `verdict-missing` 1 · `base-mismatch` 2 · `scratch_bytes` 4 · `viola-pre-push-scratch`
    2 · `download-artifact` 1 · `syn|proc-macro2` 3 · `union` 19 · `AGENT_RUN_CHUNK_BASE` 2 · `TMPDIR` 4 (only :662's is
    the retired one) · `mutation leg` 7 · `quality-gate-config-emit` 4.
  - architecture §Infrastructure Patterns → CI/CD approach (job list + 9/18 → 7/15; pre-push paragraph) — **carried**
    (Counts). Sites: architecture :572 (jobs/count, `--mutants-legs`, `cfg_legs`), :588 (pre-push, `TMPDIR`,
    `viola-pre-push-scratch`), :559 `mutation leg`, :408 `--leg`.
  - architecture §Occupied Resources (`mutants-verdict-<leg>.json`, distro `viola-pre-push-scratch` retired;
    `AGENT_RUN_CHUNK_BASE` reworded) — **carried**. Sites: architecture :408, :427 (`scratch_bytes`), :377
    (`AGENT_RUN_CHUNK_BASE`, "Mutation gate").
  - architecture §Stack and Technologies (syn / proc-macro2 out; download-artifact pin out; cargo-mutants worded) —
    **carried** (Dependencies). Sites: architecture :36 (download-artifact, mutation leg), :38 and :465 (syn,
    proc-macro2, cfg_legs, union), :571.
  - security-plan §Dependency Security CI integration (pinned-action list 5 → 4; concurrency note's mutation clause;
    "mutation base derived by the harness" line) — **carried**. Sites: security-plan :131, :346 (download-artifact,
    mutants-verdict), :348 (`AGENT_RUN_CHUNK_BASE`), :161 (union).
  - security-plan §Bootstrap `secret-scanning-ci-gate` (the `mutants-verdict-<os>.json` unscanned upload) — **carried**.
    Site: :395 (`mutants-verdict`, mutation leg), :394.
  - security-plan §Secret Management, Development (`TMPDIR` "on the Linux mutation leg" carve-out retires) + a narrowing
    Decisions Log entry citing the 2026-09-27 founder ratification — **carried** (Symbols: the WSL call has no further
    assignment). Sites: :432 (viola-pre-push-scratch), :447, :692 (`TMPDIR` + mutation leg); the other `TMPDIR` hits
    (:57 :66 :155 :203 :503 :604) are the Unix socket dir, not this carve-out — no change.
  - security-plan §Threat Model Summary CI line — **carried**, the wrap weighs verbatim-copy vs in-place. Sites: :738,
    :748 (`18 check-runs|9 jobs`), :718 (union), :656 (mutants-verdict).
  - obs-plan §9 Mutation row → the audit; §8 item 6 (`mutants-verdict-<os>.json` upload clause, pre-push copied-back
    verdict); §10 surviving-mutant clause → the audit; §1 untouched — **carried**. Sites: obs-plan :1208 :1210 :1212
    (§8 item 6: mutation leg, scratch_bytes, mutants-verdict), :1243, :1256 (§9: mutants-verdict, --mutants-legs,
    union, Mutation gate), :1289.
  - a11y-plan §10 SLO Invariants & A11y Budgets — **carried** (not in the plan's list; found by the sweep). Site: :1153 ("under the
    tests' mutation gate"); the sweep's other hit :1145 is "the union of the per-state axe verdicts" — no change.
  - Wrap route-resolve (CARRYs 4, 5; WSL CARRY moves on) — **carried** to P5.
  - Wrap cascade leaves (`.claude/rules/testing.md` :48 + 2026-09-24/25 union additions; `verification-harness.md` shims
    `pre-push` paragraph, `run --mutants` bullet, 2026-09-24/25 additions; `.claude/docs/{commands,workflow,
    tests-summary,security-summary,obs-summary,stack}.md`; CLAUDE.md gate list) — **carried** to the cascade.
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element (removals only).
  - `run --mutants --leg x` / `gate --mutants-legs a` (retired flags) → validation clap-unknown-argument✓ ·
    instrumentation n/a · PII n/a · tests integ (`crates/viola-e2e/tests/cli.rs`
    `gate_usage_errors_and_the_retired_leg_flags_are_exit_2`) · a11y n/a · tokens n/a

## Deviations from intent
- **Entry-13 grep probe vs step 6 (plan defect, operator ruling):** the plan's `grep -rlE
  'mutants-legs|leg_verdict|…' crates src scripts .github Cargo.toml` probe (expect exit 1, no output) cannot hold beside
  step 6's own retargeted test, which names `--mutants-legs` (tests/cli.rs:82) to prove it is refused. The operator and
  the overseer ruled at implement: keep the literal, have the wrap NARROW the probe to exclude the refusal test rather
  than accept a standing red. This wrap narrows it in `plan.md` (the one refusal-test file filtered out, every other
  hit still red) — see Decisions.
- **`gate()` lost `root` too** (only the union read it) — an in-list file, beyond the plan's "loses its legs parameter".
- **`linux.rs`**: `cmd_env` folded into `cmd`, `linux_harness` lost its `env` parameter (only the retired leg's TMPDIR
  used them); `dir_bytes` private.
- **Mutants tests**: leg-only tests deleted (incl. the swamp-leg test; `mutants_suite_is_red_when_unviable_outnumbers_caught`
  keeps the unit oracle); `run_mutants_without_a_leg_keeps_survivors_red` → `run_mutants_with_survivors_is_red`
  (survived 2, verdict `counted`). Pre-push tests rewritten for the reduced stages, one pins the document's exact key
  order.
- **Step 9 took the runner-side/unnamed arm** (the plan's third branch): the phase is named, its cause is not;
  `#[cfg(not(target_os = "macos"))]` on the two tests with the measured reason; no second measurement push.
- **The fold of a red outside the diff** (`tests/channel_endpoint.rs`) contradicts the acceptance "(arch) No product
  crate's source, root test or `Cargo.toml` changes" in its "root test" clause — see Outcome (UNMET-in-letter, routed to
  P2 escalation).
- **Scope record** (`gate.py scope`: `clean — changed 20 · listed 18 · recorded 2 (companion 1 · widening 1)`,
  `record: not changed — .config/nextest.toml`):
  - companion — `crates/viola-e2e/src/harness/run/mutants/scratch.rs` · serves `pre_push.rs` · self
    (`host_scratch_bytes` lost its only callers).
  - in-intent — `.config/nextest.toml` · serves step 8 · self (the measurement override; removed, byte-identical to base).
  - widening — `tests/channel_endpoint.rs` · serves step 9 · word: "Fold every red into this chunk" — the operator.

## Decisions & corrections
- Operator/overseer at implement P2: a plan pair that cannot both hold (a test naming a retired flag vs a grep for
  that flag) is resolved by NARROWING the probe to the refusal test, never by a token-split in the test and never by a
  standing red.
- Operator: "Fold every red into this chunk" — both CI reds of the operator pass were folded here (one caused by the
  measurement code, one a pre-existing macOS race in a root test).
- Measurement design (step 8): moving the nested cold build into the same `target/mutants` cargo-mutants copies kept
  the one measurement push under the 120 s kill (the baseline then read `0s build` on Windows); passing-test output is
  made visible by a temporary `[[profile.ci.overrides]] success-output = "immediate"` + `junit.store-success-output`.
- **Found:** the harness's throwaway crate (package `viola`, like the product) inherits cargo-llvm-cov's `RUSTC_WRAPPER`
  and `LLVM_PROFILE_FILE`, so it is built coverage-instrumented and every run of its binaries writes a `viola-%p-%m`
  profile into the OUTER coverage run — the measurement's extra executions produced a corrupt-header profile and broke
  the ubuntu coverage merge. The measurement is gone; the channel predates this chunk (every harness test that builds
  and runs the throwaway crate). Route item with an owner at P5 (operator direction for this wrap).
- **Found:** macOS AF_UNIX reports ENOTCONN (os error 57) as well as EPIPE on a write that races the peer's close.
- Sweep hazard: `TMPDIR` in security-plan has 8 hits and only :432/:447/:692 are the retired carve-out — the rest are the
  Unix socket dir (`$TMPDIR/viola/`); `union` hits in test-plan include non-mutation uses — read each hit.
- Sweep hazard: after an operator pass the plan's `git diff --quiet HEAD -- …` probes (entries 9, 10, 11) compare against
  a HEAD that already holds the chunk, so they pass vacuously at the wrap's light gate; this report re-read them against
  the pre-CI base `537ac36` (wsl-provision 0, nextest 0, product paths: `tests/channel_endpoint.rs` only).

## Outcome
- **Acceptance, re-asserted against the diff (base `537ac36`):**
  - (arch, tests, security) no `mutants`/`mutants-verdict` job, no `download-artifact`, nothing `needs:` them, the
    test-job `tool:` line unchanged, `wsl-provision.sh` untouched (vs base: exit 0), deny + deny-probes green — **MET**.
  - (obs, a11y) every remaining job byte-identical; the ci.yml diff touches only the two removed jobs, their comment
    block and the :159 comment (entry 12 recorded, 1 insertion / 65 deletions); the final CI run green on all 15
    check-runs incl. `test (ubuntu-latest)` `gate --require coverage,doctest,playwright` — **MET** (ci#36483042659).
  - (tests) pre-push exits 0 at `"stage":"windows-tests"`, no `legs`/`union`; `pre_push_document_carries_no_absolute_path`
    passes — **MET** (operator entry 24 ×4; `run --unit`).
  - (tests) no selector / `--all` selects no mutants; `run --mutants` [`--file`] classifies, runs, judges; the two
    real-cargo-mutants tests pass in `run --unit` — **MET** on this host and ubuntu/windows CI.
  - (tests) the retired flags exit 2 with a usage document; no source/script/workflow/manifest names the retired
    machinery — **MET** with the narrowed probe (the one hit is the refusal test itself).
  - (arch) "No product crate's source, root test or `Cargo.toml` changes" — **UNMET in its "root test" clause**:
    `tests/channel_endpoint.rs` changed (a test tolerance for a macOS race, folded on the operator's word). No product
    source or product manifest changed (`git diff --stat 537ac36 -- src crates/viola-{core,pty,channel,state,agent-claude}`:
    none). Unlinked to the matrix → **P2 escalation**. `viola-e2e` no longer depends on syn/proc-macro2 — MET.
  - (tests) `evidence/macos-mutants-phases.md` names the one measurement run (ci#36481260151), the phase holding the
    time (cargo-mutants' baseline build, 80.9 s) and the arm taken (runner-side/unnamed); the final HEAD's macOS JUnit
    holds neither test (915 testcases, 0 failures); both pass on ubuntu (1.79 s) and windows (5.8–6.1 s); the 120 s
    kill and `retries = 0` byte-unchanged — **MET**.
- **Gates (implement run `.andromeda/runs/2026-09-28T20-16-40-implement`, by `run`):**
  `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green ·
  `bash scripts/orphans-check.sh` green · `cargo deny check` green · `bash scripts/deny-probes.sh` green ·
  `grep -cE '^  (mutants|mutants-verdict):'` green (exit 1, last line 0) · `grep -c download-artifact` green ·
  `grep -c "tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1"` green ·
  `git diff --quiet HEAD -- scripts/wsl-provision.sh` green · `git diff --quiet HEAD -- .config/nextest.toml` green ·
  `git diff --quiet HEAD -- src tests crates/…` green at implement (before the ENOTCONN fold) ·
  `git diff HEAD -- .github/workflows/ci.yml` recorded · `grep -rlE 'mutants-legs|…'` **red · exit 1 ✗** (the one hit
  `crates/viola-e2e/tests/cli.rs` — narrowed at this wrap) · `run --unit` green (716 passed) ·
  `run --integration --filter 'binary(=cli)'` green (9) · `run --integration` green ·
  MSRV `cargo check` green · smoke `cleanup`/`boot`/`status`/`cleanup` green · `pre-push` green (121 s).
- **Operator entries** (evidence `evidence/operator-pass.md`): hygiene clean before every commit · pre-push green before
  every push (4×) · push guard held 4× · `ci.py conclusion`: d5deb01 green 15/15, a7c1560 red (folded), 9da5f67 red
  (folded), **17b93c7 green 15/15** · `gh run download` of ci#36480299135, ci#36481260151, ci#36483042659 (all three OSes).
- **Smoke:** boot → status `ready` → cleanup `processes_gone`/`endpoint_gone` (entries + by hand).
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran (pre-CI commit `d5deb01`); the gate verdicts above rest on its final state —
  Setup 4's commit list (`537ac36..17b93c7`, 4 commits) and the final HEAD's CI run ci#36483042659 recorded in
  `evidence/operator-pass.md`; implement's report (this session) for what only it holds.
- **Process hygiene:** implement's census — smoke sessions `p-mut-smoke`, `p-mut-smoke2` terminated; WSL VM terminated
  by every pre-push (`vm.terminated: true`); 4 `viola.exe` of `additional/viola-lab/prototype` left running, not this
  chunk's (the handoff counted 5).
