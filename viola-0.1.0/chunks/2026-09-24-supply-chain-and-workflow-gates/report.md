# Report — 2026-09-24-supply-chain-and-workflow-gates

**Chunk:** deny.toml (4 families, C/telemetry/tokio/feature bans), deny + zizmor CI gates, weekly advisory run, tokio-free sync check, least-privilege workflows
**Date:** 2026-09-24T09:45:00Z
**Commits:** none since last_wrap (2026-09-24T09:00:42Z); HEAD `61f9676` — this wrap commits the chunk

## Changes (structured — detectors read this)
- **Files:**
  - new: `deny.toml`, `deny-sync.toml`, `scripts/sync-crates.txt`, `scripts/deny-probes.sh`, `.github/workflows/nightly.yml`
  - modified: `.github/workflows/ci.yml` (jobs `lint` and `supply-chain` appended; `test` and `mutants` byte-identical — `git diff` shows only additions after the `mutants` job)
  - chunk folder: `viola-0.1.0/chunks/2026-09-24-supply-chain-and-workflow-gates/{scope,research,plan,report}.md`
- **Symbols / APIs:** none (no Rust source changed: `gate.py delta` puts only `deny.toml` in its rust bucket, and `--defer-check rust` reads 0 changed files read by a rust source).
- **Crates / modules:** none added, removed or changed. Workspace members stay `viola` (root), `viola-core`, `viola-e2e` (`ls crates`).
- **Dependencies:** none added or bumped (`Cargo.toml` / `Cargo.lock` untouched).
- **Schema / config:**
  - `deny.toml` (workspace policy, cargo-deny 0.20.2): `[graph] targets` = `x86_64-pc-windows-msvc`, `aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`; `[advisories] unmaintained = "all"`, `unsound = "all"`, `yanked = "deny"`, one ignore RUSTSEC-2017-0008 with a reason; `[licenses] allow = ["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]`, `private = { ignore = true }`; `[sources] unknown-registry = "deny"`, `unknown-git = "deny"`; `[bans] deny` = `cc`, `libsqlite3-sys`, `openssl-sys`, `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender` (each with a reason); `[[bans.features]]` = `veil`→`toggle`, `rmcp`→`transport-streamable-http-server`+`auth`, `axum`→`http2`, `tracing-subscriber`→`env-filter` (each with a reason). No tokio entry, no `wrappers`, no `skip`/`allow` exception.
  - `deny-sync.toml`: only `[bans] deny = [tokio]`, the same three `[graph] targets`, `exclude-dev = true`; run once per sync crate as sole root: `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans`.
  - `scripts/sync-crates.txt`: the single sync-crate list — `viola-core` (one line); CI and local gates fail on an empty list.
