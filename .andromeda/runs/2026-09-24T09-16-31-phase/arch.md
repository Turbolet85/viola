# arch extract

## Relevance
Relevant. The chunk lands the dependency-policy and CI jobs that architecture §Infrastructure Patterns (Build system; CI/CD approach) assigns to "Supply-chain gates". It also enforces the §Established Decisions [Concurrency] KEYSTONE.

## Constraints
- Tokio containment is a KEYSTONE. Per architecture §Established Decisions [Concurrency / Backend Framework] and §Cross-cutting Patterns (Tokio containment), CI must enforce it two ways. The first is a `cargo deny` `tokio` ban evaluated for the Windows, macOS and Linux target triples. The second is a `cargo check` of every sync crate on all three OSes without `viola-channel`'s `tokio` feature (CI job 3). Only `viola-mcp` and `viola-ui` may list `tokio`, per §Established Decisions [Module Boundaries].
- The tokio ban covers the whole normal-dependency graph, direct or transitive, of `viola-core`, `viola-pty`, `viola-channel` (without its feature), `viola-state` and `viola-agent-claude`. It is evaluated with `viola`, `viola-mcp` and `viola-ui` excluded, and a direct-parent allowlist is explicitly not acceptable (per architecture §Infrastructure Patterns → Build system). The plan does not name `viola-e2e` in that exclusion list. The scope adds it, so the plan text and the chunk differ here (see amendment note below).
- The same Build system bullet requires the dependency policy to ban C-building crates (`cc`, `libsqlite3-sys`, `openssl-sys`) and to cover licences. §Established Decisions [Database / State Store] adds the rationale "no C in the build" (SQLite was rejected), and so does the state-file primitives row in §Stack and Technologies ("no C code").
- Tool pins: cargo-deny 0.20.2 (per architecture §Stack and Technologies → Code quality row). zizmor has no pin or row anywhere in the plan. The scope's 1.30.1 is a new tool version that arch does not yet register.
- The CI shape is fixed by architecture §Infrastructure Patterns → CI/CD approach:
  - one workflow, `ci.yml`, triggered on push and pull_request;
  - job 3 is the sync-crate `cargo check` per OS;
  - job 4 is `cargo deny check`, run once on ubuntu;
  - least privilege: `permissions: {}` at the top, `contents: read` per job, every `uses:` pinned by full SHA with a version comment, and checkout with `persist-credentials: false`.
- Per architecture §Established Decisions [Module Boundaries] and §Occupied Resources (Workspace crates), each product crate is created by the first chunk that consumes it. The job-3 package list and the tokio-ban root set in the plan name crates that may not exist yet. Which workspace members exist today is research's question. The gate has to follow the plan's full target list without failing, or passing vacuously, on absent packages.
- Per architecture §Infrastructure Patterns → Build system and §Inherited Defaults (Publishability), every workspace member sets `publish = false` and nothing goes to crates.io in v1. This matters to the licences family, which may treat private or unpublished workspace crates specially. Whether each manifest actually carries `publish = false` is research's question.

## Patterns to follow
- Install version-pinned cargo tools (cargo-deny, and possibly zizmor) with SHA-pinned `taiki-e/install-action` v2.87.19, next to the existing `actions/checkout` v7.0.1 and `Swatinem/rust-cache` v2.9.2 setup. Install the toolchain with `rustup toolchain install` from `rust-toolchain.toml`, never a toolchain action (per architecture §Infrastructure Patterns → CI/CD approach and §Stack and Technologies → CI/CD row).
- Pass event-payload values to steps only through `env:`. The `mutants` job's base sha (`github.event.before` / the PR base, passed via `env:`) is the existing precedent (per architecture §Infrastructure Patterns → CI/CD approach, "Jobs wired today").
- Pin every third-party version once in `[workspace.dependencies]`, with portable-pty `=0.8.1` and rmcp `~3.4` (per architecture §Infrastructure Patterns → Build system). The advisory ignore for `serial` via portable-pty `=0.8.1` is tied to this pin, per the scope and §Established Decisions [PTY].
- Put `deny.toml` at the repo root, as the directory tree in architecture §Infrastructure Patterns → Project directory structure shows.
- The Cross-platform discipline in architecture §Cross-cutting Patterns requires every OS-specific branch to compile on its own runner. Job 3 is the per-OS proof of that for the sync crates.

## Anti-patterns to avoid
- Do not implement the tokio ban as a direct-parent allowlist (cargo-deny `wrappers`) or as a single root `[bans] deny` entry scoped by parent. Architecture §Infrastructure Patterns → Build system rejects this because Tokio "could otherwise arrive unnoticed through a third-party feature such as interprocess's or notify's".
- Do not add a toolchain action or unpinned or mutable `uses:` refs, and do not widen permissions beyond `contents: read` per job (per architecture §Infrastructure Patterns → CI/CD approach).
- Do not let any new infrastructure add hosting, a deploy stage, telemetry, reporting or egress. v1 is local-only with no deploy stage (per architecture §Infrastructure Patterns → Deployment model and CI/CD approach, and §Established Decisions [Hosting]).

