# security extract

## Relevance
Relevant. This chunk carries out security-plan §Bootstrap phases `dep-audit-tooling-install` and `dep-security-ci-gate`, and it builds the Supply-chain attack-surface controls listed in §Threat Model Summary.

## Constraints
- `deny.toml` must include the security additions in security-plan §Dependency Security ("`deny.toml` additions"):
  - `[advisories]`: `unmaintained = "all"`, `unsound = "all"`, `yanked = "deny"`. The only ignore is RUSTSEC-2017-0008, and it needs a `reason` that names portable-pty `=0.8.1` and says serial ports are never opened.
  - `[sources]`: `unknown-registry = "deny"` and `unknown-git = "deny"`, so only crates.io is allowed.
  - `[[bans.features]]`: rmcp denies `transport-streamable-http-server` and `auth`; axum denies `http2`.
  - The plan says this is the only expected ignore and must be re-checked whenever the portable-pty pin moves. Research must answer whether an ignore for an advisory missing from today's graph triggers cargo-deny's "advisory-not-detected" diagnostic, and whether that diagnostic can fail the gate.
- The existing arch bans must stay as written: C-building crates, and `tokio` in the sync crates' graph for three target triples (per security-plan §Dependency Security). §Data Protection and the Decisions Log open question also depend on the C-build ban: the SHA-256 crate picked later must be pure Rust and pass it. So the C-build ban's class must be real in effect, not just the three names listed.
- CI wiring must follow security-plan §Dependency Security "CI integration":
  - Job 4 (ubuntu) runs `cargo deny check` over all four families and fails on any advisory, yanked crate or unknown source.
  - A separate `schedule:` weekly cron runs `cargo deny check advisories`, because the advisory DB changes without code changes (default `maximum-db-staleness` P90D).
  - `zizmor .github/workflows/` runs on the ubuntu leg and fails on unpinned actions, `excessive-permissions`, template injection and cache poisoning.
  - The plan puts the weekly run in "a separate workflow trigger" on `ci.yml`, while the scope notes that the test-plan names a `nightly.yml`. Whichever file hosts it must meet every constraint in this list.
- Least privilege applies to every workflow file (per security-plan §Dependency Security, CI integration):
  - workflow-level `permissions: {}` and job-level `contents: read`;
  - `actions/checkout` with `persist-credentials: false`;
  - event-payload values reach steps only through `env:`, never through `${{ }}` inside `run:`;
  - the toolchain comes from a `rustup toolchain install` step that reads `rust-toolchain.toml`, never from a toolchain action.
- Every `uses:` must be pinned by full commit SHA with a version comment. security-plan §Dependency Security lists the pinned set: checkout `3d3c42e5…` v7.0.1, rust-cache `6323deb1…` v2.9.2, install-action `7623a79c…` v2.87.19, upload-artifact `043fb46d…` v7.0.1. Any action this chunk adds, such as an installer for cargo-deny or zizmor, must meet the same rule.
- Tool floors, not exact pins: cargo-deny `>=0.20.2` and zizmor `>=1.30.1` (per security-plan §Dependency Security, Audit tool and the Tool-version syntax note). cargo-audit is not gated in v1 and is reserved for v1.x `cargo audit bin`, which matches the chunk's Boundaries.
- Pinning floors the deny gate sits beside (per security-plan §Dependency Security, Pinning):
  - `Cargo.lock` is committed;
  - rmcp takes the minor range `>=3.4.1, <3.5`, and `Cargo.lock` must resolve bytes `>=1.11.1`;
  - tracing-subscriber `>=0.3.20` if obs picks it;
  - notify stays on 8.2.0.
  These crates come in with later chunks, so this chunk only writes the policy they will be checked against.

## Patterns to follow
- Look up action SHAs with `gh api repos/{repo}/commits/{tag}` and write them as `owner/action@<full-sha> # vX.Y.Z` (per security-plan §Dependency Security, CI integration; amendment 2026-09-24-three-os-ci-headless-harness-skeleton).
- Write an ignore for an advisory in a disabled feature or an unused path as an `ignore = [{ id, reason }]` entry whose `reason` names why it does not apply (per security-plan §Dependency Security, Critical CVE response SLA).
- Carry the event payload into steps through `env:` (e.g. `AGENT_RUN_CHUNK_BASE`), as the existing `ci.yml` pattern does per security-plan §Dependency Security. Research should confirm that the shipped `ci.yml` actually does this.
- Use zizmor as the check that asserts the least-privilege and pinning rules, rather than relying on manual review (per security-plan §Dependency Security, CI integration).

