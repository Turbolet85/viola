# security extract

## Relevance
Partial. The chunk's CI workflow, workspace/lockfile pinning and toolchain floor are supply-chain items in this domain. Its one diagnostics line and kept test homes touch the file-permission and NEVER-log floors. cargo-deny, zizmor, the weekly advisory run and full redaction belong to later chunks (per the scope's Boundaries).

## Constraints
- security-plan §Dependency Security (CI integration) requires every `uses:` in `.github/workflows/ci.yml` to be pinned by full commit SHA with a version comment. The rule covers `actions/checkout`, `actions/upload-artifact`, any toolchain or install action, and `Swatinem/rust-cache` if P4 puts it in this chunk. The plan names canonical SHAs for `dtolnay/rust-toolchain` (`6bed0761d98439e5a578e2877258200ad565ba87 # stable`) and `Swatinem/rust-cache` (`6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2`). §Security Anti-Patterns § Code Patterns forbids mutable refs (`@stable`, `@v2`).
- security-plan §Dependency Security (CI integration) requires workflow-level `permissions: {}` and job-level `contents: read`. No job may widen these.
- security-plan §Dependency Security (Pinning) requires `Cargo.lock` to be committed and `[workspace.dependencies]` to pin every third-party version exactly. It also moves the host toolchain to Rust `>=1.96` to match CI's stable. Each dependency the harness or fake agent adds now (e.g. serde_json) must be pinned exactly and come from crates.io only, so the later `[sources]` `unknown-git`/`unknown-registry = "deny"` gate passes without rework.
- security-plan §Dependency Security (`deny.toml` additions; the existing arch bans stay) keeps the arch bans on C-building crates and on `tokio` in the sync crates' graph. cargo-deny itself is not wired until the Supply-chain chunk, but dependencies picked here must not introduce C-build crates. serde_json must never enable `unbounded_depth` (§Security Anti-Patterns § Input).
- security-plan §Data Protection (At rest, Logs) and §Security Anti-Patterns § Data Protection / § Logging require every `diagnostics/` file to be 0600 inside a 0700 directory on Unix, never created with the default umask. This covers the chunk's `<home>/diagnostics/<role>.ndjson` writer even though `viola-state` does not exist yet. Whether the one-line writer sets `OpenOptionsExt::mode(0o600)` and creates its directories 0700 is research's question.
- security-plan §Bootstrap phases (`logging-redaction-wire`, NEVER-log floor) and §Security Anti-Patterns § Secrets forbid the per-role JSON line, the panic hook output and the harness JSON envelopes from carrying `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value, or any upstream text. This matters most because kept homes are uploaded as CI artifacts. The full redaction floor belongs to the Log redaction chunk; this chunk must just not dump environment variables or payloads.
- security-plan §Bootstrap phases (`secret-scanning-ci-gate`) and §Security Anti-Patterns § Secrets require `.gitignore` to exclude local `--home` test directories so recorded state never lands in git. Here that means `target/e2e-home/` and `target/agent-run/`, which are normally covered by ignoring `target/`.

## Patterns to follow
- security-plan §Dependency Security (CI integration): write each action as `uses: owner/repo@<40-hex-sha> # <tag>`. Put `permissions: {}` at the top and `permissions: contents: read` on each job.
- security-plan §Dependency Security (Tool-version syntax): the plan states external CLI tool versions as minimum floors, not exact pins. Pinning the CI install of nextest and cargo-mutants to one version (per the scope) does not conflict with this; the plan must not also record an exact pin.
- security-plan §Dependency Security (CI integration, zizmor): write `ci.yml` so it already passes zizmor's checks before the Supply-chain chunk turns zizmor on. That means no template injection: pass `github.event.pull_request.base.sha` to `AGENT_RUN_CHUNK_BASE` through `env:` instead of interpolating `${{ }}` into `run:`, and grant no excess permissions.
- security-plan §Data Protection (At rest, Logs): use the 0600-file / 0700-directory creation pattern (`OpenOptionsExt::mode`, never the umask) for any file a product role writes under a home.

## Anti-patterns to avoid
- NEVER reference a GitHub Action by `@stable`, `@v2` or any other mutable ref (per security-plan §Security Anti-Patterns § Code Patterns).
- NEVER create files under a home with the default umask, and NEVER create `diagnostics/` files readable by other users (per security-plan §Security Anti-Patterns § Data Protection and § Logging).
- NEVER commit local `--home` test directories or `.env` files (per security-plan §Security Anti-Patterns § Secrets).

## Contract bindings
- security ↔ tests §CI Integration: `ci.yml` is the one workflow. Its SHA pins and `permissions: {}` / `contents: read` settings are security's. The later `cargo deny check` job (ubuntu), the zizmor step and the weekly `schedule:` advisory trigger join this same workflow in the Supply-chain chunk (security-plan §Dependency Security, CI integration).
- security ↔ obs §3 (per-role JSON line, panic hook): obs owns the format and the panic-hook ordering. Security adds the 0600/0700 file-mode floor and the NEVER-log floor on that line and on panic stderr (security-plan §Bootstrap phases `logging-redaction-wire`; §Error Handling, Internal logging).
- security ↔ tests §3 (kept homes and artifacts): `AGENT_RUN_KEEP_HOMES=1` plus the `agent-run-${{ matrix.os }}` upload publishes home contents. They must hold no secrets or inherited `CLAUDE*` values (security-plan §Secret Management, Storage).

## Acceptance criteria contributions
- Every `uses:` line in `.github/workflows/*.yml` matches `@[0-9a-f]{40}` followed by a `# <version>` comment, and none uses a tag or branch ref. A grep verifies this (per security-plan §Dependency Security, CI integration).
- `ci.yml` has `permissions: {}` at workflow level, and every job declares at most `contents: read` (per security-plan §Dependency Security, CI integration).
- `Cargo.lock` is committed, every entry in `[workspace.dependencies]` has an exact version, and no dependency uses a `git =` or `path =` source outside the workspace (per security-plan §Dependency Security, Pinning).
- On Unix, `<home>/diagnostics/<role>.ndjson` is created with mode 0600 and its parent directory is 0700. The line holds no `CLAUDE*` environment values (per security-plan §Data Protection, Logs, and §Bootstrap phases `logging-redaction-wire`).

## Relevant amendment history
(none). `D:/dev/projects/viola/.andromeda/security-plan-amendments.md` does not exist yet. The plan's inline Security Decisions Log has only the 2026-09-23 initial entry and the Phase 3.5 review. Neither touches CI, the harness or toolchain pinning beyond what §Dependency Security already states.
