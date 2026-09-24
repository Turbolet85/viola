# security extract

## Relevance
Partial. The chunk is behaviour-preserving cleanup. Security applies at four points: the `MAX_FRAME` pin, which witnesses a security-plan constant; the `config.json` read at its sole consumer; the `viola-harness` surfaces that CI's security gates depend on (secret-scan, mutants verdict); and keeping the fake agent test-only.

## Constraints
- security-plan §Input Validation → Constants sets `MAX_FRAME = 16 MiB`, one const in `viola-core`. §Security Decisions Log (2026-09-23 Phase 3.5, "`MAX_FRAME` stays 16 MiB by default"; arch amendment 6) fixes that value. The chunk pins the value and must not change it. A pin written as `16 << 20` or `16 * 1024 * 1024` both satisfy the 16 MiB requirement.
- security-plan §Input Validation, Configuration values row, requires the `config.json` read, which lives in root-bin `viola::obs`, to be capped at `MAX_FRAME`. If P3 picks a boundary witness at `src/obs.rs:195`, it should show that input over the cap is not fully consumed and that the read cannot succeed without the cap. Whether the current code already falls back to `info` or `parse-rejected` on an over-cap read is a question for research.
- security-plan §Bootstrap phases, `secret-scanning-ci-gate`, and §Secret Management ("Secret scanning in CI") require two things of `viola-harness`. First, `secret-scan` keeps its canary classes, never prints or writes matched bytes, and still gates the `diag-`, `junit-` and `harness-<os>` uploads. Second, the harness-authored `mutants-verdict-<os>.json` contains only repo-relative source locations and outcomes, with no absolute path, argv, log path or test output. If the `run.rs` split or the `viola-harness.rs` `main` decomposition moves either code path, both must stay byte-identical in behaviour. Which submodule each path lives in is for research to establish.
- security-plan §Threat Model Summary → Infrastructure (CI/CD) requires the per-OS release build to go through `scripts/release-check.sh`, which refuses any test-only binary. The `viola-fake-agent` `main` decomposition must keep the binary behind the `fake-agent` feature and out of the `--bin viola` release build.
- security-plan §Dependency Security → Pinning pins every third-party version in `[workspace.dependencies]`, and `cargo deny check` gates the root lock. The clean-up needs no new crate. Any dev-dependency that shared test helpers would bring in has to pass the same gate.
- security-plan §Security Anti-Patterns → Data Protection (the fixtures ban) and → Secrets (the `.env` / local `--home` ban) apply to the test-helper extraction in `tests/cli_fake_agent.rs` and the `src/main.rs` / `src/obs.rs` test modules. Extracted helpers keep synthetic content only, and no test home or recorded state is committed.
- security-plan §Bootstrap phases, `logging-redaction-wire` (the NEVER-log floor), applies to the shared panic-line and diag-detail validator helpers between `main.rs` and `obs.rs`. Merging the two copies must not weaken any assertion that witnesses the floor, such as no absolute paths or upstream text in stderr and diagnostics output. Whether the current asserts witness the floor is for research to determine.

## Patterns to follow
- One shared const in `viola-core`, read by every consumer (security-plan §Input Validation → Constants). The pin test should reference `viola_core::MAX_FRAME` and not copy the literal into consumers.
- `Read::take(MAX_FRAME)` wraps every external reader (security-plan §Bootstrap phases, `input-validation-library-install`). A consumer-side witness exercises this wrapper at the boundary.
- CI tests use their own `--home` (security-plan §Secret Management → Development). Shared helpers keep per-test homes and never share state across tests.
- Harness JSON and verdict output is repo-relative and content-free (security-plan §Bootstrap phases, `secret-scanning-ci-gate`, the admissible-by-content uploads).

## Anti-patterns to avoid
- security-plan §Security Anti-Patterns → Input bans calling `read_to_end` or `read_line` on external input without `Read::take(MAX_FRAME)`. A refactor or witness must not add an uncapped read path, including in test helpers that feed the real reader.
- security-plan §Security Anti-Patterns → API bans skipping request size limits, and the cap is `MAX_FRAME`. No mutation-kill test or helper may raise or bypass the constant.
- security-plan §Security Anti-Patterns → Universal bans letting `config.json`, a `VIOLA_*` variable or a CLI flag switch off a control. Extracted harness or fake-agent helpers must not add a knob that disables the cap or the strict-modes check.

## Contract bindings
- security ↔ tests: the `MAX_FRAME` pin and any `obs.rs:195` boundary witness are test-plan-owned cases of a security constant (security-plan §Input Validation → Parser surfaces: "tests owns the cases"). The founder rule's remove-the-guard mutation run is where each pin is shown to catch the value mutants.
- security ↔ obs: `viola-harness secret-scan` is obs-plan §9 step 3, checked against security-plan's NEVER-log floor (§Secret Management). A split that moves it must keep that obs contract.
- security ↔ tests CI integration: the `mutants-verdict-<os>.json` content rule (security-plan §Bootstrap phases, `secret-scanning-ci-gate`) binds to the CI mutation legs that this chunk's `--in-diff` scope widens.

## Acceptance criteria contributions
- A `viola-core` unit test asserts `MAX_FRAME == 16 MiB`. Each of the four mutants at `lib.rs:9` (`*`→`+`, `*`→`/` at `:9:31` and `:9:38`) is caught, and the remove-the-guard run fails the test (per security-plan §Input Validation → Constants; §Security Decisions Log 2026-09-23 Phase 3.5).
- `cargo deny check` and the fuzz lockfile audit stay green with no new third-party crate in the root graph (per security-plan §Dependency Security → CI integration).
- `scripts/release-check.sh` still refuses test-only binaries after the fake-agent refactor, so the release build carries `viola` only (per security-plan §Threat Model Summary → Infrastructure CI/CD).
- The `secret-scan` step and the `mutants-verdict-<os>.json` output behave the same after the harness split: canary classes are unchanged, matched bytes are never emitted, and the verdict holds repo-relative paths only (grep of the verdict for absolute paths finds nothing) (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`).

## Relevant amendment history
- 2026-09-24-diagnostics-plane (§Input Validation Configuration row; Constants). This added the `config.json` read, in root-bin `viola::obs`, to the `MAX_FRAME` consumer list, with the closed `diagnostics_level` fallback. Why: the chunk shipped the read and the detector noted the omission. This is the sole consumer (`src/obs.rs:195`) behind the candidate boundary witness.
- 2026-09-24-observability-gates (§Secret Management; §Bootstrap `secret-scanning-ci-gate`). This recorded that CI runs `viola-harness secret-scan` before every test-home upload and never emits matched bytes. Why: the new CI contradicted the old "no scanning" text. That code is in the `viola-harness` binary this chunk decomposes.
- 2026-09-24-quality-gates (§Bootstrap `secret-scanning-ci-gate`; Decisions Log). This replaced the `mutants.out/` upload with the harness-authored `mutants-verdict-<os>.json`, admissible by content as "repo-relative source locations only, never absolute paths". Why: the operator ratified it, because the raw output held absolute argv paths. The rule binds whatever harness code the split moves.
- 2026-09-24-workspace-tree-and-code-graph-planes (§Threat Model CI/CD job list). This put the per-OS release build through `scripts/release-check.sh` (refuses any test-only binary). Why: the chunk shipped it. The same chunk deferred the Rust gates this chunk closes, and the release-check rule bounds the fake-agent refactor.
