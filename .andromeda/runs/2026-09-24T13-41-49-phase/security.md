# security extract

## Relevance
Partial. The chunk is CI and harness gate work. Security applies to the CI workflow rules, new tools and dependencies, fuzz corpus contents, and the carried `mutants.out/` upload scan. None of the product-boundary controls apply.

## Constraints
- Every new `uses:` must be pinned to a full commit SHA with a version comment. This covers the MSRV job's toolchain step and any fuzz or nightly steps (per security-plan §Dependency Security, CI integration; §Security Anti-Patterns, Code Patterns, the mutable-ref ban).
  - The plan's pinned-action set is exactly four actions: checkout, rust-cache, install-action and upload-artifact. Toolchains come from a `rustup toolchain install` step, not a toolchain action (amendment 1 below).
  - So the scope's "dtolnay/rust-toolchain `1.96` (SHA-pinned)" either becomes a `rustup toolchain install 1.96` step, or adds a fifth pinned action and amends §Dependency Security.
- New and edited workflows keep workflow-level `permissions: {}` and job-level `contents: read`. Any event-payload value reaches a step only through `env:`, never through `${{ }}` inside `run:` (per security-plan §Dependency Security, CI integration). This covers `github.event.before` for the mutants diff base, and any `concurrency:` group expression the item-7 decision adds.
- zizmor keeps running over `.github/workflows/` and keeps failing on unpinned actions, `excessive-permissions`, template injection and cache poisoning (per security-plan §Dependency Security, CI integration).
  - The item-7 `concurrency-limits` decision may add a block or record a rejection. It must not suppress, ignore or re-persona any zizmor finding. The scope's Boundaries section says the same: never widen an ignore or silence zizmor.
- `cargo deny check` keeps failing on any advisory, yanked crate or unknown source. The C-build, feature and tokio bans stay live (per security-plan §Dependency Security, `deny.toml` additions and CI integration).
  - New lock-pinnable dependencies (proptest 1.11.0, arbitrary 1.4.2, and any fuzz-side crates) must be exact-pinned in `[workspace.dependencies]` or the fuzz manifest. External CLI tools (cargo-llvm-cov, cargo-fuzz) use minimum floors (per security-plan §Dependency Security, Pinning and the Tool-version syntax note).
  - Research must answer three questions: does `fuzz/`'s libfuzzer-sys (C++ build) trip the C-build ban if it enters the main graph; how is `fuzz/` kept out of the deny graph; and does the fuzz manifest's own dependency set get any audit.
- Uploads that carry test output leave CI only after `viola-harness secret-scan` passes, checked against the NEVER-log floor (per security-plan §Secret Management, "Secret scanning in CI"; §Bootstrap phases `secret-scanning-ci-gate`).
  - The `mutants.out/` upload is recorded as unscanned there, as a CARRY on "Quality gates". This chunk must either scan it before upload or stop uploading it.
  - New artifacts must follow the same rule if they carry test output: LCOV, `llvm-cov-summary.json`, fuzz crash or replay output, and gate verdicts.
- Committed fuzz corpora and proptest seeds are repository fixtures. Their inputs must be synthetic, and anything taken from recorded payloads must be checked for home paths, usernames and tool `input` before it is committed (per security-plan §Data Protection, "Repository fixtures").
- The nightly fuzz toolchain is not covered by the exact-pin rule for `rust-toolchain.toml` (per security-plan §Dependency Security, Pinning). Whether the nightly should be pinned to a dated channel is research's question.
  - Also: `nightly.yml` is specified as "no cache" (§Dependency Security, CI integration). If a time-boxed fuzz job is added there, it must not add a cache restore to that workflow unless the plan is amended.

## Patterns to follow
- Use the parser-surface list as the fuzz and property target set: `validate_paste_text`, the `Last-Event-ID` parser, channel ndjson framing with `MAX_FRAME`, the hook stdin parser (including `hook statusline`), the `claude agents --json` output parser, and the vt100 feed under `catch_unwind` (per security-plan §Input Validation, "Parser surfaces"). Which of these parsers exist at HEAD is research's question. Missing ones are recorded forward, not stubbed.
- Fuzz targets that exercise capped readers should drive them through the real `Read::take(MAX_FRAME)` path, using the `MAX_FRAME` const from `viola-core` (per security-plan §Input Validation, Constants).
- Put the scan gate before uploads the same way the `test` job does: `id: secret-scan`, `if: always()`, and uploads conditioned on its success (per security-plan §Secret Management, "Secret scanning in CI").
- Toolchain installs go through `rustup toolchain install`, not a toolchain action (per security-plan §Dependency Security, CI integration; amendment 1).

