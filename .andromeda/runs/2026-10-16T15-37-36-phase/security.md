# security extract

## Relevance
partial — no product trust boundary moves, but the chunk edits the supply-chain gates, the CI workflows, the pre-push launcher's child environment, `deny.toml` / `deny-sync.toml` and the artifact secret scan, all of which this plan governs.

## Constraints
- The browser test leg (the `e2e-web/` npm lockfile and `scripts/npm-audit.sh`, the pinned Node and `scripts/install-node.sh`, Playwright, and their CI and pre-push steps) is named in the plan as standing until this entry removes it, and this entry's wrap amends that text. Implement edits no plan text. The plan for this chunk lists as Expected amendments the npm-graph and Node-pin bullets, the `supply-chain` step and the `npm-advisories` job, the `Node (pinned)` toolchain sentence, and the pre-push `tools` stage's `NODE_PIN_VERSION` check (per security-plan §Dependency Security).
- An audit gate goes only together with the graph it audits. The npm lockfile audit (CI `supply-chain` step and nightly `npm-advisories`) is removed in the same change that deletes `e2e-web/package-lock.json`, so no committed lockfile is left without an audit. The root `Cargo.lock` audit, the `fuzz/Cargo.lock` audit and the ConPTY vendor verification must stay as they are (per security-plan §Dependency Security).
- The pre-push launcher must still be `/usr/bin/env -i` with exactly `HOME` and `PATH`. Every PATH component must stay a constant under the passwd home or a system dir. `HOME` must still come only from the two `id -u` / `getent passwd` probes. Dropping `<home>/.local/viola-node/bin` is a narrowing. No assignment may take a host value, and no uid-0 launch may be added (per security-plan §Secret Management).
- The pre-push `tools` stage must still install nothing and must not become a new version source. After the Node check goes, it must still check `cc`, the `rust-toolchain.toml` channel and the nextest / mutants / llvm-cov pins parsed from ci.yml's `test`-job `tool:` line, with the same `tool-missing` / `tool-pin-mismatch` refusals (per security-plan §Dependency Security).
- Workflow edits to `ci.yml`, `nightly.yml` and `windows-mutants.yml` must keep: every action pinned by full SHA; workflow `permissions: {}` and job `contents: read`; no `github.event` value in a `run:`; no toolchain action; no new `uses:`. Any upload that stays scan-gated must still wait for `secret-scan` (per security-plan §Dependency Security).
- `deny.toml` / `deny-sync.toml`: rewording the tokio-confinement comments must not change any ban set. The rmcp `transport-streamable-http-server` / `auth` feature bans, the C-crate bans and the sole-root tokio ban per sync crate must stay. Removing the `axum` `http2` feature ban is a change to this plan's `[[bans.features]]` list. Its `deny-probes.sh` probe must go with it, or both stay, so every remaining ban is still proven live (per security-plan §Dependency Security).
- The artifact secret scan must keep its scope, which is every file under `target/agent-run/` except the exact-path `chunk.diff`. It must never print or write matched bytes, and scan-gated uploads must still wait for it to pass. Removing the `token-query` / `gui-token` classes narrows the class list the plan names: `?t=`, `cookie:` / `viola_<port>=`, and the GUI token from `ui/*.url`. The stripped `CLAUDE*` canaries, the content canary and the non-0600 diagnostics check must stay. Which harness classes map to `?t=` and to `cookie:` / `viola_<port>=` is research's question (per security-plan §Secret Management).

## Patterns to follow
- One audit per lockfile, never an exemption, following the `fuzz/Cargo.lock` precedent. After removal, the `supply-chain` job covers the root lockfile, the fuzz lockfile and the vendored ConPTY binaries (per security-plan §Dependency Security).
- `scripts/deny-probes.sh` shows that every `deny.toml` / `deny-sync.toml` ban fires. Re-run it after any ban or comment edit (per security-plan §Dependency Security).
- The canary pair proves the pre-push crossing: a launch through the launcher with an outer `CLAUDE_CANARY` reads 0 `CLAUDE*` names, and the same launch without `-i` reads 1. Re-run it after the PATH narrowing (per security-plan §Secret Management).

## Anti-patterns to avoid
- NEVER reference a GitHub Action by a mutable ref while editing the workflows. Use a full SHA with a version comment (per security-plan §Security Anti-Patterns → Code Patterns).
- NEVER reach green by exempting a still-present graph or silencing a gate instead of removing the graph it audits. A gate leaves only together with its subject (per security-plan §Dependency Security).

## Contract bindings
- The secret-scan class list binds to obs-plan §9 (the artifact canary scan step) and to the test-plan harness's `secret-scan` verb. The classes are removed in one pass, with the obs-plan text amended at the wrap.
- The pre-push launcher's `env -i` PATH binds to test-plan's pre-push stages and to `.claude/rules/security.md` (Dependencies and CI). The rule's PATH literal follows the plan amendment through the wrap's cascade.
- The CI `supply-chain` / `nightly.yml` job sets bind to test-plan §CI Integration. ci.yml's `gate --require` suite list and the `junit-playwright.xml` scan-gated upload change together with the harness `gate.rs` suite names.

## Acceptance criteria contributions
- `cargo deny check`, each per-sync-crate `deny-sync.toml` run and `bash scripts/deny-probes.sh` all pass. Every ban still in `deny.toml` / `deny-sync.toml` has a probe that fires, and no probe names a ban that was removed (per security-plan §Dependency Security).
- `zizmor .github/workflows/` passes with no new finding. Every `uses:` in `ci.yml`, `nightly.yml` and `windows-mutants.yml` is a full SHA. Workflow `permissions: {}` and job `contents: read` are unchanged, and no workflow step installs or runs `node` / `npm` / `npx` (per security-plan §Dependency Security).
- The pre-push launcher still carries exactly `HOME` and `PATH`, its PATH holds no `.local/viola-node` element, and the canary pair still reads 0 `CLAUDE*` names through the launcher and 1 without `-i` (per security-plan §Secret Management).
- `viola-harness secret-scan` still fails on a planted stripped-`CLAUDE*` canary and on a non-0600 diagnostics file (Unix). Every scan-gated upload in `ci.yml` still runs only when it passes (per security-plan §Secret Management).
