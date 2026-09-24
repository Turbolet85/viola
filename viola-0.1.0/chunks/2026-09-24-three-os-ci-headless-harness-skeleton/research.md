# Codebase Research — 2026-09-24-three-os-ci-headless-harness-skeleton

## Scope
- **Depth:** minimal (cold start — no Rust source exists) · **Reads:** 6 · **Globs/Greps:** 5 · **Host/remote probes:** 5
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (47 lines); `## Session Additions` is empty (0 additions). No live leg against a real product process exists yet in this chunk.
- **Platform issues consulted:** no runner-only bullet folded (Setup 5a: 0 check-runs on `13b7ee3`, `gh api repos/Turbolet85/viola/commits/13b7ee3…/check-runs` → `total_count 0`), so no failure signature exists to search. The plan's CI-reading entry targets this chunk's first run, so the runner labels were checked against `actions/runner-images` README (fetched 2026-09-24 via `gh api repos/actions/runner-images/contents/README.md`). It says `ubuntu-latest` = Ubuntu 24.04 x64; `macos-latest` = `macos-26` = macOS 26 **arm64**; `windows-2025` = Windows Server 2025 x64 with the VS2026 image (`windows-2025-vs2026`), which is also `windows-latest`. This matches test-plan §9's matrix description (macOS 26 arm64).

## Files inspected
- `.gitignore` (full) — `target/` already ignored (covers `target/e2e-home/`, `target/agent-run/`); `mutants.out/`, `mutants.out.old/` ignored; `Cargo.lock` explicitly committed; `.env*` ignored. No change needed.
- `rustfmt.toml` (full) — `edition = "2024"` only.
- `scripts/agent-run.sh` (full, 73 lines, `i/lf w/lf`) — ALREADY EXISTS (setup-project Phase 4): thin shim `exec cargo run -q -p viola-e2e --bin viola-harness -- "$@"`, dispatch `boot|run|status|cleanup|logs|supervise|ui-restart|gate`, a no-op `ensure_fresh_artifacts`, and its own usage fallback printing `{"v":1,"cmd":null,"ok":false,"reason":"usage"}` + exit 2 on an unknown command.
- `scripts/agent-run.ps1` (full, 82 lines, `i/lf w/lf`) — the PowerShell twin, same dispatch and fallback; forwards `$LASTEXITCODE`.
- `.claude/rules/verification-harness.md` (full) — restates test-plan §3; notes the harness/fake-agent print-lint exemption.
- `.andromeda/test-plan.md` §3 (lines 500-778), §9-§10 (1377-1482) — read directly in P1 for the contract.

## Graph impact
- cold-start — `.andromeda/cache/` holds no `{plane}/tree.db` (`ls .andromeda/cache/*/` → no such directory); `derived-without-graph` for the rust plane. Setup-project states the Rust plane builds at this chunk's wrap.