- **Spec-master edits:** none during implement (this wrap's P2 applies the amendments below).
- **Counts / qualifiers moved:**
  - arch §Stack Code-quality row / tree comment "licences, C-crate bans, tokio bans" (architecture.md:36, :418) → the policy now carries 4 families: advisories, licences, sources, bans (C, telemetry, feature bans); the tokio ban lives in `deny-sync.toml`.
  - CI workflows 1 → 2 (`ci.yml` + `nightly.yml`); `ci.yml` jobs 2 → 4 (`test`, `mutants`, `lint`, `supply-chain`). Basis: `.github/workflows/*.yml` at the working tree.
- **Dev-tool versions:** dev host (Windows): **cargo-deny** (dependency-policy tool) 0.19.4 → 0.20.2 and **zizmor** (workflow linter) absent → 1.30.1, both by `cargo install --locked` at 2026-09-24T09:34Z (`.andromeda/runs/2026-09-24T09-34-05-implement/tool-install.log`: "Replaced package `cargo-deny v0.19.4` with `cargo-deny v0.20.2`", "Installed package `zizmor v1.30.1`"). CI image: cargo-deny 0.20.2 via taiki-e `tool:` list, zizmor 1.30.1 via `cargo install --locked` (in `ci.yml`). **cargo-deny 0.20.2 CLI change measured:** `--config <PATH>` is a global option given BEFORE `check`; `check -c` is rejected (`error: unexpected argument '-c' found`, gate log 4 of the first implement run).
- **Harness / gate surface:**
  - `ci.yml` job `lint` (matrix windows-2025 / macos-latest / ubuntu-latest, `contents: read`): `test -s scripts/sync-crates.txt` then `cargo check` with one `-p` per listed crate, no `--features` (arch CI job 3).
  - `ci.yml` job `supply-chain` (ubuntu, `contents: read`): taiki-e `cargo-deny@0.20.2`, `cargo install --locked zizmor@1.30.1`, version self-checks, then fail-closed bash steps: `cargo deny --format json check` (JSON on stderr → `target/supply-chain/deny.json`), the sole-root tokio ban per listed crate, `bash scripts/deny-probes.sh`, `zizmor --format=json .github/workflows/` (→ `target/supply-chain/zizmor.json`); `if: always()` upload of `target/supply-chain/` as artifact `supply-chain`, 7 days.
  - `.github/workflows/nightly.yml`: `on: schedule` (cron `17 4 * * 1`) + `workflow_dispatch`, `permissions: {}`, job `advisories` (ubuntu, `contents: read`): checkout, `rustup toolchain install`, taiki-e `cargo-deny@0.20.2`, `cargo deny check advisories`; no cache.
  - `scripts/deny-probes.sh`: 13 negative probes (throwaway Cargo projects under `target/deny-probes/run-<utc>-<pid>/`, each with its own empty `[workspace]`), each must exit non-zero with its own diagnostic (`error[banned]: crate '<c> = ` / `error[feature-banned]: feature '<f>' for crate '<c> = `), plus a clean control under both configs; last line `deny-probes: 13/13 banned, control clean`; `tool-missing: cargo-deny` exit 1 when the tool is absent.
  - New gitignored output dirs: `target/deny-probes/`, `target/supply-chain/` (`git check-ignore -q` exit 0 on both).
  - No new harness suite (`scripts/agent-run.*` untouched).
  - Actions: no new action; the four already-pinned SHAs reused (checkout v7.0.1, rust-cache v2.9.2, install-action v2.87.19, upload-artifact v7.0.1).
- **Cross-project / external claims:**
  - GitHub docs "Events that trigger workflows" (fetched 2026-09-24): "Scheduled workflows run on the latest commit on the default branch."; `workflow_dispatch` "will only trigger a workflow run if the workflow file exists on the default branch." Repo default branch = `build/viola-0.1.0` (`gh api repos/{owner}/{repo} --jq .default_branch`).
  - CI read at take-up: sha `61f9676882bbc06116ff9b47fc5c7328e8de1373`, check-runs test ×3 + mutants all `success`. This chunk's own CI run is read after this wrap's push (operator entries).
- **Reverted / negative API facts:** the first-written `ci.yml`, `scripts/deny-probes.sh` and `deny-sync.toml` comment used `check bans -c <config>` (the 0.19.4 form P3/P5 ran under); replaced by `--config` before `check` after cargo-deny 0.20.2 rejected it.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. architecture.md:392 (§Infrastructure Patterns → Build system) — the tokio ban "is checked over the graph with `viola`, `viola-mcp` and `viola-ui` excluded". Measured false as a mechanism (P3 scratch workspace, cargo-deny 0.19.4, re-confirmed by the 0.20.2 probes): `cargo deny --exclude <async crate> check bans` FALSE-FAILS when a shared crate's optional tokio feature is enabled by the excluded crate (workspace feature unification: `cargo metadata` reports the unified features). The sole-root form (`--manifest-path crates/<sync>/Cargo.toml`) decides the case: passes a tokio-free crate, fails one that pulls tokio (`error[banned]`). Limit: the crate OWNING the optional tokio feature (`viola-channel`) false-fails as its own root once `viola-mcp` enables that feature — it is covered via the other sync roots that depend on it. Evidence: research.md §Measured facts.
  2. test-plan.md:1617 — viola-e2e "is added to the cargo-deny tokio wrappers list". Disproved as the mechanism: under sole roots `viola-e2e` is not a root, no `wrappers` list exists, and a `wrappers` allowlist is the direct-parent form architecture.md:392 rejects.
- **Expected amendments (from plan):**
  - architecture §Infrastructure Patterns directory tree + §Occupied Resources (deny-sync.toml, scripts/sync-crates.txt, scripts/deny-probes.sh, nightly.yml; `target/deny-probes/`, `target/supply-chain/`) — carried: Changes → Files / Harness surface. Sites: `grep -n "deny.toml" architecture.md` → 1 hit (:418 tree); `grep -n "target/" architecture.md` → §Occupied Resources filesystem list :380-382.
  - architecture §Build system (tokio ban mechanism) — carried: Spec claims disproved 1. Site: architecture.md:392 (1 hit, `grep -n "excluded, not with a direct-parent"`).
  - architecture §CI/CD approach + §Stack Code-quality row (lint/supply-chain jobs, nightly.yml, zizmor 1.30.1, families) — carried: Changes → Harness surface, Counts moved, Dev-tool versions. Sites: architecture.md:36 (Code-quality row), :456 ("one workflow `ci.yml`", 1 hit `grep -n "one workflow"`), :459-466 (jobs wired / target jobs 3-4); zizmor: 0 hits in architecture.md (`grep -n zizmor` → none) = a new row.
  - test-plan §9 Lint row + L1617 wrappers sentence — carried: Spec claims disproved 2, Harness surface. Sites: test-plan.md:1399 (Lint row), :1617 (`grep -n wrappers` → the only tokio-wrappers hit).
  - obs-plan §9 Platform (nightly.yml beside ci.yml) — carried: Harness surface. Site: obs-plan.md:1217 ("single workflow `.github/workflows/ci.yml`", `grep -n Platform` → 1 CI hit).
  - Not listed in the plan, same fact, found by the same grep: a11y-plan.md:1108 ("one workflow `ci.yml`") — `ci.yml` stays the single push/PR workflow; `nightly.yml` is schedule/dispatch only (a11y extract: must not fork the E2E/a11y leg — it does not).
  - Matrix: no ledger-note entry listed.
- **Coverage of new surfaces:**
  - `ci.yml` job `lint` → validation n/a · instrumentation n/a · PII n/a · tests CI (exit-code gate) · a11y n/a · tokens n/a
  - `ci.yml` job `supply-chain` → validation n/a · instrumentation n/a · PII n/a · tests CI (exit-code gates + JSON artifacts) · a11y n/a · tokens n/a
  - `nightly.yml` job `advisories` → validation n/a · instrumentation n/a · PII n/a · tests CI (dispatch witness owed) · a11y n/a · tokens n/a
  - `scripts/deny-probes.sh` → validation n/a · instrumentation n/a · PII n/a · tests self-proving (13 negatives + control) · a11y n/a · tokens n/a

## Deviations from intent
- **Plan gate texts corrected after implement (operator-directed):** gate "sole-root tokio ban" `run` changed from `… check bans -c deny-sync.toml` to `cargo deny --config deny-sync.toml --manifest-path … check bans` (cargo-deny 0.20.2 dropped `check -c`; P3/P5 had run 0.19.4); the mutants gate lost `artifact = 'mutants.out/'` (a `no-rust-delta` run never refreshes it, so the freshness check was STALE by construction while every verdict atom held). Step 5/7 prose and one acceptance line updated to the same flag form. Justification: both entries could not pass as written; the implementation met their intent (corrected form run by hand: exit 0, `bans ok`).
- **Pedantic zizmor findings left open:** default-persona zizmor reports no findings "(2 suppressed)"; `--persona=pedantic` shows them as `help[concurrency-limits]` (low) on `ci.yml` and `nightly.yml`. Suppressed by zizmor's default persona, not by any config; left as-is because a `concurrency:` block changes run-cancellation behaviour the plan did not scope.
- None else — steps 1-10 executed in order; no Rust delta.

## Decisions & corrections
- P4 operator decision: the weekly advisory run lives in `nightly.yml` (test-plan owns CI layout), not a `schedule:` trigger on `ci.yml`.
- P4 operator decision: ban probes are PERMANENT in CI ("a ban that cannot be seen failing is a vacuous gate").
- Operator directive after implement: apply both plan gate edits and re-run them (done, green).
- Sweep hazard: a grep for `github.event` inside `run:` must use `grep -H` — over a one-file glob grep drops the filename prefix and a `^[^:]+:[0-9]+:` exemption never matches (P4 control).
- Hazard: cargo-deny exit 2 is ALSO its usage-error exit — a probe that accepts "non-zero" without the named diagnostic passes vacuously; the probes key on the diagnostic text (this held: the `-c` usage error read 0/13, not 13/13).
- Hazard: cargo-deny `--exclude` does not prune a feature-unified optional dependency (measured) — never scope a crate-set ban with it.
- Tool fact: cargo-deny 0.20 `--config` is global (before the subcommand); `check -c` is gone.

## Outcome
Acceptance criteria re-asserted against the diff:
- (security) `cargo deny check` exits 0, four families; `deny.toml` carries the five settings and exactly one ignore — **met** (gate `cargo deny check` green; `deny.toml` as listed above).
- (arch) tokio ban per sync crate as sole root, probe tokio fails with `error[banned]` — **met** (sole-root gate green; probe `banned  tokio`).
- (arch) CI job 3 `cargo check` of the listed sync crates passes on all 3 OSes — **met locally on Windows** (gate green); macOS/Linux legs **owed** to the CI read after the push.
- (obs/security) 13 bans proven live — **met** locally (`deny-probes: 13/13 banned, control clean`); in CI **owed** to the push.
- (tests/security) zizmor exits 0 over both workflows, no silencing config; CI JSON upload — **met** locally; CI **owed**.
- (security) least-privilege shape on every workflow, no `github.event` in a `run:` body — **met** (zizmor green; grep probe exit 1, no output).
- (security/tests) nightly.yml weekly schedule + one dispatched run `success` — **owed** (operator dispatch after push).
- (tests) CI on the pushed sha all `success`; mutants `no-rust-delta` — mutants **met** locally; CI **owed**.

Gates (implement run 2026-09-24T09-34-05, final state after the operator-directed plan edits):
- `cargo deny --version` — green (`contains cargo-deny 0.20.2`)
- `zizmor --version` — green (`contains zizmor 1.30.1`)
- `cargo deny check` — green (exit 0; warnings only: `duplicate` syn, `advisory-not-detected` RUSTSEC-2017-0008)
- `test -s scripts/sync-crates.txt && while read -r c; do cargo deny --config deny-sync.toml …` — green (re-run after the plan edit)
- `test -s scripts/sync-crates.txt && cargo check $(sed …)` — green
- `bash scripts/deny-probes.sh` — green (`last line deny-probes: 13/13 banned, control clean`)
- `zizmor .github/workflows/` — green
- `grep -HnE '\$\{\{\s*github\.event' …` — green (exit 1, no output)
- `bash scripts/agent-run.sh run --unit` — not run — defer (key): zero .rs delta; `gate.py delta --defer-check rust` → 0 changed files read by a rust source (deferral stands)
- `AGENT_RUN_CHUNK_BASE=61f9676… bash scripts/agent-run.sh run --mutants` — green (re-run after the plan edit; `"verdict":"no-rust-delta"`)
- `git diff --quiet && git diff --cached --quiet && git push …` — leg operator: this wrap's P7 push discharges it
- `gh api …/check-runs …` — leg operator: owed after the push
- `gh workflow run nightly.yml …` — leg operator: owed (operator)
- `gh run list --workflow nightly.yml …` — leg operator: owed after the dispatch
- Smoke: skipped — no boot-path or UI-surface change.

Outcome basis: implement's P4 report in this conversation + the operator directive after it (the two plan gate edits, re-run green).

Process hygiene: this chunk's runs started cargo-deny, zizmor, cargo check and the harness `run --mutants` — all exited within their gate entries (no survivors). Host re-measured at implement P4 (`tasklist`): the only `viola.exe` processes (12172, 58624) are the operator's viola-lab prototype (`D:\dev\projects\additional\viola-lab\prototype\target\debug\viola.exe`), not started by this chunk.
