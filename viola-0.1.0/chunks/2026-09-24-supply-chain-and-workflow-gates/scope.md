# Scope — Supply-chain and workflow gates

**Marker:** `2026-09-24-supply-chain-and-workflow-gates` · **Version:** viola-0.1.0 · **Epoch:** Epoch 1 — Foundation

## Working-route entry (verbatim)
> Supply-chain and workflow gates — cargo-deny families with telemetry-crate and redaction-toggle bans, weekly advisory run, zizmor, tokio-free sync-crate check, least-privilege permissions

No annotations (`CARRY:` / `PREREQ:` / `BLOCKED-ON:` / `CONTEXT:`) stand on the entry. CI verdict for the last shipped sha
(`61f9676882bbc06116ff9b47fc5c7328e8de1373`): all four check-runs `success` (3 test legs + `mutants`) — nothing to fold.

## What this chunk builds
1. **`deny.toml` at the repo root, covering all four cargo-deny check families** (advisories · bans · licenses · sources),
   so `cargo deny check` stops failing for want of a config (handoff note: it has no `deny.toml` yet).
   - **advisories:** `unmaintained = "all"`, `unsound = "all"`, `yanked = "deny"`; the one expected ignore is
     RUSTSEC-2017-0008 (`serial` via portable-pty `=0.8.1`) with a `reason`.
   - **sources:** crates.io only — `unknown-registry = "deny"`, `unknown-git = "deny"`.
   - **licenses:** an explicit allow-list covering the current graph.
   - **bans — C-building crates:** `cc`, `libsqlite3-sys`, `openssl-sys` (and the class they stand for).
   - **bans — telemetry crates:** `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender` denied
     workspace-wide (obs-plan otel-sdk-install: no reporter / egress in v1).
   - **bans — feature bans (`[[bans.features]]`):** the redaction toggle `veil` → deny `toggle`; `rmcp` → deny
     `transport-streamable-http-server`, `auth`; `axum` → deny `http2`; `tracing-subscriber` → deny `env-filter`
     (added at P3, obs-plan §9 / §11 Logs / D-15).
   - **bans — tokio in the sync crates' graph:** `tokio` anywhere (direct or transitive) in the normal-dependency graph
     of `viola-core`, `viola-pty`, `viola-channel` (without its feature), `viola-state`, `viola-agent-claude`,
     evaluated for the Windows, macOS and Linux target triples, with `viola`, `viola-mcp`, `viola-ui` (and the
     test-only `viola-e2e`) excluded from that graph — never a direct-parent allowlist (arch §Build system).
2. **Tokio-free sync-crate check** — `cargo check` of the sync crates without `viola-channel`'s `tokio` feature, per OS
   (arch CI job 3), proving they compile on each OS without Tokio; the ban itself is enforced by the deny gate.
3. **CI wiring in `.github/workflows/ci.yml`** — a `cargo deny check` step over all four families on ubuntu (arch CI
   job 4) that fails on any advisory, yanked crate, unknown source, banned crate/feature or licence; a `zizmor`
   step over `.github/workflows/` on ubuntu that fails on unpinned actions, `excessive-permissions`, template
   injection and cache poisoning; the tokio-free `cargo check` per OS.
4. **Weekly advisory run** — a `schedule:` weekly cron running `cargo deny check advisories` (the advisory DB moves
   without code changes).
5. **Least-privilege workflow permissions** — `permissions: {}` at every workflow's top, `contents: read` per job,
   every `uses:` pinned by full commit SHA with a version comment, `persist-credentials: false` on checkout, and
   event-payload values reaching steps only through `env:` — held across every workflow this chunk adds or edits,
   with zizmor as the assertion.

## Boundaries (not this chunk)
- fmt / clippy fatal lint, MSRV 1.96 job, coverage floors, fuzz/property replay, per-job gate verdict → *Quality gates*.
- `cargo build --release` job, release graph free of test binaries, `cargo tree` rmcp-feature assertion on the release
  graph, cargo-modules boundary review → *Workspace tree and code-graph planes* / *Quality gates* (as the route places them).