## Host and remote facts (measured 2026-09-24)
- **Toolchain:** `rustc 1.95.0 (59807616e 2026-04-14)`, default toolchain `stable-x86_64-pc-windows-gnu` (msvc also installed); `rustup check` → stable `1.95.0 -> 1.98.1 (48a229cea 2026-09-01)`; rust-lang/rust releases: `1.98.1` published 2026-09-03 is the current stable. → pin `1.98.1` (≥ 1.96 floor). The host is below the floor: `rust-toolchain.toml` makes rustup install the pin on first `cargo` call in the repo.
- **Host default triple is `-gnu`**, CI windows runners use `-msvc`. The pin names no target, so each side uses its own default host triple. Pure-Rust deps only (no C-build crates), so both link; noted, not a blocker.
- **Tools on host:** `cargo-nextest 0.9.133` (CI pin 0.9.146), `cargo-mutants 27.1.0` (= CI pin), `cargo-llvm-cov`, `cargo-deny`, `jq`, `rg` present; `jaq` MISSING (`command -v jaq` → none). Tool versions are floors per F-19, so 0.9.133 locally is acceptable for the nextest config keys used (junit, retries, slow-timeout, test-groups).
- **Git:** local branches: only `build/viola-0.1.0`; remote heads: only `refs/heads/build/viola-0.1.0` (`git ls-remote --heads origin`). **No `main` / `origin/main` exists.** → `git merge-base HEAD origin/main` cannot resolve anywhere today; the mutation base must come from `AGENT_RUN_CHUNK_BASE`. This chunk's own base is `13b7ee32e6227d1cb6cecc96fbe412fe34a4e0e6` (HEAD at take-up).
- **Action SHAs** (resolved via `gh api repos/{r}/commits/{tag} --jq .sha` / tags listing):
  - `actions/checkout` v7.0.1 → `3d3c42e5aac5ba805825da76410c181273ba90b1`
  - `actions/upload-artifact` v7.0.1 → `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`
  - `taiki-e/install-action` v2.87.19 → `7623a79cdfecb99d681017af368ca353d9f49bb5`
  - `Swatinem/rust-cache` v2.9.2 → `6323deb102c322ba6fcbdcafc7e3dddab59af2b6` (matches security-plan's canonical SHA)
  - `dtolnay/rust-toolchain` `stable` branch head → `6bed0761d98439e5a578e2877258200ad565ba87` (matches security-plan). With a SHA ref the action needs an explicit `toolchain:` input; alternatively the runner's preinstalled rustup honours `rust-toolchain.toml` on the first cargo call.

## Patterns detected
- **Shim usage fallback lives in the shim** (`scripts/agent-run.sh:55-59`): an unknown top-level command never reaches Rust; an unknown FLAG does, so `viola-harness` owns flag-level usage errors (exit 2 + one JSON doc).
- **Setup-seeded shims already match the contract** (`scripts/agent-run.sh:47-50`, `scripts/agent-run.ps1:36-45`): `cargo run -q -p viola-e2e --bin viola-harness --` with stdout/exit forwarded. No rewrite is needed; they become live the moment `crates/viola-e2e` exists.

## Conventions to follow
- **LF pinned** (`.gitattributes`, `git ls-files --eol` → `i/lf w/lf`): every new file LF.
- **Edition 2024 rustfmt** (`rustfmt.toml:1`).
- **Test naming** `<subject>_<condition>_<expected>`, inline `#[cfg(test)] mod tests` (test-plan §2, via tests extract).

## New files to create
- `Cargo.toml` — root `viola` bin package + `[workspace]` (members `crates/*`, resolver 3, `[workspace.package]` edition/rust-version/version/publish, `[workspace.dependencies]`, `[workspace.lints]`, profiles `panic = "unwind"`), feature `fake-agent`, `[[bin]] viola-fake-agent` with `required-features`.
- `Cargo.lock` — generated, committed.
- `rust-toolchain.toml` — `channel = "1.98.1"`, `components = ["rustfmt", "clippy"]`.
- `src/main.rs` — the `viola` bin: panic hook first, `catch_unwind` body, minimal verb surface (P4 fork 1).
- `src/bin/viola-fake-agent.rs` — minimal fake agent.
- `crates/viola-e2e/Cargo.toml`, `crates/viola-e2e/src/lib.rs` (`harness::{boot, run, status, cleanup, logs}` library), `crates/viola-e2e/src/bin/viola-harness.rs` — the harness.
- `.config/nextest.toml` — profiles `ci`, `mutants`, test group `fixed-port`.
- `.github/workflows/ci.yml` — the 3-OS workflow.

## Files to modify
- none existing — `scripts/agent-run.{sh,ps1}` are consumed unchanged (no change: already the contract shape); `.gitignore` no change (covers every new output path).

## Scope premise closure
- Item 4 `[inferred]` skeleton → **VERIFIED**: the tree holds no `Cargo.toml`, `src/` or `crates/` (`ls` at root), so no product verb exists; what `boot` spawns is P4 fork 1.
- Item 5 `[inferred]` fake-agent surface → **VERIFIED** against test-plan §7 line 1319 (`--version` in the real CLI's format for `--cli-version <ver>`); scripted modes stay out.
- Item 6 `[inferred]` per-role line → **VERIFIED** as to fields and location; the role that writes it depends on P4 fork 1 (roles are `run-<name>`, `hook-<name>`, `mcp`, `ui-<port>`, `cli-<name>` per test-plan §3 Log format).
- Item 8 `[inferred]` rust-cache / release build → **resolved**: rust-cache rides this chunk (test-plan §9 Lint/Unit rows; SHA already canonical in security-plan); the release-build step is the Workspace-tree chunk's ("release build free of test binaries").
- Boundaries `[inferred]` unbuilt selectors → **VERIFIED** as out of build scope; their refusal shape is P4 fork 3.
- Item 7 "Runs locally through the harness and in CI" → **[premise-corrected: no `main`/`origin/main` exists locally or on the remote, and test-plan §9 runs the mutation job on pull_request only, while this project pushes one long-lived build branch — so neither the documented base fallback nor the PR-only job ever fires per chunk]** → the CI trigger and base are P4 fork 2.

## Open questions
- Fork 1 — how deep is the boot skeleton: does this chunk add a minimal `viola run` stub (direct child spawn, no PTY/channel) so boot/status/cleanup/logs drive a real process pair, or stop at the contract surface with no product process? → blocks: plan-decision
- Fork 2 — the per-chunk mutation base and CI trigger given no `main` and no PRs. → blocks: plan-decision
- Fork 3 — the refusal shape for `run` selectors whose suites are not built yet (`--browser`, `--coverage`, `--perf`, `--fuzz-replay`, `--local-live`) without an off-enum reason. → blocks: plan-decision
