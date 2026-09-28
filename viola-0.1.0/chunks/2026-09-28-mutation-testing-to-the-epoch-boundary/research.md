# Codebase Research — 2026-09-28-mutation-testing-to-the-epoch-boundary

## Scope
- **Depth:** moderate · **Reads:** 16 · **Globs/Greps:** 19 (plus 1 code-graph query, trace
  `.andromeda/runs/2026-09-28T19-54-53-phase/tree-query-2026-09-28-mutation-testing-to-the-epoch-boundary.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (in full, 5 Session Additions — the `run --mutants`
  bullet, the `pre-push` shim paragraph and the 2026-09-24/25 leg-union additions are the retirement sites) ·
  `.claude/rules/testing.md` (in full, 19 Session Additions — :48 mutants/union line; 2026-09-28 "a timing red is never
  fixed by raising a bound: measure the phases on the runner that failed (timing-only pushes allowed)… then remove the
  slow work" sets the 73 s method)
- **Platform issues consulted:** query "macos-latest github actions runner slow first rustc link xcrun cc cold compile"
  → actions/runner-images #12545 / #3885 / #8434 / #10098 describe whole-runner slowness, nothing that isolates one
  nested build. Query "macOS syspolicyd XProtect first execution newly built binary slow cargo test" → FETCHED
  nnethercote.github.io/2025/09/04/faster-rust-builds-on-mac.html states: XProtect scans a new executable on first
  launch, "the XprotectService daemon runs in a single thread, so if you try to launch 10 new binaries at once, the
  slowdown will be more than a second"; disabling it took one test suite "from 9m42s to 3m33s"; the only mitigation it
  names is adding the terminal as a Developer Tool. The search summary (not fetched) also cites reports of a first exec
  of a newly linked binary taking 24–77 s with the cost tracking the debug symbol table — unverified here, a lead only.

## Files inspected
- `.github/workflows/ci.yml` (full) — `mutants` matrix job :264-300 (ubuntu-latest, windows-2025; `run --mutants --leg
  "$LEG"`; uploads `mutants-verdict-<os>.json`), `mutants-verdict` job :302-323 (`needs: mutants`, the ONLY
  `actions/download-artifact` use in the file, `gate --require mutants --mutants-legs ubuntu-latest,windows-2025`). No
  other job `needs:` either. `test` job :40-41 installs `cargo-mutants@27.1.0` "the harness's own tests drive `run
  --mutants` against a temp project"; `msrv` :343 installs it for the same tests under `run --unit`. :159 comment calls
  `chunk.diff` "the mutation leg's diff". 18 check-runs today = test×3 + perf×3 + mutants×2 + mutants-verdict + msrv +
  fuzz-replay + lint×3 + release×3 + supply-chain (re-derived: job/matrix count of this file; CI read 18/18 on
  ci#36473870288) → 15 after.
- `crates/viola-e2e/src/harness/pre_push.rs` :1-268 — `stages` :139-216: tools → sync → cache → linux-tests →
  **linux-leg** (:161) → vm-release → windows-tests → **windows-leg** (:184-202, `run_with(.. mutants ..,
  Some(HOST_LEG))`) → **union** (:203-215, base equality + `gate(.., ["mutants"], Some(legs))`). `Doc.legs` / `Doc.gate`
  exist only for the legs; `cache.windows_scratch_bytes*` (:153, :198) only for the host leg. `LINUX_LEG`/`HOST_LEG`
  consts :35, :39. Module doc :1-6 describes the legs.
- `crates/viola-e2e/src/harness/pre_push/linux.rs` (outline) — `SCRATCH_DIR` :20 + `Linux.scratch` (the distro
  `TMPDIR` scratch, security-plan-ratified carve-out), `cache` :269 wipes/recreates it, `leg` :330, `linux_leg`
  :369-395 (runs `run --mutants --leg ubuntu-latest` under `TMPDIR=`, copies back the verdict). `tools` :138 checks the
  cargo-mutants pin — still needed: the distro's `run --coverage` runs the real-cargo-mutants harness tests.
- `crates/viola-e2e/src/harness/gate.rs` :1-175 — `gate` :59 dispatches `("mutants", Some(legs)) => union` :65;
  `union` :120-163; `parse_legs` :49; `SUITES` keeps `mutants` (the run document's closed suite enum — `run --mutants`
  still writes it); the single-leg `summary_checks` survived-breach :111-114 stays generic.
- `crates/viola-e2e/src/harness/cfg_legs.rs` (outline, 410 lines) — `compiled_legs` :33, a `syn` visitor over
  `#[cfg]`s; its only production caller is `gate::union`.
- `crates/viola-e2e/src/harness/run/mutants.rs` :1-270 — the kept arm: `mutants` :113-230 (diff → classification →
  host scratch → prebuild into `target/mutants` → `cargo mutants --in-diff … --test-tool=nextest --copy-target=true`
  with `NEXTEST_PROFILE=mutants`, `AGENT_RUN_KEEP_*=0`); `leg` threads only into `write_leg_verdict` (:89, :205) and the
  stale-verdict removal (:122-124). `run_private` :250-264 gives the two slow tests their private `CARGO_HOME`.
  Module doc :1 "The per-chunk mutation gate". Writes `target/agent-run/chunk.diff` (:120) — so the secret-scan exact-path
  skip stays load-bearing for a local scan.
- `crates/viola-e2e/src/harness/run/mutants/leg.rs` (83 lines) — `leg_verdict`, `leg_verdict_path`, `write_leg_verdict`.
- `crates/viola-e2e/src/harness/run.rs` :36-62, :91-135, :610-660 — `Selection::from_flags` :53-61 puts `mutants` in
  the DEFAULT selection ("No selector, or `--all`, selects unit, integration and mutants", :51), so a bare `run` or
  `run --all` is still the per-chunk mutation gate; pinned by `selection_defaults_to_unit_integration_and_mutants`
  :613 and the `with_all.mutants` assertion :656. `run` :91 passes `leg: None`; `run_with` :117-124 carries `leg:
  Option<&str>` and refuses `scoped-leg` :130-132; `tool_arms` :281 threads `leg`.
- `crates/viola-e2e/src/bin/viola-harness.rs` (grep) — `--leg` :96-98 (`requires = "mutants"`), `invalid-leg` :148-149;
  `gate --mutants-legs` :71, `parse_legs` :203-205.
- `crates/viola-e2e/tests/cli.rs` :76-90 — `gate_and_leg_usage_errors_are_exit_2` pins `--mutants-legs a/b` and
  `--leg ../x` as `invalid-leg`.
- `crates/viola-e2e/Cargo.toml` (full) — `proc-macro2` / `syn` deps :25, :28 and the machete ignore :31-33 exist only
  for `cfg_legs`; `Cargo.toml` :185-186 / :195-196 workspace pins say so in their comments.
- `C:/Users/turbo/.claude/skills/andromeda-code-audit/references/collectors.md` :56-80 — C1 runs `cargo mutants -p
  {unit} --json --output {run_dir}/mutants-{unit}` directly and classifies host-excluded mutants with its own `cover()`
  recipe; "A project's own union verdict … is NAMED beside the list by pointer … never fetched, never merged".
- `.config/nextest.toml` (grep) — `[profile.mutants]` :20-26 serves the kept arm (`NEXTEST_PROFILE=mutants`).
- `scripts/wsl-provision.sh` (grep) — :166-167 probe the cargo-mutants pin from ci.yml's text; unchanged.
- `viola-0.1.0/chunks/2026-09-28-capability-ledger-and-viola-verify/evidence/macos-mutants-baseline-carry.md` — the six
  measured runs; baseline build 76 s at ci#36448654074 with prebuild 0.2 s and mutant rebuilds 1.3–2.1 s in the same
  copied tree; both tests' baselines end within 0.02 s of each other in every run.
- JUnit of ci#36473870288 (`537ac36`, downloaded to the session scratchpad, never committed) — macOS 108.112 / 108.376 s,
  ubuntu 1.349 / 1.382 s, windows 5.787 / 5.850 s for the two tests; the unbuildable-root case 0.450 s on macOS.

## Graph impact
- **union** — 1 caller: `gate` @ `crates/viola-e2e/src/harness/gate.rs:65`. Dies with the `--mutants-legs` branch.
- **compiled_legs** — production caller only `union` @ `gate.rs:144`; the rest are `cfg_legs.rs`'s own tests.
- **leg_verdict_path** — production callers `union` (`gate.rs:123`), `linux_leg` (`pre_push/linux.rs:375`), `mutants`
  (`run/mutants.rs:123`), `write_leg_verdict` (`run/mutants/leg.rs:50`); tests in `gate.rs`, `pre_push.rs:727`,
  `run/mutants.rs` :523, :552, :690, :712, :807, :825, :833, `leg.rs:79`.
- **parse_legs** — `gate_cmd` @ `crates/viola-e2e/src/bin/viola-harness.rs:203`; test `gate.rs:642-646`.
- **run_with** (signature changes if `leg` goes) — callers `run_cmd` (`viola-harness.rs:161`), `run` (`run.rs:97`),
  pre-push `stages` (`pre_push.rs:185`) and `windows_tests` (`pre_push.rs:252`), and tests in `run/browser.rs` :103,
  :317, `run/coverage.rs` :71, :111, `run/fuzz.rs` :95, :226, `run/mutants.rs` :235, :256, :504, :535, :612, :734, :770,
  :815, :835, `run/perf.rs` :206, :512.
- **host_scratch_bytes** — after pre-push loses its two reads (`pre_push.rs:153`, :198), only `run/mutants/scratch.rs`
  tests and the `run.rs:26` re-export remain.
All names probed > 0 on the rust plane (trace `probe_hits`); lines cited editor-1-based (SCIP line + 1).

## Patterns detected
- **Runner seam** (`run.rs:117` `run_with(.., runner)`): every tool arm takes its commands through `Runner`, so harness
  tests never nest cargo except the three real-cargo mutants tests, which use `run_private` (`run/mutants.rs:250`).
- **Fail-fast staged document** (`pre_push.rs:139`): each stage sets `doc.stage` before it runs; a removed stage leaves
  no gap in the enum order, and `vm-release` then follows `linux-tests` directly.
- **Closed usage codes** (`viola-harness.rs:148`): a flag clap no longer knows exits 2 through the generic `arguments`
  usage path (`cli.rs:86` pins `run --unit --leg x` → `arguments` today).

## Conventions to follow
- **Measure before fixing a timing** (`testing.md` 2026-09-28): phase timings on the runner that failed, in a
  measurement-only push, removed before the chunk commit — the precedent is the capability-ledger chunk's
  `chore(…): … (measurement only)` commits `1027f87`, `65dd401`, `8cc9f14`, then `6486276` removing them.
- **Retired enum values go through a Decisions Log entry** (test-plan §3 Closed enums): `invalid-leg`, `scoped-leg`,
  pre-push stages `linux-leg` / `windows-leg` / `union`, reasons `verdict-missing` / `base-mismatch` — expected wrap
  amendments, not phase edits.
- **Docs are wrap-cascade leaves** (chunk 2026-09-24-quality-gates research.md:51, 2026-09-26 research.md:71):
  `.claude/docs/*` and `.claude/rules/*` are re-derived at wrap, never implement touchpoints.

## New files to create
- none

## Files to modify
- `.github/workflows/ci.yml` — drop the `mutants` and `mutants-verdict` jobs and their comment block; reword the :159 comment
- `crates/viola-e2e/src/harness/pre_push.rs` — drop the windows-leg and union stages, `Doc.legs`/`Doc.gate`, the scratch-byte reads, their tests; module doc
- `crates/viola-e2e/src/harness/pre_push/linux.rs` — drop `linux_leg`, `leg`, the `TMPDIR` scratch and its cache fields and tests
- `crates/viola-e2e/src/harness/gate.rs` — drop `union`, `parse_legs`, the `legs` parameter and their tests (fork A)
- `crates/viola-e2e/src/harness/cfg_legs.rs` — delete the module (fork A)
- `crates/viola-e2e/src/harness/mod.rs` — drop `pub mod cfg_legs` (fork A)
- `crates/viola-e2e/src/harness/run.rs` — mutants leave the default and `--all` selection; drop the `leg` parameter, the `scoped-leg` refusal and the leg re-exports (fork A)
- `crates/viola-e2e/src/harness/run/mutants.rs` — drop the leg threading and leg tests (fork A); module doc; the macOS measurement and cause fix in `run_private`
- `crates/viola-e2e/src/harness/run/mutants/leg.rs` — delete the module (fork A)
- `crates/viola-e2e/src/harness/run/browser.rs` — `run_with` callers lose the `leg` argument (fork A)
- `crates/viola-e2e/src/harness/run/coverage.rs` — `run_with` callers lose the `leg` argument (fork A)
- `crates/viola-e2e/src/harness/run/fuzz.rs` — `run_with` callers lose the `leg` argument (fork A)
- `crates/viola-e2e/src/harness/run/perf.rs` — `run_with` callers lose the `leg` argument (fork A)
- `crates/viola-e2e/src/bin/viola-harness.rs` — drop `run --leg` and `gate --mutants-legs` (fork A)
- `crates/viola-e2e/tests/cli.rs` — retarget `gate_and_leg_usage_errors_are_exit_2` to the retired flags exiting 2 (fork A)
- `crates/viola-e2e/Cargo.toml` — drop `syn`, `proc-macro2` and the machete ignore (fork A)
- `Cargo.toml` — drop the workspace `syn` and `proc-macro2` pins (fork A)
- derived `Cargo.lock` by `cargo check --workspace` — whatever the dependency removal changes (syn and proc-macro2 stay as serde_derive's)

## Open questions
- Delete the leg/union machinery (`run --leg`, `mutants-verdict-<leg>.json`, `gate --mutants-legs`, `cfg_legs`, `syn`
  / `proc-macro2`) or keep it unwired? No caller survives the CI and pre-push removal and the audit never calls it, but
  the directive's "only the per-chunk gate, the pre-push leg and the CI legs go" can be read either way → blocks:
  plan-decision (P4 fork; the files marked "fork A" leave the list under B). **Resolved at P4: A (delete)** — the
  overseer, 2026-09-28: "It has no caller left, and the founder rule is no code without a real consumer … the specs
  must stop describing it."
- The 73 s mechanism (inherited coverage/jobserver environment vs XProtect first-launch scanning vs something else) is
  runner-only and unmeasured → blocks: implementation-scope (the cause fix's file is provisional until the measurement
  push reads the phases; `run/mutants.rs` is listed).
