# tests extract

## Relevance
partial. Items 3, 4, 7 and 8 plus the PREREQ witness are CI and quality-gate surfaces that test-plan owns. Items 2 and 6 (arch tree, TS plane) touch tests only because test-plan names the artifacts that the tree must list. Item 5 (Rust code-graph plane) is outside the tests domain.

## Constraints
- **Release build.** test-plan §9 Pipeline structure (Release build row) requires a per-OS `cargo build --release --bin viola` with the fake-agent feature off. rust-cache is allowed only in `ci.yml`.
  - The plan's command is `--bin viola`, not the bare root `cargo build --release` the scope names. P3 has to measure whether a bare build can pull in `viola-fake-agent` (`required-features`) or the `viola-e2e` bin `viola-harness`.
  - §12 Initial entry (Fake agent placement) sets this intent: `required-features = ["fake-agent"]` keeps the fake agent out of the release build.
  - §6 hook fail-open matrix: the forced-panic hook trigger compiles only with `fake-agent`, so the release `viola` must not contain it.
- **Release-flavoured builds that are not the release job.** `--perf` builds `--release --workspace --features fake-agent` into `target/perf` (§10 Performance budgets, Binary under test). The harness builds into `target/harness` (amendment, three-os-ci). A "no test-only binary" check must be scoped to the release-build job's output, not to these dirs.
- **Lint row.** test-plan §9 Pipeline structure (Lint row) puts these steps in the per-OS `lint` job:
  - `cargo tree -e features -p viola --edges normal`, asserting rmcp shows only `server` and `transport-io`;
  - `cargo modules dependencies --package <crate> --acyclic` per crate;
  - `cargo modules orphans --package <crate> --deny` per crate.

  §9 Build failure conditions, §10 Build failure conditions and §3 Bootstrap `quality-gate-config-emit` all name cargo-modules as a CI exit-code gate, installed via `cargo install --locked cargo-modules@0.27.0` (§3 Bootstrap `ci-tool-install`). If the chunk rules "on demand" (arch wording) instead of wiring it, those four sites need an amendment and a §12 Decisions Log entry.
- **No vacuous passes.** Per test-plan §3 preamble (Exit codes: "never a vacuous pass") and §3 `gate` ("A missing artifact is a breach, never a pass"), no check may pass because its subject does not exist yet.
  - This applies to the rmcp `cargo tree` assertion while rmcp is absent from the graph (no `viola-mcp` yet).
  - It also applies to any release-output check.
- **Supply-chain job.** test-plan §9 Pipeline structure (Supply-chain row) requires fail-closed `shell: bash` steps, cargo-deny JSON written to `target/supply-chain/`, and the `supply-chain` artifact uploaded `if: always()`.
  - Per the §9 tool paragraph, cargo-deny 0.20 takes `--config` as a global option before the subcommand, and `check -c` is rejected.
  - The fuzz audit (`--manifest-path fuzz/Cargo.toml`) is not in the row as written, so the row must be amended when it is wired.
  - §12 quality-gates entry records `fuzz/` as its own workspace with its own lockfile and `fuzz/rust-toolchain.toml`.
- **Tokio ban.** test-plan §9 Supply-chain row and §12 Initial entry (Test crate deviation) require the tokio ban to run once per crate in `scripts/sync-crates.txt`, each as sole root via `deny-sync.toml`.
  - `viola-e2e` is never a root, with no `wrappers` allowlist and no `--exclude`.
  - The Lint row's `cargo check` has one `-p` per listed crate, and an empty list fails.
  - Whether HEAD still holds all of this is research's question.
- **Mutation gate.** test-plan §10 Mutation gate:
  - A diff that names no `.rs` path passes only with the explicit `no-rust-delta` verdict.
  - Any `.rs` change (for example in `viola-e2e`) is `counted` and must show zero missed or timed-out mutants under the two-leg union.
  - §3 `gate` defines the union rule that the PREREQ witness reads: a mutant is red only when no leg caught it and some leg missed it or timed out.
- **New gate values.** Per test-plan §3 Internal harness subcommands (Closed enums: "A new value needs a Decisions Log entry"), a new gate or suite joining `viola-harness gate` needs a §12 entry. `--require` accepts only the nine closed `suite` values.

## Patterns to follow
- End every new CI job with `viola-harness gate --require <exactly the suites that job ran>` (test-plan §3 `gate`; §3 Bootstrap `quality-gate-config-emit`). A job that is a plain exit-code gate with no suite (lint, supply-chain) follows the existing Lint and Supply-chain rows, which have no gate step.
- Install tools version-pinned from exactly one version source: taiki-e SHA-pinned for cargo-deny, `cargo install --locked` for cargo-modules 0.27.0 (test-plan §9 tool paragraph; §3 Bootstrap `ci-tool-install`).
- Write tool JSON under `target/<stage>/` and upload it as a named artifact with `retention-days: 7` (test-plan §9 Supply-chain row, Test report format).
- The artifact inventory the arch tree must name comes from test-plan §2 Test directory + naming conventions:
  - `crates/viola-e2e`, the root `[[bin]] viola-fake-agent`, `e2e-web/tests/*.spec.ts`, `fuzz/fuzz_targets/`, `fixtures/claude/<cli-version>/`, `tests/cmd/`, `tests/snapshots/`, `tests/support/`, `scripts/agent-run.{sh,ps1}`.
  - From §3 Bootstrap: `.config/nextest.toml`, committed `proptest-regressions/`, and `e2e-web/package.json` / `playwright.config.ts`.
  - From §3 and §9: the report dirs `target/agent-run/`, `target/e2e-home/`, `target/harness/`, `target/perf/`, `target/nextest/ci/`, `target/supply-chain/`, `target/secret-scan/`, `target/tools/ripgrep/` and `e2e-web/test-results/`.
