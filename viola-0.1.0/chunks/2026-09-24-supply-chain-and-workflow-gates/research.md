# Codebase Research — 2026-09-24-supply-chain-and-workflow-gates

## Scope
- **Depth:** moderate (policy + CI chunk; no Rust source in the modify-set) · **Reads:** 9 · **Globs/Greps:** 7 · **Probes:** 4 scratch-workspace cargo-deny runs + 1 read-only draft-config run against the repo
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (full, 0 Session Additions) · `.claude/rules/testing.md` (full, 3 Session Additions — the throwaway-temp-Cargo-project rule for tests that drive cargo applies to any permanent negative probe)
- **Platform issues consulted:** no runner-only bullet (CI on the last shipped sha `61f9676` is 4/4 success). One platform-semantics fetch for the weekly run: GitHub docs "Events that trigger workflows" (fetched 2026-09-24) state "Scheduled workflows run on the latest commit on the default branch." and, for `workflow_dispatch`, "This event will only trigger a workflow run if the workflow file exists on the default branch." The repo's default branch is `build/viola-0.1.0` (`gh api repos/{owner}/{repo} --jq .default_branch`; no `origin/main` ref exists), so a `schedule:` / `workflow_dispatch:` workflow pushed on this branch is live at once and a dispatched run can witness the advisory job in this chunk.

## Files inspected
- `.github/workflows/ci.yml` (full) — `permissions: {}` top (L7), `contents: read` per job (L13-14, L87-88), every `uses:` full-SHA + version comment (L22, 27, 28, 76, 90, 96, 97, 107), `persist-credentials: false` on both checkouts (L24, L93), event payload only via `env:` (`AGENT_RUN_CHUNK_BASE`, L102-103). Jobs: `test` (3-OS matrix) and `mutants` (ubuntu). No lint/deny/zizmor step, no `schedule:` trigger.
- `Cargo.toml` (root, L1-80) — `[workspace] members = ["crates/*"]`, resolver 3, `publish = false` at `[workspace.package]` (L61); every third-party dep `=`-pinned in `[workspace.dependencies]`; `tracing-subscriber` features `fmt,json,registry,std` (no `env-filter`).
- `crates/viola-core/Cargo.toml`, `crates/viola-e2e/Cargo.toml` (full) — both `publish.workspace = true`; viola-core deps `nutype` (+ dev `proptest`); viola-e2e deps `viola-core, clap, serde, serde_json, sysinfo, tempfile, thiserror` — no tokio.
- `crates/` listing — exactly `viola-core`, `viola-e2e` (re-derived: `ls crates`). `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui` do not exist.
- `.claude/docs/commands.md` L5, L9-10, L50 — already names `cargo install --locked … cargo-deny@0.20.2` and `… zizmor@1.30.1`, and says `deny.toml` lands with a later chunk (a wrap-cascade leaf to re-derive).
- `.andromeda/test-plan.md` L1411 (tool sourcing), L756-762 (`ci-tool-install`, `reason:"tool-missing"`), L1617 (viola-e2e "added to the cargo-deny tokio wrappers list").
- `scripts/agent-run.sh` L14 — the `run` suite enum (`--unit|--integration|--e2e|--browser|--mutants|--coverage|--perf|…`); no policy suite.

## Graph impact
- **graph not applicable** — the modify-set is `deny.toml`, a second deny config and workflow YAML; no Rust symbol is added, changed or called, so the code-graph query is skipped (no caller set to enumerate).

## Measured facts (each with its derivation)
- **Tokio is absent from the whole graph today**, all targets, normal+build+dev: `cargo tree -i tokio --target all -e normal,build,dev` → exit 101 `package ID specification 'tokio' did not match any packages`.
- **Graph size / licences:** 167 packages (`cargo metadata --format-version 1 --locked`, `len(packages)`); licence expressions are MIT/Apache-2.0 families plus `Zlib` (1 sole), `Unicode-3.0` (1, in an AND), `Unlicense OR MIT`, `BSD-2-Clause OR …`, `MIT-0`, `LGPL-2.1-or-later` (only as an OR alternative), 3 `None` = the workspace crates.
- **Bare `cargo deny check` (no config) exits 4**: 82 `error[rejected]` licence errors + `error[unlicensed]` for `viola`, `viola-core` (scratchpad `deny-bare.txt`); advisories/bans/sources ok.
- **A draft config (security §deny.toml additions + obs bans + licence allow `MIT, Apache-2.0, Zlib, Unicode-3.0` + `private = { ignore = true }`) passes on the repo**: `cargo deny check -c <draft>` exit 0, `advisories ok, bans ok, licenses ok, sources ok`; warnings only: `advisory-not-detected` (the RUSTSEC-2017-0008 ignore — a WARNING, not a failure), `license-not-encountered` (`MIT-0` — unneeded in the allow-list), `duplicate` (`syn` ×2). `--format json` also exit 0. Local tool: cargo-deny **0.19.4**.
- **Tokio-ban scoping — the load-bearing equality, measured in a scratch 3-crate workspace** (`sync → chan`, `chan` with optional `tokio` feature, `mcp` enabling `chan/tokio` + direct tokio), ban config = `[bans] deny = [tokio]`, `[graph] targets` = 3 triples, `exclude-dev = true`:
  - `cargo deny --exclude mcp check bans` → **exit 2, FALSE FAIL** (`tokio ← chan ← sync`): workspace feature unification keeps `chan`'s `tokio` feature on after `mcp` is excluded. So `--exclude` (and `[graph] exclude`) does NOT express "the sync graph without the async crates".
  - `cargo deny --manifest-path sync/Cargo.toml check bans` → **exit 0** (sync as sole root, `chan` reached through a feature-less edge → tokio pruned).
  - same form with `tokio` added to `sync`'s own deps → **exit 2** (`error[banned]`). Equality holds: sole-root form passes a tokio-free sync crate and fails one that pulls tokio.
  - `cargo deny --manifest-path chan/Cargo.toml check bans` (± `--no-default-features`, ± `--exclude mcp`) → **exit 2, FALSE FAIL**: `cargo metadata` reports `chan`'s features as the workspace-unified `['tokio']` even with `chan` as root. ⇒ The crate that OWNS the optional tokio feature (`viola-channel`) cannot be checked as its own root once `viola-mcp` enables that feature; it is covered as a dependency of the other sync roots.
  - real repo: `cargo deny --manifest-path crates/viola-core/Cargo.toml check bans -c <tokio-ban config>` → exit 0; a non-existent manifest path → exit 1 (`--manifest-path must point to a Cargo.toml file`) — fail-closed, no vacuous pass.
