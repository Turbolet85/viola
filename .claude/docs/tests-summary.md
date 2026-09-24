# Tests Summary — viola

_Distilled from `.andromeda/test-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Test tier

**Tier:** Comprehensive (2)

**Justification (one sentence):** nine security vectors needing negative tests, seven parser surfaces handed over for property/fuzz coverage, eight agent-driven surfaces across three required OSes and seven cross-surface critical paths — plus founder-mandated mutation testing from chunk 1, crash-safe-state chaos, hook-deadline perf budgets and multi-version CLI compat.

## Harness contract (§3)

The harness is the agent-driven verification surface. `scripts/agent-run.{sh,ps1}` are thin shims over `cargo run -q -p viola-e2e --bin viola-harness -- <command>`; all logic lives in Rust. See `.claude/rules/verification-harness.md` for path-scoped enforcement.

- **Test runner:** cargo-nextest 0.9.146 (process-per-test, JUnit, `retries = 0`) + `cargo test --doc`; Playwright 1.63.0 headless Chromium (ubuntu) for the page
- **5-command discipline:** boot / run / status / cleanup / logs — each prints one JSON document `{"v":1,"cmd":…,"ok":…}`, exit 0/1/2
- **Status:** `agent-run status` aggregates `viola list --json`, `GET /ready` and cookie-gated `/api/sessions` (`state: ready|degraded|down`, `api_sessions_equal_list`)
- **PID file:** none as a product file — pids live in `instances/<name>/snapshot.json`; the harness record is `target/agent-run/<session>/session.json`
- **Log format:** JSON-per-line — `events.ndjson` (arch event line) + process logs in `<home>/diagnostics/*.ndjson`; required fields `timestamp level target message event process instance corr` (bound to obs-plan §3)
- **Tempdir convention:** a not-yet-existing home under `target/e2e-home/viola-session-*/home` (viola creates it); kept in CI (`AGENT_RUN_KEEP_HOMES=1`) until obs gates and the secret scan have read it
- **Fake agent:** `viola-fake-agent` (root `[[bin]]`, feature `fake-agent`) replays `fixtures/claude/<cli-version>/` recorded by `viola verify`; CI stamps homes only by running `viola verify` against it; the real `claude` never runs in CI

## E2E coverage (§6)

- **Path 1 — `run` start sequence** — event order `wheel{start}` → `budget-gate` → `session-start`; duplicate / squatted / tampered-exe starts exit 1; plugin files rewritten with absolute pinned paths
- **Path 2 — confirmed `send` + CL-1 records** — `cursor` = pre-paste offset, `send-issued` then `prompt-submitted{driver}`; `no-prompt-submitted` → exit 13 + `send-refused`; local command → `unconfirmable`; readback `open → read`
- **Path 3 — `wait` / `last`** — wakes only on driver-relevant kinds, returns already-logged events at once, typed timeout, exit 21 on a vanished wrapper
- **Path 4 — dialog → `answer`** — insta-pinned decision bodies (S3/S7/S8), one pending dialog, `unknown-dialog`, no decision without stamp + wheel `driver`
- **Path 5 — the wheel** — human key → `human-typing` (exit 10), `pause` → `manual-pause`, `release` back, `release` with `from` → exit 20, harness turns never flip it
- **Path 6 — budget governor** — 90/85 thresholds, exit 11, per-instance `release --budget` override, statusline pass-through
- **Path 7 — unverified CLI** — transport works, dialog answers withheld (exit 12)
- **E1–E5** — unwrapped hooks no-op · R8 env strip · link/unlink markers · SSE `Last-Event-ID` resume · snapshot corruption → replay
- **Non-path suites** — security sweep, secret scan + canary, schema conformance (G4 body), exit-cause matrix, security-control negatives, `list` row integrity, cross-surface parity, bay layout states, chaos, property (7 parsers), contract

## Quality gates (§10)

| Gate | Threshold | Tool |
|---|---|---|
| Coverage (line) | ≥ 85 % per OS | cargo-llvm-cov 0.9.1 `--fail-under-lines` |
| Coverage (branch → region) | ≥ 80 % | `--fail-under-regions` |
| Coverage (function) | ≥ 95 % | `--fail-under-functions` |
| Mutation | 0 missed, 0 timeout in the chunk diff | cargo-mutants 27.1.0 `--in-diff`, verdict from `outcomes.json` |
| Flakiness budget | zero — no retries, a flake keeps the chunk red | nextest `retries = 0`, Playwright `retries: 0` |
| Performance budget | hook `max` < 1.0 s (SessionEnd; spine provisional until arch names the constant) | hyperfine 1.20.0 `--warmup 3 --runs 30`, gated on `max` |
| Per-job verdict | every required suite present, 0 failed, 0 skipped, artifacts present | `viola-harness gate --require …` |

## Universal anti-patterns

- NEVER use `sleep(N)` for synchronization, a retry budget, or `#[ignore]` / `test.skip` as a parking place.
- NEVER parse the rendered child screen; verdicts come from events, payloads, receipts and DOM attributes.
- NEVER run the real `claude` or real-CLI `viola verify` in CI; NEVER treat a green fake-agent run as proof of real-CLI behaviour.
- NEVER hand-write stamps, snapshots or `budget.json`; NEVER use `std::env::set_var`.
- NEVER write a test that auto-approves a dialog.

## Critical decisions

- `crates/viola-e2e` (test-only, `publish = false`) hosts `viola-harness` and the Tokio clients, keeping the sync root package tokio-free.
- Stamps conflict resolved: `viola verify` against the fake agent is the only writer of `ledger/stamps.json` in tests and CI.
- Branch coverage is enforced as LLVM region coverage; mutation testing covers branch strength.
- hyperfine is the perf gate (criterion does not exit non-zero on regression).
- Arch requests pending: name the send-confirmation window and spine-hook deadline constants; `viola ui --port 0`.

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md`, `.claude/rules/verification-harness.md`.