## Anti-patterns to avoid
- NEVER reference a GitHub Action by `@stable`, `@v2` or any mutable ref (per security-plan §Security Anti-Patterns › Code Patterns).
- NEVER enable rmcp's `transport-streamable-http-server` or `auth` features, or axum's `http2` feature (per security-plan §Security Anti-Patterns › API). In this chunk these bans take the form of `[[bans.features]]` entries.
- NEVER restore `Swatinem/rust-cache` in the v1.x release workflow (per security-plan §Security Anti-Patterns › Code Patterns). No release workflow is in scope, but no workflow this chunk adds may be shaped as a release or publish path that restores the cache. Research must check whether zizmor's cache-poisoning audit flags any trigger on the scheduled workflow.

## Contract bindings
- CI security gate ↔ tests §CI pipeline structure / dependency-policy entity. The deny, zizmor and weekly-advisory steps are jobs in one workflow set that the tests plan owns. The job placement (ubuntu leg, the scheduled home in `ci.yml` vs `nightly.yml`) must agree across both plans.
- The telemetry-crate ban (`opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender`) and the `veil` → `toggle` feature ban ↔ obs-plan otel-sdk-install / pii-scrubbing-wire. The security plan has no list of these entries; obs owns their content. On the security side they back §Error Handling "Error reporting integration: None", which means no outbound reporter.
- The tokio-in-sync-graph ban ↔ arch §Build system. Security's only requirement is that it stays unchanged (§Dependency Security); arch owns the mechanism.
- The C-build ban ↔ security-plan Decisions Log open question (the SHA-256 crate must pass it before the `viola-state` `bin/` chunk).

## Acceptance criteria contributions
- `cargo deny check` (all four families) passes on ubuntu CI, and `deny.toml` contains `unmaintained = "all"`, `unsound = "all"`, `yanked = "deny"`, `unknown-registry = "deny"`, `unknown-git = "deny"`, and exactly one advisory ignore (RUSTSEC-2017-0008 with a `reason`) (per security-plan §Dependency Security).
- The rmcp `transport-streamable-http-server`/`auth` and axum `http2` feature bans are shown to work by a negative probe that makes `cargo deny check bans` fail when a banned crate or feature is added. Config text alone does not count, because the crates are not in today's graph (per security-plan §Dependency Security, `[[bans.features]]`; §Security Anti-Patterns › API).
- `zizmor .github/workflows/` exits 0 on the ubuntu leg over every workflow file, and a CI step fails the build on its findings (per security-plan §Dependency Security, CI integration).
- A workflow with an `on: schedule:` weekly cron runs `cargo deny check advisories`. Every workflow file has top-level `permissions: {}`, `contents: read` per job, SHA-pinned `uses:` with version comments, and `persist-credentials: false` on checkout. A grep finds no `${{ github.event… }}` inside `run:` (per security-plan §Dependency Security, CI integration; §Security Anti-Patterns › Code Patterns).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton: pinned actions and toolchain source** (§Dependency Security, CI integration and Pinning; §Threat Model Summary, Supply chain).
  - **What changed:**
    - The SHA-pinned action set became exactly what `ci.yml` uses: checkout v7.0.1 with `persist-credentials: false`, rust-cache v2.9.2, install-action v2.87.19, upload-artifact v7.0.1.
    - `dtolnay/rust-toolchain` was replaced by a `rustup toolchain install` step that reads `rust-toolchain.toml` (1.98.1 exact, `rust-version` floor 1.96).
    - Event-payload values reach steps only through `env:`.
  - **Why:** that chunk shipped `ci.yml` and `rust-toolchain.toml`, and the plan was brought into line with them. The mutable-ref ban (§Anti-Patterns › Code Patterns) was left unchanged.
  - **Effect on this chunk:** this chunk edits that same `ci.yml`. Any action it adds, for example to install cargo-deny or zizmor, extends the pinned set and needs a follow-on amendment to that list.