## Anti-patterns to avoid
- NEVER reference a GitHub Action by `@stable`, `@v2`, `@1.96` or any other mutable ref (per security-plan §Security Anti-Patterns, Code Patterns).
- NEVER commit fixtures (here, fuzz corpora or proptest seeds) recorded from sessions with real, non-synthetic prompts, or before checking them for home paths and usernames (per security-plan §Security Anti-Patterns, Data Protection).
- NEVER let an env var or flag switch off a plan control (per security-plan §Security Anti-Patterns, Universal). The `LLVM_PROFILE_FILE` pass-through for `env_clear()` tests may preserve only that variable. It must not re-admit stripped `CLAUDE*` values or turn into a general env pass-through in the tests that check the R8 strip.

## Contract bindings
- CI security gate ↔ tests §CI Integration and §9 rows: supply-chain, zizmor and secret-scan jobs stay in the same workflows. The new MSRV, coverage and fuzz-replay jobs follow the same `permissions` and SHA-pin discipline.
- `mutants.out/` upload ↔ obs-plan §9 artifact canary scan (`viola-harness secret-scan` scan roots). Scan roots must be extended to `mutants.out/`, or the upload removed.
- Fuzz corpus / proptest seeds ↔ tests §7 recorded payloads (`fixtures/claude/<cli-version>/`): these must be synthetic, with no real PII.
- `concurrency:` decision ↔ tests (mutants chunk-diff base via `github.event.before`, passed through `env:` only).

## Acceptance criteria contributions
- (security) `zizmor .github/workflows/` passes on `ci.yml` and `nightly.yml` with no new ignore, suppression or persona change. Every `uses:` line in both files carries a 40-hex SHA plus a version comment, and no `${{ github.event.* }}` appears inside a `run:` body (per security-plan §Dependency Security, CI integration).
- (security) `cargo deny check` passes on the ubuntu supply-chain job, and `scripts/deny-probes.sh` still proves every ban live. `fuzz/` adds no crate to the audited graph that would trip the C-build ban (per security-plan §Dependency Security, `deny.toml` additions).
- (security) The `mutants` job either has no `mutants.out/` upload, or has a `secret-scan` step over `mutants.out/` that the upload is conditioned on. The same holds for every other new upload that carries test output (per security-plan §Secret Management, "Secret scanning in CI").
- (security) Committed `fuzz/corpus/**` and `proptest-regressions/**` contain no home path or username: grep for `C:/Users/`, `/home/` and `/Users/` returns no real username (per security-plan §Data Protection, "Repository fixtures").

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton (pinned actions and toolchain source):** set the pinned-action set to exactly what `ci.yml` used. It replaced `dtolnay/rust-toolchain` with a `rustup toolchain install` step reading `rust-toolchain.toml`, fixed the 1.98.1 pin and the 1.96 `rust-version` floor, and required event-payload values to reach steps via `env:` only. Why: the chunk shipped `ci.yml`. This matters here because the scope's MSRV item names dtolnay again.
- **2026-09-24-supply-chain-and-workflow-gates:** added `nightly.yml` (weekly `schedule` + `workflow_dispatch`, no cache) as a separate workflow for `cargo deny check advisories`, plus the ubuntu supply-chain job (deny, sole-root tokio ban, ban probes, zizmor). The `deny-probes.sh` live-ban proof was recorded. The pinned-action set was left unchanged because no new action was added. Why: that chunk shipped these. This matters because the scope adds a fuzz run to `nightly.yml` and CARRYs the zizmor `concurrency-limits` finding from that chunk.
- **2026-09-24-observability-gates:** §Secret Management and §Bootstrap `secret-scanning-ci-gate` now record CI's obs canary scan before every test-home upload, and note the `mutants.out/` upload as unscanned: a CARRY on "Quality gates" (this chunk). Why: new CI contradicted "no CI secret scanning". Resolving the CARRY here likely requires a follow-up amendment to those two sites.