- The TypeScript surface test-plan owns is only `e2e-web/`. It is Node test-side only (§12 User review 1: "Node stays test-side only"), and its files arrive with the Web UI chunk (§3 Bootstrap `test-runner-install`, Node side). The plan names no `tsconfig.json`. Whether one exists or is needed for a TS plane is research's question.

## Anti-patterns to avoid
- NEVER use rust-cache in a release workflow; it is `ci.yml` only. NEVER reference an Action by mutable tag; SHA-pin everything, with `permissions: {}` at the top and `contents: read` per job (test-plan §11 CI; §9 least-privilege paragraph).
- NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to fit a new tree (test-plan §11 CI; §10 Stack adjustments).
- NEVER skip quality gates "just this once", and never trust the cargo-mutants exit code alone (test-plan §11 Quality). This applies when reading the PREREQ `mutants-verdict` result: it must come from the per-leg verdict JSON, not a job's exit status.

## Contract bindings
- **tests ↔ security:** the Supply-chain stage (cargo deny, sole-root `deny-sync.toml`, `scripts/deny-probes.sh` ending `13/13 banned, control clean`, zizmor) is the V9 gate (test-plan §2 trigger map, V9 row; §9 Supply-chain row). The fuzz-lockfile audit joins this binding under the operator-ratified test-only exemption.
- **tests ↔ obs:** `target/secret-scan/` is the obs-plan §9 secret-scan report dir, driven by the tests-owned `secret-scan` subcommand (test-plan §3 Internal harness subcommands; §9 Test report format). Listing it in the arch Occupied Resources must match those paths.
- **tests ↔ a11y:** `e2e-web/*` hosts both the Playwright suite and the a11y lint (eslint-plugin-lit-a11y and html-validate over `crates/viola-ui/` sources and `assets/index.html`), run in the ubuntu browser job (test-plan §9 E2E row; §12 fix pass 3, Z6).
- **tests ↔ arch:** §12 Initial entry (Test crate deviation) says the arch statement "No separate test crate is declared" needs updating. The chunk's arch-tree work (item 2) is where that request lands.

## Acceptance criteria contributions
- A per-OS CI job runs `cargo build --release --bin viola` with the fake-agent feature off. A check fails the job if the release output contains `viola-fake-agent[.exe]` or `viola-harness[.exe]`, and that check must fail, not pass, when its input is missing (per test-plan §9 Pipeline structure, Release build row; §12 Initial entry, Fake agent placement; §3 `gate`, missing artifact is a breach).
- The per-OS `lint` job runs `cargo modules dependencies --acyclic` and `cargo modules orphans --deny` per crate, plus the rmcp `cargo tree -e features` assertion, and the assertion does not pass vacuously while rmcp is absent. Otherwise test-plan §9 Lint row, §9 and §10 Build failure conditions and §3 Bootstrap are amended with a §12 Decisions Log entry (per test-plan §9 Pipeline structure, Lint row).
- The `supply-chain` job runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` as a fail-closed bash step, with JSON output under `target/supply-chain/`, and test-plan §9 Supply-chain row names it (per test-plan §9 Pipeline structure, Supply-chain row).
- The chunk's own mutation verdict is green. It is `no-rust-delta` if the diff has no `.rs` path. Otherwise it is `counted` with the union over `ubuntu-latest` and `windows-2025` showing zero mutants that are uncaught and missed or timed out. Any new `viola-harness gate` suite value has a §12 Decisions Log entry (per test-plan §10 Mutation gate; §3 Closed enums).

## Relevant amendment history
- **2026-09-24-quality-gates** (two-leg mutation union, seeded fuzz replay, separator-agnostic ignore regex, rustup MSRV):
  - This chunk's PREREQ witness reads that chunk's `mutants-verdict-ubuntu-latest` artifact, the `msrv` log and the union job.
  - Why: cargo-mutants reports other-OS `#[cfg]` bodies as missed (CI run 36005608858, `file_mode`), and the fuzz workspace is separate with its own lockfile.
  - The fuzz-lockfile audit CARRY comes from this chunk.
- **2026-09-24-supply-chain-and-workflow-gates** (Lint row from `sync-crates.txt`, new Supply-chain stage, wrappers sentence retired):
  - This is the tokio-ban mechanism item 3 re-confirms.
  - Why: the report disproved the wrappers mechanism, and `--exclude` false-fails on a feature-unified optional tokio.
- **2026-09-24-three-os-ci-headless-harness-skeleton** (harness in its own target dir; `viola-e2e` no-op `fake-agent` feature; mutation prebuild with `--copy-target=true`):
  - Relevant to item 4. The fake-agent feature is enabled via `--features viola/fake-agent` into `target/harness`, and `viola-e2e` declares a no-op `fake-agent` feature.
  - A release-output check must not confuse these build dirs with the release profile.
  - Why: Windows `os error 5` relinking the running harness exe, and cargo-mutants scoping the baseline to the touched packages.
- **2026-09-24-fake-agent-and-test-data-fixtures** (mutation verdict for Rust-free diffs):
  - A chunk that is mostly `ci.yml` and docs gets `no-rust-delta`, but touching `viola-e2e` makes it `counted`.
  - Why: cargo-mutants exits 0 on Rust-free diffs and leaves stale `outcomes.json`.
- **2026-09-24-observability-gates** (scan-gated uploads, `target/secret-scan/`, ripgrep in the `lint` job):
  - This is the source of the report dirs that item 2 must list in arch Occupied Resources.
  - Why: the unscanned `agent-run-<os>` upload was retired.