- **Ban effect on absent crates** (scratch workspace, draft config): adding `tracing-subscriber` with `env-filter` → exit 2 `error[feature-banned]`; adding `cc` → exit 2 `error[banned]`. A ban on a crate absent from the graph emits NO diagnostic, so its effect is only observable by a negative probe.
- **zizmor**: absent on the host (`zizmor: command not found`); no `uvx`/`pipx`; PyPI has `zizmor 1.30.1` (`python -m pip index versions zizmor`). Findings on HEAD's `ci.yml` are therefore UNMEASURED.

## Patterns detected
- **Least-privilege workflow shape** (`.github/workflows/ci.yml:7,13-14,22-24`): top `permissions: {}`, job `contents: read`, SHA-pinned `uses:` with `# vX.Y.Z`, `persist-credentials: false`, `rustup toolchain install` from `rust-toolchain.toml` (L25-26), tools via SHA-pinned `taiki-e/install-action` `tool:` list (L28-31). New jobs copy this shape.
- **Fail-closed per-OS shell split** (`ci.yml:33-46`): pwsh step checks `$LASTEXITCODE` per command on Windows; bash elsewhere.
- **Artifact upload** (`ci.yml:74-81`): `if: always()`, SHA-pinned upload-artifact, `retention-days: 7`.

## Conventions to follow
- **Tool floors, CI pins:** CI pins `cargo-deny@0.20.2` in the taiki-e `tool:` list and `cargo install --locked zizmor@1.30.1` (test-plan L1411; commands.md L9-10); security §Tool-version syntax states floors (`>=`).
- **No new harness suite:** a new `run`/`gate` suite value needs a test-plan Decisions Log entry first (test-plan §3 Closed enums; verification-harness.md L23) — the policy gates run as direct CI steps / local commands.
- **Mutation gate:** a diff with no `.rs` path yields `verdict:"no-rust-delta"` (verification-harness.md L43).

## New files to create
- `deny.toml` (repo root, per arch tree) — the workspace policy: advisories · licenses · sources · bans (C-crates, telemetry crates, feature bans incl. `tracing-subscriber/env-filter`); `[graph] targets` = the three runner triples.
- A second cargo-deny config holding only the tokio ban (+ `[graph]` targets, `exclude-dev = true`), run once per sync crate as sole root via `--manifest-path` (a separate file is forced: the workspace `deny.toml` cannot ban tokio, which `viola-mcp`/`viola-ui` will legitimately use, and a `wrappers` allowlist is rejected by arch).
- `.github/workflows/nightly.yml` OR a `schedule:` trigger on `ci.yml` — the weekly `cargo deny check advisories` home (plan-decision, below).

## Files to modify
- `.github/workflows/ci.yml` — add the ubuntu supply-chain job (deny all four families + JSON artifact; zizmor + JSON artifact) and the per-OS tokio-free check (`cargo check` of the present sync crates + the sole-root tokio-ban run), each fail-closed.
- `.claude/docs/commands.md` / `CLAUDE.md` Workflow gate list — wrap-cascade leaves (not /implement touchpoints).

## Open questions
- Weekly advisory home: `nightly.yml` (test-plan §9 Platform) vs a `schedule:` trigger inside `ci.yml` (arch §CI/CD "one workflow", obs §9 Platform; security "separate workflow trigger") → blocks: plan-decision.
- Permanence of the negative ban probes: a one-time recorded proof at /implement vs a permanent CI probe (a throwaway temp Cargo project that pulls each banned crate/feature and asserts `cargo deny check bans` fails — network + build cost on every run) → blocks: plan-decision.
- zizmor's verdict on HEAD `ci.yml` (e.g. whether its cache-poisoning audit flags `Swatinem/rust-cache` on an unfiltered `push:` trigger) → blocks: implementation-scope (first measured at /implement after installing zizmor 1.30.1).