## Contract bindings
- arch ↔ security: the security plan's §Dependencies owns the `deny.toml` contents: advisories settings, the sources lock, the licence allow-list, the CVE SLA and the action-pinning policy. Arch owns only the gate's placement (job 4, ubuntu) and the tokio and C-crate bans.
- arch ↔ obs: the telemetry-crate bans (`opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender`) and the `veil` `toggle` feature ban come from obs (otel-sdk-install, pii-scrubbing-wire). Arch's side is the §Stack and Technologies Logging row: tracing plus tracing-subscriber with fixed feature sets, root bin only. Obs owns the logger. The bans must not catch the pinned `tracing` / `tracing-subscriber`.
- arch ↔ tests: the test plan's §CI pipeline structure and its dependency-policy entity bind the CI job layout. Two points are open between them:
  - The weekly advisory run's home is either a `schedule:` trigger on `ci.yml` or a `nightly.yml`. This conflicts with architecture §CI/CD approach's "one workflow `ci.yml`, push and pull request".
  - The gate may be exposed through `scripts/agent-run.{sh,ps1}` and `viola-harness` (§Occupied Resources → Binary; test-plan §3 command contract).
- arch ↔ tests (graph scoping): the test-only `viola-e2e` sits in the same workspace graph as the tokio ban. Whether it pulls `tokio`, and so whether it must be excluded from the ban graph, is research's question.

## Acceptance criteria contributions
- `cargo deny check` runs once on ubuntu as CI job 4 with cargo-deny 0.20.2. A negative probe proves that `tokio` pulled transitively into a sync crate's normal-dependency graph fails the gate on each of the three target triples, while `tokio` under `viola-mcp` / `viola-ui` does not (per architecture §Infrastructure Patterns → Build system; §Established Decisions [Concurrency / Backend Framework]).
- CI job 3 runs `cargo check` on windows-2025, macos-latest and ubuntu-latest over the sync crates that exist, without `viola-channel`'s `tokio` feature. Each crate the plan lists but that is still absent joins with a one-line addition, and the job never passes vacuously (per architecture §Infrastructure Patterns → CI/CD approach, job 3; §Established Decisions [Module Boundaries]).
- A negative probe proves `cc`, `libsqlite3-sys` and `openssl-sys` are each denied by the bans family (per architecture §Infrastructure Patterns → Build system).
- Every workflow the chunk adds or edits has `permissions: {}` at the top, `contents: read` per job, full-SHA `uses:` with version comments, and checkout with `persist-credentials: false`. zizmor passes over `.github/workflows/` (per architecture §Infrastructure Patterns → CI/CD approach).

## Relevant amendment history
- 2026-09-24-three-os-ci-headless-harness-skeleton, "CI setup and wired jobs" (§Stack CI/CD row; §Infrastructure Patterns CI/CD approach):
  - This amendment recorded the rustup-from-`rust-toolchain.toml` install, the SHA-pinned action set (checkout 7.0.1, rust-cache 2.9.2, install-action 2.87.19, upload-artifact 7.0.1), `permissions: {}` at the top with `contents: read` per job, and the two wired jobs (`test`, `mutants`).
  - It kept the six target jobs and marked them as owned by later chunks. Jobs 3 and 4 belong to this chunk.
  - Why: the prior chunk shipped `ci.yml`. The same sweep also amended 2 security-plan sites.
  - This chunk inherits that baseline. Expect wrap-time amendments for:
    - the zizmor tool and its version, which have no Stack or Code-quality row;
    - the weekly advisory trigger or workflow, which departs from "one workflow, push and pull_request";
    - the Code-quality row and the `deny.toml` tree comment, which list only "licences, C-crate bans, tokio bans", while the chunk adds advisories, sources, telemetry bans and feature bans;
    - `viola-e2e`'s place in the ban-graph exclusion list.
- 2026-09-24-three-os-ci-headless-harness-skeleton, "toolchain floor and exact pin": Rust 1.98.1 is pinned exactly and the workspace `rust-version` is 1.96. This matters because cargo-deny 0.20.2 and zizmor must install and run under that setup. Why: intent F-19.
- 2026-09-24-three-os-ci-headless-harness-skeleton, "test-only crate, bins, env vars, paths": this amendment registered `viola-e2e` as a test-only workspace member and stated that each product crate is created by its first consumer. Both bear on the tokio-ban root set and exclusions, and on the job-3 package list. Why: the report's Crates and Symbols sections, where the grep count was 0.