- print / raw-log lint bans, `disallowed-macros`, canary secret scan → *Observability gates*.
- Adding any product dependency (portable-pty, rmcp, axum, veil, interprocess, notify…) — those land with their
  owning crates' chunks; this chunk writes the policy they will be checked against.
- A release workflow, cargo-auditable, cargo-audit (deferred to v1.x).

## Surfaces / contracts touched
- New: `deny.toml` (arch §Infrastructure Patterns tree lists it at the root).
- Modified: `.github/workflows/ci.yml`; possibly a new scheduled workflow file.
- Possibly: `scripts/agent-run.{sh,ps1}` if the dependency-policy gate is exposed through the harness, and the
  `.claude/docs/commands.md` gate list.
- Specs read: architecture §Build system / §CI/CD approach; security-plan §Dependencies (deny.toml additions, CI
  integration, pinning, CVE SLA); obs-plan otel-sdk-install / pii-scrubbing-wire (veil toggle ban) / §9; test-plan
  §CI pipeline structure and the dependency-policy entity.

## Premises (closed at P3 — research.md §Measured facts)
- Only `viola-core` (sync) and `viola-e2e` (test-only) exist today; `viola-pty`, `viola-channel`,
  `viola-state`, `viola-agent-claude`, `viola-mcp`, `viola-ui` do not — so the tokio ban and the sync-crate
  `cargo check` cover the crates present now and are shaped so each later crate chunk joins by a one-line
  addition, without a vacuous pass over absent packages. (VERIFIED: `ls crates`.)
- [premise-corrected: scratch-workspace probe — `--exclude` false-fails under workspace feature unification; the
  sole-root `--manifest-path` form decides the case] The tokio ban is a SEPARATE cargo-deny config (tokio ban only,
  three-triple `[graph] targets`, `exclude-dev = true`) run once per sync crate as sole root via
  `--manifest-path crates/<crate>/Cargo.toml` — not `--exclude`/`[graph] exclude` over the workspace (a crate whose
  optional feature another member enables stays unified and false-fails), not `wrappers`. Limit carried forward:
  the crate that OWNS the optional tokio feature (`viola-channel`) false-fails as its own root once `viola-mcp`
  enables that feature (`cargo metadata` reports unified features), so it is covered through the other sync roots
  that depend on it — the channel chunk's concern, recorded for wrap to carry.
- RUSTSEC-2017-0008's crate is not in today's graph; its ignore raises `warning[advisory-not-detected]`, which does
  NOT fail the gate (draft config exit 0) — the ignore stays in the policy. (VERIFIED.)
- The banned telemetry / feature-banned crates are absent from the graph and a ban on an absent crate emits no
  diagnostic, so each ban's live effect is shown only by a negative probe (probed: `env-filter` → `feature-banned`,
  `cc` → `banned`, both exit 2). (VERIFIED.)
- The weekly advisory run's home is open between a `schedule:` trigger on `ci.yml` and a scheduled `nightly.yml`
  (test-plan §9 Platform) — a P4 fork. (VERIFIED open.)
- [premise-corrected: `cargo deny --version` → 0.19.4; `zizmor` not on PATH, PyPI carries 1.30.1] cargo-deny is
  installed locally but BELOW the 0.20.2 floor, and zizmor is absent: /implement installs both at their CI versions
  before running the gates locally; the local runs are recorded with the tool versions that produced them.
- [added at P3 per obs extract: obs-plan §9 Pipeline integration, §11 Logs, D-15] The feature bans also deny
  `tracing-subscriber` → `env-filter` (levels come only from `config.json` via `filter::Targets`; no `EnvFilter` /
  `RUST_LOG`). HEAD's pinned tracing-subscriber features (`fmt,json,registry,std`) already exclude it.
